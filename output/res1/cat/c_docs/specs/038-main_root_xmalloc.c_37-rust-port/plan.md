# Implementation Plan

## Summary

This module ports the allocation helpers from `xmalloc.c` into Rust with behavior centered on checked size computation, allocation growth support, zero-initialized allocation, and consistent failure handling for impossible or overflowing allocation requests.

The Rust implementation should stay narrowly scoped to the existing C file and function set. The port should migrate the semantics of the current helpers rather than introduce a broader allocator abstraction. The core technical approach is:

- represent the C allocation helpers as Rust functions in a single module corresponding to `xmalloc.c`
- use `std` allocation primitives built around owned byte buffers and size-checked capacity calculations
- convert C integer-overflow-sensitive allocation arithmetic into explicit checked operations with `checked_mul`, `checked_add`, and bounded growth logic
- preserve the C module’s fail-fast allocation model by centralizing out-of-memory / overflow termination in internal helper logic rather than returning recoverable errors
- keep all implementation decisions focused on replacing the existing file and functions with Rust equivalents on the target branch

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - keep allocation helpers as thin wrappers over standard allocation/container behavior
  - avoid redundant initialization except where zeroed allocation is required
  - preserve amortized growth behavior for reallocation helpers
  - ensure overflow checks are constant-time and occur before allocation attempts
  - maintain behavior suitable for frequent small and medium-sized allocations without adding abstraction overhead

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xmalloc.c` | `src/xmalloc.rs` | Single-module port of allocation helper logic |
| `xmalloc.c` exported functions | `src/xmalloc.rs` public functions | Keep function coverage aligned with the C module |
| internal allocation failure path in `xmalloc.c` | `src/xmalloc.rs` private helper(s) | Centralize fail-fast behavior for overflow/allocation failure |

### Function Mapping

| C Function | Rust Mapping | Migration Notes |
|---|---|---|
| `_GL_ATTRIBUTE_PURE` | omitted | C attribute macro has no direct Rust equivalent; rely on normal compiler optimization |
| `xmalloc` | `pub fn xmalloc(size: usize) -> Vec<u8>` or equivalent owned allocation form | Provide exact-size byte allocation with failure-on-error semantics |
| `ximalloc` | `pub fn ximalloc(size: isize) -> Vec<u8>` | Validate signed input before converting to `usize` |
| `xcharalloc` | `pub fn xcharalloc(size: usize) -> Vec<u8>` | Alias-level wrapper unless call sites require distinct naming |
| `xrealloc` | `pub fn xrealloc(buf: Vec<u8>, size: usize) -> Vec<u8>` | Resize owned byte storage to requested size |
| `xirealloc` | `pub fn xirealloc(buf: Vec<u8>, size: isize) -> Vec<u8>` | Signed-size validation plus resize |
| `xreallocarray` | `pub fn xreallocarray(buf: Vec<u8>, n: usize, s: usize) -> Vec<u8>` | Checked multiplication before resize |
| `xireallocarray` | `pub fn xireallocarray(buf: Vec<u8>, n: isize, s: isize) -> Vec<u8>` | Signed validation and checked multiplication |
| `xnmalloc` | `pub fn xnmalloc(n: usize, s: usize) -> Vec<u8>` | Checked `n * s` allocation |
| `xinmalloc` | `pub fn xinmalloc(n: isize, s: isize) -> Vec<u8>` | Signed validation and checked multiplication |
| `x2realloc` | `pub fn x2realloc(buf: Vec<u8>, current: &mut usize) -> Vec<u8>` | Preserve growth-helper behavior using doubling/next-capacity logic |
| `x2nrealloc` | `pub fn x2nrealloc(buf: Vec<u8>, current: &mut usize, elem_size: usize) -> Vec<u8>` | Checked growth for element-count-driven reallocation |
| `xpalloc` | `pub fn xpalloc(...) -> Vec<u8>` | Port only the existing parameter/behavior contract used by current callers |
| `xzalloc` | `pub fn xzalloc(size: usize) -> Vec<u8>` | Zero-filled allocation |
| `xizalloc` | `pub fn xizalloc(size: isize) -> Vec<u8>` | Signed validation plus zero-filled allocation |

## Data Model

This module has no named C structs in the provided analysis. The data mapping is therefore limited to pointer-and-size allocation patterns.

| C Concept | Rust Representation | Notes |
|---|---|---|
| raw allocated memory block | `Vec<u8>` | Standard owned byte storage for heap allocation |
| `size_t` length/count | `usize` | Native Rust size type for allocation APIs |
| signed size parameters | `isize` | Validate non-negative values before conversion |
| reallocated memory pointer | moved/resized `Vec<u8>` | Rust ownership replaces manual pointer replacement |
| zeroed memory block | `Vec<u8>` initialized with zeros | Use `vec![0; size]` or equivalent |
| allocation failure path | non-returning helper, e.g. `fn xalloc_die(...) -> !` | Centralized fail-fast handling to match C behavior |
| overflow-sensitive multiplication/addition | checked arithmetic on `usize`/`isize` | Required before every allocation size computation |

### Data-Structure Mapping

No C structs or enums are present for this module.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and failure model

- Add `src/xmalloc.rs` as the Rust destination for `xmalloc.c`.
- Define the public function surface for the functions listed in the module analysis.
- Add private helpers for:
  - signed-to-unsigned size conversion
  - checked multiplication/addition for allocation sizes
  - centralized fail-fast termination for invalid or unallocatable requests
- Keep the implementation local to this module; do not introduce a general allocator layer.

### Deliverables

- module file created
- public function signatures established
- internal overflow/failure helpers in place

### Validation

- compile-time verification that the module builds
- unit tests for size validation helpers and overflow detection logic

## Phase 2: Port direct allocation and resize helpers

- Implement the direct allocation family first:
  - `xmalloc`
  - `ximalloc`
  - `xcharalloc`
  - `xzalloc`
  - `xizalloc`
  - `xnmalloc`
  - `xinmalloc`
- Implement the direct resize family next:
  - `xrealloc`
  - `xirealloc`
  - `xreallocarray`
  - `xireallocarray`
- Use checked arithmetic before any allocation-size calculation.
- Ensure zeroed variants explicitly produce zero-filled memory.
- Keep naming aligned with the C module even where Rust wrappers share most logic.

### Deliverables

- direct allocation helpers implemented
- realloc helpers implemented
- shared checked-size logic reused across these functions

### Validation

- unit tests covering:
  - zero-size requests
  - normal allocation sizes
  - negative signed inputs
  - multiplication overflow
  - resize to larger and smaller sizes
  - zero-initialization behavior

## Phase 3: Port growth-oriented helpers

- Implement:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- Preserve the existing C growth semantics as closely as practical, especially:
  - geometric growth behavior
  - lower-bound growth when current size is zero
  - overflow-safe count-to-byte conversions
  - limits implied by requested element size and capacity calculations
- Keep this work driven by existing function behavior only; do not generalize beyond current callers.

### Deliverables

- growth helpers implemented in `src/xmalloc.rs`
- internal growth calculation helpers added only as needed for these functions

### Validation

- unit tests covering:
  - empty-to-initial growth
  - repeated growth steps
  - overflow near `usize::MAX`
  - element-size-aware growth
  - lower/upper bound handling reflected from the C behavior

## Phase 4: Integrate and verify module parity

- Wire the module into the crate using standard Rust module declarations.
- Update current call sites on the branch to use the Rust functions from `src/xmalloc.rs`.
- Remove or stop depending on the C implementation for this module once replacement is complete.
- Confirm that the Rust implementation remains limited to the migrated file/function scope.

### Deliverables

- Rust module integrated into the crate
- call sites migrated for this module
- C module replacement completed on branch `038-main_root_xmalloc.c_37-rust-port`

### Validation

- `cargo test`
- targeted regression tests for call patterns using growth and checked-allocation helpers
- successful build with the Rust port in place