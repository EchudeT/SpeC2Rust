# Implementation Plan: module_src_parseopt_parseopt_01

## Summary

This module ports the C parse-option support cluster into Rust, covering option value setters, usage/help/version output, and single-dash option recognition. The Rust implementation should preserve the existing behavioral surface of the C files `src/parseopt/help.c`, `src/parseopt/optset.c`, and `src/parseopt/parseopt.c` without expanding scope.

The implementation approach is to consolidate the migrated logic into a focused Rust module under the existing crate layout, using standard-library types for owned strings, booleans, counters, and output streams. C patterns based on mutable pointers, string duplication, and file-descriptor writes should be translated into explicit mutable references, `String`/`Option<String>`, and `std::io::Write`-based output. Any C global or static usage text state should become a small internal Rust state container with controlled initialization and mutation.

The migration should proceed by first defining the Rust data model for options and usage metadata, then porting the setter functions from `optset.c`, then the help/usage/version emitters from `help.c`, and finally the option classification logic from `parseopt.c`. The resulting code should favor direct behavioral equivalence over architectural refactoring.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required based on the provided module scope
- **Testing**:
  - `cargo test`
  - Unit tests for setters, usage formatting, and dash-option detection
- **Performance Goals**:
  - Match the C module’s expected lightweight CLI-processing cost
  - Avoid unnecessary allocations except where the C code duplicates strings
  - Keep output generation linear in the size of formatted help/usage text
  - Preserve simple in-place mutation for option target values

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/help.c`
  - Port to: `src/parseopt.rs` or `src/parseopt/mod.rs`
  - Rust responsibilities:
    - usage state initialization
    - usage text mutation
    - usage/help/version output helpers

- `src/parseopt/optset.c`
    - counter increment setter
    - string assignment/copy/allocation setters
    - boolean setters

- `src/parseopt/parseopt.c`
    - single-dash option classification helper

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `set_usage_var` | `fn set_usage_var(...)` | Internal mutable update of usage-related state |
| `init_usage_vars` | `fn init_usage_vars(...)` | Initialize per-module or per-parser usage state |
| `parseopt_usage_std` | `fn parseopt_usage_std(...)` | Emit standard usage text to a writer |
| `parseopt_usage_sdash` | `fn parseopt_usage_sdash(...)` | Emit variant usage text for short-dash mode |
| `parseopt_usage_fd` | `fn parseopt_usage_fd<W: Write>(...)` | Replace fd-oriented output with `Write` |
| `parseopt_help_fd` | `fn parseopt_help_fd<W: Write>(...)` | Help emission through `Write` |
| `parseopt_version_fd` | `fn parseopt_version_fd<W: Write>(...)` | Version emission through `Write` |
| `optset_incr` | `fn optset_incr(...)` | Increment integer target |
| `optset_string_copy` | `fn optset_string_copy(...)` | Copy into existing mutable string storage |
| `optset_string` | `fn optset_string(...)` | Assign string value semantics directly |
| `optset_string_alloc` | `fn optset_string_alloc(...)` | Allocate owned string equivalent of C duplication |
| `optset_true` | `fn optset_true(...)` | Set bool target to true |
| `optset_false` | `fn optset_false(...)` | Set bool target to false |
| `optset_bool` | `fn optset_bool(...)` | General bool assignment |
| `option_dash` | `fn option_dash(...)` | Detect/classify dash-prefixed option form |

## Data Model

The C analysis only exposes anonymous structures, so the Rust plan should derive types from usage patterns in the functions rather than inventing unrelated abstractions. The goal is to introduce only the minimal named Rust types needed to hold existing state safely.

### Data-Structure Mapping

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| anonymous usage/help state struct | `struct UsageState` | Holds strings and flags used by usage/help/version functions |
| anonymous option target/value holder | `enum OptTarget<'a>` or focused field references | Use only if needed to preserve shared setter dispatch behavior |
| anonymous parser/option descriptor struct | `struct ParseOption` | Minimal representation of option metadata used by formatting and dash detection |
| anonymous array/table of option descriptors | `&[ParseOption]` | Replace C pointer/length pairs where possible |
| `char *` mutable string target | `Option<String>` or `String` | `Option<String>` when absence is meaningful |
| string copied into owned storage | `String` | Direct ownership instead of manual allocation/free |
| integer counter target | `usize`, `u32`, or `i32` | Final choice should follow call-site semantics already present in the crate |
| boolean flag target | `bool` | Direct replacement |
| file descriptor output target | `impl std::io::Write` | Safer than raw fd while preserving streamed output |

