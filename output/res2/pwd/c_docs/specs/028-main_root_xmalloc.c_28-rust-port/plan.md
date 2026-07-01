# Implementation Plan

## Summary

Port `xmalloc.c` into a single Rust module that preserves the C module’s role as the project’s allocation helper layer. The Rust implementation should focus on migrating the existing allocation entry points and their size-growth logic, while replacing raw C heap management patterns with Rust standard-library allocation primitives.

The Rust design should keep the module narrow:

- provide Rust equivalents for the C allocation helpers,
- preserve the distinction between plain allocation, zero-initialized allocation, reallocation, and growth-oriented allocation helpers,
- centralize overflow checks and capacity-growth calculations,
- use abort/panic-on-allocation-failure semantics where the original C helpers would not return `NULL`.

Because the source module is a utility file rather than a data-owning subsystem, the Rust port should be implemented as a small internal module with free functions, not as a new abstraction layer. The main technical approach is to translate byte-count and element-count calculations into checked arithmetic over `usize`, then delegate storage creation and resizing to standard Rust containers or allocation APIs as appropriate.

## Technical Context

### Language/Version

- Rust 1.78+
  This is sufficient for stable checked arithmetic, `std::alloc`, and conventional module organization.

### Primary Dependencies

- Rust standard library only
  - `std::alloc` for low-level allocation behavior when direct size-based allocation is required
  - `std::process` only if the final project convention requires immediate process termination instead of panic
  - `std::mem` and `std::ptr` where raw memory operations are unavoidable

No third-party crates are recommended, since the input provides no evidence of external dependency requirements.

### Testing

- `cargo test`

Test coverage should focus on:

- successful allocation paths for representative sizes,
- zero-length and minimal-length behavior,
- overflow detection in multiplication and growth calculations,
- reallocation growth behavior,
- zero-initialization behavior for `xzalloc`/`xizalloc` equivalents,
- abort/panic expectations for impossible allocations where testable through internal helper factoring.

### Performance Goals

- Match the C module’s operational role as a thin allocation helper with negligible wrapper overhead.
- Use `usize` checked arithmetic to prevent overflow without adding unnecessary indirection.
- Preserve amortized growth behavior for `x2realloc`, `x2nrealloc`, and `xpalloc` equivalents.
- Avoid extra copying beyond what reallocation semantics require.
- Keep allocation paths based on standard-library primitives so generated code remains close to direct allocator usage.

## Module Mapping

### C to Rust File Mapping

- `xmalloc.c` -> `src/xmalloc.rs`

If the project already exposes these helpers from the crate root or a main-cluster module tree, wire in only the minimal existing `mod`/`pub(crate)` declarations needed to replace the C file’s functionality.

### Function Mapping

The C module exposes allocator helpers as free functions. Keep the same overall shape in Rust as module-level functions.

- `_GL_ATTRIBUTE_PURE`
  - No direct Rust equivalent.
  - Apply no special marker unless a specific helper can naturally be `#[inline]`.
  - Treat this as a C annotation with no migration target.

- `xmalloc`
  - Rust: `pub(crate) fn xmalloc(size: usize) -> /* allocation result type */`
  - Purpose: allocate uninitialized-or-byte-buffer storage of `size` bytes with non-optional success semantics.

- `ximalloc`
  - Rust: `pub(crate) fn ximalloc(size: usize) -> /* allocation result type */`
  - Keep as a distinct wrapper only if the C code distinguishes signed/input-normalized sizing. Otherwise implement as a thin alias over the same checked path.

- `xcharalloc`
  - Rust: `pub(crate) fn xcharalloc(size: usize) -> Vec<u8>` or equivalent byte buffer helper.
  - Map directly to byte-oriented allocation.

- `xrealloc`
  - Rust: `pub(crate) fn xrealloc(buf: /* existing allocation */, size: usize) -> /* resized allocation */`
  - Implement with explicit resize semantics and failure-as-fatal behavior.

- `xirealloc`
  - Rust: `pub(crate) fn xirealloc(...) -> ...`
  - Keep only as a wrapper over `xrealloc` unless signed-size normalization requires a separate checked entry point.

- `xreallocarray`
  - Rust: `pub(crate) fn xreallocarray(..., n: usize, s: usize) -> ...`
  - Centralize `n * s` checked multiplication before resize.

- `xireallocarray`
  - Rust: `pub(crate) fn xireallocarray(..., n: usize, s: usize) -> ...`
  - Same migration approach as above.

- `xnmalloc`
  - Rust: `pub(crate) fn xnmalloc(n: usize, s: usize) -> ...`
  - Allocate `n * s` bytes/elements after checked multiplication.

- `xinmalloc`
  - Rust: `pub(crate) fn xinmalloc(n: usize, s: usize) -> ...`
  - Thin wrapper unless the original code has a separate integer-domain contract.

