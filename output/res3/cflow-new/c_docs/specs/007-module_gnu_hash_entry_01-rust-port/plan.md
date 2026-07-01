# Implementation Plan: module_gnu_hash_entry_01

## Summary

This module ports the hash-table entry management logic from `gnu/hash.c` into a focused Rust module within the `cflow-new` crate. The implementation should preserve the existing behavior and traversal/update semantics of the C code while replacing manual allocation, pointer-linked storage, and explicit lifetime management with Rust-owned containers and borrowing rules.

The Rust approach should center on a single internal hash-table module that migrates the existing functions in place rather than redesigning the feature set. The key technical decisions are:

- represent the table, buckets, and entries with explicit Rust structs,
- replace raw allocation/free paths with ownership via `Vec`, `Box`, and standard drop behavior,
- model callback-driven hashing and iteration in a way that preserves current call patterns,
- keep validation, lookup, traversal, clearing, and transfer logic close to the original function boundaries so the migration remains auditable.

The port should prefer behavioral equivalence over abstraction. Helper logic should only be introduced where needed to replace C memory and pointer operations safely.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended from the available evidence
- **Testing**:
  - `cargo test`
  - unit tests placed alongside the Rust module
- **Performance Goals**:
  - preserve expected average-case lookup behavior of the original bucketed hash table
  - avoid unnecessary heap churn during lookup and iteration
  - keep clear/free and transfer operations linear in the number of stored entries
  - maintain bucket traversal costs comparable to the C implementation
  - avoid cloning entry payloads unless required by ownership constraints

## Module Mapping

### Source File Mapping

- **C source**: `gnu/hash.c`
- **Rust target**: `src/gnu/hash.rs`

If the crate already exposes a `gnu` namespace, this module should be added there directly. If the existing tree uses a different path convention, the implementation should still remain a single Rust module corresponding to `gnu/hash.c`, without splitting behavior across extra files unless required by the current crate layout.

### Function Mapping

Each C function should map to a Rust function or method with closely corresponding responsibility:

- `hash_get_max_bucket_length`
  - Rust: `HashTable::max_bucket_length(&self) -> usize`
- `hash_table_ok`
  - Rust: `HashTable::is_valid(&self) -> bool`
- `safe_hasher`
  - Rust: internal helper such as `fn safe_hasher(...) -> usize`
- `hash_lookup`
  - Rust: `HashTable::lookup(...) -> Option<...>`
- `hash_get_first`
  - Rust: `HashTable::first_entry(...) -> Option<...>`
- `hash_get_next`
  - Rust: `HashTable::next_entry(...) -> Option<...>`
- `hash_get_entries`
  - Rust: `HashTable::entries(...) -> usize` or collection-filling helper, depending on original C output pattern
- `hash_do_for_each`
  - Rust: `HashTable::for_each(...)`
- `compute_bucket_size`
  - Rust: internal helper `fn compute_bucket_size(...) -> usize`
- `hash_clear`
  - Rust: `HashTable::clear(&mut self)`
- `hash_free`
  - Rust: ownership-driven drop path, plus optional explicit consuming function if the surrounding API expects it
- `allocate_entry`
  - Rust: internal helper creating an `Entry`
- `free_entry`
  - Rust: removed as explicit API; covered by ownership/drop, with a helper only if deallocation side effects must be preserved
- `hash_find_entry`
  - Rust: internal lookup helper returning bucket/entry position information
- `transfer_entries`
  - Rust: internal rehash/move helper used during resize or rebuild

## Data Model

The analysis only exposes anonymous C structures, so the Rust data model should be reconstructed around the observed function set and the storage needs implied by lookup, traversal, and transfer.

### Core Mapping Strategy

| C construct | Rust mapping |
|---|---|
| anonymous hash table struct | `struct HashTable<T>` |
| anonymous entry/node struct | `struct Entry<T>` |
| bucket array | `Vec<Option<Box<Entry<T>>>>` or `Vec<Option<usize>>` with indexed storage |
| linked next pointer | `Option<Box<Entry<T>>>` or indexed next link |
| function pointer for hashing | generic callback field or function pointer type |
| function pointer for equality | generic callback field or function pointer type |
| manual allocation/free | `Box`, `Vec`, and automatic drop |
| nullable pointer return | `Option<T>` / `Option<&T>` / `Option<&mut T>` |
| status integer / boolean checks | `bool` or `Result` where failure is not just false |

### Recommended Rust Structures

The implementation should stay minimal and close to the C layout:

```rust
pub struct HashTable<T, H, E> {
    buckets: Vec<Option<Box<Entry<T>>>>,
    items: usize,
    hasher: H,
    equals: E,
}

struct Entry<T> {
    value: T,
    next: Option<Box<Entry<T>>>,
}
```

