# Implementation Plan: module_src_symbol.c_34

## Summary

This plan covers the Rust port of `src/symbol.c` for branch `097-module_src_symbol.c_34-rust-port`. The work is limited to migrating the existing symbol-management logic and associated list operations from the C implementation into a Rust module with equivalent behavior and scope.

The Rust implementation should preserve the current module responsibilities:

- initialization of symbol/identifier records
- installation of identifiers into the active symbol structures
- removal of symbols from linked collections
- storage-class changes on identifiers
- deletion of symbols grouped by lifetime/category
- installation and reset of starter/target state

The technical approach is a direct, conservative translation of the C module into one Rust source module, using explicit owned data structures and indices/handles instead of raw pointer manipulation where possible. Internal mutable state should remain centralized in the Rust module so that C-style global or shared list behavior can be expressed without unsafe pointer-heavy code unless a specific edge of the original design requires it. The port should prioritize behavioral parity, predictable ownership, and simple control flow over abstraction.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.76+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:
- None required initially

If the broader project already uses a common error crate, this module may align with that existing project choice, but no new third-party dependency is required for this port based on the current input.

### Testing

- `cargo test`

Testing focus:
- symbol insertion/removal ordering
- repeated deletion behavior across symbol categories
- storage-class transition behavior
- starter/target install and reset behavior
- initialization defaults and cleanup effects

### Performance Goals

- Preserve the effective asymptotic behavior of the C implementation for symbol-list insertion, unlinking, and category cleanup.
- Avoid unnecessary cloning of symbol names or records during list operations.
- Keep allocation behavior bounded to symbol creation/installation paths.
- Prefer straightforward in-memory traversal over additional layers of indirection not present in the original implementation.

## Module Mapping

### Source File Mapping

- C: `src/symbol.c`
- Rust: `src/symbol.rs`

If the existing Rust crate uses `mod` declarations from `src/lib.rs` or `src/main.rs`, expose this file as:

- `mod symbol;`

No extra helper modules should be introduced unless the existing Rust project structure already requires them.

### Function Mapping

The following C functions should be migrated into Rust with close naming alignment, converted to idiomatic snake_case where needed only if not already aligned:

- `symbol_unlink_from_list` -> `symbol_unlink_from_list`
- `ident_change_storage` -> `ident_change_storage`
- `init_ident` -> `init_ident`
- `install_ident` -> `install_ident`
- `delete_symbol` -> `delete_symbol`
- `delete_statics` -> `delete_statics`
- `delete_autos` -> `delete_autos`
- `delete_parms` -> `delete_parms`
- `install_starter` -> `install_starter`
- `set_default_starter` -> `set_default_starter`
- `clear_starters` -> `clear_starters`
- `install_target` -> `install_target`

### Responsibility Mapping

The Rust module should continue to own:

- identifier record initialization
- mutation of identifier storage classification
- insertion/removal of symbols from maintained collections
- cleanup of symbols by category/lifetime bucket
- starter and target registration/reset state

It should not absorb unrelated parsing, code generation, or cross-module policy decisions during the port.

## Data Model

Because the input only exposes anonymous C data structures, the Rust data model should be derived from actual field usage in `src/symbol.c` and mapped conservatively. The intent is to replace pointer-linked mutable records with explicit Rust-owned records while preserving identity and list membership semantics.

### C Struct to Rust Mapping Strategy

For each anonymous C struct used in `src/symbol.c`:

- Create a named Rust `struct` or `enum` based on its role in the file.
- Name types from observed usage rather than preserving anonymity.
- Keep field sets minimal and based only on fields actually referenced by this module.

### Expected Core Rust Types

#### Symbol/Identifier Record

If the C module uses a mutable symbol node with next/prev links and classification metadata, map it to a Rust struct such as:

```rust
pub struct Symbol {
    pub name: String,
    pub storage: StorageClass,
    pub next: Option<SymbolId>,
    pub prev: Option<SymbolId>,
    // additional migrated fields only as required by symbol.c usage
}
```

Where:

- `String` replaces owned C string storage when this module owns the name text.
- `StorageClass` becomes a Rust `enum` if the C code uses symbolic constants.
- `Option<SymbolId>` replaces nullable next/prev pointers.
- `SymbolId` should be a stable handle type such as `usize` into an owning arena/vector, if that fits the surrounding project.

If the surrounding project already has a shared symbol record type, this module should reuse it instead of introducing a duplicate local type.

#### Storage Classification

C integer constants or macros for storage categories should become an enum:

```rust
pub enum StorageClass {
    Static,
    Auto,
    Param,
    Other(i32),
}
```

Use exact variants only for values actually present in the C module. If the original code depends on raw integer preservation, add conversion helpers rather than keeping untyped integers everywhere.

#### Module State for Lists

If the C file maintains several head pointers for symbol groups, map them to a state container:

```rust
pub struct SymbolState {
    pub statics_head: Option<SymbolId>,
    pub autos_head: Option<SymbolId>,
    pub parms_head: Option<SymbolId>,
    pub starters: Vec<StarterEntry>,
    pub target: Option<TargetEntry>,
    pub symbols: Vec<Option<Symbol>>,
}
```

This captures C global-list behavior in one mutable owner and avoids exposing raw aliasing.

#### Starter/Target Records

If starter/target entries are distinct node types or contain only references to symbols, create small role-specific structs:

