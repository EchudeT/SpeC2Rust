# Implementation Plan

## Summary
Port `gnu/free.c` into a Rust module that preserves the existing module boundary and behavior of `rpl_free` without introducing new capabilities. The Rust implementation should focus on the same low-level ownership and deallocation semantics as the C source, while making null handling and unsafe memory interaction explicit.

Because the source module contains a single function, the Rust work should remain narrowly scoped:
- migrate the logic of `rpl_free`;
- keep the module structure minimal and aligned to the original file;
- isolate any unavoidable unsafe operations;
- avoid adding abstractions beyond what is required to represent the C behavior in Rust.

The preferred technical approach is to implement the replacement free routine as a small Rust module function whose interface matches the surrounding porting strategy for C-style pointers. If the wider project is preserving C-callable signatures internally, the function should accept raw pointers and handle null pointers safely before deallocation. If the module is only consumed by Rust code, use the narrowest raw-pointer-based internal API necessary to preserve behavior.

## Technical Context

### Language/Version
- Rust stable
- Recommended minimum version: **1.76** or newer

### Primary Dependencies
- Rust standard library only
- `core::ptr` / `std::ptr` for raw pointer checks and manipulation
- `std::alloc` only if required by the existing project’s allocation/deallocation strategy

No third-party crates are recommended because the input provides no evidence that external dependencies are needed.

### Testing
- `cargo test`

Tests should remain focused on:
- null-pointer behavior;
- basic deallocation path validity;
- interface compatibility within the Rust port.

### Performance Goals
- Match the constant-time behavior expected of a thin deallocation wrapper
- Introduce no additional allocations
- Keep wrapper overhead effectively negligible relative to the underlying free operation
- Preserve predictable low-level behavior suitable for a direct C-to-Rust port

## Module Mapping

### C to Rust File Mapping
- `gnu/free.c` -> `src/gnu/free.rs`

### Function Mapping
- `rpl_free` -> `pub(crate)` or module-visible Rust function `rpl_free`

Visibility should be limited to the minimum required by the existing project structure. Do not broaden the public API unless the surrounding port already requires it.

### Suggested Rust Module Layout
- `src/gnu/mod.rs` declares `mod free;`
- `src/gnu/free.rs` contains the migrated `rpl_free` implementation

If the current Rust crate already has a different established module tree for GNU compatibility shims, place `free.rs` within that existing tree rather than introducing a parallel structure.

## Data Model

This module has no standalone C structs or custom data structures to port.

### C to Rust Type Mapping
- `void *` -> `*mut core::ffi::c_void` or another raw mutable pointer type consistent with the project
- null pointer -> `core::ptr::null_mut()`

### Memory Model Notes
- Rust ownership types such as `Box<T>` should not replace raw-pointer semantics here unless the broader port guarantees allocator compatibility and equivalent call patterns.
- The deallocation path must only free pointers that were allocated by the matching allocator strategy used elsewhere in the port.
- Unsafe code should be kept local to the deallocation boundary.

## Implementation Phases

### Phase 1: Establish Module Skeleton
- Create `src/gnu/free.rs`
- Wire the file into the existing Rust module tree
- Define the Rust signature for `rpl_free` based on the surrounding port’s pointer conventions
- Add minimal inline documentation describing allocator and pointer assumptions

### Phase 2: Port `rpl_free` Logic
- Translate the C function body directly, preserving null-handling behavior
- Implement the deallocation path with the smallest necessary unsafe block
- Ensure the function does not introduce Rust-side ownership claims that differ from the C contract
- Keep error handling aligned with C behavior; if the original function has no error return, do not add one

### Phase 3: Validate Memory Semantics
- Review allocator compatibility with the rest of the ported code
- Confirm that null pointers are accepted and handled without invalid access
- Verify that the function remains a thin wrapper and does not alter pointer state beyond the original behavior
- Check that visibility and call sites match the original module role

### Phase 4: Add Focused Tests
- Add unit tests for null-pointer acceptance where meaningful in the project structure
- Add tests covering any project-level wrapper or call path that reaches `rpl_free`
- Ensure tests compile and run under `cargo test`
- Keep tests narrow and behavior-based; do not add benchmark or stress-test infrastructure