# Implementation Plan: module_gnu_hash.c_31

## Summary

This module ports the hash-table logic from `gnu/hash.c` into a Rust module that preserves the existing behavior and function boundaries as closely as practical. The Rust implementation should focus on migrating the current table lifecycle and mutation operations—initialization, tuning validation/reset, hashing/comparison adapters, rehashing, insertion, deletion, and statistics output—without introducing broader API redesign or extra capabilities.

The technical approach is to implement a dedicated hash-table module using Rust standard-library primitives and explicit data structures rather than replacing behavior with `std::collections::HashMap`, because the C source exposes internal tuning and rehash behavior that must remain under module control. Ownership and memory management will be made explicit through Rust structs, `Vec`-backed bucket/storage management, and `Option`-based occupancy representation. Error paths that were implicit or pointer-based in C should become explicit `Result`/`Option` returns in Rust where required by the migrated function signatures.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are planned, since the input does not require external hashing, allocation, or compatibility libraries
- **Testing**:
  - `cargo test`
  - Unit tests focused on initialization, tuning validation, insert/delete behavior, rehash triggers, and statistics formatting stability where applicable
- **Performance Goals**:
  - Preserve expected average-case hash-table operation costs comparable to the C module
  - Avoid unnecessary heap allocations during lookup/insert/delete hot paths
  - Maintain controlled rehash behavior driven by migrated tuning logic
  - Keep data layout simple and predictable, using contiguous storage where possible

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/hash.c` | `src/gnu/hash.rs` | Primary port target containing the hash-table types and migrated functions |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `hash_print_statistics` | `pub fn hash_print_statistics(...)` or internal helper in `src/gnu/hash.rs` | Preserve diagnostic/statistics calculation; adapt output handling to Rust writer APIs if needed |
| `hash_reset_tuning` | `pub fn hash_reset_tuning(...)` | Translate tuning reset logic directly onto Rust tuning struct |
| `raw_hasher` | `fn raw_hasher(...)` | Keep as internal helper matching original low-level hashing role |
| `raw_comparator` | `fn raw_comparator(...)` | Keep as internal helper for equality checks between stored entries |
| `check_tuning` | `fn check_tuning(...) -> bool/Result` | Validate tuning fields explicitly before table creation/rehash |
| `hash_initialize` | `pub fn hash_initialize(...) -> Result<..., ...>` | Construct Rust table state with validated tuning and allocated buckets |
| `hash_rehash` | `fn hash_rehash(...) -> Result<(), ...>` | Rebuild internal storage while preserving existing entries |
| `hash_insert` | `pub fn hash_insert(...) -> Result<..., ...>` | Port insertion semantics, including growth checks and duplicate handling |
| `hash_delete` | `pub fn hash_delete(...) -> Option<...>` | Port deletion semantics with slot cleanup and size updates |

## Data Model

Because the input only exposes anonymous C data structures, the Rust design should introduce named internal types that correspond to the roles inferred from the functions. Naming should stay narrowly scoped to the migrated module.

| C Structure Role | Rust Type | Notes |
|---|---|---|
| Hash table state | `struct HashTable<T>` | Owns bucket/storage state, counters, tuning, and callback behavior if still required |
| Hash tuning/configuration | `struct HashTuning` | Holds growth/shrink thresholds and candidate sizing parameters migrated from C |
| Bucket/slot entry | `enum Slot<T>` or `Option<Entry<T>>` | Represents empty/occupied/deleted state as needed by the original probing/deletion strategy |
| Stored entry wrapper | `struct Entry<T>` | Stores value plus any cached hash or metadata if the C code relies on it |
| Statistics snapshot/input state | `struct HashStatistics` if needed | Only if statistics logic is clearer with a dedicated temporary struct; otherwise keep local |
| Comparator callback role | generic bound or stored function pointer | Use function pointers or generic closures only if required by the C API shape |
| Hasher callback role | generic bound or stored function pointer | Preserve explicit hashing control rather than deferring to derived `Hash` implementations |

### Suggested Rust Shape

```rust
pub struct HashTuning {
    // migrated numeric tuning fields
}

