# Implementation Plan: module_gnu_xmalloc.c_56

## Summary

Port `gnu/xmalloc.c` to a single Rust module that preserves the current allocation-helper behavior and function boundaries as closely as practical. The Rust implementation should focus on migrating the existing allocation entry points (`xmalloc`, `xrealloc`, `xcalloc`, growth helpers, and zero-initializing variants) into idiomatic but low-level Rust code using the standard library’s allocation APIs and checked size arithmetic.

The module should keep the original technical intent:

- centralize memory allocation helpers,
- detect size multiplication/addition overflow before allocation,
- route allocation failure into a single non-returning failure path,
- preserve byte-count-based APIs rather than redesigning around higher-level containers.

Because the source module is an allocator utility layer, the Rust port should avoid introducing broader abstractions. The implementation should operate primarily on raw pointers and `std::alloc` / `std::ptr` facilities where needed, with careful use of `checked_mul`, `checked_add`, and explicit null/failure handling. The migration should preserve the existing family structure of wrappers such as “integer-sized count helpers”, “realloc array helpers”, and “grow-by-policy helpers”.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum practical compiler target: Rust 1.75+

### Primary Dependencies

- Standard library only
- No third-party crates are recommended based on the provided module scope

### Testing

- `cargo test`

### Performance Goals

- Match the C module’s role as a thin allocation helper layer with minimal abstraction overhead
- Keep size checking constant-time and branch-light
- Avoid unnecessary intermediate buffers or container conversions
- Preserve amortized growth behavior in `x2realloc`, `x2nrealloc`, and `xpalloc`
- Keep zero-fill behavior limited to the functions that require it

## Module Mapping

### Source to Destination

- C source: `gnu/xmalloc.c`
- Rust destination: `src/gnu/xmalloc.rs`

If the project already exposes GNU-derived helpers through a module tree, the expected Rust module path should remain narrow and direct:

- `src/gnu/mod.rs` -> declares `pub mod xmalloc;`
- `src/gnu/xmalloc.rs` -> contains the migrated functions only

No additional helper submodules should be introduced unless required by existing project structure.

### Function Mapping

Each C function should map to a Rust function with the same conceptual role and closely corresponding name.

- `xmalloc` -> `pub unsafe fn xmalloc(size: usize) -> *mut u8`
- `ximalloc` -> `pub unsafe fn ximalloc(n: isize) -> *mut u8`
- `xcharalloc` -> `pub unsafe fn xcharalloc(n: usize) -> *mut u8`
- `xrealloc` -> `pub unsafe fn xrealloc(p: *mut u8, size: usize) -> *mut u8`
- `xirealloc` -> `pub unsafe fn xirealloc(p: *mut u8, n: isize) -> *mut u8`
- `xreallocarray` -> `pub unsafe fn xreallocarray(p: *mut u8, n: usize, s: usize) -> *mut u8`
- `xireallocarray` -> `pub unsafe fn xireallocarray(p: *mut u8, n: isize, s: isize) -> *mut u8`
- `xnmalloc` -> `pub unsafe fn xnmalloc(n: usize, s: usize) -> *mut u8`
- `xinmalloc` -> `pub unsafe fn xinmalloc(n: isize, s: isize) -> *mut u8`
- `x2realloc` -> `pub unsafe fn x2realloc(p: *mut u8, pn: &mut usize) -> *mut u8`
- `x2nrealloc` -> `pub unsafe fn x2nrealloc(p: *mut u8, pn: &mut usize, s: usize) -> *mut u8`
- `xpalloc` -> `pub unsafe fn xpalloc(p: *mut u8, pn: &mut usize, n_incr_min: isize, n_max: isize, s: usize) -> *mut u8`
- `xzalloc` -> `pub unsafe fn xzalloc(size: usize) -> *mut u8`
- `xizalloc` -> `pub unsafe fn xizalloc(n: isize) -> *mut u8`
- `xcalloc` -> `pub unsafe fn xcalloc(n: usize, s: usize) -> *mut u8`

### Mapping Notes

- Use `unsafe` function boundaries to reflect raw allocation semantics rather than hiding them behind partial safe wrappers.
- Size-related signed C arguments should be converted carefully into `usize` only after validating non-negativity.
- A single internal non-returning helper should handle overflow and allocation failure consistently.
- Where the original C module depends on realloc behavior with null pointers, preserve that behavior directly.

## Data Model

This module does not define persistent C structs in the provided analysis, so the Rust port should not invent new public data structures.

### Data-Structure Mapping

