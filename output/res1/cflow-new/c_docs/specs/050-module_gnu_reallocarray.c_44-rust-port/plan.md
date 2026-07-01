# Implementation Plan

## Summary

Port `gnu/reallocarray.c` into an idiomatic Rust module that preserves the original module’s narrow purpose: checked reallocation for an array-sized allocation request, with explicit overflow detection on `nmemb * size` before attempting to resize memory.

The Rust implementation should stay minimal and focused on migrating the existing functionality rather than introducing broader allocation abstractions. The technical approach is:

- map the single C function `reallocarray` to a single Rust-facing allocation utility in the corresponding module,
- perform multiplication overflow checks using standard-library checked arithmetic,
- use Rust allocation primitives that most closely match the original low-level behavior,
- make allocation failure and invalid layout conditions explicit through a small, module-local result/error shape,
- preserve null/zero-size edge-case handling deliberately rather than relying on incidental behavior.

This module is fundamentally about memory-size validation and safe reallocation mechanics, so the Rust port should prioritize correctness of size computation, pointer/layout transitions, and clear ownership rules.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only (`std::alloc`, `std::ptr`, `std::num` patterns via checked arithmetic)
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve constant-time overflow checking before allocation,
  - avoid extra copying beyond what reallocation requires,
  - keep the implementation as a thin wrapper over standard allocation primitives,
  - maintain behavior suitable for low-level allocation paths without adding avoidable heap metadata or indirection.

## Module Mapping

### C to Rust File Mapping

- `gnu/reallocarray.c` → `src/gnu/reallocarray.rs`

### Function Mapping

- `reallocarray` → `pub(crate)` or `pub` Rust function in `src/gnu/reallocarray.rs`

Recommended Rust signature should remain close to the original semantics of checked array reallocation, while making error handling explicit. A practical migration shape is:

```rust
pub unsafe fn reallocarray(
    ptr: *mut u8,
    nmemb: usize,
    size: usize,
    old_layout: Option<std::alloc::Layout>,
) -> Result<*mut u8, ReallocArrayError>
```

Notes for the mapping decision:

- The C API relies on raw pointers and allocator semantics; Rust should also use raw pointers for this module rather than introducing container types.
- Rust’s allocator APIs require layout knowledge; unlike C `realloc`, Rust cannot safely resize an arbitrary pointer without the original allocation layout.
- Because of that, the Rust port should explicitly carry the previous layout information needed for correct `realloc` behavior.
- If the surrounding project already has a raw-allocation convention, this function should match it exactly; otherwise, keep the module-local function narrow and explicit.

If the project requires stronger compatibility with a C-like interface, an alternate internal split is acceptable:

- checked size helper for `nmemb * size`,
- raw reallocation function using `std::alloc::realloc`.

This still remains a single-module migration and does not expand functionality.

## Data Model

This module has no C structs to migrate.

### Data-structure Mapping

- **C structs**: none
- **Rust structs/enums**:
  - introduce only a minimal module-local error enum if needed for explicit failure reporting.

Recommended shape:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReallocArrayError {
    SizeOverflow,
    InvalidLayout,
    AllocFailed,
}
```

### Primitive and Memory Mapping

- `void *` → `*mut u8`
- `size_t` → `usize`
- `nmemb * size` checked multiplication → `usize::checked_mul`
- allocation/reallocation/free behavior → `std::alloc::{alloc, realloc, dealloc, Layout}`

### Ownership Model

- Input pointer is treated as raw allocated memory owned by the caller.
- Returned pointer, on success, becomes the caller’s owned allocation handle.
- On error:
  - overflow must fail before any allocator call,
  - allocation failure must not invalidate the original pointer,
  - invalid layout must be treated as an explicit error, not undefined behavior hidden behind unchecked operations.

## Implementation Phases

## Phase 1: Establish the Rust module and size-checking logic

### Goals
- Create the Rust module corresponding to `gnu/reallocarray.c`.
- Implement the checked array-size computation exactly once and use it as the gate for all allocation paths.

### Tasks
- Add `src/gnu/reallocarray.rs`.
- Add the module declaration in the existing `src/gnu/mod.rs` or nearest equivalent existing module tree.
- Implement a private helper for total-size calculation:
  - inputs: `nmemb: usize`, `size: usize`
  - output: `Result<usize, ReallocArrayError>`
  - behavior: fail on multiplication overflow using `checked_mul`.
- Define the minimal `ReallocArrayError` enum only if the surrounding codebase does not already expose a suitable allocation error type.

### Migration Notes
- Keep overflow handling separate from allocator calls so the behavior is auditable and testable.
- Do not introduce generic allocation traits or wrappers.
- Do not infer semantics for unrelated allocation helpers; keep the logic local to this module.

### Exit Criteria
- Module compiles.
- Overflow detection is implemented and covered by unit tests for representative edge cases.

## Phase 2: Port raw reallocation behavior using Rust allocator primitives

### Goals
- Recreate the reallocation semantics with explicit layout handling and controlled unsafe code.

### Tasks
- Implement the Rust `reallocarray` function in `src/gnu/reallocarray.rs`.
- Use `Layout::from_size_align` with a fixed alignment policy appropriate to the actual allocation contract used by the surrounding project.
- Handle the primary cases explicitly:
  - `nmemb * size` overflow → `Err(SizeOverflow)`
  - null input pointer with nonzero total size → fresh allocation path
  - existing pointer with valid old layout → `realloc`
  - zero total size → choose a single documented behavior consistent with project usage, such as deallocate-and-return-null or allocator-consistent zero-size handling
- Treat allocator null returns as `Err(AllocFailed)` where applicable.
- Keep all unsafe blocks narrow and documented:
  - pointer passed to `realloc` must come from the same allocator,
  - old layout must match the original allocation,
  - returned pointer must be checked before use.

### Migration Notes
- Rust’s allocator API is stricter than C `realloc`; the plan must preserve behavior by requiring valid old layout information rather than guessing.
- Avoid implementing compatibility shims for arbitrary foreign pointers.
- Do not add panic-based error paths for normal allocation failures.

### Exit Criteria
- Function compiles with explicit unsafe boundaries.
- Null pointer, normal resize, and allocation-failure paths are represented in code.
- No unchecked multiplication or implicit layout construction remains.

## Phase 3: Validate edge cases and finalize integration

### Goals
- Confirm behavioral correctness for memory-management edge cases and complete module integration.

### Tasks
- Add unit tests covering:
  - multiplication overflow,
  - null pointer + nonzero size allocation,
  - successful growth reallocation,
  - successful shrink reallocation,
  - zero-size request behavior,
  - invalid layout rejection if exposed by the API shape.
- Verify that failure paths do not mutate ownership assumptions for the original allocation.
- Run `cargo test`.
- Align visibility (`pub(crate)` vs `pub`) with actual project usage and avoid exporting more surface area than required.

### Migration Notes
- Tests should focus on functional parity and safety-relevant conditions, not benchmark behavior.
- Keep test allocations local and deallocate them correctly to avoid leaks during test runs.
- If exact zero-size behavior depends on surrounding project conventions, document and test only the chosen convention used in this branch.

### Exit Criteria
- All tests pass under `cargo test`.
- Module is integrated into the existing Rust tree with no extra support layers.
- The ported module remains limited to the original file’s responsibility.