pub struct HashTable<T> {
    buckets: Vec<Slot<T>>,
    items: usize,
    tuning: HashTuning,
    // optional callback fields if required by source behavior
}

enum Slot<T> {
    Empty,
    Deleted,
    Occupied(Entry<T>),
}

struct Entry<T> {
    value: T,
    hash: u64,
}
```

### Memory Management Notes

- Replace C manual allocation/free with owned `Vec` storage.
- Eliminate raw pointer lifetime ambiguity by storing owned values or clearly borrowed references with explicit lifetimes if the original API requires non-owning semantics.
- If the C implementation stores opaque pointers, the Rust port should avoid unsafe by modeling the contained item type explicitly where the surrounding codebase allows it.
- Rehashing should allocate a new bucket array and move entries into it, relying on Rust move semantics rather than manual element copying.
- Deleted-slot handling must preserve the original probing semantics; use an explicit `Deleted` variant if open addressing requires tombstones.

### Error Handling Notes

- Initialization and rehash allocation failures should map to `Result`.
- Invalid tuning data should be rejected explicitly instead of relying on sentinel returns alone.
- Insert/delete should use `Option`/`Result` according to whether absence/failure is a normal outcome or a true error condition.
- Statistics printing should propagate I/O errors if the Rust signature accepts a writer; otherwise keep it infallible if output remains internal.

## Implementation Phases

## Phase 1: Establish module skeleton and core data structures

- Create `src/gnu/hash.rs` as the direct destination for `gnu/hash.c`.
- Define Rust equivalents for the table state, tuning state, and slot/entry representation.
- Implement `hash_reset_tuning` and `check_tuning` first, since they constrain initialization and rehash behavior.
- Implement `raw_hasher` and `raw_comparator` as internal helpers with signatures shaped around the chosen table representation.
- Add focused unit tests for tuning reset/validation and helper behavior.

### Deliverables

- Compiling module skeleton
- Named Rust types replacing anonymous C structs
- Basic helper tests passing under `cargo test`

## Phase 2: Port table construction and resizing behavior

- Implement `hash_initialize` using the migrated tuning checks and bucket allocation logic.
- Implement `hash_rehash` with a new bucket array and reinsertion of existing occupied entries.
- Preserve resize thresholds and table-size decisions from the C logic as closely as possible.
- Verify item counts, empty-table behavior, and resize correctness with unit tests.

### Deliverables

- Functional table creation path
- Rehash path validated by tests
- Explicit handling of allocation/validation errors

## Phase 3: Port mutation operations

- Implement `hash_insert` with duplicate detection, probe/update rules, and pre-insert growth checks.
- Implement `hash_delete` with correct removal semantics and tombstone or compaction behavior matching the original strategy.
- Ensure that size counters and post-delete table invariants remain correct.
- Add tests for:
  - insert into empty table
  - duplicate insert behavior
  - insert triggering rehash
  - delete existing item
  - delete missing item
  - continued lookup/probing correctness after deletions

### Deliverables

- Working insert/delete operations
- Stable table invariants across mutations
- Mutation-focused test coverage

## Phase 4: Port statistics output and finalize behavior matching

- Implement `hash_print_statistics` using Rust-side traversal of buckets and counters.
- Compare the output structure and computed values against the C module’s intent, preserving technical meaning without expanding reporting scope.
- Perform a final pass over signatures and visibility so the Rust module exposes only the migrated surface needed by the project.
- Clean up any remaining C-oriented sentinel logic into idiomatic but behavior-preserving Rust returns.

### Deliverables

- Statistics function migrated
- Final behavior alignment pass completed
- Complete module test suite passing

## Acceptance Notes

- The Rust module should remain a direct migration target for `gnu/hash.c`, not a generalized collection framework.
- Internal algorithm choices must remain driven by the original table tuning and rehash design.
- Unsafe code should be avoided unless the surrounding project API makes it unavoidable; if any unsafe block becomes necessary, keep it isolated to the smallest possible scope and document the invariant it preserves.
- Completion is reached when all listed functions have Rust counterparts in `src/gnu/hash.rs` and the module passes `cargo test`.