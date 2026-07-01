# Implementation Plan: module_gnu_getdtablesize.c_29

## Summary

This module ports `gnu/getdtablesize.c` into Rust with a narrow scope centered on the `_setmaxstdio_nothrow` function. The C source appears to provide a compatibility-oriented, non-throwing helper related to file descriptor or stdio table sizing. The Rust implementation should preserve that limited behavior without introducing broader abstractions or platform layers beyond what is required by the existing function.

The implementation approach is to map the original function into a small Rust module that:
- keeps the same operational scope as the C file,
- uses Rust primitive integer types to model the C interface,
- avoids heap allocation and unsafe code unless a platform call is strictly necessary,
- returns a simple integer result consistent with the original C contract,
- handles invalid or unsupported cases through explicit return values rather than panic paths.

Because the analyzed module exposes only `_setmaxstdio_nothrow` and no named public data structures, the Rust port should remain minimal and file-focused.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the available evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Constant-time execution
  - No unnecessary allocations
  - Behavior comparable to the original C helper for success/failure signaling
  - Minimal call overhead and no extra indirection beyond the direct port

## Module Mapping

### C to Rust File Mapping

- `gnu/getdtablesize.c`
  - `src/module_gnu_getdtablesize.rs`

### Function Mapping

- `_setmaxstdio_nothrow`
  - `pub(crate)` or private Rust function in `src/module_gnu_getdtablesize.rs`, depending on actual crate use sites
  - Keep the function singular and local to this migrated module unless another already-existing Rust module requires direct access

### Rust Module Placement

Use standard Rust source layout with a single module file added to the crate:
- `src/module_gnu_getdtablesize.rs`

If the crate uses a `mod.rs` or `lib.rs` registration pattern, only add the minimal module declaration needed to compile the port. Do not introduce additional helper modules.

## Data Model

The analysis lists only an anonymous data structure and no named struct definitions. The Rust port should therefore avoid inventing structural types unless the C implementation requires a local representation for constants or return-state handling.

### Data-Structure Mapping

- anonymous C data structure
  - No dedicated Rust struct unless required by the function body during translation
  - Prefer:
    - local variables,
    - `const` items,
    - primitive integer types (`i32`, `u32`, `usize`) chosen according to the original C parameter and return semantics

### Type Mapping Guidance

- C integer used for stdio/file-table sizing
  - Rust `i32` if negative error/sentinel values are part of the contract
  - Rust `u32` or `usize` only if the C function is strictly non-negative and the call sites do not depend on signed error signaling

### Memory Management

- No owned heap-backed data expected
- Keep all state on the stack
- No manual memory management should be needed in the Rust port

### Error Handling

- Preserve non-throwing behavior from the C function
- Prefer return-value-based signaling rather than `Result` if the original function contract is integer/status oriented
- Do not panic for normal failure cases such as unsupported limits or invalid requested values

## Implementation Phases

## Phase 1: Inspect and Define the Exact Rust Signature

- Review `gnu/getdtablesize.c` to determine:
  - the exact parameter types of `_setmaxstdio_nothrow`,
  - the precise return-value contract,
  - any platform-specific branches or compile-time conditionals,
  - any constants or macros used in the function body
- Choose Rust primitive types that match the observed C semantics
- Create `src/module_gnu_getdtablesize.rs`
- Add only the minimal module declaration required by the crate

### Deliverables

- Rust module file stub
- Finalized Rust signature for `_setmaxstdio_nothrow`
- Inline notes/comments only where needed to preserve C semantic assumptions

## Phase 2: Port Function Logic Directly

- Translate `_setmaxstdio_nothrow` into Rust in the same control-flow shape as the C source
- Preserve:
  - boundary checks,
  - fallback paths,
  - sentinel/error returns,
  - non-throwing behavior
- Replace C macros and conditional compilation with Rust `cfg` only when the source actually requires platform branching
- Use standard library facilities first; introduce `unsafe` only if there is no standard safe equivalent for the required system interaction
- Keep the implementation local and avoid extracting generic helpers unless the original file already separates logic that way

### Deliverables

- Compiling Rust implementation of `_setmaxstdio_nothrow`
- Minimal comments documenting any unavoidable semantic deviations from C APIs or platform behavior

## Phase 3: Validate Behavior with Focused Tests

- Add unit tests covering the directly observable contract of `_setmaxstdio_nothrow`
- Test categories should include, as supported by the actual function semantics:
  - valid input path,
  - invalid or boundary input path,
  - unsupported-operation/failure path if representable in-process
- Keep tests deterministic and local to the module
- Avoid environment-dependent assertions unless the C behavior itself is environment-bound; in that case, assert only stable invariants such as “does not panic” and “returns a contract-valid integer”

### Deliverables

- `cargo test` coverage for the migrated function
- Verification that the Rust port preserves return-value conventions and does not panic under normal failure cases

## Phase 4: Integration Cleanup

- Connect the module to existing crate call sites, if any
- Remove any temporary scaffolding added during migration
- Confirm that visibility is no broader than needed
- Run formatting and full test pass
- Ensure the final module remains limited to the original C file’s responsibility and does not add unrelated compatibility utilities

### Deliverables

- Final integrated Rust module
- Clean compile and test pass on branch `035-module_gnu_getdtablesize.c_29-rust-port`