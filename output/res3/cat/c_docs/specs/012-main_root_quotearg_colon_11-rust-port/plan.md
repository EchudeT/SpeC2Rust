# Implementation Plan: main_root_quotearg_colon_11

## Summary

This module ports the `quotearg_colon` and `quotearg_colon_mem` logic from `quotearg.c` into Rust for the `cat` project branch `012-main_root_quotearg_colon_11-rust-port`.

The Rust implementation should preserve the existing quoting behavior for the colon-specific variant while keeping the scope limited to the two listed functions and only the data they directly require from the original C file. The implementation approach is to translate the relevant quoting path into safe Rust using borrowed byte slices (`&[u8]`) and owned `String`/`Vec<u8>` outputs as appropriate, avoiding any expansion into unrelated quoting modes or generalized infrastructure beyond what these functions already depend on.

The port should prioritize:
- behavior parity with the C implementation for colon quoting,
- explicit handling of byte-oriented inputs corresponding to C pointer-plus-length patterns,
- minimal and local data-model translation from the C internals needed by these functions,
- safe ownership and lifetime handling in place of C-managed buffers.

## Technical Context

### Language/Version
- Rust 1.78+
  A current stable Rust toolchain is sufficient. The implementation should rely primarily on the standard library.

### Primary Dependencies
- Rust standard library only

No third-party crates are recommended based on the provided input. The module appears to be a direct translation of existing C quoting logic and does not provide evidence requiring external dependencies.

### Testing
- `cargo test`

Testing should cover:
- direct output equivalence for `quotearg_colon`,
- length-bounded behavior for `quotearg_colon_mem`,
- edge cases involving empty input, embedded punctuation, and colon-containing inputs,
- byte-oriented inputs that are not guaranteed to be valid UTF-8 if the original C path operates on raw memory.

### Performance Goals
- Maintain linear-time processing relative to input length.
- Avoid unnecessary intermediate allocations where possible.
- Preserve practical parity with the C implementation for short and medium argument strings.
- Use preallocation when the output growth pattern is predictable enough from the quoting rules.

## Module Mapping

### C to Rust File Mapping
- `quotearg.c` -> `src/quotearg.rs`

If the project already has an existing Rust file for quoting utilities, these functions should be added there rather than creating extra modules. The mapping should remain constrained to the file that owns the quoting port.

### Function Mapping
- `quotearg_colon` -> `pub fn quotearg_colon(...) -> String` or project-local equivalent return type
- `quotearg_colon_mem` -> `pub fn quotearg_colon_mem(input: &[u8]) -> String` or project-local equivalent byte-aware API

Preferred Rust-facing shape:
- `quotearg_colon`: accepts a string-like argument corresponding to NUL-terminated C input
- `quotearg_colon_mem`: accepts `&[u8]` corresponding to pointer + explicit length in C

If the surrounding port already uses byte-returning helpers for quoting, the implementation may return `Vec<u8>` internally and convert at the public boundary only when valid and already consistent with the rest of the port. The plan should follow existing project conventions rather than introducing a new abstraction layer.

## Data Model

The analysis lists only anonymous C data structures from `quotearg.c`. Since these two functions are part of a larger quoting subsystem, the Rust port should not recreate every anonymous C structure wholesale. Instead, only the data actually referenced by `quotearg_colon` and `quotearg_colon_mem` should be migrated.

### Data-Structure Mapping Strategy
- C anonymous structs used only as internal option/grouping records
  - -> private Rust `struct` with named fields if the fields are needed by the colon quoting path
- C anonymous constant tables or flag groupings
  - -> private Rust `const`, `static`, or small private `enum`/bitflag-like representation
- C pointer-based string/memory descriptors
  - -> `&str` for text inputs when NUL-terminated string semantics are intended
  - -> `&[u8]` for explicit-length memory inputs
- C mutable output buffers
  - -> `String` or `Vec<u8>` depending on whether the quoting logic is naturally text-only or byte-oriented in the existing port

