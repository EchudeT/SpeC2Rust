# Implementation Plan: module_src_symbol.c_34

## Summary

This module migration covers the symbol-table lifecycle and list-management logic currently implemented in `src/symbol.c`. The Rust implementation should preserve the existing mutation order and ownership semantics of symbol installation, storage reclassification, and category-based deletion routines (`statics`, `autos`, `parms`, starters, and targets), while replacing manual pointer manipulation with explicit ownership and borrowing.

The technical approach is a direct port of the existing C logic into a single Rust module with narrowly scoped supporting types. The implementation should prioritize behavioral equivalence over redesign: convert linked-list and table mutation code into Rust collection updates, keep the existing operational boundaries aligned to the original functions, and model deletion/unlink steps so that symbol references cannot outlive their owning container. Where the C code relies on null checks and in-place mutation, Rust should use `Option`, enums, and explicit mutable access to encode the same state transitions safely.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve near-equivalent asymptotic behavior to the C implementation for symbol insertion, unlinking, and category-based cleanup.
  - Avoid unnecessary cloning of symbol records during list transitions and deletion passes.
  - Keep allocation patterns simple and bounded to migrated container ownership rather than introducing extra indirection beyond what is required for safe mutation.

## Module Mapping

### C to Rust File Mapping

- `src/symbol.c` -> `src/symbol.rs`

### Function Mapping

The Rust module should retain one primary function per migrated C function where practical, with names converted to idiomatic snake_case only if the surrounding codebase already follows that convention. If the existing Rust port effort preserves original naming patterns for traceability, keep the mapping close to the C names.

- `symbol_unlink_from_list` -> `symbol::symbol_unlink_from_list`
- `ident_change_storage` -> `symbol::ident_change_storage`
- `init_ident` -> `symbol::init_ident`
- `install_ident` -> `symbol::install_ident`
- `delete_symbol` -> `symbol::delete_symbol`
- `delete_statics` -> `symbol::delete_statics`
- `delete_autos` -> `symbol::delete_autos`
- `delete_parms` -> `symbol::delete_parms`
- `install_starter` -> `symbol::install_starter`
- `set_default_starter` -> `symbol::set_default_starter`
- `clear_starters` -> `symbol::clear_starters`
- `install_target` -> `symbol::install_target`

### Scope Boundaries

The Rust module should include only:
- symbol record initialization
- insertion into owned symbol collections
- unlink/removal from symbol lists
- storage-class mutation
- category-specific deletion passes
- starter/target registration state

It should not introduce new abstraction layers or split functionality into additional subsystems unless required by already-existing Rust project structure.

## Data Model

Because the provided analysis exposes only anonymous C data structures, the Rust plan should derive the final type set from field usage in `src/symbol.c` and map each concrete C struct shape to a named Rust type during migration.

### Data-Structure Mapping Strategy

- **Anonymous symbol record struct** -> `Symbol`
  - Represents the primary identifier/symbol state.
  - Convert nullable links and optional owned references to `Option`.
  - Convert C integer storage/category flags to Rust enums where the values are closed and known; otherwise preserve as integer newtypes until all call sites are migrated.

- **Anonymous linked-list node usage embedded in symbol records** -> embedded linkage fields on `Symbol` or index-based collection membership
  - If the C code uses intrusive next/prev pointers, prefer representing ownership at the collection level and unlink by position/key rather than reproducing raw pointers.
  - If exact intrusive semantics are required by neighboring migrated code, use stable symbol IDs plus container-managed adjacency metadata instead of self-referential references.

- **Anonymous storage classification fields** -> `StorageClass` enum or integer-backed enum
  - Used by `ident_change_storage`, `delete_statics`, `delete_autos`, and `delete_parms`.

- **Anonymous starter configuration records/list entries** -> `StarterEntry` or direct collection fields on owning state
  - Keep representation minimal and based only on fields used by `install_starter`, `set_default_starter`, and `clear_starters`.

- **Anonymous target registration records/list entries** -> `TargetEntry` or direct collection fields on owning state
  - Only model fields actually touched by `install_target`.

- **Anonymous module-global symbol registries/lists** -> `SymbolTableState`
  - Central owner for active symbols and category lists if the C file maintains globals or file-static registries.
  - This should hold the collections needed to support deletion and unlinking without raw pointers.

