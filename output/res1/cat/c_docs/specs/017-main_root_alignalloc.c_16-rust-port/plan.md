# Implementation Plan: main_root_alignalloc.c_16

## Summary

This module ports the aligned-allocation helper logic from `alignalloc.c` into Rust with behavior kept narrowly equivalent to the existing C implementation. The Rust work should preserve the original allocation model: compute alignment boundaries, over-allocate as needed, store the original allocation address in recoverable form, and release memory through the matching free path.

The implementation should stay focused on migrating the existing functions:

- `align_down`
- `address_of_pointer_to_malloced`
- `alignalloc`
- `alignfree`

The preferred technical approach is to implement this as a small internal Rust module using `std::alloc` and raw-pointer operations where necessary, rather than introducing broader abstractions. Because this module manages pointer arithmetic and manual deallocation, the Rust port will likely contain small, well-contained `unsafe` sections with explicit invariants documenting alignment, offset calculation, metadata placement, and deallocation pairing.

## Technical Context

- **Language/Version**: Rust 1.78+
  A stable modern Rust version is sufficient; no nightly features are needed.

- **Primary Dependencies**:
  - Rust standard library only
  - `std::alloc` for low-level allocation/deallocation
  - `std::ptr` for pointer reads/writes and null handling
  - `std::mem` for size/alignment calculations

- **Testing**:
  - `cargo test`

- **Performance Goals**:
  - Match the C implementation’s constant-time alignment and free-path behavior.
  - Avoid extra heap allocations beyond the intentional over-allocation required for alignment.
  - Keep pointer metadata storage minimal and local to the allocated block.
  - Preserve low-overhead behavior suitable for frequent allocation/free operations.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `alignalloc.c` | `src/main_root_alignalloc.rs` or equivalent internal module file | Single-module migration; do not split further unless the existing crate layout requires integration into an existing `mod.rs`. |
| `align_down` | `fn align_down(...) -> ...` | Port as a small internal helper for boundary rounding on addresses or integer values. |
| `address_of_pointer_to_malloced` | `fn address_of_pointer_to_malloced(...) -> ...` | Port as an internal helper that computes the metadata slot used to store the original base allocation pointer. |
| `alignalloc` | `pub(crate) unsafe fn alignalloc(...) -> *mut u8` or closest required signature | Main aligned allocation routine; preserve null/error behavior compatible with caller expectations. |
| `alignfree` | `pub(crate) unsafe fn alignfree(...)` or closest required signature | Must reverse `alignalloc` exactly by recovering and freeing the original allocation pointer. |

## Data Model

This module analysis reports no named C structs, so the Rust port should not introduce new public data structures unless strictly required by the existing crate API.

| C Data Model | Rust Mapping | Notes |
|---|---|---|
| Raw allocation pointer returned by `malloc` | `*mut u8` | Keep as raw pointer to preserve exact allocation/free semantics. |
| Pointer-sized stored metadata | `*mut u8` written into reserved memory, or `usize` if internal arithmetic requires it | Prefer storing the original pointer value directly in pointer-sized space immediately preceding or near the aligned result, matching the C strategy. |
| Integer address arithmetic | `usize` | Use for alignment calculations and offset computations. |

### Memory Management Notes

- Use `std::alloc::alloc` / `std::alloc::dealloc` with a layout derived from:
  - requested size
  - requested alignment
  - extra space needed to store the original allocation pointer
- Validate alignment assumptions explicitly:
  - alignment should be non-zero
  - alignment should normally be a power of two if required by the original implementation’s arithmetic
- Any metadata written to the allocation region must be placed at a well-defined offset and read back symmetrically in `alignfree`.
- `alignfree` must tolerate the same input class as the C function, especially null-pointer handling if present in the original behavior.

### Error Handling Notes

- Preserve C-like failure signaling where applicable:
  - allocation failure returns null pointer rather than introducing `Result` if the surrounding API is pointer-based
- Guard internal arithmetic against overflow when computing:
  - total allocation size
  - metadata offset
  - aligned address
- Keep panics out of the normal path; use checked arithmetic and return null on invalid/overflow cases if that matches the C behavior expected by callers.

## Implementation Phases

## Phase 1: Establish module skeleton and helper arithmetic

- Create the Rust module file for `alignalloc.c`.
- Port `align_down` as a private helper using `usize` arithmetic.
- Port `address_of_pointer_to_malloced` as a private helper that computes the metadata location from an aligned pointer.
- Document the unsafe invariants for:
  - converting pointers to integer addresses and back
  - assuming alignment is valid for bit-mask based rounding
  - accessing pointer-sized metadata near the returned allocation

**Exit criteria:**
- Module compiles with helper functions present.
- Helper-level unit tests cover representative alignment calculations and metadata-address positioning logic.

## Phase 2: Port allocation path

- Implement `alignalloc` using `std::alloc`.
- Reproduce the C allocation strategy:
  - compute required over-allocation
  - allocate the raw block
  - derive an aligned user pointer within the block
  - store the original raw allocation pointer in the reserved metadata slot
- Keep the function signature close to existing call sites and avoid introducing ownership wrappers that would alter module boundaries.
- Handle failure cases explicitly:
  - invalid alignment
  - size/layout overflow
  - allocation failure

**Exit criteria:**
- `alignalloc` returns correctly aligned pointers for tested alignments and sizes.
- Metadata round-trip from aligned pointer back to original allocation pointer is verified by tests.

## Phase 3: Port free path and verify symmetry

- Implement `alignfree` to:
  - no-op on null if the C implementation does so
  - recover the original allocation pointer from metadata
  - reconstruct the correct layout/deallocation call pairing
- Ensure the deallocation path is exactly compatible with how `alignalloc` computed the original layout.
- Review all unsafe blocks for minimal scope and explicit comments on why each operation is valid.

**Exit criteria:**
- Allocation/free round-trip tests pass across multiple sizes and alignments.
- Null handling and edge-condition tests pass.
- No mismatched allocation/deallocation strategy remains.

## Phase 4: Integration cleanup and regression tests

- Wire the module into the existing crate structure using standard Rust module conventions only.
- Adjust visibility to the minimum needed by current callers (`pub(crate)` if sufficient).
- Add focused tests for:
  - power-of-two alignments
  - small and zero-size requests if applicable to current C behavior
  - pointer alignment correctness
  - repeated allocation/free cycles
  - overflow/invalid-input rejection behavior

**Exit criteria:**
- `cargo test` passes.
- Module is integrated without introducing new supporting subsystems or expanded APIs.
- The Rust port remains functionally constrained to the original `alignalloc.c` responsibilities.