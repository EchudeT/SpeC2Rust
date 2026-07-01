# Implementation Plan: module_src_symbol.c_34

## Summary

This module centers on symbol-table lifecycle management for identifiers and related symbol classes currently implemented in `src/symbol.c`. The Rust port should preserve the existing mutation-oriented behavior: installing identifiers into lists/tables, changing storage classification, unlinking symbols from list structures, and deleting grouped symbols such as statics, autos, and parameters.

The technical approach is to migrate the current C logic into a single Rust module with closely corresponding functions and minimal reshaping of control flow. The implementation should replace manual pointer manipulation and explicit deletion with ownership-aware Rust data structures while keeping list ordering, deletion semantics, and table update behavior aligned with the C implementation. Where the C code relies on global or shared mutable state, the Rust port should model that state explicitly in module-local structs passed by mutable reference, avoiding unnecessary expansion into broader abstractions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve effectively constant-time or linear-time behavior matching the C routines, depending on current list traversal requirements.
  - Avoid unnecessary cloning of identifier names or symbol records during install/unlink/delete operations.
  - Keep allocation patterns simple and bounded to symbol creation and retained table/list storage.
  - Maintain deterministic deletion and relinking behavior for grouped symbol cleanup passes.

## Module Mapping

- **C source**: `src/symbol.c`
- **Rust target**: `src/symbol.rs`

Recommended Rust migration scope within `src/symbol.rs`:

- `symbol_unlink_from_list` -> `pub(crate) fn symbol_unlink_from_list(...)`
- `ident_change_storage` -> `pub(crate) fn ident_change_storage(...)`
- `init_ident` -> `pub(crate) fn init_ident(...)`
- `install_ident` -> `pub(crate) fn install_ident(...)`
- `delete_symbol` -> `pub(crate) fn delete_symbol(...)`
- `delete_statics` -> `pub(crate) fn delete_statics(...)`
- `delete_autos` -> `pub(crate) fn delete_autos(...)`
- `delete_parms` -> `pub(crate) fn delete_parms(...)`
- `install_starter` -> `pub(crate) fn install_starter(...)`
- `set_default_starter` -> `pub(crate) fn set_default_starter(...)`
- `clear_starters` -> `pub(crate) fn clear_starters(...)`
- `install_target` -> `pub(crate) fn install_target(...)`

If the existing Rust crate already has neighboring parser/compiler state modules, this module should integrate with them through existing crate-local types rather than introducing a new subsystem. The implementation should keep the symbol-related state concentrated in this file unless another already-existing Rust file clearly owns shared compiler state.

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust port should derive its concrete mappings directly from the actual field layouts in `src/symbol.c` and its referenced headers. The goal is not to redesign the model, but to translate each storage-bearing and list-linked C struct into explicit Rust structs/enums with equivalent semantics.

### Core mapping strategy

| C pattern | Rust mapping |
|---|---|
| Anonymous struct used as a symbol record | Named `struct Symbol` with explicit fields |
| Anonymous struct used as identifier record | Named `struct Ident` or merged into `Symbol` if C layout indicates identity |
| Integer storage-class flags/constants | `enum StorageClass` or `#[repr(i32)] enum` if exact numeric parity matters internally |
| Linked-list next/prev pointers | Index-based links or owned collection plus explicit position management |
| Global mutable symbol lists | Fields inside a `SymbolState` / existing compiler-state struct |
| C string pointer for names | `String` if owned, `&str` only when lifetime is naturally borrowed from caller |
| Nullable pointers | `Option<T>` / `Option<usize>` / `Option<NonZeroUsize>` depending on representation |
| Manual free/delete | Removal from owning collection; memory released by Rust ownership |

### Recommended Rust structures

The exact field list must be taken from the C source and headers, but the migration should likely resolve into a small set like the following:

```rust
pub(crate) struct Symbol {
    // fields migrated directly from C symbol record
}

pub(crate) struct Ident {
    // fields migrated directly from C identifier record
}

pub(crate) enum StorageClass {
    // variants matching C storage classifications
}

pub(crate) struct Starter {
    // fields migrated from starter-related anonymous struct
}

pub(crate) struct Target {
    // fields migrated from target-related anonymous struct
}

pub(crate) struct SymbolState {
    // owning collections and list heads/tails that replace C globals
}
```

