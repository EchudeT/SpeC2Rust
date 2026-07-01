# Implementation Plan

## Summary

This module ports the C file `propername-lite.c` into Rust with a narrow scope: migrate the existing `proper_name_lite` behavior into an idiomatic Rust function while preserving the original call shape and output semantics as closely as possible.

The Rust implementation should stay minimal and centered on direct translation of the existing logic:
- move the logic from `propername-lite.c` into a single Rust source module,
- represent string handling with borrowed string slices where possible,
- avoid manual memory management by relying on Rust ownership and borrowing,
- express fallible paths explicitly with `Option` or `Result` only if the original C behavior requires detectable failure.

No additional facilities or abstractions should be introduced beyond what is needed to replace the original file and function.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the lightweight nature implied by the original C module.
  - Avoid unnecessary heap allocation unless required by the C logic.
  - Keep string processing linear in input size.
  - Match or improve the C implementation’s practical runtime and memory behavior for normal command-line usage.

## Module Mapping

### C to Rust File Mapping

- `propername-lite.c` → `src/propername_lite.rs`

### Function Mapping

- `proper_name_lite` → `pub(crate) fn proper_name_lite(...)`

### Integration Mapping

- Expose the migrated function from the crate module tree only as far as needed by the existing `pwd` Rust port.
- Register the module in the nearest existing `mod` declaration structure without creating extra layers.

## Data Model

No custom C data structures are identified for this module.

### Data Mapping

- C string inputs (`const char *`, `char *`) → `&str` when UTF-8 text is already guaranteed by surrounding Rust code
- C string outputs:
  - → `String` if the function constructs or returns owned text
  - → `&str` if the Rust translation can return a borrowed view of an input
- Nullable C pointers:
  - → `Option<&str>` for optional borrowed string inputs
  - → `Option<String>` for optional owned string results
- Integer status/error signaling:
  - → `bool`, `Option<T>`, or `Result<T, E>` depending strictly on the original function contract

### Memory Management Notes

- Replace any C-managed temporary buffers with stack-local variables and `String` only where mutation or owned output is necessary.
- Eliminate manual allocation/free patterns entirely.
- Keep lifetimes simple by preferring borrowed inputs and returning owned output only if unavoidable.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Port

- Create `src/propername_lite.rs`.
- Add the Rust equivalent of `proper_name_lite` with a signature derived from its actual usage in the `pwd` port.
- Wire the module into the existing crate module tree.
- Identify whether the function is purely internal or needs `pub(crate)` visibility.

### Phase 2: Logic Translation

- Translate the body of `proper_name_lite` directly from C control flow into Rust.
- Replace pointer-based string traversal with slice/string operations.
- Preserve edge-case behavior from the C implementation, especially around:
  - empty inputs,
  - null-like optional inputs if present,
  - formatting or normalization details,
  - return-value conventions.
- Keep helper logic local unless the original C file already clearly separates reusable logic.

### Phase 3: Error Handling and Ownership Tightening

- Review the translated function for places where C relied on implicit assumptions.
- Encode those assumptions explicitly with Rust types:
  - borrowed inputs where possible,
  - owned return values only where needed,
  - `Option`/`Result` only if the original logic has meaningful failure states.
- Remove any unnecessary cloning or intermediate allocations introduced during the first translation pass.

### Phase 4: Tests and Behavioral Verification

- Add unit tests in the module or adjacent test section covering the migrated behavior of `proper_name_lite`.
- Include cases for normal input, boundary conditions, and any special-case name formatting behavior present in the C code.
- Run `cargo test` and adjust the implementation until behavior is stable and consistent with the original C logic.