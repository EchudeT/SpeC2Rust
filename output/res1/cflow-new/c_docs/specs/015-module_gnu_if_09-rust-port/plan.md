# Implementation Plan: module_gnu_if_09

## Summary

This module migration covers the C sources `gnu/printf-parse.c` and `gnu/vasnprintf.c`, with the Rust work focused strictly on porting the existing parsing and formatted-output control flow that appears under the identified `if` sites in those files. The Rust implementation should preserve the current module boundaries and behavior, translating pointer-driven C logic into slice- and index-based Rust code and replacing manual allocation paths with owned buffers from the standard library.

The technical approach is:

- port `printf-parse.c` logic into a Rust parsing module that works over `&str` or `&[u8]` with explicit cursor/index tracking;
- port `vasnprintf.c` logic into a Rust formatting/output-building module using `String` and `Vec<u8>` as appropriate to mirror the original allocation and append behavior;
- keep data flow and function decomposition close to the original C files rather than redesigning APIs;
- convert C error signaling and allocation failure handling into `Result`-based Rust returns;
- avoid adding capabilities beyond the existing file/function behavior under migration.

## Technical Context

- **Language/Version**: Rust 1.78+
  A current stable toolchain is sufficient; no nightly features are needed.

- **Primary Dependencies**:
  - Rust standard library only

  No third-party crate is recommended from the available evidence. The source files indicate parsing and buffer construction work that can be implemented with `core`/`std` types directly.

- **Testing**:
  - `cargo test`

  Tests should be written as unit tests near the migrated modules, with focused cases covering:
  - format-string parsing branches corresponding to the migrated `if` logic;
  - output buffer growth and append behavior;
  - invalid format handling and edge cases;
  - parity-oriented regression inputs derived from the original C control flow.

- **Performance Goals**:
  - Maintain behaviorally equivalent linear scanning of format input.
  - Avoid unnecessary intermediate allocations during parsing.
  - Use amortized buffer growth via `String` or `Vec<u8>` to stay close to the allocation profile of the C implementation.
  - Preserve predictable branch structure from the original implementation rather than introducing abstraction-heavy layers.

## Module Mapping

The Rust project should keep the mapping close to the two C source files.

| C File | Rust Module/File | Migration Notes |
|---|---|---|
| `gnu/printf-parse.c` | `src/gnu/printf_parse.rs` | Port format parsing logic with explicit index-based scanning over the input. Keep helper functions local to this file/module where possible. |
| `gnu/vasnprintf.c` | `src/gnu/vasnprintf.rs` | Port formatted output assembly and buffer management. Reuse parsing results from `printf_parse` without broadening the API surface. |

Recommended crate layout:

```text
src/
  gnu/
    mod.rs
    printf_parse.rs
    vasnprintf.rs
```

Recommended module declarations:

- `src/gnu/mod.rs`
  - `pub(crate) mod printf_parse;`
  - `pub(crate) mod vasnprintf;`

The Rust module interfaces should remain internal unless another existing project module already requires public exposure. Keep function names and responsibilities as close as practical to the C originals, adjusted only for Rust naming conventions and signature safety.

## Data Model

The analysis reports only an `anonymous` data structure, so the plan should treat C aggregate state conservatively and derive Rust types from actual usage during migration.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| anonymous struct used for parser or formatting state | private `struct` in the owning Rust module | Introduce only when the C code groups related state that must travel together across functions. |
| C enum-like integer flags | `u32`/`usize` bit fields or a small private `enum` | Prefer constants plus integer storage if the C code relies on bitwise combinations; use `enum` only when values are exclusive. |
| C string pointer + length pairs | `&str`, `&[u8]`, or `(slice, usize)`-style state | Use `&str` when input is valid text and character semantics are not widened; use `&[u8]` if the original logic is byte-oriented. |
| mutable output buffer allocated manually | `String` or `Vec<u8>` | Choose `String` for text output, `Vec<u8>` if byte-exact behavior is required before final conversion. |
| sentinel/error integer returns | `Result<T, ModuleError>` or `Option<T>` | Use `Result` when the C branch distinguishes invalid input from allocation/formatting failure. |

### Memory Management

Key migration decisions:

- Replace `malloc`/`realloc`-style ownership with `String`/`Vec` growth managed by Rust.
- Eliminate raw pointer arithmetic by using indices into slices.
- Keep borrowing simple: parsing should borrow the source format string; formatting should own its output buffer.
- If the original code mutates buffers in place, model this as `&mut String` or `&mut Vec<u8>` passed through helper functions rather than shared raw memory.

### Error Handling

Introduce a small module-local error type only if the migrated functions need to distinguish categories such as:

- malformed format specification;
- unsupported state encountered in existing code paths;
- output construction failure conditions that map from C failure returns.

If the original functions only signal success/failure, use `Result<T, ()>` internally during the first pass and refine only where needed by tests. Do not generalize beyond the needs of these two files.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and port parser control flow

Scope:

- create `src/gnu/mod.rs`, `src/gnu/printf_parse.rs`, and `src/gnu/vasnprintf.rs`;
- port the control flow from `gnu/printf-parse.c` first;
- identify any anonymous C state used by the parser and define minimal private Rust structs for it;
- translate pointer walks into index-based scans over the input slice/string;
- convert parser return values into Rust `Result`/`Option` forms.

Deliverables:

- compiling parser module with internal helper functions mapped from the C file;
- unit tests for the migrated parsing branches, especially around the identified `if`-driven decisions;
- no formatting/output generation yet beyond any parser-required stubs.

## Phase 2: Port vasnprintf buffer assembly and integrate parser output

Scope:

- migrate `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`;
- connect formatting/output-building logic to the Rust parser module from Phase 1;
- replace manual allocation and resizing logic with `String` or `Vec<u8>`;
- preserve existing branch order and output construction behavior without redesigning formatting APIs.

Deliverables:

- compiling `vasnprintf` Rust module;
- internal buffer growth logic implemented with safe owned containers;
- tests covering end-to-end parse-plus-format paths represented by the original C control flow.

## Phase 3: Reconcile data mappings and edge-case behavior

Scope:

- review anonymous state and integer flag handling across both migrated files;
- tighten any provisional types introduced in earlier phases;
- normalize error propagation between parser and formatter;
- verify behavior on malformed input, empty input, and boundary cases implied by the C branches.

Deliverables:

- simplified private structs/enums/constants reflecting actual usage only;
- consistent `Result`-based internal error handling;
- expanded regression tests for edge cases and invalid formats.

## Phase 4: Final parity pass and code cleanup

Scope:

- compare Rust logic against the original C file structure to ensure no behavior-expanding changes were introduced;
- remove temporary compatibility code or unused helpers;
- align naming and visibility with Rust conventions while keeping file/function mapping recognizable;
- ensure all tests pass under `cargo test`.

Deliverables:

- final migrated module pair with restrained internal API surface;
- passing unit tests;
- implementation ready for review on branch `015-module_gnu_if_09-rust-port`.