### Recommended Rust Type Shapes

These are migration targets, not new functionality:

```rust
pub struct Symbol {
    // fields derived from src/symbol.c
}

pub enum StorageClass {
    // variants derived from C constants
}

pub struct StarterEntry {
    // only if starter state is structurally distinct
}

pub struct TargetEntry {
    // only if target state is structurally distinct
}

pub struct SymbolTableState {
    // symbol ownership and category/grouping collections
}
```

### Memory Management Decisions

- Replace manual allocation/free in `init_ident`, `install_ident`, and deletion routines with owned Rust values stored in `Vec`, `VecDeque`, or `HashMap`, depending on the original access pattern.
- Prefer `Vec` plus indices when the C code primarily walks linear lists.
- Prefer `HashMap` only if the C logic clearly depends on keyed lookup by identifier name or handle; otherwise stay with sequential containers.
- Avoid `Rc`, `Arc`, `RefCell`, or unsafe raw pointers unless the surrounding already-migrated code makes direct self-referential list structure unavoidable.
- Express absence of a linked node, starter, or target with `Option`.

### Error Handling Decisions

- C routines that assume success and mutate in place should return `Result` in Rust only where failure is already meaningful in the source behavior, such as allocation replacement, invalid storage transitions, or missing symbol removal targets.
- Internal helper functions that cannot fail under preserved invariants may use debug assertions and private non-`Result` returns.
- Publicly exposed migration boundaries should not panic on ordinary missing-entry conditions if the original C behavior handled null or no-op deletion.

## Implementation Phases

## Phase 1: Recover concrete data layout and establish Rust state container

- Inspect `src/symbol.c` and identify each anonymous struct by field usage, then assign stable Rust names for the migration.
- Create `src/symbol.rs` and define:
  - primary symbol record type
  - storage classification type
  - starter/target entry types only if they are structurally distinct
  - a single owning state container for symbol collections maintained by this module
- Map all nullable pointers and sentinel states to `Option`.
- Map C constants/macros used for storage/category decisions to Rust enums or associated constants.
- Add minimal unit tests for type initialization and default state construction.

## Phase 2: Port symbol lifecycle and list mutation functions

- Implement `init_ident`, `install_ident`, `ident_change_storage`, and `symbol_unlink_from_list`.
- Preserve original mutation ordering when moving a symbol between storage classes or lists.
- Convert C unlink logic from pointer rewiring into collection mutation with stable ownership:
  - if symbols are uniquely owned in one list, remove by index/key
  - if symbols participate in multiple registries, store a stable symbol ID and update all affected collections explicitly
- Implement `delete_symbol` as the single record teardown path used by higher-level delete passes.
- Add focused tests covering:
  - initialization defaults
  - insertion into the owning container
  - storage-class updates
  - unlinking head, middle, tail, and missing entries

## Phase 3: Port category-based deletion passes

- Implement `delete_statics`, `delete_autos`, and `delete_parms` using the shared deletion/unlink machinery from Phase 2.
- Ensure deletion walks are safe against iterator invalidation by using:
  - two-phase collect-then-remove logic, or
  - `retain`/drain patterns where behavior remains equivalent
- Verify that deleting one category does not disturb unrelated symbol groups beyond what the C code already does.
- Add tests for mixed symbol sets demonstrating selective removal and post-delete container integrity.

## Phase 4: Port starter and target registration logic and finalize behavioral checks

- Implement `install_starter`, `set_default_starter`, `clear_starters`, and `install_target`.
- Keep starter/target state colocated in the same module state if the C code manages them there; do not introduce separate service objects.
- Preserve any overwrite, reset, or default-selection behavior exactly as observed in the C implementation.
- Add tests for:
  - starter installation order
  - default starter replacement/reset semantics
  - clearing starter state
  - target installation without disturbing symbol registries
- Finish by reconciling function signatures with the rest of the Rust branch and removing any temporary integer-backed placeholders that can now be expressed as enums.

## Validation Notes

- Use unit tests in `src/symbol.rs` or adjacent module tests via `cargo test`.
- Validate behavior against the C implementation by matching:
  - state transitions
  - no-op deletion cases
  - category cleanup effects
  - default/reset handling for starters and targets
- Keep the final Rust module compact and source-aligned so future verification against `src/symbol.c` remains straightforward.