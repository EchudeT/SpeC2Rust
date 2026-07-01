# Implementation Plan

## Summary

Port `xalignalloc.c` into a focused Rust module that preserves the existing allocation behavior and call shape as closely as practical within Rust’s safety model. The Rust implementation should center on a single allocation routine corresponding to `xalignalloc`, using `std::alloc` for aligned heap allocation and explicit validation of size/alignment inputs before allocation.

The implementation should avoid introducing broader allocation abstractions or new support layers. Error handling should follow the existing project’s Rust-side conventions for fatal allocation failure versus invalid input, but remain limited to what is needed to migrate this file and function. Memory ownership must be made explicit in Rust so that any allocated region has a clear deallocation path using the same layout metadata assumptions used at allocation time.

## Technical Context

- **Language/Version**: Rust stable, edition 2021, targeting Rust 1.74+.
- **Primary Dependencies**:
  - Rust standard library only.
  - Use `std::alloc::{alloc, handle_alloc_error, Layout}` for aligned allocation behavior.
- **Testing**:
  - `cargo test`
  - Unit tests for alignment validation, successful allocation, and failure-path semantics that can be tested deterministically.
- **Performance Goals**:
  - Preserve constant-time allocation setup aside from standard allocator cost.
  - Avoid extra copying, buffering, or wrapper allocations.
  - Match the C module’s direct allocation role with minimal overhead beyond required Rust safety checks.

## Module Mapping

- **C source file**: `xalignalloc.c`
- **Rust module target**: `src/xalignalloc.rs`

Function mapping:

- **C** `xalignalloc`
  - **Rust** `pub(crate) fn xalignalloc(...) -> ...`
  - Final Rust signature should be chosen to match existing crate conventions and callers, but should remain a direct one-function migration rather than introducing a new allocation API surface.

If the current Rust port keeps low-level pointer-oriented helpers, this function should remain low-level and local to the crate. If the surrounding port already uses a small wrapper type for owned raw allocations, the function may return that type only if such a type already exists elsewhere in the branch; otherwise keep the migration limited to the direct function.

## Data Model

This module has no named C structs to migrate.

Data and type mapping:

- **C** raw allocated memory pointer
  - **Rust** `*mut u8` or `NonNull<u8>` depending on existing crate conventions.
- **C** size/alignment integer parameters
  - **Rust** `usize`
- **C** allocator layout assumptions
  - **Rust** `std::alloc::Layout`

Notes:

- Prefer `Layout::from_size_align` to encode the alignment contract explicitly.
- Invalid layout creation must be handled before calling the allocator.
- If raw pointers are returned, document the required deallocation contract in the Rust module comments and keep use sites consistent with the same size/alignment basis.

## Implementation Phases

### Phase 1: Establish module skeleton and function signature

- Create `src/xalignalloc.rs`.
- Add the Rust counterpart for `xalignalloc` with the narrowest visibility needed by current callers.
- Determine the Rust signature from actual usage in the port branch:
  - preserve low-level pointer semantics if callers expect raw allocation;
  - avoid introducing new wrapper types unless already present in the codebase.
- Add module-level notes describing:
  - required alignment invariants;
  - ownership expectations for the returned allocation;
  - the matching deallocation requirement.

### Phase 2: Implement aligned allocation logic

- Translate the C allocation path into Rust using `std::alloc::Layout`.
- Validate:
  - alignment is non-zero;
  - alignment satisfies allocator layout rules;
  - size/alignment combination forms a valid `Layout`.
- Allocate with `std::alloc::alloc`.
- Handle null allocation result using `handle_alloc_error(layout)` if the C behavior is fail-fast for allocation failure.
- Keep unsafe code tightly scoped to the allocation call and raw pointer handling only.
- Do not add generalized allocator helpers beyond what this function directly needs.

### Phase 3: Integrate with callers and preserve memory contract

- Replace imports or declarations that referenced the C implementation with the Rust module path.
- Update call sites only as needed to satisfy Rust typing and ownership rules.
- Ensure that whatever deallocation path exists in the port uses compatible size/alignment assumptions.
- Keep migration changes local to this module and direct callers; do not broaden refactoring into unrelated allocation utilities.

### Phase 4: Add tests and finalize edge-case behavior

- Add unit tests covering:
  - valid aligned allocation returns a properly aligned pointer;
  - invalid alignment or impossible layout handling matches chosen Rust-side contract;
  - zero-sized allocation behavior is explicitly defined and tested according to project needs.
- Where direct allocator-failure testing is not practical, test the pre-allocation validation boundaries instead.
- Run `cargo test` and fix any integration mismatches caused by pointer type or visibility choices.