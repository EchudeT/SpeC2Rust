# Implementation Plan: module_gnu_hash.c_31

## Summary

This plan ports `gnu/hash.c` into a single Rust module that preserves the existing hash-table behavior and migration boundaries without adding new capabilities. The Rust implementation should focus on the existing function set:

- table initialization and tuning validation
- raw hashing and comparison callbacks
- insert/delete operations
- rehash behavior
- tuning reset
- statistics output

The technical approach is to translate the current C hash-table logic into an owned Rust table implementation using standard-library collections and allocation primitives only where they match the original behavior. Where the C code relies on callback-driven hashing/comparison and explicit memory handling, the Rust version should model those with typed function pointers or generics constrained to the migrated module scope, while keeping the public surface narrow and aligned to the original file.

The implementation should prefer safe Rust for table state management, bucket storage, and resizing logic. Any low-level compatibility behavior that cannot be expressed ergonomically in safe code should be isolated and minimized. Error handling should replace C-style null/error signaling with explicit `Result`/`Option` return values, but only to the extent needed to represent existing outcomes from the original functions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain expected constant-time average behavior for insert, delete, and lookup-related internal operations.
  - Preserve rehash/tuning behavior closely enough to avoid regressions in load-factor handling.
  - Avoid unnecessary allocations during normal insertion/deletion paths beyond bucket growth and rehash events.
  - Keep callback dispatch and statistics collection lightweight and local to the module.

## Module Mapping

### Source to Destination

- `gnu/hash.c` → `src/module_gnu_hash.rs`

If the crate already groups migrated ports by original path, this may instead be placed at:

- `gnu/hash.c` → `src/gnu/hash.rs`

The preferred choice is the location already used by the repository for migrated C modules. The port should remain a single Rust source module corresponding directly to this file.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `hash_print_statistics` | `pub(crate) fn hash_print_statistics(...)` | Preserve diagnostic/statistics behavior; route output through Rust writer interfaces or stderr/stdout only if the original behavior requires direct printing. |
| `hash_reset_tuning` | `pub(crate) fn hash_reset_tuning(...)` | Reset tuning state using mutable references instead of raw pointers. |
| `raw_hasher` | `fn raw_hasher(...) -> u64` or `usize` | Internal callback adapter; exact integer type should match indexing/resizing needs. |
| `raw_comparator` | `fn raw_comparator(...) -> bool` | Internal equality adapter replacing integer comparison conventions. |
| `check_tuning` | `fn check_tuning(...) -> Result<..., ...>` or `bool` | Internal validator for tuning parameters. |
| `hash_initialize` | `pub(crate) fn hash_initialize(...) -> Result<HashTable, ...>` | Replace nullable allocation result with explicit construction result. |
| `hash_rehash` | `fn hash_rehash(...) -> Result<(), ...>` | Internal resizing operation with ownership-preserving bucket rebuild. |
| `hash_insert` | `pub(crate) fn hash_insert(...) -> Result<..., ...>` or `Option<...>` | Preserve insertion semantics, especially duplicate/existing-entry behavior. |
| `hash_delete` | `pub(crate) fn hash_delete(...) -> Option<...>` | Return removed entry/value if the original C API exposes it; otherwise bool-like success can be used. |

## Data Model

The C analysis reports only anonymous structures, so the Rust plan should introduce minimal named internal types derived from actual usage in `gnu/hash.c`.

### Proposed Rust Data Structures

| C Structure | Rust Representation | Purpose |
|---|---|---|
| anonymous table state struct | `struct HashTable<T>` | Owns buckets, counts, tuning, and callback configuration. |
| anonymous tuning struct | `struct HashTuning` | Stores load/growth/shrink thresholds and resize policy fields. |
| anonymous bucket/entry node | `struct HashEntry<T>` | Represents one stored element or one bucket node, depending on original collision strategy. |
| anonymous statistics state | `struct HashStatistics` | Temporary/internal aggregation for `hash_print_statistics`. |
| anonymous callback bundle | `struct HashCallbacks<T>` | Holds hasher and comparator function pointers used by the table. |
| anonymous resize/result flags | `enum InsertResult<T>` / `enum HashError` as needed | Encodes C success/failure/duplicate outcomes only where necessary. |

### Ownership and Memory Mapping

- C raw table allocations map to owned Rust containers such as `Vec<Option<HashEntry<T>>>`, `Vec<Vec<HashEntry<T>>>`, or another direct bucket representation that matches the original collision model.
- C pointer-based mutable state maps to `&mut HashTable<T>` and `&mut HashTuning`.
- C null checks map to `Option`.
- C allocation or invalid-parameter failures map to `Result`.
- C callback pointers map to plain function pointers, for example:
  - `type Hasher<T> = fn(&T) -> u64`
  - `type Comparator<T> = fn(&T, &T) -> bool`

