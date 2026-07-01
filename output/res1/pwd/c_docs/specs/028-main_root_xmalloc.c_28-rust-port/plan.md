# Implementation Plan

## Summary

Port `xmalloc.c` into a single Rust allocation utility module that preserves the original call surface and failure behavior as closely as practical within Rust. The Rust implementation should focus on migrating the existing allocation helper functions only: size-checked allocation, reallocation, zero-initialized allocation, and geometric growth helpers.

The technical approach is to implement these routines in one Rust source file, using `std::alloc` and explicit checked arithmetic to mirror the C code’s overflow-sensitive size computations. Since the original module is centered on allocation helpers that typically abort on allocation failure rather than returning recoverable errors, the Rust port should keep that contract through process-terminating failure paths (for example via `handle_alloc_error` or a small internal abort helper), instead of introducing new `Result`-based APIs.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::alloc`, `std::ptr`, `std::mem`, `std::process` if needed for fatal termination behavior)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-overhead allocation wrappers without adding abstraction-heavy layers.
  - Keep overflow checks explicit and branch-minimal.
  - Maintain behavior suitable for low-level utility use, especially for repeated resize/growth operations.
  - Avoid extra copying beyond what reallocation/growth semantics require.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xmalloc.c` | `src/main_root_xmalloc.rs` | Single-module port of allocation helpers; keep functions grouped in one file to match the original migration unit. |

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `_GL_ATTRIBUTE_PURE` | Omit | C attribute macro has no direct runtime equivalent; rely on normal Rust function semantics. |
| `xmalloc` | `pub unsafe fn xmalloc(size: usize) -> *mut u8` | Raw allocation wrapper with fatal failure semantics. |
| `ximalloc` | `pub unsafe fn ximalloc(s: usize) -> *mut u8` | Same behavior as C intent; preserve separate entry point if present in callers. |
| `xcharalloc` | `pub unsafe fn xcharalloc(n: usize) -> *mut u8` | Thin wrapper for byte allocation. |
| `xrealloc` | `pub unsafe fn xrealloc(p: *mut u8, size: usize) -> *mut u8` | Raw reallocation preserving null-pointer semantics if required by the original. |
| `xirealloc` | `pub unsafe fn xirealloc(p: *mut u8, s: usize) -> *mut u8` | Preserve separate migrated function. |
| `xreallocarray` | `pub unsafe fn xreallocarray(p: *mut u8, n: usize, s: usize) -> *mut u8` | Checked multiplication before reallocating. |
| `xireallocarray` | `pub unsafe fn xireallocarray(p: *mut u8, n: usize, s: usize) -> *mut u8` | Same as above, retained as a distinct migrated symbol. |
| `xnmalloc` | `pub unsafe fn xnmalloc(n: usize, s: usize) -> *mut u8` | Checked multiplication before allocation. |
| `xinmalloc` | `pub unsafe fn xinmalloc(n: usize, s: usize) -> *mut u8` | Preserve separate migrated entry point. |
| `x2realloc` | `pub unsafe fn x2realloc(p: *mut u8, pn: &mut usize) -> *mut u8` | Geometric growth helper updating size/count by mutable reference. |
| `x2nrealloc` | `pub unsafe fn x2nrealloc(p: *mut u8, pn: &mut usize, s: usize) -> *mut u8` | Checked growth helper for element counts and element size. |
| `xpalloc` | `pub unsafe fn xpalloc(pa: *mut u8, pn: &mut usize, n_incr_min: usize, n_max: Option<usize>, s: usize) -> *mut u8` | Preserve original growth policy with checked bounds; `n_max` may use `Option<usize>` to represent “unbounded” if that matches the call pattern. |
| `xzalloc` | `pub unsafe fn xzalloc(size: usize) -> *mut u8` | Zero-initialized allocation wrapper. |
| `xizalloc` | `pub unsafe fn xizalloc(s: usize) -> *mut u8` | Preserve separate migrated function. |

## Data Model

This module has no explicit C structs to migrate.

### Data-Structure Mapping

| C Construct | Rust Construct | Notes |
|---|---|---|
| Raw heap pointer (`void *`, `char *`) | `*mut u8` | Keep raw-pointer API to match low-level allocation helper usage and avoid inventing ownership abstractions not present in the source module. |
| Size parameters (`size_t`) | `usize` | Direct width-compatible mapping for allocation sizes and counts. |
| Optional allocation limit semantics | `Option<usize>` or exact `usize` sentinel chosen from source behavior | Final choice should be driven by the original `xpalloc` contract in the C file. |

### Memory and Error Handling Decisions

- Use `checked_mul`, `checked_add`, and guarded growth calculations before calling alloc/realloc paths.
- Use `std::alloc::{alloc, realloc, alloc_zeroed, Layout}` for raw allocation operations.
- Centralize fatal out-of-memory and overflow handling in a small internal helper to keep all public wrappers behaviorally consistent.
- Keep functions `unsafe` where they operate on raw pointers or rely on caller-provided allocation invariants.
- Do not convert the API into owned containers like `Vec` or `Box`, since this would alter the module’s role and caller expectations.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and core allocation primitives

- Add `src/main_root_xmalloc.rs`.
- Introduce internal helpers for:
  - layout construction from `usize`
  - checked size multiplication/addition
  - fatal allocation failure handling
- Implement the direct allocation routines first:
  - `xmalloc`
  - `ximalloc`
  - `xcharalloc`
  - `xzalloc`
  - `xizalloc`
- Implement `_GL_ATTRIBUTE_PURE` as no-op by omission rather than replacement.

### Phase 1 Exit Criteria

- Module compiles.
- Basic allocation and zero-allocation tests pass under `cargo test`.
- Overflow in requested size is handled by the chosen fatal path consistently.

## Phase 2: Add reallocation and counted-allocation functions

- Implement:
  - `xrealloc`
  - `xirealloc`
  - `xnmalloc`
  - `xinmalloc`
  - `xreallocarray`
  - `xireallocarray`
- Preserve null-pointer handling and zero-size corner cases according to the C source behavior.
- Ensure multiplication overflow is checked before any array-style allocation or reallocation.

### Phase 2 Exit Criteria

- Unit tests cover:
  - fresh allocation via realloc-style entry where applicable
  - resizing existing allocations
  - array-size overflow detection
- Public function set matches the migrated C file for all direct and counted allocation helpers.

## Phase 3: Port growth-policy helpers

- Implement:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- Reproduce the C growth policy in Rust with explicit checked arithmetic and mutable size/count updates.
- Keep parameter mutation semantics close to the original by using `&mut usize` for in/out size values.
- Resolve `n_max` representation based on the exact C contract while keeping the implementation local to this module.

### Phase 3 Exit Criteria

- Growth helpers compile and pass tests for:
  - initial growth from zero capacity
  - repeated expansion
  - upper-bound enforcement
  - overflow-triggered fatal handling

## Phase 4: Validation and cleanup against the original module surface

- Review all function names, parameter order, and semantics against `xmalloc.c`.
- Minimize duplicate internal logic by routing wrappers through shared checked helpers without changing the external function list.
- Add focused tests for edge conditions:
  - zero-size requests
  - near-`usize::MAX` arithmetic
  - zeroed allocation contents
  - growth counter updates after reallocation

### Phase 4 Exit Criteria

- The Rust file is a complete migration unit for `xmalloc.c`.
- No extra modules or capabilities have been introduced beyond the original function set.
- `cargo test` passes cleanly.