### Ownership and Mutability Decisions

- C setter functions that mutate caller-owned storage should become functions taking `&mut` references.
- C string-allocation behavior should map to replacing target storage with a newly owned `String`.
- If the original code distinguishes between borrowed and owned string assignment, Rust should preserve this distinction only as far as required by current call sites:
  - direct assignment path: prefer `&str` to `String` conversion at boundary
  - allocation path: explicitly store an owned `String`
- Any C global mutable usage variables should be collapsed into a passed mutable state object unless crate structure proves that static process-wide state is required.

### Error Handling Mapping

- C write failures should map to `std::io::Result<()>`.
- Pure setters that cannot fail should return `()`.
- If any string-copy operation in C could fail only because of allocation, Rust should rely on standard allocation behavior and avoid introducing synthetic error enums unless required by existing crate APIs.
- Boolean/dash classification helpers should return plain values (`bool`, enum, or small integer equivalent) matching actual call-site needs.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and State Types

- Create the Rust destination module for this cluster using the project’s existing parseopt area, keeping all migrated logic together.
- Define minimal Rust structs/enums required to represent:
  - usage/help/version state
  - option metadata referenced by formatter/parser helpers
  - mutable option target categories only if existing call sites require shared dispatch
- Replace C implicit memory layout assumptions with explicit Rust fields and references.
- Decide final scalar types for counters and flags by inspecting the current Rust port branch call sites and matching their expectations exactly.
- Add initial unit-test scaffolding for:
  - usage-state initialization
  - option-dash classification inputs
  - setter side effects on mutable targets

## Phase 2: Port `optset.c` Setter Functions

- Implement:
  - `optset_incr`
  - `optset_string_copy`
  - `optset_string`
  - `optset_string_alloc`
  - `optset_true`
  - `optset_false`
  - `optset_bool`
- Translate pointer-based mutation into `&mut` argument updates.
- For string setters:
  - use `String`/`Option<String>` to replace manual allocation and copying
  - preserve overwrite behavior explicitly
  - avoid introducing string interning, shared ownership, or lifetime-heavy designs unless forced by existing interfaces
- Confirm that setter behavior matches the C semantics for repeated invocation and replacement of previous values.
- Add unit tests for each setter, especially:
  - increment from zero and nonzero
  - assignment/reassignment of string targets
  - distinction between direct set and allocated/copy semantics as reflected in stored ownership
  - true/false/bool toggling

## Phase 3: Port Usage, Help, and Version Output from `help.c`

- Implement:
  - `set_usage_var`
  - `init_usage_vars`
  - `parseopt_usage_std`
  - `parseopt_usage_sdash`
  - `parseopt_usage_fd`
  - `parseopt_help_fd`
  - `parseopt_version_fd`
- Replace raw file-descriptor writes with `std::io::Write`.
- Keep formatting logic close to the C structure to reduce migration risk; do not redesign output composition beyond what Rust I/O requires.
- Convert any C static/global text variables into a local `UsageState` or parser-owned state object, using explicit initialization instead of implicit zeroed storage.
- Ensure line endings, ordering, and conditional sections remain compatible with the original behavior.
- Add tests using in-memory buffers (`Vec<u8>`) to validate:
  - standard usage output
  - short-dash usage variant
  - help output composition
  - version output text emission
  - empty/missing optional fields handling

## Phase 4: Port `option_dash` and Integrate Module Behavior

- Implement `option_dash` using Rust string/byte inspection while preserving the C logic for dash-prefixed option recognition.
- Prefer byte-level checks on `&str` where exact ASCII dash behavior matters.
- Integrate the dash helper with the option metadata/types defined earlier, but do not broaden parsing behavior beyond the original function.
- Run through the branch’s existing parseopt-related call sites and adapt signatures minimally so the module compiles cleanly with the rest of the Rust port.
- Finalize unit tests around edge cases such as:
  - empty string
  - single dash
  - double dash
  - non-dash-prefixed arguments
  - compact short-option style inputs if supported by the original code
- Complete a final `cargo test` pass and resolve any behavioral mismatches by aligning with the C implementation rather than refactoring.