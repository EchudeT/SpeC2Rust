# Implementation Plan: module_src_parseopt_parseopt_01

## Summary

This module cluster covers command-line option state mutation and user-facing output for usage, help, and version text. The Rust port should preserve the existing behavior and migration boundaries of:

- `src/parseopt/help.c`
- `src/parseopt/optset.c`
- `src/parseopt/parseopt.c`

The implementation approach is a direct C-to-Rust migration centered on:

- translating option-setting functions into safe Rust functions operating on explicit mutable state,
- replacing C string and file-descriptor handling with standard-library string slices, owned `String`, and `std::io::Write`,
- keeping output formatting logic close to the original file/function split,
- representing nullable and mutable C fields with `Option<T>` and `&mut` references rather than introducing new abstractions beyond what is needed for the port.

The Rust module should remain narrowly scoped to the listed functions and their associated state. The plan avoids broad parser redesign and instead ports the existing logic in place with idiomatic ownership and error propagation.

## Technical Context

### Language/Version

- Rust 1.78+
  This is a reasonable current stable baseline for a straightforward standard-library port.

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:

- No third-party crates required for the initial port.

Standard library areas expected to be used:

- `std::io::{self, Write}`
- `std::fmt`
- `std::borrow::Cow` only if needed to minimize string copying during migration
- `std::string::String`
- `std::vec::Vec`

### Testing

- `cargo test`

Testing scope:

- unit tests for each option-setting function,
- output-oriented tests for usage/help/version writers using in-memory buffers,
- focused behavior tests for dash-option classification in `option_dash`.

### Performance Goals

Performance should be at least comparable to the original C implementation for normal command-line parsing and output generation workloads.

Specific goals:

- avoid unnecessary heap allocation when writing static usage/help/version text,
- allocate only where the original C behavior requires string copying or ownership transfer,
- keep option state mutation O(1),
- maintain linear behavior for formatting/output proportional to emitted text size.

## Module Mapping

Map the C files into a single Rust module subtree that preserves the original responsibility split without adding unrelated layers.

### C to Rust file mapping

- `src/parseopt/help.c`
  - Rust target: `src/parseopt/help.rs`
  - Functions:
    - `set_usage_var`
    - `init_usage_vars`
    - `parseopt_usage_std`
    - `parseopt_usage_sdash`
    - `parseopt_usage_fd`
    - `parseopt_help_fd`
    - `parseopt_version_fd`

- `src/parseopt/optset.c`
  - Rust target: `src/parseopt/optset.rs`
    - `optset_incr`
    - `optset_string_copy`
    - `optset_string`
    - `optset_string_alloc`
    - `optset_true`
    - `optset_false`
    - `optset_bool`

- `src/parseopt/parseopt.c`
  - Rust target: `src/parseopt/parseopt.rs`
    - `option_dash`

### Rust module layout

Use a restrained structure:

```text
src/
  parseopt/
    mod.rs
    help.rs
    optset.rs
    parseopt.rs
```

Suggested exports in `src/parseopt/mod.rs`:

- re-export only the migrated functions and the minimum shared state types needed by sibling files,
- keep internal helper types crate-private unless they are already used elsewhere in the existing Rust port.

### Responsibility boundaries

- `help.rs`: usage/help/version formatting and shared usage-variable initialization.
- `optset.rs`: mutation helpers for option target storage.
- `parseopt.rs`: low-level option token inspection logic such as `option_dash`.

This keeps migration aligned with the existing C source separation and avoids introducing a new parser façade.

## Data Model

The C analysis reports anonymous structures only, so the Rust data model should be derived from actual field usage in these files rather than inventing new generic models. The mapping below is intentionally minimal and should be finalized from the concrete C declarations during implementation.

### Data-structure mapping strategy

