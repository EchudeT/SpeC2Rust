# Implementation Plan

## Summary

This module ports the GNU-style hash entry management logic from `gnu/hash.c` into a Rust module that preserves the existing responsibilities of:

- inserting an entry only when absent,
- removing an entry,
- printing hash contents or entry state.

The Rust implementation should stay narrowly aligned with the existing C file and functions, using standard-library collections and ownership rules to replace manual memory handling. The core technical approach is:

- represent the hash table state with Rust structs using `std::collections`,
- migrate each C function into a direct Rust function or inherent method with the same operational scope,
- replace pointer-based lifetime management with ownership/borrowing,
- express failure and absence cases with `Option`/`Result` rather than sentinel values or null pointers,
- keep printing behavior explicit and side-effecting, matching current module scope without introducing broader formatting infrastructure.

The migration should focus on preserving behavior and call boundaries rather than redesigning the module.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the available evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve expected hash-table operation characteristics comparable to the C implementation
  - Maintain average-case constant-time insert/remove/lookup behavior through standard-library hash collections
  - Avoid unnecessary cloning during insertion, removal, and printing paths
  - Keep memory ownership transitions explicit so removed entries are dropped promptly and safely

## Module Mapping

### Source File Mapping

- C: `gnu/hash.c`
- Rust: `src/gnu/hash.rs`

If the project already exposes GNU-related modules, the minimal conventional mapping is:

- `src/gnu/mod.rs` to declare `pub mod hash;`
- `src/gnu/hash.rs` to contain the ported implementation

No additional helper modules should be introduced unless required by existing crate layout.

### Function Mapping

| C Function | Rust Mapping | Notes |
|---|---|---|
| `hash_insert_if_absent` | `fn hash_insert_if_absent(...) -> ...` or method on table type | Should preserve insert-only-if-missing semantics; return type should distinguish existing vs newly inserted entry with `Option`/`Result` as appropriate to the observed C behavior |
| `hash_remove` | `fn hash_remove(...) -> ...` or method on table type | Should transfer ownership of removed value when needed, avoiding dangling references |
| `hash_print` | `fn hash_print(...)` or `impl Display`-adjacent internal printer | Keep as an explicit printing function if the C API is side-effect based; do not broaden into a general formatting subsystem |

### Rust Module Shape

A restrained layout is recommended:

```text
src/
  gnu/
    mod.rs
    hash.rs
```

Within `hash.rs`, keep:

- the primary hash table state type,
- any entry/key/value structs directly migrated from the C file,
- the three migrated operations,
- local tests for insertion, removal, and printing behavior.

## Data Model

The C analysis reports only anonymous data structures, so the plan should derive Rust names from actual usage during migration rather than inventing broad abstractions. The mapping approach is:

### Data-Structure Mapping Strategy

| C Construct | Rust Mapping Strategy | Notes |
|---|---|---|
| anonymous struct used as table state | named `struct` for table/container | Name from role in `hash.c`; likely the central owner of buckets/entries |
| anonymous struct used as entry | named `struct` for a hash entry | Fields become owned Rust fields or indices/handles as needed |
| anonymous struct used as node/link | named `struct` or eliminated into collection storage | If C uses linked nodes, prefer collection-managed storage instead of manual next-pointers |
| anonymous pointer fields | references, `Box<T>`, or collection membership | Choose ownership-bearing forms; avoid raw pointers unless forced by existing crate boundaries |
| nullable object references | `Option<T>` / `Option<&T>` / `Option<usize>` | Replace null checks directly |
| status/error integer returns | `bool`, `Option<T>`, or `Result<T, E>` | Select the narrowest type matching actual outcomes |
| print-related callback/data fields | function parameters or trait bounds only if already required | Do not generalize unless present in the original interface |

### Preferred Rust Representations

Because the available evidence indicates a hash-management module, the default representation should be:

