# Implementation Plan

## Summary

Port `gnu/xmalloc.c` to a Rust module that preserves the existing allocation-oriented API surface and behavior boundaries without adding new capabilities. The Rust implementation should centralize checked size computations, use standard allocation primitives (`Vec`, `Box`, and allocation-aware helpers from `std`) where possible, and keep failure handling explicit and consistent with the project’s expectations for fatal allocation failure.

The module should focus on migrating the existing functions:

- `xmalloc`
- `ximalloc`
- `xcharalloc`
- `xrealloc`
- `xirealloc`
- `xreallocarray`
- `xireallocarray`
- `xnmalloc`
- `xinmalloc`
- `x2realloc`
- `x2nrealloc`
- `xpalloc`
- `xzalloc`
- `xizalloc`
- `xcalloc`

Technical approach:

- Represent allocation sizes with `usize` in Rust.
- Recreate C overflow checks using `checked_mul`, `checked_add`, and bounded growth logic.
- Keep the Rust module narrowly scoped to allocation and reallocation helpers corresponding to the C file.
- Use explicit error/abort paths for allocation overflow or allocation failure, matching the original module’s role as an `xmalloc` utility layer.
- Preserve migration order around shared internal helpers first, then thin public wrappers.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain constant-overhead wrappers around standard allocation paths.
  - Avoid redundant zeroing except where required by `xzalloc`, `xizalloc`, and `xcalloc`.
  - Preserve amortized growth behavior for resizing helpers such as `x2realloc`, `x2nrealloc`, and `xpalloc`.
  - Ensure overflow checks are branch-light and performed before allocation.

## Module Mapping

### Source Mapping

| C File | Rust File |
|---|---|
| `gnu/xmalloc.c` | `src/gnu/xmalloc.rs` |

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `xmalloc` | `pub fn xmalloc(size: usize) -> Vec<u8>` or allocation helper returning owned buffer | Base uninitialized/byte-capacity allocation semantics should be represented using a safe owned allocation strategy consistent with actual call-site needs. |
| `ximalloc` | `pub fn ximalloc(size: usize) -> Vec<u8>` | Integer-sized wrapper; in Rust this should collapse to `usize`-based handling. |
| `xcharalloc` | `pub fn xcharalloc(size: usize) -> Vec<u8>` | Alias-level helper for character storage. |
| `xrealloc` | `pub fn xrealloc(buf: Vec<u8>, size: usize) -> Vec<u8>` | Reallocation wrapper with checked target size. |
| `xirealloc` | `pub fn xirealloc(buf: Vec<u8>, size: usize) -> Vec<u8>` | Same underlying implementation as `xrealloc`. |
| `xreallocarray` | `pub fn xreallocarray(buf: Vec<u8>, n: usize, s: usize) -> Vec<u8>` | Must perform checked multiplication before resize. |
| `xireallocarray` | `pub fn xireallocarray(buf: Vec<u8>, n: usize, s: usize) -> Vec<u8>` | Same checked-array resize path. |
| `xnmalloc` | `pub fn xnmalloc(n: usize, s: usize) -> Vec<u8>` | Checked multiplication followed by allocation. |
| `xinmalloc` | `pub fn xinmalloc(n: usize, s: usize) -> Vec<u8>` | Same implementation path as `xnmalloc`. |
| `x2realloc` | `pub fn x2realloc(buf: Vec<u8>, size: &mut usize) -> Vec<u8>` | Growth helper updating size according to original doubling policy. |
| `x2nrealloc` | `pub fn x2nrealloc(buf: Vec<u8>, pn: &mut usize, s: usize) -> Vec<u8>` | Count-based growth helper with checked element-size multiplication. |
| `xpalloc` | `pub fn xpalloc(buf: Vec<u8>, pn: &mut usize, n_incr_min: usize, n_max: Option<usize>, s: usize) -> Vec<u8>` | Preserve bounded growth and overflow checks; use `Option<usize>` for “no max” if needed by migrated call sites. |
| `xzalloc` | `pub fn xzalloc(size: usize) -> Vec<u8>` | Zero-initialized allocation. |
| `xizalloc` | `pub fn xizalloc(size: usize) -> Vec<u8>` | Same zeroed allocation path. |
| `xcalloc` | `pub fn xcalloc(n: usize, s: usize) -> Vec<u8>` | Checked multiplication plus zero initialization. |

### Internal Helper Mapping

The Rust file should introduce only the minimum internal helpers needed to avoid duplicating checked arithmetic and growth logic, for example:

- checked size multiplication helper
- checked size addition helper
- growth computation helper
- common fatal allocation path

These helpers should remain private to `src/gnu/xmalloc.rs`.

## Data Model

