# Implementation Plan

## Summary

Port `gnu/xmalloc.c` to a single Rust module that preserves the current allocation API surface and semantics as closely as practical within Rust. The Rust implementation should focus on migrating the existing allocation helpers and growth calculations, not on redesigning the allocator interface or adding new facilities.

The core technical approach is:

- Implement the C functions as Rust functions in one module, maintaining close naming and behavioral correspondence.
- Use Rust’s standard allocation facilities (`std::alloc`, `Vec`, and allocation-related integer checks) to reproduce:
  - checked size multiplication,
  - realloc-style growth behavior,
  - zero-initialized allocation,
  - failure-on-overflow / failure-on-allocation semantics.
- Centralize overflow detection and allocation failure handling in small internal helpers so each migrated function remains close to the C source structure.
- Keep ownership and raw memory behavior explicit where needed, since this module models low-level heap allocation utilities rather than idiomatic container-based APIs.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended based on the available input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the asymptotic behavior of the original allocation helpers
  - Keep allocation-path overhead minimal beyond required overflow checks
  - Avoid unnecessary initialization except in the `xzalloc`, `xizalloc`, and `xcalloc` paths
  - Preserve efficient growth behavior for `x2realloc`, `x2nrealloc`, and `xpalloc`

## Module Mapping

### Source Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/xmalloc.c` | `src/module_gnu_xmalloc.rs` | Single-module port of the allocation helper implementation |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `xmalloc` | `pub fn xmalloc(size: usize) -> *mut u8` | Allocate raw block; fail on zero/overflow policy according to original behavior analysis during implementation |
| `ximalloc` | `pub fn ximalloc(size: usize) -> *mut u8` | Integer-sized wrapper retained as distinct function if required by call sites |
| `xcharalloc` | `pub fn xcharalloc(size: usize) -> *mut u8` | Thin alias/wrapper over byte allocation |
| `xrealloc` | `pub fn xrealloc(ptr: *mut u8, size: usize) -> *mut u8` | Raw reallocation preserving C-like behavior |
| `xirealloc` | `pub fn xirealloc(ptr: *mut u8, size: usize) -> *mut u8` | Wrapper variant retained for compatibility |
| `xreallocarray` | `pub fn xreallocarray(ptr: *mut u8, n: usize, s: usize) -> *mut u8` | Checked multiplication before reallocation |
| `xireallocarray` | `pub fn xireallocarray(ptr: *mut u8, n: usize, s: usize) -> *mut u8` | Wrapper variant retained for compatibility |
| `xnmalloc` | `pub fn xnmalloc(n: usize, s: usize) -> *mut u8` | Checked multiplication before allocation |
| `xinmalloc` | `pub fn xinmalloc(n: usize, s: usize) -> *mut u8` | Wrapper variant retained for compatibility |
| `x2realloc` | `pub fn x2realloc(ptr: *mut u8, size: &mut usize) -> *mut u8` | Grows allocation size and updates caller-visible size |
| `x2nrealloc` | `pub fn x2nrealloc(ptr: *mut u8, n: &mut usize, s: usize) -> *mut u8` | Element-count growth with checked multiplication |
| `xpalloc` | `pub fn xpalloc(pa: *mut u8, pn: &mut isize, n_incr_min: isize, n_max: isize, s: isize) -> *mut u8` | Preserve existing growth logic and bounds handling carefully |
| `xzalloc` | `pub fn xzalloc(size: usize) -> *mut u8` | Zero-initialized allocation |
| `xizalloc` | `pub fn xizalloc(size: usize) -> *mut u8` | Wrapper variant retained for compatibility |
| `xcalloc` | `pub fn xcalloc(n: usize, s: usize) -> *mut u8` | Checked multiplication plus zero-initialized allocation |

### Internal Helper Mapping

The Rust module should add only minimal internal helpers required to support the migrated functions:

| Purpose | Rust Helper | Notes |
|---|---|---|
| checked byte-count computation | `fn checked_mul_sizes(n: usize, s: usize) -> usize` | Triggers failure path on overflow |
| allocation failure path | `fn alloc_failure() -> !` | Centralized non-returning failure behavior |
| growth computation | `fn grow_count(...) -> usize/isize` | Only if needed to keep `x2nrealloc`/`xpalloc` readable |

## Data Model

