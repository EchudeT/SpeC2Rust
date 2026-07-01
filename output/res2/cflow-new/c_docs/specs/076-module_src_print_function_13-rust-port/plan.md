# Implementation Plan: module_src_print_function_13

## Summary

This module ports the C printing logic centered on `print_function_name` and `print_function` from `src/gnu.c` and `src/output.c` into Rust, preserving existing behavior and call structure as closely as practical. The Rust implementation should remain narrowly scoped to migration of the current printing path: formatting and emitting function-related output, handling the existing data passed through these routines, and reproducing the current control flow without introducing new abstractions beyond what is needed for safe ownership and clearer parameter typing.

The technical approach is to translate the C functions into Rust functions inside a matching module area, replacing pointer-based access and implicit global/mutable state interactions with explicit references, slices, and small Rust structs/enums where the C code currently relies on anonymous record layouts. Memory ownership should be made explicit, borrowed data should use `&str` / `&[T]` / `&mut T` where possible, and output should be expressed through standard library formatting traits or `std::io::Write` depending on whether the original C path writes to a stream-like destination.

## Technical Context

### Language/Version
- Rust stable
- Recommended minimum version: **Rust 1.76+**

### Primary Dependencies
- **Rust standard library only**
- No third-party crates are recommended from the available input, since the module scope is limited to direct migration of printing logic from existing C files.

### Testing
- `cargo test`

### Performance Goals
- Preserve the current asymptotic behavior of the C implementation.
- Avoid unnecessary heap allocation during formatting where borrowed string data or direct writer output is sufficient.
- Keep per-call overhead near the C version by:
  - preferring borrowed inputs over cloning,
  - writing directly into output sinks where the original code streams output,
  - using stack-based formatting paths where practical.

## Module Mapping

### Source File Mapping
- `src/gnu.c`
  - migrate only the logic needed for `print_function_name`
- `src/output.c`
  - migrate only the logic needed for `print_function`

### Rust Module Layout
Use standard Rust source layout without adding extra layers:

- `src/gnu.rs`
  - Rust port of `print_function_name`
- `src/output.rs`
  - Rust port of `print_function`

If these functions depend on shared record definitions already needed elsewhere in the port, place those definitions in the existing closest Rust module already representing shared program state. If no such module exists yet, add only a minimal shared definitions file such as:

- `src/types.rs`
  - only for structs/enums directly required by these two functions

### Function Mapping
- `print_function_name` (C) -> `pub(crate) fn print_function_name(...) -> ...` (Rust)
- `print_function` (C) -> `pub(crate) fn print_function(...) -> ...` (Rust)

Return types should follow existing behavior:
- use `()` when the C function only performs output and side effects,
- use `std::io::Result<()>` if the function writes to an abstract writer and C code can fail through stream operations,
- use `fmt::Result` only if the implementation is purely formatter-based.

## Data Model

The analysis reports multiple anonymous C data structures. Because the exact field sets are not listed, the Rust plan should convert them only as needed by these two functions, and only after confirming their effective field usage in `print_function_name` and `print_function`.

### Mapping Rules
- Anonymous C structs used only within one file:
  - map to private Rust `struct`s in the corresponding module.
- Anonymous C structs shared between `gnu.c` and `output.c`:
  - map to named Rust `struct`s in the nearest shared definitions module.
- Integer flag fields:
  - map to `u32`, `i32`, or `bool` based on actual semantics observed in use.
- C strings:
  - `const char *` -> `&str` when UTF-8 is guaranteed by project assumptions,
  - otherwise -> `&CStr` internally converted at formatting boundaries only if required by broader port constraints.
- Optional pointers:
  - map to `Option<&T>` / `Option<&mut T>` / `Option<Box<T>>` depending on ownership.
- Arrays plus count pairs:
  - map to slices `&[T]` / `&mut [T]` where borrowing is sufficient.
- Tagged mode/state values expressed as integer constants:
  - map to Rust `enum` when the function logic branches on a closed set of values.
- Output stream pointers (`FILE *`-style):
  - map to `&mut dyn std::io::Write` or a concrete writer type already used by the port.

