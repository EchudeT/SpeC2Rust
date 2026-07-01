# Implementation Plan

## Summary

Port `gnu/malloca.c` into a focused Rust module that preserves the existing allocation/freeing behavior of `mmalloca` and `freea` without introducing broader allocation abstractions. The Rust implementation should mirror the C module’s ownership and deallocation rules closely, with explicit handling for memory that may originate from different allocation paths. The technical approach is to translate the file into a single Rust module, represent the returned allocation as a raw-pointer-oriented internal API, and centralize deallocation logic so that `freea` applies the same decision rules as the original C code.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the C module’s low-overhead allocation behavior as closely as practical in Rust.
  - Avoid unnecessary wrapper allocations, copies, or indirection beyond what is required to reproduce the original semantics.
  - Keep allocation/deallocation paths constant-time aside from allocator behavior.
  - Maintain predictable freeing behavior for all supported allocation cases.

## Module Mapping

- **C source file**
  - `gnu/malloca.c`
- **Rust target**
  - `src/module_gnu_malloca.rs`

### Function Mapping

- `mmalloca`
  - Port to a Rust function in `src/module_gnu_malloca.rs`
  - Keep the implementation close to the original allocation-path logic
  - Expose as a crate-internal function unless the wider port requires broader visibility

- `freea`
  - Implement paired deallocation logic corresponding exactly to the allocation metadata/layout used by `mmalloca`

## Data Model

This module does not define named C structs in the provided input, so the Rust data model should stay minimal and allocation-layout-driven.

### Data-structure Mapping

- **C implicit allocation metadata/layout**
  - **Rust representation**: internal pointer layout plus helper constants/functions
  - Use raw pointers and explicit offset calculations only where required to preserve the original free-path behavior.
  - Prefer `std::alloc` and pointer arithmetic within small, well-contained `unsafe` blocks.
  - Avoid inventing public wrapper structs unless they are strictly necessary to encode the allocation contract used by both `mmalloca` and `freea`.

### Memory Representation Notes

- If the original C logic stores an allocation marker adjacent to the returned pointer, represent that in Rust through internal header encoding rather than a new external type.
- Null-pointer handling and zero-size allocation behavior should be made explicit and tested.
- Any distinction between stack-like and heap-like release behavior in the C logic should be preserved through internal metadata checks, not through expanded API surface.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and map the allocation contract

- Add `src/module_gnu_malloca.rs`.
- Identify the exact control flow and memory layout rules in `gnu/malloca.c` for:
  - allocation decision path in `mmalloca`
  - deallocation decision path in `freea`
  - any sentinel/header bytes or alignment assumptions
- Define the minimal Rust function signatures needed for the port.
- Add module declarations in the existing crate structure without creating extra support modules.

### Deliverables

- Rust module file created
- Function signatures stubbed
- Documented internal notes in code for pointer layout and ownership rules

## Phase 2: Implement `mmalloca` with explicit allocation-path translation

- Port `mmalloca` into Rust using `std::alloc` and raw pointers as needed.
- Reproduce the original allocation metadata/header layout exactly enough for `freea` to identify how to release memory.
- Handle edge cases explicitly:
  - zero-size requests
  - null returns on allocation failure if the original semantics require nullable behavior
  - alignment and header-offset correctness
- Keep all unsafe operations tightly scoped and justified by nearby comments.

### Deliverables

- Working Rust implementation of `mmalloca`
- Internal helper constants/functions only if directly required by the original file logic
- Unit tests covering allocation return validity and layout-sensitive cases

## Phase 3: Implement `freea` and complete ownership/deallocation behavior

- Port `freea` to inspect the allocation metadata/layout and choose the correct release action.
- Ensure the function is safe against:
  - null input
  - double-interpretation of pointers due to incorrect marker handling
  - mismatched header offset calculations
- Keep deallocation logic in the same module and avoid introducing generalized memory-management utilities.

### Deliverables

- Working Rust implementation of `freea`
- Tests covering:
  - freeing null
  - freeing heap-managed allocations
  - no-op behavior for any non-heap case implied by the original C design
  - repeated layout validation between `mmalloca` and `freea`

## Phase 4: Validation and cleanup

- Run `cargo test` and fix behavioral mismatches against the C module’s intended semantics.
- Review all `unsafe` blocks for minimal scope and correct pointer provenance assumptions.
- Remove any temporary scaffolding not required for the final port.
- Confirm the module remains a direct migration of `gnu/malloca.c` only, with no added abstraction layer or unevidenced features.

### Deliverables

- Finalized Rust module port
- Passing tests
- Concise inline safety documentation for allocation and deallocation logic