| C pattern | Rust mapping | Notes |
|---|---|---|
| anonymous struct used only within one C file | private Rust `struct` in the corresponding `.rs` file | Keep visibility narrow. |
| anonymous struct shared across the three files | `pub(crate)` Rust `struct` in `src/parseopt/mod.rs` or the owning file | Define only fields actually referenced by migrated functions. |
| `char *` borrowed string | `&str` or `Option<&str>` at call boundaries | Use borrowed text when ownership is not transferred. |
| `char *` stored mutable field | `Option<String>` or `String` | Choose `Option<String>` if null is meaningful in C. |
| `const char *` static text | `&'static str` | Appropriate for usage/help/version labels. |
| integer counters/flags | `i32`, `u32`, `usize`, or `bool` | Match semantic use, not C surface type blindly. |
| output file descriptor | generic `W: Write` or `&mut dyn Write` | Replaces fd-based printing with writer-based emission. |
| function pointer option setter | Rust function item or small enum-driven dispatch | Only if directly required by current file interactions. |

### Expected Rust state types

The following are the likely Rust equivalents needed for these files.

#### Usage/help state

Create a dedicated struct for usage-related variables if the C code currently groups multiple mutable fields for formatting:

```rust
pub(crate) struct UsageVars {
    // program name, usage line fragments, option summary fragments, etc.
}
```

Use this struct only if the C code maintains grouped mutable usage state. If the C code instead passes independent pointers/values, keep the Rust functions parameter-based rather than forcing aggregation.

#### Option target state

The option-set functions imply mutable destinations for several target categories:

- integer increment target,
- string target with copy/borrow/allocated semantics,
- boolean target.

These should become explicit mutable references or small wrapper structs, depending on how the C code currently couples them.

Possible direct mappings:

- `int *` increment target -> `&mut i32` or `&mut usize`
- `char **` string target -> `&mut Option<String>` for owning cases
- borrowed string assignment target -> `&mut Option<String>` if the Rust port normalizes to ownership
- boolean flag target -> `&mut bool`

Prefer direct mutable references in function signatures over a more abstract setter object unless the C code already uses a shared option descriptor that requires a common field shape.

### Function-level memory management decisions

#### `optset_string_copy`

- C behavior likely duplicates incoming string content.
- Rust mapping: assign `Some(input.to_owned())` into the destination.
- No manual free path is required; prior contents are dropped automatically.

#### `optset_string`

- If the original function stores a non-owning pointer, Rust cannot safely mirror that without lifetimes spreading through the parser.
- For a restrained and safe port, normalize this to owned storage unless an existing surrounding Rust API already enforces borrowed lifetimes.
- Preferred mapping: `&mut Option<String>` plus `to_owned()`.

#### `optset_string_alloc`

- If distinct from `optset_string_copy` only by allocation provenance in C, collapse implementation onto the same safe owned-string behavior while retaining a separate function for behavioral parity and call-site stability.

#### `optset_true` / `optset_false` / `optset_bool`

- Represent flags with `bool`.
- If tri-state/null behavior exists in C, use `Option<bool>` only where required by observed field usage.

### Error handling model

C functions that write to file descriptors or rely on allocation failure should map to Rust results:

- output functions:
  - `parseopt_usage_fd`
  - `parseopt_help_fd`
  - `parseopt_version_fd`
  - return `io::Result<()>`

- pure state mutation helpers:
  - return `()`, unless the original behavior can fail for reasons still meaningful after the Rust conversion.

- `option_dash`:
  - return a simple classification value such as `bool`, small enum, or integer equivalent matching existing call-site expectations.

Do not emulate C null checks by allowing invalid states where Rust types can prevent them.

## Implementation Phases

## Phase 1: Establish module skeleton and shared types

### Goal

Create the Rust file layout and define the minimum shared state required by the migrated functions.

### Tasks

- Add:
  - `src/parseopt/mod.rs`
  - `src/parseopt/help.rs`
  - `src/parseopt/optset.rs`
  - `src/parseopt/parseopt.rs`
- Inspect the C declarations referenced by the listed functions and identify:
  - shared anonymous structs,
  - field types,
  - nullability expectations,
  - cross-file dependencies.
