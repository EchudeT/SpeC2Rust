# Implementation Plan: module_gnu_hash_entry_01

## Summary

This module ports the hash table entry-management logic from `gnu/hash.c` into Rust, preserving the existing operational behavior and call structure as closely as practical. The Rust implementation should focus on the current responsibilities present in the C file: bucket sizing, entry allocation and release, lookup and iteration, table validation, table clearing/freeing, and entry transfer during table reorganization.

The technical approach is to replace manual memory management and pointer-linked structures with ownership-based Rust data structures while keeping the original function boundaries recognizable in the migrated code. The preferred implementation is a private hash-table module backed by standard-library containers and explicit internal entry records, with iteration and mutation logic written to match the C behavior rather than redesigning the API around idiomatic Rust collections. Error-prone C patterns such as null checks, manual free, and unchecked traversal should be translated into `Option`, bounded indexing, and explicit result handling where needed.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended by default, since the input provides no evidence requiring external hashing, allocation, or iterator libraries
- **Testing**:
  - `cargo test`
  - Unit tests colocated with the Rust module
  - Focused behavior-preservation tests for lookup, iteration order assumptions, bucket sizing, clear/free-equivalent state transitions, and entry transfer
- **Performance Goals**:
  - Maintain comparable asymptotic behavior to the C implementation for lookup, insertion-path support functions, and traversal
  - Avoid unnecessary cloning of keys or values during lookup and transfer
  - Keep bucket operations bounded to the same conceptual structure as the original implementation
  - Preserve efficient clear and teardown behavior through Rust-owned deallocation rather than per-call manual frees where possible

## Module Mapping

### Source Mapping

- **C source file**
  - `gnu/hash.c`

- **Rust target**
  - `src/gnu/hash.rs`

If the project already exposes GNU-related modules from a parent file, this module should be added through the existing Rust module tree only. No additional architectural layers should be introduced beyond what is required to host the migrated file.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `hash_get_max_bucket_length` | `hash_get_max_bucket_length` | Keep as a table-inspection helper |
| `hash_table_ok` | `hash_table_ok` | Convert internal consistency checks to safe traversal/index validation |
| `safe_hasher` | `safe_hasher` | Wrap hash computation in total, safe Rust logic |
| `hash_lookup` | `hash_lookup` | Port as lookup against internal bucket/entry representation |
| `hash_get_first` | `hash_get_first` | Port as first-entry traversal helper |
| `hash_get_next` | `hash_get_next` | Port as next-entry traversal helper |
| `hash_get_entries` | `hash_get_entries` | Port as entry collection/count helper based on existing semantics |
| `hash_do_for_each` | `hash_do_for_each` | Port as callback-style traversal or closure-based internal helper |
| `compute_bucket_size` | `compute_bucket_size` | Keep as internal sizing function |
| `hash_clear` | `hash_clear` | Reset contents while preserving reusable table allocation if C behavior does so |
| `hash_free` | `hash_free` | In Rust, likely consume or clear the owning table structure |
| `allocate_entry` | `allocate_entry` | Replace raw allocation with constructor/helper creating owned entry values |
| `free_entry` | `free_entry` | Replace explicit free with drop/removal helper if separate function remains useful |
| `hash_find_entry` | `hash_find_entry` | Internal bucket-chain search helper |
| `transfer_entries` | `transfer_entries` | Rebuild or move entries during resize/rebucket operation |

## Data Model

The input exposes only anonymous C structures, so the Rust plan should define named internal types based on actual use in `gnu/hash.c`, without inventing extra capabilities.

### Planned Rust Data Structures

| C Shape | Rust Type | Purpose |
|---|---|---|
| anonymous table struct | `struct HashTable<T>` or concrete internal table struct | Owns buckets, entry count, sizing metadata, and hasher/comparison hooks if present in C logic |
| anonymous entry struct | `struct HashEntry<T>` | Stores payload plus next-link or equivalent chaining metadata |
| anonymous bucket representation | `Vec<Option<EntryIndex>>` or `Vec<Option<Box<HashEntry<T>>>>` | Represents bucket heads safely |
| anonymous traversal state | explicit index/cursor type or borrowed iterator state | Supports `hash_get_first` / `hash_get_next` behavior |
| anonymous callback/result-related structs | closure parameters or small helper enums/structs | Only if needed to preserve `hash_do_for_each` semantics |

### Representation Strategy

