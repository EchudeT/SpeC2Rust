# Implementation Plan: module_src_table_entry_06

## Summary

This module ports the symbol-table entry management logic currently implemented in `src/symbol.c` into Rust, preserving existing behavior and migration boundaries. The implementation should focus on the current function set only: hashing and comparison of symbol keys, lookup and insertion into the table, unlink/removal operations, object cleanup, and the two iterator/callback-style helpers used during collection and parameter deletion.

The Rust approach should use standard-library collections and ownership to replace manual C memory handling. In practice, the C hash/compare pair should map to Rust key types implementing `Eq` and `Hash`, while explicit free/unlink paths should become structured ownership and `Drop`-driven cleanup where possible, with explicit removal methods retained where the original module requires them. Callback-style traversal helpers should be migrated as narrow internal functions operating over the same table/container used by lookup and install.

The plan should keep the migration localized to the logic from `src/symbol.c`, avoiding new subsystem design or cross-module refactoring not required for this port.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are required based on the available module analysis
- **Testing**:
  - `cargo test`
  - Unit tests should cover hash-key equivalence behavior, lookup/insert/update behavior, unlink/removal paths, and cleanup-sensitive edge cases
- **Performance Goals**:
  - Preserve expected hash-table lookup and insertion characteristics comparable to the C implementation
  - Avoid unnecessary cloning of symbol keys or owned entry payloads
  - Keep removal and traversal behavior linear only where the original design requires iteration
  - Maintain predictable memory release by relying on ownership and scoped drops instead of deferred/manual free chains

## Module Mapping

### Source Mapping

- **C source file**: `src/symbol.c`
- **Rust target file**: `src/symbol.rs`

If the Rust project already exposes module declarations from `src/lib.rs` or `src/main.rs`, add only the minimal `mod symbol;` / `pub mod symbol;` declaration needed to include the migrated file.

### Function Mapping

| C Function | Rust Mapping |
|---|---|
| `hash_symbol_hasher` | Internal hash/key implementation, likely via `Hash` on the Rust key type or a small internal helper function if exact behavior must be preserved |
| `hash_symbol_compare` | `Eq`/`PartialEq` implementation on the symbol key type, or a narrow comparison helper retained during transition |
| `lookup` | Method or internal function on the symbol table wrapper returning an immutable or mutable reference/option |
| `install` | Method or internal function inserting a new entry or returning the existing one per original behavior |
| `unlink_symbol` | Explicit removal method from the Rust symbol table/container |
| `static_free` | Destructor-style cleanup logic, usually absorbed by ownership; keep as a narrow internal function only if entry teardown contains nontrivial nested cleanup |
| `collect_processor` | Internal traversal/collector helper over stored entries |
| `delete_parms_itr` | Internal iterator helper for deleting parameter-related subordinate data |

### Rust Module Scope

The Rust module should remain narrowly scoped to:
- symbol key representation
- symbol entry representation
- symbol table storage
- helper cleanup/traversal functions directly replacing the listed C functions

Do not split this into additional support modules unless existing Rust project layout already requires that.

## Data Model

Because the input analysis exposes only anonymous C structures, the Rust data model should be derived directly from usage in `src/symbol.c` during implementation. The migration should begin by naming each anonymous structure by role, based on how it participates in the listed functions.

### Planned Mapping Strategy

| C Data Shape | Rust Mapping Strategy |
|---|---|
| Anonymous hash key fields used by `hash_symbol_hasher` / `hash_symbol_compare` | A named Rust key struct, e.g. `SymbolKey`, implementing `Hash`, `Eq`, and `PartialEq` |
| Anonymous symbol table entry struct | A named Rust struct, e.g. `SymbolEntry`, containing key data plus migrated payload fields |
| Anonymous table/container struct | A named Rust struct, e.g. `SymbolTable`, wrapping `HashMap<SymbolKey, SymbolEntry>` or an equivalent standard container |
| Anonymous linked-list nodes, if present | Replace with `Vec`, `HashMap`, or owned nested structs as dictated by call patterns; only retain explicit node structs if required by existing semantics |
| Anonymous parameter/subobject records cleaned by `delete_parms_itr` | Named subordinate struct or enum owned by `SymbolEntry` |
| Anonymous callback context for `collect_processor` | Private Rust struct or borrowed context parameter, only if stateful traversal is necessary |

### Core Rust Type Decisions

#### Symbol key
The C hasher/compare pair strongly suggests a custom key definition. In Rust:
- define a dedicated key struct rather than using loosely grouped tuples
- implement `Hash` and `Eq` to preserve exact key identity semantics
- use owned `String` for textual key fields if the C code stores duplicated strings
- use borrowed lookup forms only if this can be done simply without widening scope

#### Symbol entry ownership
The C module likely manages allocation and explicit freeing of symbol entries and nested members. In Rust:
- represent entries as owned structs stored directly in the table where possible
- use `Box<T>` only when stable indirection is actually required by the translated logic
- translate nullable pointers to `Option<T>` or `Option<Box<T>>`
- translate C linked ownership chains into nested owned fields or vectors if they are not externally aliased

#### Table storage
Default to:
- `std::collections::HashMap<SymbolKey, SymbolEntry>`

