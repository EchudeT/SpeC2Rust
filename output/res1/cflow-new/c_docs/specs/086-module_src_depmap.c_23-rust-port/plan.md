# Implementation Plan: module_src_depmap.c_23

## Summary

Port `src/depmap.c` to a single Rust module that preserves the existing dependency-map behavior and function boundaries: allocation, row access, bit setting/query, and transitive-closure computation.

The Rust implementation should replace manual memory management and raw indexing with owned standard-library containers while keeping the same compact matrix-oriented representation. The likely shape is a contiguous bitmap or byte-backed matrix stored in a dedicated `DepMap` struct, with methods corresponding directly to:

- `depmap_alloc`
- `depmap_rowptr`
- `depmap_set`
- `depmap_isset`
- `transitive_closure`
- `depmap_tc`

Technical focus should remain on a faithful migration of the existing file and functions, not on redesign. Unsafe code should be avoided unless the original row-pointer semantics require an internal slice exposure that cannot be expressed cleanly otherwise. Error handling should convert allocation and bounds-sensitive operations into explicit Rust results or debug-checked invariants, depending on how the surrounding code currently uses the module.

## Technical Context

### Language / Version
- Rust 1.78+ stable

### Primary Dependencies
- Rust standard library only:
  - `Vec`
  - slice APIs
  - integer bit operations
  - `Option` / `Result` as needed

No third-party crates are recommended because the input provides no evidence that external bitset, matrix, or error-handling libraries are necessary.

### Testing
- `cargo test`

Testing should cover:
- allocation and internal sizing
- row access behavior
- setting and querying dependencies
- idempotent repeated sets
- transitive closure on small fixed graphs
- empty/minimal-size edge cases

### Performance Goals
- Preserve the original asymptotic behavior of the C implementation.
- Keep matrix storage contiguous to maintain cache-friendly row traversal.
- Avoid per-cell heap allocation.
- Ensure transitive closure remains implemented with direct indexed access over the backing storage rather than higher-overhead abstractions.
- Minimize bounds-check overhead where naturally eliminated by structured iteration.

## Module Mapping

### C to Rust File Mapping
- `src/depmap.c` -> `src/depmap.rs`

### Function Mapping
- `depmap_alloc` -> `DepMap::new(...)` or `DepMap::with_size(...)`
- `depmap_rowptr` -> `DepMap::row_mut(...)` / `DepMap::row(...)` returning slices over the backing storage
- `depmap_set` -> `DepMap::set(...)`
- `depmap_isset` -> `DepMap::is_set(...)`
- `transitive_closure` -> private helper function or private `DepMap` method
- `depmap_tc` -> public `DepMap::transitive_closure(...)` or `DepMap::compute_tc(...)`

### Visibility Plan
- Keep the closure worker private unless external callers require direct access.
- Expose only the migrated public surface that corresponds to the original module’s externally used entry points.

## Data Model

Because the analysis reports an anonymous data structure only, the Rust plan should infer a minimal dedicated struct matching the C storage layout rather than introducing extra abstraction layers.

### C Structure Mapping
- anonymous dependency-map storage -> `struct DepMap`

### Proposed Rust Structure
```rust
pub struct DepMap {
    rows: usize,
    cols: usize, // include only if required by the original layout semantics
    data: Vec<u8>, // or Vec<usize> if the C code is word-packed
}
```

If the C implementation stores a packed bit matrix by machine word rather than byte, prefer:
```rust
pub struct DepMap {
    rows: usize,
    stride_words: usize,
    data: Vec<usize>,
}
```

The exact backing element type should follow the original C representation:
- use `u8` if the C code addresses bytes directly,
- use `usize` or a fixed-width unsigned integer if the C code uses word-level bit packing and row-pointer arithmetic by words.

### Data Handling Decisions
- Replace raw heap allocation with `Vec`.
- Replace pointer arithmetic with computed row offsets.
- Represent row access as slices derived from the contiguous backing vector.
- Preserve compact bit operations instead of expanding into `Vec<Vec<bool>>`, which would change layout and likely performance.

### Memory Management
- Ownership is fully contained in `DepMap`.
- No manual free path is required.
- Any previous null-return allocation behavior should map to `Result<DepMap, _>` only if callers currently handle allocation failure explicitly; otherwise use infallible construction with standard Rust allocation panic behavior.

### Error Handling
- Bounds-sensitive APIs should be decided based on current C assumptions:
  - internal/private paths may use `debug_assert!` if indices are guaranteed by callers,
  - public methods may return `bool`/`Option`/`Result` if the original call sites need defensive handling.
- Do not add new recovery layers beyond what is needed to safely mirror existing use.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Storage Layout
- Create `src/depmap.rs`.
- Define `DepMap` with the backing storage shape that matches the C implementation’s row layout.
- Implement constructor logic corresponding to `depmap_alloc`.
- Encode row/stride calculations explicitly and document the invariant used for indexing.
- Add basic unit tests for:
  - successful construction
  - backing storage size/stride correctness
  - zeroed initial state

## Phase 2: Port Row and Bit Access Operations
- Port `depmap_rowptr` into row-slice accessors over the contiguous buffer.
- Port `depmap_set` using the same bit/offset calculation as the C code.
- Port `depmap_isset` with matching read semantics.
- Verify repeated writes and reads against small hand-built examples.
- Keep method names and call ordering close to the C code to simplify review.

## Phase 3: Port Transitive Closure Logic
- Port `transitive_closure` as a private helper operating directly on `DepMap` storage.
- Port `depmap_tc` as the public closure entry point.
- Preserve the original iteration order unless Rust safety constraints require only local refactoring.
- Ensure no unnecessary cloning of the matrix during closure computation.
- Add targeted tests for:
  - linear dependency chains
  - branching dependencies
  - already-closed graphs
  - empty/self-dependency boundary cases as applicable

## Phase 4: Integrate and Tighten Semantics
- Wire the new Rust module into the project’s standard module tree on branch `086-module_src_depmap.c_23-rust-port`.
- Align signatures with surrounding call sites and adjust visibility only as needed for compatibility.
- Review for any remaining C-style assumptions:
  - integer width expectations
  - row-stride calculations
  - index validity assumptions
- Remove any unnecessary unsafe code if introduced during initial migration.
- Finalize unit tests under `cargo test` and confirm the module remains scope-limited to the original file’s responsibilities.