- `HashMap<K, V>` when key/value ownership is clear and the C code is fundamentally table-oriented
- `HashSet<T>` only if the table stores uniqueness without separate values
- small dedicated structs for entry payloads when the C file stores metadata alongside key/value content

### Memory Management Decisions

- Replace heap allocation and free logic with owned Rust values stored in the table.
- Removal should return owned data where the C code would have transferred or exposed removed storage.
- Eliminate manual cleanup paths by relying on `Drop`.
- Avoid interior mutability unless required by existing call patterns in the Rust crate.

### Error Handling Decisions

- Use `Option` for “not found” or “already present” style outcomes when no diagnostic is needed.
- Use `Result` only where the port requires preserving a meaningful failure path beyond presence/absence.
- Printing should return `std::io::Result<()>` only if it writes to a generic writer; if it prints directly to stdout and the C behavior is fixed-side-effect, keep the function simple and narrow.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Data Structures

- Create `src/gnu/hash.rs` and wire it through `src/gnu/mod.rs` if not already present.
- Inspect `gnu/hash.c` and assign stable Rust names to each anonymous C structure based on operational role.
- Define the minimal Rust structs/enums needed to represent:
  - the table state,
  - entry storage,
  - any auxiliary insertion/removal metadata used by the three target functions.
- Replace nullable fields and sentinel states with `Option`.
- Decide whether the central container maps best to `HashMap` or `HashSet` based strictly on the C function signatures and accessed fields.
- Preserve module-local visibility as much as possible; expose only items required by existing crate callers.

### Deliverables

- Compiling Rust type definitions in `src/gnu/hash.rs`
- Module declarations updated
- Placeholder signatures for:
  - `hash_insert_if_absent`
  - `hash_remove`
  - `hash_print`

## Phase 2: Port Insert and Remove Semantics

- Implement `hash_insert_if_absent` first, preserving:
  - duplicate detection behavior,
  - ownership transfer rules,
  - return semantics distinguishing existing and new entries.
- Implement `hash_remove` next, preserving:
  - key matching behavior,
  - return of removed data or absence indication,
  - cleanup semantics now handled by ownership and drop.
- During both ports, convert any C-side mutation-through-pointer flows into:
  - `&mut self` methods, or
  - functions taking `&mut` references to the table state.
- Minimize cloning by:
  - moving owned keys/values into the table where possible,
  - borrowing for lookups before insertion if needed.

### Validation Focus

- inserting a new item succeeds and stores exactly one copy
- inserting an existing item leaves table state unchanged
- removing an existing item deletes it once
- removing a missing item reports absence without side effects

## Phase 3: Port Printing Logic and Normalize API Surface

- Implement `hash_print` with behavior aligned to the C routine’s output order and scope as closely as practical.
- Keep printing logic in the same module instead of extracting formatter helpers.
- If the C function prints to a passed stream-like target, map to a writer-accepting function using `std::io::Write`.
- If the C function prints directly, keep the Rust function similarly direct unless existing Rust project conventions require a writer parameter.
- Review function signatures to ensure they are consistent with the actual migrated call sites and do not expose unnecessary generics or traits.

### Validation Focus

- printed output includes current table contents after insertions
- removed entries are no longer printed
- empty-table printing remains defined and non-panicking

## Phase 4: Add Focused Tests and Finalize Behavioral Parity

- Add unit tests in `src/gnu/hash.rs` or adjacent module tests covering only the migrated functions.
- Create tests for:
  - insert-when-absent behavior
  - duplicate insertion behavior
  - successful removal
  - removal miss
  - stable and expected print output for representative contents
- Resolve any mismatches between C assumptions and Rust ownership by tightening types rather than adding workaround layers.
- Perform a final pass to remove unused transitional code and confirm `cargo test` passes.

### Completion Criteria

- `gnu/hash.c` responsibilities for the targeted functions are represented in `src/gnu/hash.rs`
- no raw-pointer-style lifetime emulation remains unless forced by existing interfaces
- insert/remove/print paths are covered by tests
- module compiles cleanly and passes `cargo test`