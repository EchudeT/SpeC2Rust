# Implementation Plan

## Summary

Port `alignalloc.c` into a Rust module that preserves the existing allocation behavior and call structure for aligned heap allocation and release. The Rust implementation should stay narrowly scoped to the current C file responsibilities:

- alignment adjustment logic
- recovering the original allocated pointer from an aligned pointer
- aligned allocation entry point
- aligned free entry point

Technical approach:

- Implement the module in Rust using `std::alloc` and raw pointer handling where necessary.
- Keep the migrated API surface close to the C function set rather than introducing broader allocator abstractions.
- Represent allocation failure and invalid parameter cases explicitly through Rust return types where possible, while keeping internal logic compatible with low-level pointer arithmetic.
- Centralize unsafe code in a small portion of the module, especially around pointer offset calculation, metadata storage, and deallocation.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::alloc`, `std::ptr`, `std::mem`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain constant-time alignment adjustment and pointer recovery.
  - Avoid extra heap allocations beyond the single underlying allocation required by the C implementation.
  - Keep overhead limited to the metadata needed to recover the original allocation during free.
  - Match the C module’s expected low-level behavior without adding wrapper layers that would change allocation cost.

## Module Mapping

### Source Mapping

- `alignalloc.c` → `src/main_root_alignalloc.rs`

### Function Mapping

- `align_down` → `fn align_down(...) -> ...`
- `address_of_pointer_to_malloced` → `unsafe fn address_of_pointer_to_malloced(...) -> ...`
- `alignalloc` → `pub fn alignalloc(...) -> ...`
- `alignfree` → `pub unsafe fn alignfree(...)`

### Rust Module Placement

Use a single Rust source file for this migration unit under the existing crate structure, exposed from the current main-cluster module tree only as needed by the rest of the port. Do not split helpers into extra submodules.

## Data Model

This C module does not define dedicated structs, but it relies on implicit allocation metadata stored adjacent to the aligned pointer.

### Mapping

- **C raw allocated block + stored back-pointer metadata**
  → **Rust raw allocation (`*mut u8`) plus manually written metadata in the allocation prefix**

### Rust Representation Decisions

- Use `*mut u8` for raw allocated memory blocks.
- Store the original base pointer in the bytes immediately preceding the aligned pointer, matching the C technique of recovering the malloc-returned address during free.
- Use `usize` for alignment and size arithmetic.
- Use `NonNull<u8>` internally where helpful for invariants, but do not force broader API redesign if the existing call sites expect nullable/raw-pointer style behavior.

### Error and Safety Model

- Invalid alignment inputs must be checked before layout/allocation work.
- Integer overflow in size or offset computation must be detected before allocation.
- Allocation failure should return a failure result rather than assuming infallible allocation.
- Pointer reads/writes used for metadata storage and recovery must be isolated in documented `unsafe` blocks.

## Implementation Phases

## Phase 1: Create Rust module skeleton and migrate pure alignment logic

### Goals

- Establish the destination Rust file and public/internal function layout.
- Port the non-allocating helper logic first.

### Tasks

- Create `src/main_root_alignalloc.rs`.
- Implement `align_down` using integer arithmetic on `usize` or pointer-address values, depending on call-site needs.
- Define internal helper signatures for:
  - alignment validation
  - metadata size calculation
  - aligned pointer computation
- Add unit tests for:
  - power-of-two alignment behavior
  - already-aligned values
  - downward rounding cases
  - edge values near zero and word-size boundaries

### Exit Criteria

- Helper logic compiles.
- Alignment arithmetic is covered by unit tests.
- No allocation/deallocation code added yet beyond signatures.

## Phase 2: Port aligned allocation and metadata storage

### Goals

- Implement `alignalloc` with a single underlying allocation and prefix metadata sufficient for later free.

### Tasks

- Translate C allocation sizing into Rust using checked arithmetic:
  - requested payload size
  - alignment slack
  - storage for the original pointer
- Build a `Layout` or equivalent allocation request that safely covers the full backing block.
- Allocate with `std::alloc::alloc`.
- Compute the aligned user pointer from the backing allocation.
- Store the original backing pointer immediately before the aligned result in a machine-word-sized slot.
- Return the aligned pointer in a form consistent with surrounding migrated code.

### Safety Notes

- Keep all pointer arithmetic in one small unsafe section.
- Ensure the stored metadata location is properly aligned for a pointer-sized write.
- Reject unsupported alignment values before calling the allocator.

### Exit Criteria

- Successful aligned allocations return pointers satisfying the requested alignment.
- Metadata can be written and later read back consistently in tests.
- Overflow and allocation-failure paths are handled deterministically.

## Phase 3: Port pointer recovery and free path

### Goals

- Implement the reverse mapping from aligned pointer to original allocation and free it correctly.

### Tasks

- Implement `address_of_pointer_to_malloced` as the helper that locates the metadata slot before the aligned pointer.
- Implement `alignfree` to:
  - no-op on null input if that matches current C behavior
  - recover the original allocation pointer from stored metadata
  - reconstruct the allocation layout needed for deallocation, or store enough size/alignment knowledge to deallocate correctly within the chosen Rust design
- Confirm the free path only releases memory obtained from `alignalloc`.

### Testing

- Add round-trip tests:
  - allocate then free for several sizes and alignments
  - minimum supported alignment
  - larger alignments
  - zero-sized or near-zero requests only if the original C behavior supports them

### Exit Criteria

- Allocation/free pairs succeed under `cargo test`.
- Pointer recovery matches stored metadata.
- Null and invalid-input behavior is explicit in the implementation.

## Phase 4: Finalize API compatibility and module-level verification

### Goals

- Align the Rust signatures and visibility with the needs of the existing port branch without widening scope.

### Tasks

- Adjust function visibility (`pub`, `pub(crate)`, private) to match actual crate use.
- Confirm naming and module exports map cleanly to current call sites in the port.
- Add focused documentation comments on:
  - required alignment constraints
  - ownership expectations for returned pointers
  - safety requirements for `alignfree`
- Run `cargo test` and fix any integration issues caused by call-site expectations.

### Exit Criteria

- The module is integrated in the branch under standard Rust project layout.
- Public API is minimal and tied to the migrated C usage.
- Tests pass with no planned follow-on expansion from this module.