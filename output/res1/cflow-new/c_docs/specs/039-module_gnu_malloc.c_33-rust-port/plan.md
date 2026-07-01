# Implementation Plan

## Summary

Port `gnu/malloc.c` into a Rust module that preserves the existing module boundary and behavior of `rpl_malloc` without introducing broader allocation abstractions or new facilities. The Rust implementation should mirror the C intent closely: provide a small replacement allocation entry point with explicit handling for edge cases around zero-sized allocation and allocation failure behavior.

The technical approach is to implement the module as a focused Rust source file using the standard library only. The port should keep logic narrow, translate C allocation semantics into Rust-owned memory handling carefully, and make error paths explicit so the resulting code is safe and maintainable while staying behaviorally aligned with the original function.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain constant-time wrapper behavior around allocation.
  - Avoid unnecessary copies or secondary allocation layers.
  - Keep overhead effectively equivalent to a thin allocation helper.
  - Preserve predictable behavior for zero-length and failure cases.

## Module Mapping

- **C source file**: `gnu/malloc.c`
- **Rust module file**: `src/gnu/malloc.rs`

Suggested Rust module exposure:

- `src/gnu/mod.rs`
  - declares `pub mod malloc;`
- `src/gnu/malloc.rs`
  - contains the Rust port of `rpl_malloc`

Function mapping:

- `rpl_malloc` -> `rpl_malloc`

The migration should remain limited to the existing function represented in this module and should not introduce extra helper modules unless a small private helper is required inside `src/gnu/malloc.rs` for direct translation clarity.

## Data Model

This module analysis lists no module-specific C structs or composite data types.

Data mapping for this port is therefore minimal:

- **C raw allocation result**: `void *`
- **Rust representation**:
  - Prefer explicit raw pointer forms only at the module boundary if required by surrounding project interfaces.
  - Use Rust allocation primitives internally where possible, with careful conversion to the required pointer/result type.

Behavioral mapping concerns:

- **Zero-size allocation handling**:
  - C replacement allocators often normalize zero to a minimum allocation size.
  - Rust implementation should encode this explicitly rather than relying on implicit allocator behavior.

- **Allocation failure handling**:
  - C code may return null or trigger process-terminating behavior depending on surrounding conventions.
  - The Rust port must follow the actual module contract used by the project and make the chosen path explicit in code and tests.

## Implementation Phases

## Phase 1: Establish module skeleton and signature mapping

- Create `src/gnu/malloc.rs`.
- Register the module in `src/gnu/mod.rs`.
- Identify the exact Rust-visible signature needed for `rpl_malloc` based on the project’s existing call sites and porting conventions.
- Port the function body structure directly from `gnu/malloc.c` into Rust with no added features.
- Keep the implementation contained to this file unless a tiny private helper is needed for allocation-size normalization.

Exit criteria:

- The Rust module compiles in isolation within the crate structure.
- `rpl_malloc` exists with a signature compatible with intended callers.
- No functionality beyond the original C module has been added.

## Phase 2: Implement allocation semantics and failure handling

- Translate the allocation logic faithfully, with special attention to:
  - zero-size requests,
  - raw memory allocation mechanics,
  - null/failure behavior.
- Use `std::alloc` if raw allocation semantics are required to match the C behavior closely.
- Ensure any unsafe block is minimal and documented by local invariants only.
- Avoid introducing custom allocator abstractions or wrapper types not present in the source module’s needs.

Exit criteria:

- The function reproduces the intended semantics of the C implementation.
- Memory handling decisions are explicit and localized.
- Unsafe usage, if any, is limited to allocation operations and pointer conversion.

## Phase 3: Add focused tests for semantic equivalence

- Add unit tests covering the observable behavior of `rpl_malloc`.
- Include tests for:
  - non-zero allocation request success path,
  - zero-size request behavior,
  - returned pointer/result expectations consistent with the module contract.
- If direct failure simulation is impractical with standard testing, document the limitation and test the deterministic branches only.
- Confirm the module passes `cargo test`.

Exit criteria:

- Tests validate the translated behavior at the module boundary.
- The implementation builds and passes `cargo test`.
- The port remains constrained to the original file/function scope.

## Phase 4: Integration verification and cleanup

- Confirm imports, visibility, and naming align with the rest of the Rust port branch.
- Remove any temporary scaffolding used during translation.
- Recheck that the final layout maps cleanly back to `gnu/malloc.c` and only covers `rpl_malloc`.
- Verify comments describe technical invariants rather than restating C source prose.

Exit criteria:

- Final Rust file placement and naming are stable.
- The module is ready for use by migrated callers.
- The implementation scope matches the original C module without expansion.