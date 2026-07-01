# Implementation Plan

## Summary

This module ports the C source file `propername-lite.c` and its single exported function `proper_name_lite` into a Rust implementation that preserves the existing behavior and call structure without adding new capabilities.

The Rust approach should keep the port narrowly scoped:

- migrate the logic of `proper_name_lite` into a single Rust module;
- represent C string inputs using borrowed string views where possible, with explicit handling for byte content or non-UTF-8 boundaries only if required by surrounding call sites;
- replace C memory-management patterns with Rust ownership and borrowing so that temporary allocations are scoped and automatically released;
- express failure paths explicitly through Rust return types rather than implicit null-pointer or buffer-state conventions.

Because the analyzed module contains one function and no declared data structures, the implementation should remain compact and avoid introducing helper subsystems beyond what is necessary to preserve the original logic.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time behavior with respect to input length;
  - avoid unnecessary intermediate allocations beyond what is required to produce the final output;
  - keep copying bounded to the same practical level as the C implementation’s string-processing path;
  - maintain predictable stack/heap usage using standard `String`/`&str` ownership rules.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `propername-lite.c` | `src/propername_lite.rs` | Direct port of the module into one Rust source file. |
| `proper_name_lite` | `pub(crate)` or private function in `src/propername_lite.rs` | Visibility should match actual crate usage; do not widen API surface unless required by existing callers. |

If this module is invoked from the crate entry path, expose it through the existing crate module tree with the smallest necessary surface, for example:

- declare `mod propername_lite;` in the nearest existing parent module;
- re-export nothing unless current Rust-side call sites require it.

## Data Model

The analyzed module does not define module-specific C structs or enums.

### C-to-Rust Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char *` / `const char *` input strings | `&str` when valid UTF-8 is guaranteed by the surrounding port; otherwise `&[u8]` or `&CStr` at the boundary, converted internally as needed | Choose the narrowest boundary type that matches existing caller behavior. |
| returned allocated string | `String` | Rust owns the produced output and frees it automatically. |
| null/error result conventions | `Result<T, E>` or `Option<T>` | Prefer `Result` if the C code distinguishes invalid input from empty output; otherwise `Option` is acceptable for simple presence/absence semantics. |

### Memory Management Notes

- Remove manual allocation/free behavior from the C implementation and rely on Rust-owned values.
- Avoid retaining borrowed references to temporary transformed strings.
- Keep transformations local so lifetimes remain simple and equivalent to the original function scope.

### Error Handling Notes

- Convert any C sentinel-based failure path into explicit Rust control flow.
- If the original function effectively assumes valid input and does not expose recoverable errors, keep the Rust API narrow and avoid inventing new error categories.
- Preserve observable behavior for empty or malformed inputs as determined during porting of the original function body.

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add `src/propername_lite.rs`.
- Establish the Rust signature for `proper_name_lite` based on how the surrounding crate passes string data.
- Wire the module into the existing crate tree with minimal visibility.
- Add a small set of placeholder unit tests covering compilation and basic invocation shape.

### Phase 2: Port `proper_name_lite` logic directly

- Translate the body of `proper_name_lite` from C into Rust in the same processing order as the original.
- Replace pointer arithmetic and mutable buffer writes with indexed or iterator-based string/byte processing.
- Preserve edge-case handling from the C implementation, especially around:
  - empty input;
  - delimiter or formatting rules;
  - output construction and ownership.
- Keep helper logic local to the module; only split into private functions if necessary to mirror a distinct block of the original function.

### Phase 3: Resolve memory and error semantics

- Confirm the Rust return type matches the effective semantics of the C implementation.
- Eliminate any translation artifacts such as unnecessary clones or temporary buffers.
- Verify that all former C allocation paths are represented by safe Rust-owned values.
- Ensure invalid input behavior is explicit and stable, without introducing broader recovery behavior.

### Phase 4: Add behavior-preserving tests and finish integration

- Add unit tests derived from the observed behavior of `proper_name_lite`, including nominal and boundary cases.
- Compare Rust outputs against the intended C behavior for representative inputs.
- Run `cargo test` and fix any integration mismatches with existing callers.
- Perform a final pass to keep the module limited to the original file/function scope and remove any unused abstractions.