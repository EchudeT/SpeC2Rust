# Implementation Plan: main_root_xalignalloc.c_34

## Summary

Port the C module `xalignalloc.c` into a focused Rust module that preserves the existing allocation behavior and call shape needed by the `cat` project, without introducing broader allocator abstractions or extra utility layers.

The Rust implementation should center on a direct translation of `xalignalloc` into a small module that performs aligned heap allocation using Rust’s low-level allocation APIs from `std::alloc`. The port should explicitly model:
- alignment validation,
- zero-size and overflow-sensitive size handling as required by the original behavior,
- null/allocation-failure handling via the project’s existing Rust-side error/termination conventions where applicable.

The approach should stay close to the original C control flow so that behavior remains easy to compare during migration.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `std::alloc` for aligned allocation
  - No third-party crates recommended based on the provided module scope
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the constant-time allocator setup characteristics of the C implementation
  - Avoid unnecessary copies or intermediate buffers
  - Keep allocation path limited to a single aligned allocation request
  - Maintain behavior suitable for command-line utility execution with negligible overhead versus the C version

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `xalignalloc.c` | `xalignalloc` | `src/xalignalloc.rs` -> `pub(crate) fn xalignalloc(...)` | Direct migration of the allocation routine using `std::alloc` |
| `xalignalloc.c` | internal allocation/error logic | same Rust module | Keep helper logic local unless reuse is already required by surrounding migrated code |

## Data Model

This module does not define persistent C structs in the provided input.

### C to Rust Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| raw allocated pointer | `*mut u8` or `NonNull<u8>` internally | Prefer internal `NonNull<u8>` where useful, but preserve external raw-pointer compatibility if callers expect it |
| allocation size (`size_t`) | `usize` | Direct platform-width mapping |
| alignment value | `usize` | Must be validated as a power of two and compatible with Rust layout rules |
| null on failure / failure path | `Option<NonNull<u8>>`, `Result<_, _>`, or immediate termination depending on project convention | Final choice should match existing migrated allocator/error behavior in the Rust branch |

## Implementation Phases

### Phase 1: Inspect and Define the Rust Interface

- Review `xalignalloc.c` to capture:
  - exact function signature,
  - expected alignment preconditions,
  - behavior for zero-size requests,
  - failure behavior (return value vs fatal exit),
  - any assumptions about returned pointer usability by callers.
- Create `src/xalignalloc.rs`.
- Define the Rust function signature to match current project migration needs as closely as possible.
- Keep the module self-contained; do not introduce a general allocator facade.

### Phase 2: Port Allocation Logic

- Implement aligned allocation with `std::alloc::Layout` and `std::alloc::alloc`.
- Translate C-side validation into Rust checks:
  - invalid alignment,
  - layout construction failure from size/alignment combinations,
  - allocator failure handling.
- Preserve low-level semantics:
  - return raw pointer form if that is what current callers use,
  - avoid wrapping allocations in owned containers such as `Vec` or `Box` when raw aligned memory is the actual contract.
- Document ownership expectations clearly in code comments if deallocation is performed elsewhere.

### Phase 3: Integrate With Existing Module Structure

- Expose the new Rust module from the crate in the same narrow scope as the original C utility function.
- Update call sites migrated on this branch, if any, to use the Rust `xalignalloc` function.
- Ensure all unsafe blocks are minimal and justified:
  - allocator call,
  - raw pointer handling,
  - any conversion between `NonNull<u8>` and `*mut u8`.

### Phase 4: Verify Behavior With Tests

- Add unit tests covering:
  - successful allocation with valid power-of-two alignments,
  - alignment correctness of returned pointers,
  - handling of invalid alignment values,
  - failure/layout rejection behavior for impossible size/alignment combinations,
  - zero-size behavior if the C implementation defines it explicitly.
- If deallocation is required within tests, use matching `std::alloc::dealloc` with the original `Layout`.
- Run `cargo test` and compare behavior against the C semantics expected by surrounding code.

## Notes on Memory Management and Error Handling

- Use `std::alloc::Layout` as the Rust equivalent of C’s explicit size/alignment contract.
- Keep allocation and any corresponding deallocation layout-compatible; mismatched layouts must be avoided.
- Do not widen the API beyond what the original module requires.
- If the original function is fail-fast rather than fallible, mirror that behavior through the project’s existing Rust-side termination/error utility instead of inventing a new error framework.
- If the original function returns a nullable pointer, preserve that contract rather than forcing a richer type at module boundaries.