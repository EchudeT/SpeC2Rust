# Implementation Plan

## Summary

This module ports the level-based symbol cleanup logic from `src/symbol.c` into Rust, covering the existing functions:

- `delete_level_autos`
- `delete_level_statics`

The Rust implementation should preserve the current deletion behavior and traversal order used by the C code, while replacing manual memory management with ownership- and borrow-checked data structures. The technical approach is to migrate the relevant symbol-table state and level-scoped deletion routines into a focused Rust module that operates on explicit collections and mutable references, with in-place removal where the C code previously unlinked or freed nodes.

The port should remain narrow in scope: only the data and routines needed by these two functions should be migrated, and surrounding symbol functionality should be represented only as required to support the same behavior.

## Technical Context

- **Language/Version**: Rust 1.78 or current stable compatible with the target workspace
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended based on the available module evidence
- **Testing**:
  - `cargo test`
  - Unit tests for level-scoped deletion behavior
  - Regression-style tests for empty, single-entry, and multi-entry symbol collections
- **Performance Goals**:
  - Match the asymptotic behavior of the C implementation for level-based deletion
  - Avoid unnecessary allocation during deletion passes
  - Perform in-place filtering/removal where possible
  - Keep per-delete overhead bounded to existing collection traversal costs

## Module Mapping

### C to Rust File Mapping

- `src/symbol.c`
  → `src/symbol.rs`

If the Rust project already organizes symbol logic under a different standard module path, this port should still remain in the existing symbol-focused Rust file rather than splitting into additional modules.

### Function Mapping

- `delete_level_autos`
  → `pub(crate) fn delete_level_autos(...)`
- `delete_level_statics`
  → `pub(crate) fn delete_level_statics(...)`

The exact parameter list should be derived from the surrounding migrated state in `symbol.c`. If these functions currently depend on file-level global state in C, that state should be converted into explicit mutable fields on a Rust symbol-management structure and passed via `&mut self` or `&mut SymbolTableState`.

## Data Model

The analysis only identifies anonymous C data structures, so the Rust mapping should be driven by actual usage inside `src/symbol.c`, especially the fields read or modified by the two deletion functions.

### Mapping Strategy

For each anonymous C struct used by `delete_level_autos` or `delete_level_statics`:

- Introduce a named Rust `struct` only when the deletion routines directly access its fields.
- Ignore unrelated fields until required by compilation or tests.
- Prefer `Option<T>` and collection ownership over raw pointers and manual free logic.

### Expected C-to-Rust Type Conversions

- C linked-list node used for symbol chains
  → Rust `struct` stored in:
  - `Vec<T>` with in-place retain/remove, if node identity is not externally required, or
  - index-linked storage / `Option<Box<T>>`, if next-link semantics must be preserved

- C integer level/scope fields
  → `usize`, `u32`, or `i32` depending on original comparison semantics

- C flags / storage-class markers
  → Rust `enum` or compact integer field, choosing `enum` only if the variants used by these functions are known

- C nullable pointers to symbol entries
  → `Option<Box<SymbolEntry>>`, `Option<usize>`, or `Option<NonZeroUsize>` depending on the chosen storage model

- C global symbol-table state
  → Rust `struct SymbolState` or similar owning the collections mutated by deletion

### Minimum Rust Structures to Introduce

The port should define only the structures necessary to express:

- symbol entries subject to auto/static deletion
- level/scope metadata used to decide removal
- container state that corresponds to the relevant mutable globals or static lists from `symbol.c`

Example shape only, to be refined from the C fields:

```rust
pub(crate) struct SymbolEntry {
    level: usize,
    // storage class / category fields if required
    // links or ownership fields as required by original traversal logic
}

pub(crate) struct SymbolState {
    // auto symbol collection(s)
    // static symbol collection(s)
}
```

### Memory Management Notes

- Replace `free`/unlink patterns with ownership-based removal.
- Ensure deletion does not leave invalid references to removed symbols.
- If other migrated code needs stable references into symbol storage, prefer an index-based container over direct references.
- Keep lifetimes local to deletion operations; avoid introducing shared ownership unless the existing C relationships make it unavoidable.

### Error Handling Notes

These functions are likely internal mutators rather than fallible APIs. Prefer:

- infallible `fn` returning `()`, if the C logic assumes valid internal state
- `debug_assert!` for invariants discovered during migration
- `Result` only if the current Rust module already models symbol-state corruption or invalid access as recoverable errors

## Implementation Phases

## Phase 1: Extract and Map Required Symbol State

- Inspect `src/symbol.c` and identify all fields, globals, and helper routines touched by:
  - `delete_level_autos`
  - `delete_level_statics`
- Determine whether each function operates on:
  - a linked list
  - an array/table
  - bucketed symbol storage
  - mixed global/local collections
- Define the minimal Rust structs/enums required to represent that state.
- Create `src/symbol.rs` with placeholder Rust equivalents for the necessary symbol data and module-local tests scaffold.

**Exit criteria**:
- All C-side data dependencies for the two functions are identified.
- Rust data model compiles with placeholders for behavior not yet ported.

## Phase 2: Port Deletion Logic Faithfully

- Implement `delete_level_autos` in Rust using the chosen container model.
- Implement `delete_level_statics` in Rust using the same approach, preserving any differences in selection criteria or target collection.
- Preserve original mutation semantics:
  - same level comparison behavior
  - same removal boundaries
  - same traversal/update behavior for head, middle, and tail removal cases
- Convert any helper logic used exclusively by these functions into private Rust helpers inside `src/symbol.rs`.

**Exit criteria**:
- Both deletion functions compile and operate against the migrated Rust state.
- No raw-memory deallocation patterns remain in the ported logic.

## Phase 3: Add Behavioral Tests for Scope Deletion

- Add unit tests covering:
  - deletion from empty collections
  - no-op deletion when no symbols match the level
  - deletion when the first entry matches
  - deletion when middle and tail entries match
  - deletion of multiple entries at the same level
  - separation between auto and static deletion paths
- Where C behavior depends on exact traversal/removal order, encode that expected result directly in tests.

**Exit criteria**:
- `cargo test` passes for all newly added deletion scenarios.
- Tests confirm collection integrity after removal.

## Phase 4: Integrate With Surrounding Symbol Module State

- Replace any temporary placeholders with the actual surrounding Rust symbol-state definitions used by the branch.
- Align function visibility and signatures with the rest of the ported symbol module.
- Remove unused transitional code introduced during the initial migration.
- Confirm that the Rust file mapping remains limited to the `src/symbol.c` port scope and does not introduce unrelated abstractions.

**Exit criteria**:
- The deletion functions are fully integrated into the branch’s Rust symbol module.
- The implementation remains scoped to the original C module responsibilities.
- Final `cargo test` passes.