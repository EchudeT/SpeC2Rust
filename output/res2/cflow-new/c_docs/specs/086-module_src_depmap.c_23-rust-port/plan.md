# Implementation Plan: module_src_depmap.c_23

## Summary

Port `src/depmap.c` to an idiomatic Rust module that preserves the existing dependency-map behavior and function boundaries closely. The module appears to manage a compact dependency matrix plus transitive-closure computation, so the Rust implementation should keep a dense row-based representation and migrate the current procedural API into a small Rust-owned type with methods corresponding to the C functions.

The implementation should prioritize:
- direct migration of existing allocation, row access, bit/set access, and transitive-closure logic,
- explicit ownership of matrix storage through `Vec`,
- bounds-checked indexing in safe Rust,
- minimal API surface beyond what is needed to replace the C module.

The technical approach is to replace C-managed contiguous memory with a Rust struct that owns the matrix buffer and dimension metadata, and then implement the existing operations as inherent methods or narrowly scoped free functions. Any internal helper corresponding to `transitive_closure` should remain internal to the module unless current call patterns require exposure.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the current dense-memory access pattern for dependency lookup and update.
  - Avoid per-access heap allocation.
  - Keep transitive-closure computation iterative and in-place where possible.
  - Maintain predictable indexing cost comparable to the C implementation.
  - Use contiguous storage to support cache-friendly row traversal.

## Module Mapping

### C to Rust File Mapping

- `src/depmap.c` → `src/depmap.rs`

### Function Mapping

The Rust port should stay close to the existing function layout, with only enough reshaping to fit Rust ownership and visibility rules.

- `transitive_closure`
  - Migrate as a private helper in `src/depmap.rs`
  - Prefer a private function or private associated method used by `depmap_tc`

- `depmap_alloc`
  - Migrate to constructor-style API such as `DepMap::new(...)`
  - If call-site compatibility matters inside the port, provide a thin free-function wrapper only if needed

- `depmap_rowptr`
  - Replace raw row-pointer exposure with a row-slice accessor
  - Use `&[T]` / `&mut [T]` as appropriate for the internal storage format
  - Keep this internal unless existing Rust call sites require direct row access

- `depmap_set`
  - Migrate to a mutating method on the Rust data type

- `depmap_isset`
  - Migrate to a read-only method returning `bool`

- `depmap_tc`
  - Migrate to the public transitive-closure entry point on the Rust data type

## Data Model

### C Structure Mapping

The input identifies only an anonymous data structure, so the Rust plan should define a single named struct that captures the same stored state required by the listed functions.

- **C anonymous struct / implicit matrix state**
  - → `struct DepMap`

### Proposed Rust Representation

```rust
pub struct DepMap {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}
```

If the original C implementation is bit-packed rather than byte-per-cell, keep the representation aligned with that implementation instead of expanding functionality. In that case, use:

```rust
pub struct DepMap {
    rows: usize,
    cols: usize,
    words_per_row: usize,
    data: Vec<usize>,
}
```

Final choice should be driven by the actual indexing and row-layout logic in `depmap.c`. The migration should preserve:
- contiguous row-major storage,
- deterministic row offset computation,
- in-place updates during closure computation.

### Memory Management Mapping

- C manual allocation/free
  - → Rust ownership via `Vec`
- C raw row pointers
  - → Rust slices derived from the owned buffer
- C unchecked pointer arithmetic
  - → indexed/sliced access with explicit dimension validation

### Error Handling Mapping

If the C code assumes successful allocation and valid indices:
- constructor should return `Result<DepMap, Error>` only where allocation size or dimension overflow must be handled explicitly,
- internal helpers should use checked arithmetic for row-offset computation,
- index-based methods should either:
  - preserve existing assumptions with `debug_assert!` plus internal checked indexing, or
  - return a simple `bool`/`Option` only if the C behavior already encodes invalid access handling.

Do not introduce a broad custom error system unless the existing code path requires it.

## Implementation Phases

## Phase 1: Establish Rust Module and Storage Layout

- Create `src/depmap.rs`.
- Inspect `src/depmap.c` to determine the exact matrix encoding:
  - byte matrix vs bitset matrix,
  - row width calculation,
  - dimension metadata required by all current functions.
- Define `DepMap` with only the fields necessary to support the existing behavior.
- Implement the allocation path corresponding to `depmap_alloc`:
  - convert dimension parameters to `usize`,
  - compute total storage with checked arithmetic,
  - allocate a zero-initialized `Vec`.
- Add minimal unit tests for:
  - successful construction,
  - correct storage size calculation,
  - zero-initialized state.

## Phase 2: Port Core Access Operations

- Port `depmap_rowptr` as an internal row accessor based on safe slicing.
- Port `depmap_set` using the same row/bit addressing scheme as the C code.
- Port `depmap_isset` with matching lookup semantics.
- Keep method names and behavior close to the original implementation to simplify verification against the C source.
- Add unit tests covering:
  - setting and reading individual dependencies,
  - first/last valid row and column positions,
  - row offset correctness,
  - repeated sets being idempotent if that matches the C behavior.

## Phase 3: Port Transitive Closure Logic

- Port `transitive_closure` directly from the C algorithm before considering any refactoring.
- Implement `depmap_tc` as the public/internal entry point matching current module usage.
- Preserve in-place update behavior and traversal order unless Rust safety requires minor structural changes.
- Avoid algorithm substitution; migrate the existing closure strategy as-is.
- Add tests for:
  - empty dependency map,
  - single-edge propagation,
  - multi-step propagation,
  - cycles/self-reachability behavior as implemented by the C code,
  - stability of repeated closure execution.

## Phase 4: Integrate and Validate Against Existing Usage

- Update the module declarations so the Rust code is compiled in the standard project layout.
- Adjust call sites on this branch to use the Rust `DepMap` API with the smallest necessary signature changes.
- Verify all migrated behaviors with `cargo test`.
- Perform a final review for:
  - unchecked integer conversions,
  - out-of-bounds indexing risks,
  - unnecessary exposure of internal row access,
  - deviations from original memory layout or closure semantics.