The exact generic shape should be kept as narrow as possible. If the migrated project already uses erased pointer-like entries to preserve C semantics, the Rust data model may instead use a less generic internal representation, but it should remain confined to this module.

### Collision Strategy

The bucket/entry representation should be chosen after reading `gnu/hash.c` and should mirror the original algorithm rather than replacing it with `std::collections::HashMap`. This is important because the file exposes tuning, raw hashing/comparison, statistics, and explicit rehash behavior, all of which are usually implementation-specific. Standard-library maps should therefore not replace the table internals unless the original exported semantics are proven not to depend on bucket layout, statistics, or tuning logic.

## Implementation Phases

### Phase 1: Module Skeleton and Type Translation

- Create the Rust destination file corresponding to `gnu/hash.c`.
- Identify the anonymous C structs in use and assign minimal Rust names based on role:
  - table
  - tuning
  - entry/bucket node
  - callback bundle
  - statistics helper
- Define the Rust function signatures for:
  - `hash_initialize`
  - `hash_insert`
  - `hash_delete`
  - `hash_print_statistics`
  - `hash_reset_tuning`
  - internal helpers `raw_hasher`, `raw_comparator`, `check_tuning`, `hash_rehash`
- Translate global constants/macros from the C file into `const` items or small helper functions.
- Establish the bucket storage representation that most directly matches the C implementation.

**Deliverable**: Compiling module skeleton with data structures, constants, and placeholder function bodies.

### Phase 2: Core Table Logic Migration

- Implement `check_tuning` first so initialization and resizing constraints are defined before table operations.
- Implement `hash_initialize` with:
  - validated tuning
  - initial bucket allocation
  - callback setup
  - count/capacity initialization
- Implement `raw_hasher` and `raw_comparator` as internal adapters around the stored callbacks.
- Implement `hash_insert` preserving:
  - duplicate detection behavior
  - bucket placement logic
  - load-factor checks leading to `hash_rehash`
- Implement `hash_delete` preserving:
  - lookup/removal behavior
  - entry count updates
  - any post-delete resize conditions that exist in the C code
- Implement `hash_rehash` to rebuild bucket state without leaking or duplicating entries.

**Deliverable**: Functional table operations matching the original file behavior and compiling cleanly.

### Phase 3: Tuning Reset, Statistics, and Error Path Alignment

- Implement `hash_reset_tuning` to restore default or caller-provided tuning state exactly as in the C logic.
- Implement `hash_print_statistics` using a Rust formatting path that preserves the original reported values and computation order.
- Audit all error/edge paths from the C file:
  - invalid tuning
  - zero/invalid capacity handling
  - insertion into full or failed-resize states
  - delete of missing entry
- Replace any remaining C-style sentinel handling with `Option`/`Result` while keeping calling semantics close to the source.
- Ensure memory ownership is explicit and no equivalent of C dangling-pointer behavior remains.

**Deliverable**: Complete port with operational diagnostics and aligned edge-case handling.

### Phase 4: Targeted Tests and Migration Verification

- Add unit tests in the same module or the crate’s standard test location covering:
  - initialization with valid and invalid tuning
  - insert of new entry
  - insert of duplicate/equivalent entry
  - delete existing and missing entry
  - rehash triggered by growth conditions
  - tuning reset behavior
  - statistics function smoke test
- Add focused tests for callback behavior to confirm `raw_hasher` and `raw_comparator` integration.
- Verify count/capacity transitions before and after rehash.
- Run `cargo test` and fix any semantic mismatches identified during migration.

**Deliverable**: Passing tests for the migrated function set and stable Rust ownership/error behavior.

## Migration Notes

### Memory Management

- Eliminate manual allocation/free patterns from the C implementation by making the table the sole owner of bucket storage.
- If entries in the original C code are externally owned and only referenced by the table, reflect that explicitly in Rust through borrowed or pointer-like representations only if required by existing crate conventions.
- Avoid introducing shared ownership unless the source semantics require it.

### Error Handling

- Use `Result` for initialization and rehash failures where allocation or invalid configuration can occur.
- Use `Option` or a small result enum for insert/delete operations when representing presence/absence or duplicate outcomes.
- Keep error types local to the module unless the surrounding crate already defines a shared error type used by other migrated modules.

### Scope Control

- Do not split this migration into extra helper modules unless the existing Rust project layout already requires it.
- Do not add new APIs beyond those needed to replace the functions in `gnu/hash.c`.
- Do not generalize the implementation beyond the behavior necessary for this file’s current call patterns.