- `x2realloc`
  - Rust: `pub(crate) fn x2realloc(..., size: &mut usize) -> ...`
  - Preserve growth-helper role; translate mutable size tracking to `&mut usize`.

- `x2nrealloc`
  - Rust: `pub(crate) fn x2nrealloc(..., n: &mut usize, s: usize) -> ...`
  - Preserve doubling-or-growth logic with checked arithmetic.

- `xpalloc`
  - Rust: `pub(crate) fn xpalloc(..., n: &mut usize, n_incr_min: usize, n_max: Option<usize>, s: usize) -> ...`
  - Keep as the generalized capacity-growth helper with explicit checked bounds.

- `xzalloc`
  - Rust: `pub(crate) fn xzalloc(size: usize) -> ...`
  - Allocate zeroed storage.

- `xizalloc`
  - Rust: `pub(crate) fn xizalloc(size: usize) -> ...`
  - Wrapper over zeroed allocation path unless separate integer-domain behavior exists.

### Visibility

- Prefer `pub(crate)` rather than `pub` unless external crate users already depend on these functions.
- Keep the module internal to the existing project structure; do not introduce a new allocator facade beyond this port.

## Data Model

No C structs or custom data structures are listed for this module.

### Data-Structure Mapping

- C raw memory blocks / `void *`
  - Rust: use the narrowest suitable representation per migrated function:
    - `Vec<u8>` for byte-oriented owned buffers,
    - `Box<[u8]>` if fixed-size ownership is a better fit for direct allocation-returning helpers,
    - raw pointers plus `std::alloc` only when the surrounding migrated code requires pointer-level compatibility.

- C `size_t`
  - Rust: `usize`

- C mutable out-parameters for capacities or element counts
  - Rust: `&mut usize`

- C null pointer semantics on allocation APIs
  - Rust: not exposed directly in public helper contracts; use non-optional returns and terminate/panic on allocation failure or overflow, consistent with the original helper intent.

### Data Handling Decisions

- Overflow-sensitive size computation must be factored into internal helpers such as:
  - checked byte-count multiplication,
  - checked capacity-growth computation,
  - checked conversion from growth policy to final allocation size.
- The Rust module should avoid inventing new stateful allocator types.
- If pointer-based interoperability is necessary in surrounding migrated code, isolate unsafe allocation logic inside this module and keep all size validation outside the unsafe blocks.

## Implementation Phases

### Phase 1: Establish the Rust module skeleton and checked size helpers

- Create `src/xmalloc.rs`.
- Add only the minimal module declaration required by the existing crate structure.
- Implement internal checked-arithmetic helpers for:
  - single-size validation,
  - `n * s` multiplication with overflow detection,
  - growth-size computation used by `x2nrealloc`/`xpalloc`.
- Decide and document the failure mode used throughout the module:
  - panic or immediate termination, chosen to match project convention for fatal allocation helpers.
- Add unit tests for pure arithmetic and growth calculations before wiring allocation behavior.

### Phase 2: Port the direct allocation and reallocation functions

- Implement the base allocation functions:
  - `xmalloc`
  - `ximalloc`
  - `xcharalloc`
  - `xzalloc`
  - `xizalloc`
- Implement the element-count allocation helpers:
  - `xnmalloc`
  - `xinmalloc`
- Implement the direct resize helpers:
  - `xrealloc`
  - `xirealloc`
  - `xreallocarray`
  - `xireallocarray`
- Keep wrappers thin where the C distinction is annotation-level rather than semantic.
- Add unit tests covering:
  - successful allocation,
  - zeroed allocation,
  - resize preserving existing content where applicable,
  - overflow-triggered failure paths for array-size helpers.

### Phase 3: Port the growth-oriented helpers

- Implement:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- Preserve the original growth intent:
  - capacity starts from a minimal nonzero value when needed,
  - capacity increases geometrically or according to the original helper contract,
  - maximum element bounds are enforced through checked arithmetic.
- Use `&mut usize` for tracked capacities/counts instead of C pointer out-parameters.
- Add focused tests for:
  - initial allocation from zero capacity,
  - repeated growth,
  - minimum increment handling,
  - upper-bound enforcement,
  - overflow rejection near `usize::MAX`.

### Phase 4: Integrate and replace C usage sites

- Update existing Rust call sites on branch `028-main_root_xmalloc.c_28-rust-port` to use the new `xmalloc` module functions in place of the C module behavior.
- Keep migrations local to existing consumers of `xmalloc.c`; do not introduce broader refactors.
- Remove any temporary duplication once all current call paths compile against the Rust module.
- Run `cargo test` and fix any semantic mismatches around size tracking, zero-length behavior, or growth contracts.