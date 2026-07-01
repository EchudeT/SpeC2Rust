# Implementation Plan

## Summary

Port `gnu/realloc.c` into a focused Rust module that preserves the observable behavior of `rpl_realloc` without adding broader allocation abstractions. The Rust implementation should mirror the C module’s role as a replacement-style reallocation entry point, with explicit handling for allocation edge cases and failure signaling.

The implementation approach should stay minimal:

- create one Rust module corresponding to `gnu/realloc.c`
- migrate `rpl_realloc` as the only public function required by this module
- model C memory-management semantics carefully, especially zero-size behavior, null-pointer handling, and allocation failure propagation
- prefer `std` allocation facilities and explicit result handling over introducing helper frameworks

Because the source module is allocation-focused, the main technical concern is preserving semantics around ownership, resizing, and failure while avoiding undefined behavior in Rust. The port should therefore isolate any unsafe allocation logic into a small, reviewable section and keep the public API narrow.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::alloc`, pointer types, and core memory primitives as needed)
- **Testing**: `cargo test`
- **Performance Goals**:
  - match the asymptotic behavior of C reallocation paths
  - avoid extra copies beyond what resizing requires
  - keep allocation-path overhead minimal
  - maintain predictable behavior for null and zero-size cases without adding wrapper layers

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/realloc.c` | `src/gnu/realloc.rs` | Direct migration target for the allocation replacement logic |
| `gnu/realloc.c` (`rpl_realloc`) | `src/gnu/realloc.rs` (`pub fn rpl_realloc(...)`) | Single-function port; keep behavior aligned with the C entry point |

If the crate already exposes a `gnu` namespace, register the module through the existing `src/gnu/mod.rs`. If not, add only the minimal module declaration needed for `realloc.rs`.

## Data Model

This module does not define custom C structs in the provided input.

### Data-structure Mapping

| C Construct | Rust Mapping | Notes |
|---|---|---|
| raw allocation pointer (`void *`) | `*mut u8` or `NonNull<u8>` internally | Use raw pointers at the API boundary if preserving C-like semantics; use `NonNull<u8>` only for internal invariants |
| allocation size (`size_t`) | `usize` | Direct platform-sized mapping |
| null pointer | `std::ptr::null_mut()` | Preserve explicit null handling where required by the original function |

### Memory Management Notes

- Reallocation behavior must distinguish:
  - null input pointer treated like allocation
  - non-null pointer resized in place or moved
  - zero-size requests handled according to the original module’s intended semantics
- Rust ownership cannot directly express arbitrary C heap ownership, so any raw-pointer-based implementation will require a tightly scoped `unsafe` block.
- Failure signaling should not panic. It should preserve the C-style contract used by the surrounding ported code, typically via null return or equivalent result translation at the integration boundary.

## Implementation Phases

### Phase 1: Establish module skeleton and API boundary

- Create `src/gnu/realloc.rs`.
- Add the minimal module declaration required to compile it within the crate.
- Define the Rust signature for `rpl_realloc` based on the intended call pattern of the surrounding port:
  - pointer parameter mapped from `void *`
  - size parameter mapped from `size_t`
  - return value preserving replacement realloc semantics
- Document the expected behavior at the function boundary:
  - null input handling
  - zero-size request handling
  - failure behavior
- Do not introduce broader allocator interfaces or utility layers.

### Phase 2: Implement allocation and reallocation semantics

- Implement the core logic of `rpl_realloc` in `src/gnu/realloc.rs`.
- Keep unsafe logic localized to the exact operations needed for:
  - allocating new memory for null input
  - resizing an existing allocation
  - returning failure without panicking
- Use Rust standard allocation primitives only.
- Ensure the implementation does not silently change semantics for:
  - zero-byte requests
  - failure return values
  - preservation of existing memory contents across resize
- Keep the function narrowly scoped; do not split into extra modules unless compilation structure requires it.

### Phase 3: Add semantic tests for edge cases

- Add unit tests covering:
  - null pointer plus non-zero size
  - null pointer plus zero size
  - non-null pointer growth
  - non-null pointer shrink
  - zero-size reallocation from a non-null pointer
  - failure-path expectations where they can be validated safely
- Verify behavior through `cargo test`.
- Keep tests centered on contract preservation for `rpl_realloc`; do not add unrelated allocator test infrastructure.

### Phase 4: Integration review and cleanup

- Confirm the Rust module naming and exports match the expected crate layout.
- Review all unsafe blocks for:
  - correct layout calculations
  - null handling
  - no double-free or invalid-realloc paths
- Remove any temporary scaffolding added during migration.
- Ensure the final port remains limited to the original file and function scope.