This linked-bucket design is the most direct migration path from C if `gnu/hash.c` uses chained entries. It minimizes behavioral drift and makes functions such as first/next, bucket-length calculation, find-entry, and transfer-entries straightforward to port.

If the surrounding crate requires borrowed payloads rather than owned values, the generic parameter may instead be an internal pointer-like representation, but the plan should not introduce that unless the existing Rust codebase already requires it.

### Error Handling and Invariants

- Use `bool` for structural validity checks corresponding to `hash_table_ok`.
- Use `Option` for nullable lookup and iteration results.
- Use `Result` only where Rust allocation or callback setup introduces a distinct construction-time failure path.
- Enforce internal invariants:
  - bucket vector length matches the configured table size,
  - each entry belongs to exactly one bucket chain,
  - item count stays synchronized with stored entries,
  - transfer/clear operations leave no dangling ownership paths.

## Implementation Phases

## Phase 1: Establish module skeleton and core data structures

### Goals
Create the Rust file corresponding to `gnu/hash.c`, define the table and entry structures, and migrate the non-mutating/internal structural helpers first.

### Tasks
- Add `src/gnu/hash.rs`.
- Define the `HashTable` and `Entry` structs.
- Define the callback types or generic parameters for hashing and equality.
- Implement:
  - `compute_bucket_size`
  - `safe_hasher`
  - `hash_table_ok`
  - `hash_get_max_bucket_length`
- Add focused unit tests for:
  - bucket size computation,
  - empty-table validity,
  - max bucket length on empty and simple populated tables.

### Notes
This phase should settle the representation choice early. Prefer the linked-entry bucket model unless the existing Rust crate architecture makes an indexed arena clearly necessary.

## Phase 2: Port lookup and traversal behavior

### Goals
Migrate entry discovery and iteration semantics while keeping function boundaries close to the C source.

### Tasks
- Implement internal entry creation/search helpers:
  - `allocate_entry`
  - `hash_find_entry`
- Implement lookup and traversal:
  - `hash_lookup`
  - `hash_get_first`
  - `hash_get_next`
  - `hash_get_entries`
  - `hash_do_for_each`
- Resolve iteration API shape in Rust:
  - use returned references where the C code returns entry/data pointers,
  - keep callback-based traversal where the C API is callback-oriented.
- Add tests for:
  - lookup hit/miss,
  - traversal order consistency within bucket chains,
  - entry collection count,
  - callback visitation across all elements.

### Notes
If the C code exposes stateful “first/next” traversal through raw pointers, the Rust port should model that with explicit cursor-like return values only as much as needed to preserve behavior. Avoid introducing a broader iterator framework unless it directly replaces the original mechanics.

## Phase 3: Port mutation, clearing, and ownership-based destruction

### Goals
Complete mutating lifecycle behavior by replacing manual C allocation/free flows with safe Rust ownership transitions.

### Tasks
- Implement:
  - `hash_clear`
  - `hash_free`
  - `free_entry`
- Translate explicit deallocation paths into:
  - chain drops through `Option<Box<Entry<T>>>`,
  - `Vec::clear` or bucket reset behavior,
  - item count reset and invariant restoration.
- Ensure no extra explicit free API is retained unless needed for call-site compatibility.
- Add tests for:
  - clear removes all entries,
  - repeated clear is safe,
  - destruction leaves no stale accessible state through public methods.

### Notes
In Rust, `hash_free` will likely become a consuming operation or disappear into `Drop`. If call sites require a named function, keep a thin compatibility method that consumes `self` or resets internal storage without inventing extra resource-management layers.

## Phase 4: Port transfer logic and finalize parity checks

### Goals
Complete the resize/rehash-related path and validate that the Rust module preserves the structural behavior of the original C implementation.

### Tasks
- Implement:
  - `transfer_entries`
- Integrate transfer logic with existing bucket storage and item accounting.
- Verify that re-bucketing preserves all entries and does not duplicate or lose ownership.
- Add tests for:
  - transferring from one bucket layout to another,
  - preservation of lookup results after transfer,
  - stable item counts before and after transfer,
  - validity checks after mutation-heavy sequences.

### Notes
This phase should remain narrowly scoped to the behavior already present in `gnu/hash.c`. Do not add resizing policies or new public APIs beyond what is required to host the transferred logic.

## Completion Criteria

The module is complete when:

- all listed functions from `gnu/hash.c` have a direct Rust implementation or an explicit ownership-based replacement,
- the implementation resides in the Rust counterpart to the original source file,
- memory lifecycle is fully handled by Rust ownership without raw manual free logic,
- `cargo test` covers empty, single-entry, collision-chain, traversal, clear, and transfer scenarios,
- no extra capabilities beyond the migrated C behavior have been introduced.