### Expected Rust Types
Because the concrete C struct names are not provided, use a minimal translation approach:
- internal quoting options record, if required
  - `struct QuotingOptions { ... }`
- internal character classification / flags, if required
  - `enum` or integer flags with private constants
- internal output builder state
  - direct use of `String` or `Vec<u8>` rather than a dedicated struct unless already present in the Rust port

### Memory Management Notes
- Replace C-owned temporary buffers with Rust-owned outputs.
- Avoid exposing raw pointers.
- Model explicit-length inputs with slices to prevent overread risks.
- Keep helper functions private unless they correspond directly to a migrated public C function.

### Error Handling Notes
The original C functions likely produce quoted strings rather than fallible I/O results. The Rust port should therefore avoid introducing `Result` return types unless invalid state is unavoidable due to an existing project API. For byte-oriented quoting:
- operate on raw bytes when possible,
- only require UTF-8 conversion at a boundary that already assumes textual output,
- if textual output is required for arbitrary bytes, encode using the same escaped representation as the C behavior instead of failing.

## Implementation Phases

## Phase 1: Isolate Required Quoting Path

### Goal
Identify and extract only the logic in `quotearg.c` that is directly needed by `quotearg_colon` and `quotearg_colon_mem`.

### Tasks
- Inspect the C implementation of `quotearg_colon` and `quotearg_colon_mem`.
- Determine whether both functions are thin wrappers over a shared internal quoting routine.
- Identify which anonymous structs, constants, tables, and helper functions are actually referenced by this path.
- Map C signatures to Rust signatures using:
  - `&str` or `&CStr`-equivalent project convention for NUL-terminated string input,
  - `&[u8]` for explicit-length memory input.
- Record any assumptions about colon escaping, surrounding quote style, and byte handling needed for parity.

### Deliverable
- A narrowed migration list for the quoting path, limited to the two functions and their direct dependencies in `src/quotearg.rs`.

## Phase 2: Port Core Logic and Required Data

### Goal
Translate the colon-specific quoting implementation into Rust with minimal supporting data structures.

### Tasks
- Create or extend `src/quotearg.rs`.
- Port any required internal option structure or constants into private Rust definitions.
- Implement the shared quoting helper used by the colon variants, if the C code uses one.
- Implement `quotearg_colon_mem` first as the byte-oriented base function.
- Implement `quotearg_colon` as the wrapper over the memory-based form, following the C call structure.
- Replace C buffer writes and manual termination with Rust `String` or `Vec<u8>` accumulation.
- Preserve character escaping and colon treatment exactly as required by the C logic.

### Deliverable
- Compiling Rust implementation of the two functions with only the required internal support code.

## Phase 3: Validate Behavioral Parity

### Goal
Confirm that the Rust port matches the original C behavior for the colon quoting path.

### Tasks
- Add unit tests in the owning module or `tests/` according to the existing project layout.
- Cover:
  - empty input,
  - input without colon,
  - input containing one or more colons,
  - inputs containing characters that trigger quoting/escaping in the original path,
  - explicit-length inputs for `quotearg_colon_mem`,
  - non-UTF-8 byte cases if the C function accepts arbitrary memory.
- Compare outputs against known expected strings derived from the C behavior.
- Refine allocation and byte-handling details only if needed to restore parity.

### Deliverable
- Passing `cargo test` coverage for the migrated functions.

## Phase 4: Integration Cleanup

### Goal
Finalize the port in the branch without expanding module scope.

### Tasks
- Align naming, visibility, and file placement with the rest of the Rust port.
- Remove or avoid unused translated helpers not required by these two functions.
- Ensure there are no lingering C-style assumptions such as sentinel termination in internal Rust APIs.
- Confirm the implementation remains limited to the original module responsibility.

### Deliverable
- Clean, branch-ready Rust module focused strictly on `quotearg_colon` and `quotearg_colon_mem`.