- C module-level allocator helpers -> Rust free functions in `src/gnu/xmalloc.rs`
- C raw memory blocks (`void *`) -> Rust `*mut u8`
- C size values (`size_t`) -> Rust `usize`
- C signed size-like values (`ptrdiff_t`, `idx_t`, `int`, or equivalent depending on original signatures) -> Rust `isize` for direct migration unless existing project-wide type aliases already exist

### Internal Representation Decisions

- No public structs or enums are required
- If existing surrounding code expects a project-specific size alias, prefer reusing that alias instead of introducing a new type
- Internal helper functions may use:
  - `Option<usize>` only for checked arithmetic internals,
  - `std::alloc::Layout` where allocation API requirements make it useful,
  - `core::mem::size_of::<T>()` only if a typed internal helper becomes necessary during migration

## Implementation Phases

### Phase 1: Create the Rust module skeleton and base allocation/error path

Goals:

- Add `src/gnu/xmalloc.rs`
- Wire it into the existing Rust module tree
- Establish the common failure path and basic allocation helpers

Work items:

- Create a single internal non-returning helper for allocation failure and overflow termination
- Implement the lowest-level size validation helpers:
  - signed-to-unsigned conversion checks,
  - checked multiplication for count/element-size combinations,
  - checked growth arithmetic for expansion helpers
- Implement base functions first:
  - `xmalloc`
  - `xrealloc`
  - `xzalloc`
  - `xcalloc`

Technical decisions:

- Prefer standard allocation primitives and explicit failure checks
- Preserve null-pointer acceptance for realloc-style calls
- Treat zero-size cases deliberately and consistently with Rust allocator API constraints; keep behavior aligned across all wrappers

Exit criteria:

- The module compiles
- Core allocation entry points exist
- Failure behavior is centralized rather than duplicated

### Phase 2: Port the size-wrapper family and array helpers

Goals:

- Complete the direct wrapper layer around the base allocation functions
- Preserve overflow semantics for count-based allocation calls

Work items:

- Implement:
  - `ximalloc`
  - `xcharalloc`
  - `xirealloc`
  - `xreallocarray`
  - `xireallocarray`
  - `xnmalloc`
  - `xinmalloc`
  - `xizalloc`
- Ensure all signed-argument wrappers validate negative inputs before conversion
- Route all count × element-size calculations through shared checked helpers

Technical decisions:

- Keep wrappers thin and mechanical
- Do not introduce typed allocation generics unless already required by surrounding migrated code
- Return raw pointers consistently across the module

Exit criteria:

- All direct wrappers compile and reuse the same checked arithmetic path
- No wrapper duplicates independent overflow logic

### Phase 3: Port the growth-policy helpers

Goals:

- Migrate the allocation-growth routines without broad redesign
- Preserve in-place update patterns on capacity-like counters

Work items:

- Implement:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- Translate the C growth strategy into Rust using:
  - `checked_add`
  - `checked_mul`
  - explicit lower-bound and upper-bound handling
- Preserve mutation of the caller-provided size/count slot via `&mut usize`

Technical decisions:

- Keep the C-style “pointer + mutable size parameter” API shape
- Use one internal helper for growth calculation if the three functions share the same arithmetic pattern
- Preserve the distinction between:
  - doubling growth helpers,
  - general-purpose growth with minimum increment and optional maximum

Exit criteria:

- Growth helpers compile
- Counter updates occur only after successful new-size calculation
- Overflow and bound violations route through the shared failure path

### Phase 4: Testing and behavioral verification

Goals:

- Verify the Rust port’s arithmetic and allocation behavior at the module boundary
- Lock in edge-case handling for future migration work

Work items:

- Add unit tests covering:
  - basic non-null allocation success for small sizes
  - zero-size call behavior consistency across `xmalloc`, `xcalloc`, and `xzalloc`
  - overflow detection in:
    - `xnmalloc`
    - `xreallocarray`
    - `x2nrealloc`
    - `xpalloc`
  - signed wrapper rejection paths for negative inputs
  - growth helper updates to the tracked size/count value
- Where direct failure paths abort or do not return, isolate arithmetic helpers enough to test overflow decisions without requiring process-abort assertions
- Run `cargo test` and resolve any allocator API mismatches

Technical decisions:

- Prefer unit tests in the same module file unless the repository already uses a different layout
- Test internal checked arithmetic helpers directly when terminal failure behavior is intentionally non-recoverable

Exit criteria:

- All migrated functions are present
- `cargo test` passes
- Overflow, negative-size, and growth-boundary behavior are covered by tests