```rust
pub struct StarterEntry {
    pub symbol: SymbolId,
    // migrated fields as needed
}

pub struct TargetEntry {
    pub symbol: SymbolId,
    // migrated fields as needed
}
```

If the C code stores these as direct symbol pointers only, use `SymbolId` directly instead of wrapping.

### Nullability and Ownership

C patterns should map as follows:

- `NULL` pointer -> `Option<T>`
- owned heap object -> owned Rust struct
- borrowed or cross-referenced object -> stable ID/handle or shared reference based on actual usage
- linked-list pointer rewiring -> handle/index updates inside a single owner state object

### Memory Management Decisions

- Replace manual free/delete paths with ownership-based removal where possible.
- For symbol collections requiring stable references during mutation, prefer `Vec<Option<T>>` plus indices over self-referential pointer structures.
- Deletion functions should clear membership links and release owned content by taking entries out of storage rather than emulating `free()` directly.
- Avoid `unsafe` unless the surrounding codebase already exposes symbol records through raw shared mutation that cannot be migrated incrementally without compatibility glue.

### Error Handling

The original C module likely uses implicit success/failure paths and mutable global state. In Rust:

- Use `Result` only for operations that can genuinely fail in a recoverable way at module boundaries.
- Use internal assertions/invariants for impossible states caused by broken list topology.
- For direct C-equivalent helper functions that are not expected to fail, keep signatures simple and side-effect based.
- If installation functions can reject duplicates or invalid state in the C implementation, encode that as a small module-local error enum.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Data Types

Goals:
- create `src/symbol.rs`
- identify all concrete anonymous C structs referenced by `src/symbol.c`
- define the minimum Rust structs/enums needed by this file
- centralize module state required for existing symbol lists and starter/target tracking

Tasks:
- Read `src/symbol.c` and enumerate actual fields used for each anonymous struct.
- Introduce named Rust equivalents for symbol records, storage classification, and starter/target state.
- Decide whether to reuse existing crate types for symbols/storage classes if already present.
- Choose the ownership model for symbol records:
  - preferred: state-owned records with stable indices
  - fallback: boxed nodes only if existing surrounding APIs require pointer-like identity
- Encode nullable links and optional installed records using `Option`.

Exit criteria:
- Rust module compiles with type definitions and placeholder function signatures for all migrated functions.
- All C anonymous structures used by this file have a corresponding named Rust representation or reuse decision.

## Phase 2: Port Core Symbol Lifecycle and List Manipulation

Goals:
- migrate the symbol initialization, installation, unlinking, storage change, and deletion primitives first
- establish behaviorally equivalent list mutation semantics before category-wide cleanup functions are added

Tasks:
- Implement `init_ident`.
- Implement `install_ident`.
- Implement `symbol_unlink_from_list`.
- Implement `ident_change_storage`.
- Implement `delete_symbol`.
- Preserve original ordering and relinking behavior from the C implementation.
- Add focused unit tests for:
  - creating initialized symbol records
  - inserting into tracked lists
  - unlinking head, middle, and tail entries
  - changing storage class and moving between category lists if required by original logic
  - deleting installed symbols without leaving stale links

Exit criteria:
- Core symbol lifecycle functions behave equivalently to the C logic for insertion, relinking, mutation, and single-symbol deletion.
- No raw-memory management remains in these paths.

## Phase 3: Port Category Cleanup Functions

Goals:
- migrate grouped deletion logic exactly as currently scoped in the C file

Tasks:
- Implement `delete_statics`.
- Implement `delete_autos`.
- Implement `delete_parms`.
- Verify whether each function removes entire lists, resets heads only after iterative deletion, or applies additional per-node cleanup.
- Add unit tests covering:
  - cleanup on empty category lists
  - cleanup of one-entry and multi-entry lists
  - repeated cleanup calls after list exhaustion
  - interaction with previously unlinked or deleted symbols

Exit criteria:
- Category cleanup behavior matches the C file’s traversal and reset semantics.
- Module state remains internally consistent after repeated cleanup operations.

## Phase 4: Port Starter/Target Installation and Finalize Integration

Goals:
- complete remaining registration/reset functions
- finish module integration without expanding scope beyond `src/symbol.c`

Tasks:
- Implement `install_starter`.
- Implement `set_default_starter`.
- Implement `clear_starters`.
- Implement `install_target`.
- Map any C global starter/target state into the Rust module state container or existing project state object.
- Add tests for:
  - starter installation ordering or replacement semantics
  - default starter behavior
  - clearing starter state
  - target installation effects
- Perform final pass for naming, invariants, and removal of temporary placeholders.

Exit criteria:
- All functions from `src/symbol.c` are present in Rust and covered by basic behavior tests.
- `cargo test` passes for the migrated module.
- The port remains restricted to the original file’s responsibilities and does not introduce new subsystem abstractions.

## Notes and Migration Constraints

- Keep the migration file-scoped: port `src/symbol.c` into `src/symbol.rs` without splitting logic into additional architectural layers unless existing crate structure forces a small adaptation.
- Favor exact control-flow preservation over refactoring.
- Do not introduce concurrency primitives, serialization layers, FFI shims, or generalized symbol-table frameworks.
- Where the C implementation relies on global mutable state, encapsulate it in one Rust state struct rather than spreading mutable globals.
- Any unresolved details from anonymous structs must be settled by inspecting actual field access in `src/symbol.c`, not by inventing broader models.