# Implementation Plan

## Summary

Port `gnu/malloca.c` into a focused Rust module that preserves the existing allocation/free behavior exposed by `mmalloca` and `freea`, without broadening the API surface. The Rust implementation should center on explicit ownership and deterministic cleanup, replacing C pointer-tagging or mixed allocation techniques with a minimal internal representation that can safely distinguish allocation origin and release memory correctly.

The implementation should stay close to the source module layout: one Rust source file for this C file, with the two migrated functions implemented first and any internal helpers kept private to the same module. The technical approach should prefer the Rust standard library for heap allocation and raw-pointer interop where necessary, while keeping unsafe code narrowly scoped and documented around allocation and deallocation boundaries.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve constant-time free-path behavior for tracked allocations.
  - Keep allocation overhead minimal and bounded to what is required to emulate C behavior.
  - Avoid unnecessary copies or wrapper allocations beyond what is needed for memory-origin tracking.
  - Maintain behavior suitable for low-level utility use, with no extra synchronization or runtime layers.

## Module Mapping

- **C source**: `gnu/malloca.c`
- **Rust target**: `src/module_gnu_malloca.rs`

### Function Mapping

- `mmalloca`
  -> `pub(crate)` Rust function in `src/module_gnu_malloca.rs`
  - Migrated as the allocation entry point.
  - Returns a raw pointer or equivalent low-level allocation handle consistent with surrounding project expectations.
  - Internal bookkeeping should be implemented in the same file, not split into extra modules.

- `freea`
  -> `pub(crate)` Rust function in `src/module_gnu_malloca.rs`
  - Migrated as the paired release function.
  - Must safely distinguish memory that requires heap deallocation from memory that must not be freed through the heap path.
  - Any marker/header decoding logic remains private and colocated with the function.

## Data Model

This C module does not define standalone public structs in the provided input, but it relies on allocation-state encoding. The Rust port should model that state explicitly and locally.

### Data-Structure Mapping

- **C implicit allocation metadata / tagged memory convention**
  -> **Rust private enum or header representation**, for example:
  - `enum AllocationKind { Heap, NonHeap }`
  - or a private header struct placed before user memory if raw layout compatibility is required

### Rust Representation Guidance

- Prefer a **private internal metadata representation** over reproducing opaque C pointer tricks directly.
- If the original C logic depends on inspecting bytes adjacent to the returned pointer, model that using:
  - a private header struct with a stable layout (`#[repr(C)]`) only if raw memory prefixing is necessary;
  - otherwise, use a simpler internal ownership strategy if the surrounding Rust code permits it.
- No public data structures should be introduced unless required by the existing project API.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Allocation Model

- Create `src/module_gnu_malloca.rs`.
- Define the Rust signatures for `mmalloca` and `freea` based on current project usage.
- Identify whether the caller contract requires:
  - raw pointer return values,
  - nullable behavior,
  - exact size-based allocation semantics.
- Choose the minimal internal representation for tracking allocation origin:
  - prefer a private metadata header only if necessary to preserve the C free behavior;
  - otherwise keep the representation simpler and local.
- Document all unsafe assumptions at allocation boundaries.

### Deliverables

- New Rust module file with function stubs.
- Private internal constants/types for allocation tracking.
- Clear comments describing ownership and deallocation rules.

## Phase 2: Port `mmalloca`

- Implement the allocation path in Rust using `std::alloc` or `Box<[u8]>`/`Vec<u8>` only if they fit the required raw-pointer contract.
- Reproduce the original size handling and any edge-case behavior relevant to zero-size or allocation failure semantics.
- If the C implementation encodes allocation provenance in-band, allocate enough extra space for a private header and return the adjusted user pointer.
- Keep pointer arithmetic isolated to a small unsafe block.
- Ensure the returned pointer layout is compatible with what `freea` expects.

### Deliverables

- Working Rust implementation of `mmalloca`.
- Unit tests covering:
  - normal allocation,
  - zero-size or minimal-size requests as applicable,
  - repeated independent allocations,
  - null/failure behavior if the original contract requires it.

## Phase 3: Port `freea`

- Implement `freea` as the exact counterpart to the Rust `mmalloca` layout.
- Safely recover internal metadata from the incoming pointer when required.
- Only deallocate memory that was heap-allocated through the Rust port’s `mmalloca`.
- Preserve no-op behavior for pointers that represent non-heap storage, if that distinction exists in the original module contract.
- Ensure null-pointer handling matches C expectations where applicable.

### Deliverables

- Working Rust implementation of `freea`.
- Unit tests covering:
  - freeing heap-backed allocations,
  - null-pointer input,
  - non-heap/no-op release path if represented by the design,
  - double-free avoidance through documented caller contract rather than added runtime recovery logic.

## Phase 4: Validation and Integration Cleanup

- Run `cargo test` and fix any ownership, layout, or deallocation mismatches.
- Review unsafe blocks for minimal scope and correct allocator pairing.
- Confirm the Rust file/module naming is consistent with the project’s branch/module migration convention.
- Remove dead helpers and keep all logic confined to this migrated module.

### Deliverables

- Finalized `src/module_gnu_malloca.rs`.
- Passing tests for the migrated functions.
- Concise module-level documentation describing memory-management constraints.