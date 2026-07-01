# Implementation Plan: module_gnu_malloc.c_33

## Summary

This module ports the behavior of `gnu/malloc.c` with a narrow scope around the single exported function `rpl_malloc`. The Rust implementation should preserve the original allocation-facing semantics while adapting them to Rust’s ownership and allocation model.

Because the source module is centered on memory allocation, the implementation should avoid inventing higher-level abstractions. The Rust side should provide a direct module-level replacement that mirrors the original function’s role: perform allocation requests safely, handle edge conditions explicitly, and expose a result form appropriate for Rust callers. The migration should focus on translating the existing function logic, especially any special handling for zero-sized allocation and failure signaling.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep allocation overhead effectively identical to standard Rust heap allocation paths.
  - Avoid extra copies, wrapper layers, or heap indirections beyond what is required to preserve original semantics.
  - Preserve constant-time handling for allocation size checks and edge-case normalization.

## Module Mapping

- **C source file**: `gnu/malloc.c`
- **Rust target module**: `src/gnu/malloc.rs`

Recommended crate module exposure:

- `src/gnu/mod.rs`
  - declares `pub mod malloc;`

Function mapping:

- `rpl_malloc` -> `pub(crate)` or `pub` Rust function in `src/gnu/malloc.rs`

The exact visibility should match current project use sites. If the function is only consumed internally, keep it crate-visible and do not widen the API surface.

## Data Model

This module has no named C structs or persistent data containers to migrate.

Function-level data mapping:

- C allocation size parameters (`size_t`) -> `usize`
- C raw allocation result (`void *`) -> Rust raw pointer or allocation-backed type, depending on surrounding call expectations
- C null-on-failure behavior -> `Option<NonNull<u8>>`, `Result<NonNull<u8>, AllocErrorLike>`, or a raw pointer if required by existing module contracts

Preferred technical direction:

- Use `std::alloc` for low-level allocation behavior if the Rust port must preserve raw allocator semantics closely.
- Use `NonNull<u8>` internally to avoid unchecked null handling when representing successful allocations.
- Convert to the narrowest externally required return type at the module boundary.

## Implementation Phases

### Phase 1: Establish Rust Module Skeleton

- Create `src/gnu/malloc.rs`.
- Register the module from `src/gnu/mod.rs`.
- Add the Rust signature for `rpl_malloc` based on how the rest of the ported code will call it.
- Decide the return type strictly from compatibility needs:
  - use a raw pointer return only if downstream migrated code expects C-like allocation interfaces,
  - otherwise prefer a Rust result form internally.

Deliverable:
- Compiling module scaffold with placeholder implementation and unit test file or inline test module.

### Phase 2: Port Allocation Logic

- Translate the body of `rpl_malloc` from `gnu/malloc.c` into Rust without adding new policy.
- Preserve edge-case behavior around requested size values, especially zero-length allocation handling if present in the C logic.
- Implement allocation through `std::alloc::{alloc, Layout}` when raw allocation semantics are needed.
- Validate layout creation before allocation and handle invalid size cases explicitly rather than relying on unchecked behavior.
- Represent allocation failure explicitly and map it to the chosen boundary type.

Memory management considerations:
- Keep ownership expectations clear: this function allocates only; deallocation strategy must remain compatible with caller expectations elsewhere in the port.
- Avoid constructing `Vec`/`Box` merely to obtain a pointer unless the surrounding API is ownership-based, because that may change deallocation requirements.

Deliverable:
- Functional Rust implementation of `rpl_malloc` with direct semantic correspondence to the C source.

### Phase 3: Error and Edge-Case Alignment

- Verify that the Rust implementation matches the C behavior for:
  - zero-size requests,
  - allocation failure signaling,
  - any overflow-sensitive size normalization implied by the source.
- Ensure no undefined behavior is introduced through invalid `Layout` construction or null-pointer misuse.
- Keep unsafe code limited to the minimal allocation call boundary and document why each unsafe block is sound.

Deliverable:
- Reviewed implementation with constrained unsafe usage and explicit edge-case handling.

### Phase 4: Testing and Integration Validation

- Add unit tests covering:
  - normal non-zero allocation request,
  - zero-size request behavior,
  - result-form correctness for success/failure paths as far as can be tested deterministically.
- Run `cargo test` and confirm the module compiles cleanly in the target branch.
- Verify that any existing callers in the Rust port build against the selected signature without requiring broadened APIs or adapter layers.

Deliverable:
- Passing tests and integrated module replacement for `gnu/malloc.c`.