Use a custom wrapper struct around the map if:
- insertion/removal requires preserving helper methods matching C behavior
- traversal helpers need access to module-local invariants
- cleanup logic needs to remain centralized

#### Cleanup-sensitive subordinate data
For structures freed by `static_free` or traversed by `delete_parms_itr`:
- convert manual recursive free into natural owned-field destruction
- preserve explicit helper functions only where they encode selective deletion rules rather than plain deallocation
- model optional subordinate records with `Option`
- model repeated subordinate records with `Vec<T>`

### Memory Management Mapping

| C Pattern | Rust Replacement |
|---|---|
| `malloc`/`free` for entries | Owned structs with automatic drop |
| Manual unlink before free | `HashMap::remove` plus drop at scope end |
| Null pointer checks | `Option` matching |
| Recursive/static free helpers | Scoped destruction, with helper methods retained only for semantic cleanup steps |
| Iterator callbacks deleting nested records | `iter_mut`, `retain`, `drain`, or explicit loops depending on exact deletion semantics |

### Error Handling Mapping

| C Pattern | Rust Replacement |
|---|---|
| Null return for lookup miss | `Option<&SymbolEntry>` / `Option<&mut SymbolEntry>` |
| Null/failed install allocation | `Result` only if insertion can fail meaningfully in Rust-facing API; otherwise direct insertion |
| Sentinel success/failure integers | `bool` or `Result<(), Error>` depending on observed call usage |
| Assumed-valid internal pointers | Internal invariants enforced by type ownership and borrowing |

A custom error type should only be introduced if `src/symbol.c` contains explicit distinguishable failure modes beyond simple presence/absence.

## Implementation Phases

## Phase 1: Recover concrete data roles and define Rust types

### Goal
Establish the exact Rust representations for the anonymous C structures and the symbol-table key/entry/container relationships before porting behavior.

### Tasks
- Inspect `src/symbol.c` and identify each anonymous struct by operational role:
  - table container
  - entry record
  - key fields
  - parameter/subobject records
  - collection/traversal context
- Create `src/symbol.rs`
- Define named Rust structs/enums for the discovered roles
- Decide which fields are:
  - owned values
  - optional values
  - repeated collections
- Implement `Hash`, `Eq`, and `PartialEq` for the symbol key type if exact key behavior is not fully covered by derived implementations
- Introduce a minimal `SymbolTable` wrapper around the chosen standard container when needed for method parity

### Deliverables
- Compiling Rust type skeletons
- Key type with comparison/hash semantics in place
- No behavioral expansion beyond data-model replacement

## Phase 2: Port lookup, install, and unlink behavior

### Goal
Migrate the core table mutation and query functions first, since other helpers depend on stable entry ownership and removal semantics.

### Tasks
- Port `lookup` against the Rust table container
- Port `install` preserving original duplicate-handling and insertion behavior
- Port `unlink_symbol` preserving removal semantics and any return-value behavior
- Translate any internal key construction logic needed by these operations
- Ensure no unnecessary clones are introduced when inserting or looking up entries
- Add unit tests for:
  - lookup miss
  - install then lookup hit
  - duplicate install behavior
  - unlink existing/non-existing entry behavior

### Deliverables
- Working symbol table core operations
- Test coverage for insertion, search, and removal paths

## Phase 3: Port cleanup and subordinate deletion helpers

### Goal
Replace manual free paths with ownership-based cleanup while preserving any nontrivial selective deletion logic embedded in helper functions.

### Tasks
- Port `static_free`
- Port `delete_parms_itr`
- Identify whether these functions do more than deallocate memory:
  - selective nested removal
  - field reset
  - callback-driven deletion
- Collapse pure deallocation into Rust drop behavior where valid
- Retain explicit helper functions only where semantic cleanup must still be invoked
- Add tests covering:
  - nested subordinate cleanup
  - repeated deletion safety
  - removal of entries containing parameter/subobject data

### Deliverables
- Cleanup logic migrated without manual memory management
- Explicit helper functions reduced to only behaviorally necessary cases

## Phase 4: Port traversal/collection helper and finalize integration

### Goal
Complete the remaining iterator/callback logic and verify the Rust module fully replaces the C module’s local behavior.

### Tasks
- Port `collect_processor`
- Map callback-style or visitor-style C iteration to minimal Rust iteration constructs
- Confirm the helper works against the same table/entry structures introduced earlier
- Reconcile any remaining API surface with the surrounding crate’s expected call sites
- Add final unit tests for traversal-sensitive behavior and any edge cases discovered during migration
- Remove dead transitional helpers that became unnecessary after ownership conversion

### Deliverables
- Fully migrated `src/symbol.rs`
- Complete test suite for the listed function set
- Localized Rust replacement for `src/symbol.c` with no extra architectural expansion

## Acceptance Criteria

- The logic from `src/symbol.c` for the listed functions exists in Rust under `src/symbol.rs`
- Anonymous C structures used by this module are replaced with named Rust types derived from actual usage
- Core table operations (`lookup`, `install`, `unlink_symbol`) preserve original behavior
- Hash and comparison semantics are preserved through Rust key design
- Manual free-oriented logic is safely replaced by ownership-based destruction, with explicit cleanup helpers retained only where behavior requires them
- `cargo test` passes for the migrated module coverage
- The migration remains confined to this module and does not introduce unrelated infrastructure