### Provisional Structure Table

| C form | Rust form | Notes |
|---|---|---|
| anonymous struct used by `print_function_name` only | private `struct` in `src/gnu.rs` | Name by role, not by source anonymity |
| anonymous struct used by `print_function` only | private `struct` in `src/output.rs` | Keep fields minimal to actual usage |
| anonymous struct shared across both functions | `pub(crate) struct` in shared definitions module | Introduce only if both files need the same record |
| `char *` / `const char *` function-name field | `String` or `&str` | Prefer borrowing unless ownership transfer exists |
| pointer-linked records | references or `Option<Box<T>>` | Depends on whether ownership is local or external |
| C flag integers | `bool` or integer type | Preserve bitwise representation if bitmask logic exists |

### Memory Management
- Replace raw pointer traversal with borrowed references wherever the source function does not own the data.
- Use `Option` instead of null checks.
- Avoid copying names or formatted fragments unless the original logic requires materialized intermediate strings.
- Keep lifetimes local and explicit; do not introduce reference-counting unless existing shared ownership forces it.

### Error Handling
- Replace silent stream-state assumptions with explicit `Result` propagation if the Rust implementation writes through `Write`.
- Preserve non-I/O behavior exactly; do not introduce new validation paths beyond what is needed to represent nullability and invalid state safely.
- Internal invariant mismatches that were previously impossible in the C call graph may use `debug_assert!` during development, but the final API should reflect expected valid inputs rather than broad recovery logic.

## Implementation Phases

### Phase 1: Analyze C Usage and Define Minimal Rust Types
- Inspect `print_function_name` and `print_function` in `src/gnu.c` and `src/output.c`.
- Enumerate:
  - direct parameters,
  - accessed fields from each anonymous structure,
  - global/static state touched by these functions,
  - output destination style (`FILE *`, buffer, or custom sink),
  - return-value/error conventions.
- Create only the Rust struct/enum definitions required by these functions.
- Decide exact signatures for the Rust ports based on observed ownership and mutability.

**Exit criteria**
- Field-level mapping for every accessed C record is documented in code comments or implementation notes.
- Rust signatures are fixed for both functions.
- No speculative types are added beyond direct dependencies.

### Phase 2: Port `print_function_name` in `src/gnu.rs`
- Translate formatting and naming logic from C into Rust.
- Replace:
  - null checks with `Option`,
  - manual string handling with `&str`/`String` as appropriate,
  - C output calls with Rust formatting/write operations.
- Preserve output ordering and conditional formatting exactly.
- Add focused unit tests for:
  - normal name rendering,
  - optional/absent subordinate fields,
  - edge formatting branches visible in the original C function.

**Exit criteria**
- `print_function_name` compiles and passes unit tests.
- Output matches the expected C behavior for covered cases.
- No unnecessary allocations remain in obvious hot paths.

### Phase 3: Port `print_function` in `src/output.rs`
- Translate the higher-level function printing logic, reusing the Rust `print_function_name` implementation where the C path calls into it or shares formatting responsibilities.
- Preserve sequence of emitted fragments, delimiter behavior, and condition handling.
- Convert any mutable global dependencies into explicit arguments only where required by the existing Rust port structure; otherwise keep the narrowest possible adaptation.
- Add unit tests covering:
  - full function print path,
  - branch cases affecting whether a name or additional metadata is printed,
  - writer/error propagation if the Rust signature returns `Result`.

**Exit criteria**
- `print_function` compiles and integrates with `print_function_name`.
- Tests validate representative output assembly paths.
- Error handling behavior is explicit and minimal.

### Phase 4: Integration Cleanup and Behavioral Verification
- Reconcile any shared definitions between `src/gnu.rs` and `src/output.rs`.
- Remove temporary translation scaffolding left from the initial port.
- Ensure all call sites in the Rust branch use the final function signatures.
- Add regression-style tests at the module level for end-to-end output fragments if both functions are exercised together.

**Exit criteria**
- `cargo test` passes.
- The module uses standard-library-only implementation.
- The Rust code remains limited to the migrated functionality from the two identified C functions, with no added feature surface.