This module has no module-specific C structs to port.

### Scalar and Pointer Mapping

| C Type/Concept | Rust Type | Notes |
|---|---|---|
| `size_t` | `usize` | Direct size representation. |
| raw allocated memory block | `Vec<u8>` or `Box<[u8]>` depending on call-site requirement | Prefer one owned representation consistently within this module. |
| nullable pointer input to realloc | owned buffer parameter or `Option<...>` only if required by migrated callers | Avoid introducing nullable semantics unless the original call pattern requires it. |
| element count × element size computation | `usize` with `checked_mul` | Required to preserve overflow protection. |

### Error Model

| C Behavior | Rust Representation | Notes |
|---|---|---|
| allocation failure is fatal | dedicated non-returning helper, e.g. `fn xalloc_die() -> !` | Keep behavior centralized. |
| size overflow before allocation | route through same fatal path | Preserve module semantics. |

## Implementation Phases

## Phase 1: Establish module skeleton and shared arithmetic/allocation helpers

Create `src/gnu/xmalloc.rs` and wire it into the existing crate module tree using standard Rust layout only as needed for this file migration.

Work items:

- Add the Rust module file corresponding to `gnu/xmalloc.c`.
- Define the module-local fatal error function for overflow/allocation failure.
- Implement private helpers for:
  - checked multiplication of allocation counts and element sizes
  - checked addition where growth logic needs it
  - converting requested logical sizes into concrete allocation sizes
- Decide and document one owned byte-buffer representation for this module (`Vec<u8>` preferred unless caller migration requires a different shape).
- Add unit tests for arithmetic edge cases:
  - zero sizes
  - exact-boundary sizes
  - multiplication overflow
  - addition overflow

Completion criteria:

- Module compiles.
- Shared helpers are test-covered.
- No public API beyond what is needed for the migrated C functions.

## Phase 2: Port direct allocation and reallocation entry points

Implement the functions that map directly to allocate/resize operations without growth-policy state.

Work items:

- Port:
  - `xmalloc`
  - `ximalloc`
  - `xcharalloc`
  - `xrealloc`
  - `xirealloc`
  - `xnmalloc`
  - `xinmalloc`
  - `xzalloc`
  - `xizalloc`
  - `xcalloc`
  - `xreallocarray`
  - `xireallocarray`
- Collapse duplicate integer/character wrappers onto the same internal Rust implementation where semantics are identical.
- Ensure zero-initializing functions use the zeroed path only for the relevant APIs.
- Ensure array-based functions validate `n * s` before allocation/reallocation.
- Keep signatures aligned with the actual ownership model selected in Phase 1 rather than emulating C pointers mechanically.

Testing focus:

- successful allocation for small sizes
- zero-initialization verification
- resize preserving existing prefix contents
- overflow paths for `n * s`
- behavior for zero-length requests as expected by migrated call sites

Completion criteria:

- All non-growth APIs from `gnu/xmalloc.c` are present in Rust.
- Duplicate wrappers are reduced to thin forwarding functions.
- Unit tests cover both normal and overflow cases.

## Phase 3: Port growth-policy helpers

Implement the functions whose primary value is capacity-growth computation.

Work items:

- Port:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- Recreate the original growth policy in Rust:
  - initial allocation when current size/count is zero
  - geometric growth when expanding
  - minimum increment guarantees
  - optional or bounded maximum element count handling
- Route all computed sizes through the checked arithmetic helpers from Phase 1.
- Keep mutation of tracked size/count explicit through `&mut usize` parameters or the narrowest equivalent required by migrated callers.

Testing focus:

- growth from zero state
- repeated growth progression
- minimum increment behavior
- maximum-bound clamping/failure behavior
- overflow protection during growth computation
- preservation of existing data after growth

Completion criteria:

- Growth helpers match the original control flow and size-update responsibilities.
- Shared arithmetic helpers are reused consistently.
- Tests cover edge-driven growth scenarios.

## Phase 4: Integration cleanup and migration validation

Finalize the module as a direct replacement for the C file within the Rust branch.

Work items:

- Review call sites on branch `062-module_gnu_xmalloc.c_56-rust-port` and adjust only the signatures needed to consume the Rust-owned allocation representation.
- Remove any temporary duplication created during phased porting.
- Ensure naming and visibility are limited to the migrated module surface.
- Run `cargo test` and fix any behavior mismatches revealed by downstream usage.
- Confirm no extra modules, wrappers, or utility layers were introduced beyond this file’s migration needs.

Completion criteria:

- `gnu/xmalloc.c` functionality is represented in `src/gnu/xmalloc.rs`.
- Tests pass with the final module wiring.
- The port remains narrowly scoped to the original file’s responsibilities.