- Convert any shared anonymous C structs into named Rust structs with:
  - only the fields needed by these files,
  - `pub(crate)` visibility only when cross-file access is necessary.
- Decide the exact integer types for counters and indices based on C field usage.
- Define minimal function signatures for all listed functions, even if some bodies initially contain placeholders.

### Deliverables

- Compiling Rust module skeleton.
- Named Rust replacements for all anonymous structures actually touched by the migrated functions.
- Agreed signature set for output, option-setting, and dash-classification functions.

### Notes

Do not broaden the type system beyond what is required for these files. If a struct is file-local in practice, keep it local.

## Phase 2: Port option state mutation logic from `optset.c`

### Goal

Migrate the option target update helpers first, since they are small, state-focused, and useful as building blocks for later parsing work.

### Tasks

- Implement `optset_incr`
  - increment the target counter with matching semantics.
- Implement `optset_true`
  - set target to `true`.
- Implement `optset_false`
  - set target to `false`.
- Implement `optset_bool`
  - set target according to passed value or descriptor semantics.
- Implement `optset_string_copy`
  - copy incoming text into owned Rust storage.
- Implement `optset_string`
  - preserve call-site behavior while using safe owned storage where needed.
- Implement `optset_string_alloc`
  - map allocation-based C behavior onto normal Rust ownership without separate manual allocation handling.

### Validation

Add unit tests for:

- repeated increment behavior,
- overwriting existing string targets,
- empty string handling,
- toggling booleans,
- replacement of prior owned values without leaks or invalid references.

### Memory and correctness focus

- Replace all manual allocation/free patterns with ownership through `String` and `Option<String>`.
- Ensure behavior remains deterministic when setters are called multiple times on the same target.

## Phase 3: Port output and usage/help/version logic from `help.c`

### Goal

Migrate formatting and output code while replacing file-descriptor writes with `Write`-based APIs.

### Tasks

- Implement `set_usage_var`
- Implement `init_usage_vars`
- Implement `parseopt_usage_std`
- Implement `parseopt_usage_sdash`
- Implement `parseopt_usage_fd`
- Implement `parseopt_help_fd`
- Implement `parseopt_version_fd`

### Technical decisions

- Replace raw fd output with:
  - `pub(crate) fn ...<W: Write>(writer: &mut W, ...) -> io::Result<()>`
  - or `&mut dyn Write` where simpler.
- Preserve the original output order, spacing, and newline behavior as closely as possible.
- Use `write!`/`writeln!` for formatted emission.
- Keep any static text as `&'static str`; only allocate `String` where intermediate composition is unavoidable.

### Validation

Add tests using `Vec<u8>` or `Cursor<Vec<u8>>` for:

- standard usage output,
- single-dash usage output,
- help output,
- version output,
- initialization of usage variables,
- exact newline/spacing behavior where practical.

### Memory and error-handling focus

- Propagate `io::Result<()>` directly from writers.
- Remove any C-style partial-write bookkeeping unless it is explicitly present in the original logic and required by caller behavior.

## Phase 4: Port dash-option classification from `parseopt.c` and integrate module behavior

### Goal

Migrate the remaining low-level parsing helper and ensure all three translated files work together with stable interfaces.

### Tasks

- Implement `option_dash`
  - classify option strings according to the original C semantics.
- Confirm call-site expectations for:
  - empty strings,
  - `-`,
  - `--`,
  - normal short-option forms,
  - non-option arguments.
- Adjust signatures minimally if integration with the migrated setter/output code requires shared enums or constants.
- Finalize exports in `src/parseopt/mod.rs`.

### Validation

Add tests for `option_dash` covering the exact edge cases used by the C implementation.

Perform integration-level checks that:

- usage/help/version functions compile and run against the finalized shared types,
- option-setting helpers operate correctly with the parser state types used by `option_dash` call sites.

### Completion criteria

- All listed functions are implemented in Rust.
- `cargo test` passes.
- No unsafe code is introduced unless a specific existing project interface makes it unavoidable; if unavoidable, isolate it and document the exact reason in code comments.