Because the C file likely uses chained buckets and manually allocated entries, the Rust implementation should choose one of the following restrained internal representations:

1. **Preferred if direct chaining behavior matters strongly**
   - `Vec<Option<Box<HashEntry>>>` for bucket heads
   - `HashEntry` contains `next: Option<Box<HashEntry>>`
   - Closest migration from C pointer-chain logic
   - Simplifies preserving functions like `allocate_entry`, `free_entry`, `hash_find_entry`, and `transfer_entries`

2. **Alternative if ownership during transfer becomes too complex**
   - `Vec<Option<usize>>` buckets plus `Vec<EntrySlot>` arena storage
   - Entries linked by indices instead of pointers
   - Safer for relocation-heavy logic, but should only be used if needed by the migrated implementation

The plan should begin with the first representation unless inspection of `gnu/hash.c` shows that repeated chain splicing and traversal-state preservation are materially easier with index-based storage.

### Memory Management Mapping

| C Concept | Rust Handling |
|---|---|
| malloc/calloc for entries | owned construction via `Box` or plain struct creation |
| free per entry | implicit `Drop`; explicit removal helper only when preserving control flow |
| null pointer for absent link | `Option` |
| manual table free | ownership drop or explicit `clear`/consume function |
| pointer traversal | borrowed references / `Option::as_deref()` / mutable link walking |

### Error Handling Mapping

| C Pattern | Rust Approach |
|---|---|
| null or invalid table checks | `Option`, safe references, or internal validation returning `bool` |
| allocation failure paths | generally omitted at call sites under standard Rust allocation model unless the surrounding project already models allocation failure |
| invalid bucket/index access | guarded indexing and checked traversal |
| callback failure propagation | preserve only if present in the C logic; otherwise use simple boolean/count return paths |

## Implementation Phases

## Phase 1: Establish the Rust Hash Table Skeleton

- Create `src/gnu/hash.rs` and wire it into the existing module tree.
- Define the main internal table and entry types based on the actual fields used in `gnu/hash.c`.
- Port non-mutating structural helpers first:
  - `compute_bucket_size`
  - `safe_hasher`
  - `hash_table_ok`
  - `hash_get_max_bucket_length`
- Decide and lock the bucket/entry ownership representation after confirming how chains are manipulated in C.
- Add initial unit tests for:
  - bucket size computation behavior
  - empty-table validation
  - hash helper determinism within one process
  - max bucket length on empty and simple populated states

## Phase 2: Port Entry Allocation, Lookup, and Traversal

- Implement entry creation/removal helpers:
  - `allocate_entry`
  - `free_entry` as a removal/drop-oriented helper if needed for migration clarity
- Port core search logic:
  - `hash_find_entry`
  - `hash_lookup`
- Port traversal functions in the same phase so cursor assumptions remain aligned:
  - `hash_get_first`
  - `hash_get_next`
  - `hash_get_entries`
  - `hash_do_for_each`
- Keep function names and sequencing close to the C implementation to reduce migration risk.
- Add tests covering:
  - lookup hit/miss
  - first/next traversal over empty and non-empty tables
  - full entry enumeration consistency
  - callback-based iteration visiting expected entries exactly once

## Phase 3: Port Clearing, Freeing, and Entry Transfer Logic

- Implement table content reset:
  - `hash_clear`
- Implement final ownership teardown equivalent:
  - `hash_free`
  - In Rust this may become a thin wrapper or consume/reset helper, depending on how the rest of the port expects to call it
- Port rehash/rebucket logic:
  - `transfer_entries`
- Verify that transfer preserves entry reachability and count correctness.
- Add tests covering:
  - clear leaves the table reusable if that matches the C behavior
  - free-equivalent leaves no accessible contents
  - transfer preserves all entries across bucket-size changes
  - table validation before and after transfer

## Phase 4: Behavioral Alignment and Cleanup

- Compare each migrated function against `gnu/hash.c` control flow and edge handling to remove accidental semantic drift.
- Minimize unnecessary allocations, clones, or temporary collections introduced during the initial port.
- Tighten visibility so helper functions remain private unless the original module boundary requires exposure.
- Consolidate tests around:
  - repeated clear/reuse cycles
  - dense collision scenarios
  - validation of bucket-chain integrity after lookup/traversal/transfer sequences
- Finish with `cargo test` passing on the branch `007-module_gnu_hash_entry_01-rust-port`.