### Ownership and container choices

Use the simplest ownership model that matches the C behavior:

- Use `Vec<T>` when the C code maintains traversable ordered groups and deletion can be expressed by retain/remove logic without invalidating external references.
- Use `Vec<Option<T>>` plus stable indices if the C code stores many cross-references that require handle stability across deletions.
- Use `Box<T>` only when a single owned node must exist independently of a collection.
- Avoid `Rc`, `Arc`, `RefCell`, or intrusive linked-list crates unless the C code structure makes stable aliasing unavoidable; this is not the default plan.

### Memory management notes

- C deletion helpers such as `delete_symbol`, `delete_statics`, `delete_autos`, and `delete_parms` should become removal operations on owning Rust collections.
- Any logic that in C unlinks a node before freeing it should in Rust separate into:
  1. remove references from list/table state,
  2. drop the removed value naturally.
- If symbols are referenced from multiple internal tables, use stable symbolic handles (such as indices) so unlinking and deletion stay explicit and safe.

### Error handling notes

The C code may assume success paths via null checks and direct mutation. In Rust:

- Use `Result` only where migration reveals genuine failure modes that callers already need to react to, such as duplicate installation or missing symbol references.
- Use `Option` for lookup/unlink cases that are naturally absent/present.
- Keep panic usage limited to invariant violations that correspond to impossible internal states after successful parsing/setup.

## Implementation Phases

### Phase 1: Extract and map C state/layout into Rust types

- Inspect `src/symbol.c` and its dependent headers to enumerate the actual anonymous structs, typedefs, constants, and global state used by the listed functions.
- Define the minimal Rust structs/enums in `src/symbol.rs` needed to represent:
  - symbols/identifiers,
  - storage classes,
  - starter/target records,
  - list/table ownership state.
- Decide container representation based on actual pointer topology in the C code:
  - plain vectors for grouped symbol classes if external stable references are not required,
  - stable indices if unlink/delete operations touch multiple lists.
- Add crate-local constructors/default initialization needed to replace `init_ident` semantics.

### Phase 2: Port creation and mutation routines

Implement the functions that create and update symbol state first, preserving C call ordering and side effects:

- `init_ident`
- `install_ident`
- `ident_change_storage`
- `install_starter`
- `set_default_starter`
- `clear_starters`
- `install_target`

Phase goals:

- Keep names and signatures close to the C originals, adjusted only for Rust ownership and borrowing.
- Replace pointer writes with explicit mutable access to `SymbolState` and owned records.
- Preserve any defaulting behavior and overwrite rules from the C implementation.
- Add focused unit tests for initialization, install/update behavior, and default starter handling.

### Phase 3: Port unlink and deletion routines

Implement the routines that remove symbols and grouped storage classes:

- `symbol_unlink_from_list`
- `delete_symbol`
- `delete_statics`
- `delete_autos`
- `delete_parms`

Phase goals:

- Encode unlink-before-delete ordering explicitly.
- Ensure all list/table references are updated consistently before drop.
- Preserve traversal semantics when deleting while iterating through grouped collections.
- Add unit tests covering:
  - unlink of head/middle/tail entries,
  - deletion of a single symbol,
  - deletion passes over static/auto/parameter groups,
  - no stale references after deletion.

### Phase 4: Integrate and verify behavior parity

- Align function visibility and call sites with the rest of the Rust crate.
- Replace any remaining C-style sentinel/null logic with `Option`-based equivalents while preserving behavior.
- Run `cargo test` and add regression tests for edge cases discovered during porting, especially:
  - repeated install/delete sequences,
  - storage-class transitions,
  - starter clearing/reset interactions.
- Perform a final pass to remove any unnecessary unsafe code; safe Rust should be the default unless exact layout interop inside the crate proves unavoidable.

## Notes and Constraints

- Keep the port confined to the functionality already present in `src/symbol.c`.
- Do not introduce new symbol-table features, concurrency support, serialization, or cross-module architecture changes.
- Prefer direct migration of existing functions and state over abstraction-driven redesign.
- Any naming refinements in Rust should remain traceable to the original C functions to ease review against the source module.