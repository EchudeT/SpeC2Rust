# Implementation Plan: module_src_depmap.c_23

## Summary

This module is a focused port of `src/depmap.c` into Rust, preserving the existing dependency-map behavior and function boundaries as closely as practical. The C code appears to implement a compact dependency matrix with element access helpers and a transitive-closure routine over that matrix. The Rust implementation should keep the same storage model orientation: a single contiguous backing buffer representing a 2D bit or byte matrix, plus helper methods for row access, mutation, membership tests, and closure computation.

The technical approach is to migrate the C module into one Rust source file that exposes a small internal API equivalent to:

- allocation/initialization of the map,
- row lookup/index computation,
- set/test operations on dependencies,
- transitive closure execution.

Memory ownership will move from manual allocation to owned Rust containers (`Vec<_>`), eliminating explicit free logic. Error handling should be explicit where allocation dimensions or indexing can fail, but should remain restrained and shaped by existing call patterns rather than introducing new abstractions.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the current asymptotic behavior of the C implementation, especially for transitive closure.
  - Use contiguous storage to maintain cache-friendly row traversal.
  - Avoid unnecessary intermediate allocations during closure computation.
  - Keep per-element access low-overhead and suitable for frequent use inside nested loops.

## Module Mapping

### C to Rust File Mapping

- `src/depmap.c` → `src/depmap.rs`

### Function Mapping

The Rust module should preserve the original functional decomposition, implemented as inherent methods and, where helpful, a private helper:

- `depmap_alloc` → `DepMap::new(...)`
- `depmap_rowptr` → private row-offset helper such as `DepMap::row_range(...)` or `DepMap::row_start(...)`
- `depmap_set` → `DepMap::set(...)`
- `depmap_isset` → `DepMap::is_set(...)`
- `transitive_closure` → private helper `fn transitive_closure(...)`
- `depmap_tc` → `DepMap::transitive_closure(...)` or `DepMap::compute_tc(...)`

### Visibility Guidance

- Keep the module API minimal.
- Expose only the Rust items required by existing project call sites.
- Keep row-pointer replacement and low-level closure helpers private unless current cross-module usage requires otherwise.

## Data Model

The input analysis reports an anonymous C data structure. The Rust plan should therefore infer a single owning structure matching the allocation and access pattern used by the listed functions.

### C Structure to Rust Structure

- anonymous depmap-related struct → `struct DepMap`

Suggested Rust shape:

```rust
pub(crate) struct DepMap {
    size: usize,
    data: Vec<u8>,
}
```

If the original C representation uses integer words or another scalar storage unit, keep the Rust backing type aligned with that existing representation rather than optimizing it into a new form. The priority is migration fidelity, not redesign.

### Representation Notes

- `size` stores the matrix dimension.
- `data` is a flat contiguous buffer sized for the full matrix.
- 2D access should be translated into index arithmetic using checked multiplication/addition where construction boundaries are established, then direct indexing in internal hot paths once invariants are guaranteed.

### Memory Management Mapping

- C heap allocation → `Vec<u8>` ownership
- C row pointer arithmetic → slice indexing / offset calculation
- Manual lifetime discipline → Rust borrow rules
- Null/failed allocation paths → `Result` only if current callers need fallible construction; otherwise use infallible `new` if dimensions are already trusted internally

### Error Handling Mapping

- Invalid dimensions or overflow in total buffer size should be handled explicitly during construction.
- Out-of-bounds element access should not silently emulate undefined C behavior.
- For internal-only helpers, prefer `debug_assert!` plus invariant-preserving callers instead of widening the public error surface without evidence.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and data representation

### Goals

- Introduce the Rust module file.
- Define the `DepMap` structure and core storage invariants.
- Port allocation-related logic first so the remaining operations have a stable base.

### Tasks

- Create `src/depmap.rs`.
- Define `DepMap` with dimension metadata and contiguous storage.
- Implement `DepMap::new(...)` as the Rust replacement for `depmap_alloc`.
- Encode matrix-size calculations in one place to avoid repeated arithmetic mistakes.
- Decide the exact backing scalar type by matching the C storage semantics used in `depmap_set`/`depmap_isset`.

### Acceptance Criteria

- A dependency map can be constructed with the correct dimensions.
- Storage length matches the original C allocation model.
- No unsafe code is introduced unless direct evidence from surrounding project integration requires it.

## Phase 2: Port row access and element mutation/query helpers

### Goals

- Replace C pointer-based row access with Rust offset/slice helpers.
- Port all direct matrix access operations before closure logic.

### Tasks

- Implement a private row offset helper corresponding to `depmap_rowptr`.
- Implement `DepMap::set(...)` for dependency insertion.
- Implement `DepMap::is_set(...)` for dependency lookup.
- Keep index calculations centralized so both methods share the same addressing rules.
- Add unit tests covering:
  - first/last row and column access,
  - repeated set operations,
  - unset vs set lookups,
  - edge dimensions such as 0 or 1 if those are valid in current usage.

### Acceptance Criteria

- The Rust accessors reproduce the same observable matrix updates as the C helpers.
- Row addressing is internal and does not expose raw mutable aliases.
- Tests validate correctness of indexing behavior.

## Phase 3: Port transitive closure logic

### Goals

- Migrate the closure algorithm with minimal structural drift from the C implementation.
- Preserve in-place update behavior if the original code mutates the map directly.

### Tasks

- Implement a private `transitive_closure` helper mirroring the C routine structure.
- Implement the public/internal entrypoint corresponding to `depmap_tc`.
- Preserve loop ordering unless Rust-specific constraints require a minor refactor for borrowing.
- Use row-based slices where it improves clarity without changing algorithmic behavior.
- Verify that closure includes indirect dependencies exactly as in the C version.

### Acceptance Criteria

- Transitive closure results match expected reachability on small hand-built matrices.
- The implementation does not allocate temporary full-size matrices unless the original algorithm requires one.
- The closure routine works correctly for trivial, sparse, and cyclic dependency cases.

## Phase 4: Integrate and validate against module behavior

### Goals

- Finalize migration quality.
- Ensure the Rust module fits existing project structure and compiles cleanly on the target branch.

### Tasks

- Wire `src/depmap.rs` into the crate using the existing module tree conventions.
- Replace or adapt call sites that previously referenced the C implementation.
- Add focused unit tests for end-to-end `depmap_tc` behavior.
- Run `cargo test` and fix any borrow-check or visibility issues with the least invasive changes.
- Review for accidental API expansion and trim any helper exposure not needed by the project.

### Acceptance Criteria

- The Rust module builds and tests successfully.
- Existing behavior represented by this C file is covered by Rust tests.
- The resulting code remains a direct migration of `src/depmap.c`, without unrelated abstractions or features.