This module does not define custom C structs in the provided input, so no direct struct migration is required.

### Type Mapping

| C Type/Concept | Rust Mapping | Notes |
|---|---|---|
| `void *` | `*mut u8` | Closest raw-pointer representation for untyped allocated memory |
| `size_t` | `usize` | Native Rust size type |
| signed count parameters used by growth helpers | `isize` | Preserve signed arithmetic expectations where present in C API |
| allocation failure with termination semantics | `-> !` helper or panic/abort path | Final choice should match surrounding project conventions if already established |
| zeroed memory block | raw allocation with zero initialization | Use `alloc_zeroed` or equivalent standard-library mechanism |

### Ownership and Lifetime Model

- Returned pointers remain raw pointers, matching the original low-level allocation interface.
- The module should not introduce Rust ownership wrappers such as `Box<[u8]>` or `Vec<u8>` in the public API, because that would change call-site expectations.
- Any temporary standard-library allocation utilities used internally must be converted carefully into stable raw pointers without introducing double-free or layout mismatches.
- Layout calculations must use checked arithmetic before calling allocation APIs.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and base allocation helpers

### Goals
- Establish the Rust file and public function surface corresponding to `gnu/xmalloc.c`
- Implement common failure and overflow-checking helpers
- Port the simplest direct-allocation functions first

### Tasks
- Add `src/module_gnu_xmalloc.rs`
- Define public functions for:
  - `xmalloc`
  - `ximalloc`
  - `xcharalloc`
  - `xzalloc`
  - `xizalloc`
  - `xcalloc`
  - `xnmalloc`
  - `xinmalloc`
- Implement internal helpers for:
  - checked multiplication
  - conversion from size inputs to `Layout`
  - centralized failure path on allocation error or overflow
- Use only standard allocation primitives from `std::alloc`

### Verification
- Unit tests for:
  - successful small allocations
  - zero-initialized allocation behavior
  - overflow detection in multiplication-based entry points
  - consistency between wrapper pairs such as `xmalloc`/`ximalloc`

## Phase 2: Port reallocation functions and preserve raw-pointer semantics

### Goals
- Migrate the realloc-style API while keeping behavior close to the C implementation
- Ensure correct handling of null pointers, size changes, and checked array resizing

### Tasks
- Implement:
  - `xrealloc`
  - `xirealloc`
  - `xreallocarray`
  - `xireallocarray`
- Decide and document the internal strategy for tracking old layout requirements needed by Rust reallocation primitives
- If exact in-place `realloc` semantics are awkward under Rust layout requirements, implement the minimal compatible raw-allocation strategy needed for this module without broadening scope
- Keep wrappers thin and behaviorally aligned with their C counterparts

### Verification
- Unit tests for:
  - realloc from null pointer
  - grow and shrink cases where supported by the chosen implementation strategy
  - overflow rejection for `xreallocarray`
  - pointer validity and content preservation for copied prefixes in reallocation scenarios

## Phase 3: Port growth-oriented helpers

### Goals
- Migrate the size-growth logic used by callers that expand buffers incrementally
- Preserve overflow checks and caller-visible count updates

### Tasks
- Implement:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- Translate growth arithmetic carefully from C into explicit checked Rust integer operations
- Keep signed/unsigned conversions localized and validated
- Preserve mutation of the size/count arguments passed by mutable reference

### Verification
- Unit tests for:
  - initial growth from zero-sized state
  - repeated growth progression
  - upper-bound handling in `xpalloc`
  - overflow and limit-failure paths
  - correct updates to `size`, `n`, or `pn` after successful growth

## Phase 4: Integration cleanup and behavioral alignment

### Goals
- Ensure the module is ready for use in the branch with minimal divergence from the original C implementation
- Finalize naming, visibility, and edge-case behavior

### Tasks
- Review all migrated functions against `gnu/xmalloc.c` for:
  - parameter handling
  - zero-size behavior
  - null-pointer behavior
  - overflow/failure semantics
- Normalize documentation comments to describe technical constraints only
- Ensure all exported functions live in the intended Rust module and use stable signatures needed by downstream migrated code
- Remove any temporary scaffolding introduced during migration

### Verification
- Run `cargo test`
- Confirm all function mappings from the original file are present
- Confirm no extra public allocation abstractions or unrelated support modules were introduced