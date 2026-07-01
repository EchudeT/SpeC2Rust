# Implementation Plan

## Summary

Port `gnu/hash.c` into a focused Rust module that preserves the existing hash-table behavior, iteration flow, and entry-management logic without adding new capabilities. The Rust implementation should migrate the current bucketed hash-table operations, internal entry allocation/free paths, lookup helpers, traversal APIs, and table validation logic into a single Rust module aligned with the original file boundary.

The technical approach is to represent the table, buckets, and chained entries with owned Rust data structures using `Box`, `Vec`, and `Option` in place of manual C allocation and pointer linking. Functions that were internal helpers in C should remain internal Rust functions or private methods. Public surface area should be limited to the operations implied by the existing function set: lookup, traversal, bulk entry collection, per-entry iteration, clearing, and destruction-equivalent cleanup through ownership and `Drop`. Memory management concerns from C should be translated into explicit ownership and borrowing rules, with iteration APIs designed carefully so traversal does not violate Rust aliasing rules.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::collections` should not replace the original implementation; use `Vec`, `Box`, `Option`, and iterator traits as building blocks)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the existing bucket-based hashing behavior and traversal complexity.
  - Keep lookup and insertion-path helper costs comparable to the C implementation.
  - Avoid unnecessary cloning of entries or keys during lookup and traversal.
  - Maintain linear-time clear/free behavior and bounded per-bucket traversal overhead.
  - Keep allocation patterns close to the original design: one allocation per entry node, bucket storage as a contiguous vector.

## Module Mapping

### Source File Mapping

- **C source**: `gnu/hash.c`
- **Rust target**: `src/gnu/hash.rs`

If the project already has a `gnu` module namespace, expose this file through:

- `src/gnu/mod.rs` → `pub mod hash;`

If not already present, add only the minimal module declaration required by the migrated file structure. Do not split the port into extra helper modules unless forced by existing crate layout.

### Function Mapping

Map each C function into either a private free function or an inherent method on the Rust hash-table type, keeping names close to the source for migration clarity.

| C Function | Rust Mapping | Notes |
|---|---|---|
| `hash_get_max_bucket_length` | `fn get_max_bucket_length(&self) -> usize` | Prefer inherent method on table type. |
| `hash_table_ok` | `fn table_ok(&self) -> bool` | Internal consistency check; keep available for tests/debug assertions. |
| `safe_hasher` | `fn safe_hasher(...) -> usize` | Private helper wrapping hash callback handling and bucket index computation. |
| `hash_lookup` | `fn lookup(&self, key: ...) -> Option<&T>` or equivalent | Return borrowed entry payload/reference as allowed by final data model. |
| `hash_get_first` | `fn get_first(&self) -> Option<...>` | Traversal entry point; may need index-based iterator state. |
| `hash_get_next` | `fn get_next(&self, state: ...) -> Option<...>` | Preserve original traversal semantics without unsafe aliasing. |
| `hash_get_entries` | `fn get_entries(&self, out: &mut Vec<...>) -> usize` or `fn entries(&self) -> Vec<...>` | Choose form that best matches current usage; avoid extra capability growth. |
| `hash_do_for_each` | `fn do_for_each(&self, f: ...) -> usize/bool` | Use closure parameter in place of function pointer callback. |
| `compute_bucket_size` | `fn compute_bucket_size(candidate: usize) -> usize` | Private helper. |
| `hash_clear` | `fn clear(&mut self)` | Remove all entries, retain table allocation if that matches current semantics. |
| `hash_free` | ownership-driven drop; optional `fn free(self)` not required | Prefer Rust drop semantics; only add explicit method if call-site compatibility requires it. |
| `allocate_entry` | `fn allocate_entry(...) -> Box<Entry<T>>` | Private helper if a separate node type is retained. |
| `free_entry` | implicit drop or private helper consuming node | Usually unnecessary as a public function in Rust. |
| `hash_find_entry` | `fn find_entry(&self / &mut self, key: ...) -> ...` | Internal helper for lookup/update path. |
| `transfer_entries` | `fn transfer_entries(...)` | Private helper used during resize/rebucket operations if present in source logic. |

## Data Model

The source analysis exposes only anonymous C data structures, so the Rust design should name structures by role derived from function behavior rather than inventing new abstractions.

### Proposed Rust Types

| C Structure Role | Rust Type | Mapping Notes |
|---|---|---|
| hash table object | `struct HashTable<T>` | Owns bucket array, entry count, and function callbacks/configuration if the C design stores them in the table. |
| bucket head storage | `Vec<Option<Box<Entry<T>>>>` | Replaces array of linked-list head pointers. |
| chained hash entry | `struct Entry<T>` | Contains stored payload/key and `next: Option<Box<Entry<T>>>`. |
| hash/equality callbacks | generic parameters or boxed function pointers | Use the narrowest approach that matches call sites; generic type parameters are preferred if the table is internal to the crate. |
| traversal cursor state | `struct IterState` or internal `(bucket_index, offset/node ref)` representation | Needed if `hash_get_first/hash_get_next` semantics are preserved directly. |
| callback for `hash_do_for_each` | closure parameter `impl FnMut(...)` | Replaces C function pointer callback. |

### Suggested Core Structure Shape

```rust
pub(crate) struct HashTable<T, H, E>
where
    H: Fn(&T) -> usize,
    E: Fn(&T, &T) -> bool,
{
    buckets: Vec<Option<Box<Entry<T>>>>,
    n_entries: usize,
    hasher: H,
    equals: E,
}
```

```rust
struct Entry<T> {
    value: T,
    next: Option<Box<Entry<T>>>,
}
```

This shape should be adjusted if the original C table stores separate key/data fields, cached hash values, or allocator hooks. Preserve only fields required by `gnu/hash.c`.

### Ownership and Memory Mapping

- C heap-allocated entry nodes map to `Box<Entry<T>>`.
- Null pointers map to `Option`.
- Bucket arrays map to `Vec`.
- Manual `hash_free` / `free_entry` paths become ordinary Rust drop behavior.
- Any C function returning raw entry pointers should become:
  - borrowed references (`&T`, `&Entry<T>`) where lifetime-safe,
  - index/cursor state for traversal where borrowing across calls would be awkward,
  - or mutable references only where the C code truly mutates matched entries.

### Error Handling Strategy

The original function list suggests mostly total operations rather than rich error-returning APIs. Use:

- `Option` for not-found lookup/traversal cases.
- `bool` for consistency checks and callback-driven continuation where that mirrors C behavior.
- `Result` only for operations that can fail due to allocation or invalid construction requirements visible in the original code path.
- Internal `debug_assert!` for invariants that `hash_table_ok` validates, while still keeping an explicit validation function for tests.

## Implementation Phases

## Phase 1: Establish the Rust module skeleton and core table types

### Goals
- Create `src/gnu/hash.rs`.
- Define the table and entry structs required to replace the anonymous C structures.
- Port bucket sizing and low-level hashing helpers first so later logic can build on stable internals.

### Tasks
- Add the Rust module file and minimal `mod` exposure needed by the crate.
- Define:
  - `HashTable`
  - `Entry`
  - any minimal iterator/cursor state required by traversal
- Implement private helpers:
  - `compute_bucket_size`
  - `safe_hasher`
  - `allocate_entry`
  - `free_entry` only if needed as an internal compatibility step
- Encode table invariants explicitly:
  - bucket vector length is fixed after initialization until resize/transfer
  - entry count tracks chained nodes exactly
  - bucket links are acyclic by construction through ownership

### Migration Notes
- Keep helper names close to C for easy review against `gnu/hash.c`.
- Do not replace the implementation with `std::collections::HashMap`; preserve the original bucket-chain structure.
- If the C code depends on callback-configured hashing/equality, store those callbacks in the table rather than introducing a broader trait framework.

### Exit Criteria
- Module compiles with struct definitions and helper functions in place.
- Unit tests cover bucket-size computation and basic safe hashing behavior.

## Phase 2: Port lookup and traversal behavior

### Goals
- Recreate the read-only operational core of the hash table.
- Preserve the original lookup path and traversal semantics before handling teardown and bulk operations.

### Tasks
- Implement internal search helper:
  - `hash_find_entry`
- Implement public/internal lookup path:
  - `hash_lookup`
- Implement traversal operations:
  - `hash_get_first`
  - `hash_get_next`
- Implement entry enumeration helpers:
  - `hash_get_entries`
  - `hash_do_for_each`

### Migration Notes
- Choose traversal representation carefully:
  - direct borrowed node references across separate calls are usually awkward in Rust,
  - prefer a cursor/index representation if the original API pattern requires `first` then `next`.
- If current call sites only need simple iteration, keep the C-named functions but back them with internal iterator logic.
- Avoid cloning stored values just to support traversal; prefer borrowed access or collecting references where feasible.

### Exit Criteria
- Lookup returns the same match behavior as the C implementation.
- Traversal covers all entries exactly once across buckets.
- `cargo test` includes collision-chain cases and empty-table traversal cases.

## Phase 3: Port maintenance, transfer, and teardown logic

### Goals
- Complete the mutating and lifecycle-related parts of the module.
- Preserve semantics for clearing, freeing, validation, and any rebucketing/transfer logic present in the source.

### Tasks
- Implement:
  - `transfer_entries`
  - `hash_clear`
  - `hash_free` as ownership-based teardown or compatibility wrapper if necessary
  - `hash_get_max_bucket_length`
  - `hash_table_ok`
- Wire any resize/rebucket path to use `transfer_entries` if that exists in the C flow.
- Ensure `clear` removes all chains and resets counts without leaking nodes.
- Replace explicit free logic with drop-driven destruction where possible.

### Migration Notes
- `hash_free` should normally become unnecessary once ownership is correct; only expose an explicit method if existing Rust-side call sites need a named equivalent.
- `transfer_entries` should move nodes between bucket arrays without duplicating payloads.
- Validation logic should check:
  - entry counts match actual chain lengths,
  - bucket indices derived from stored entries are consistent,
  - no orphaned nodes remain after clear/transfer.

### Exit Criteria
- Clear and drop paths release all entries through normal Rust ownership.
- Validation passes on populated, cleared, and transferred tables.
- Tests cover repeated clear/use patterns if such reuse exists in the original module.

## Phase 4: Stabilize against the C behavior and finalize review

### Goals
- Confirm the Rust port matches the original module behavior and file scope.
- Finish with targeted cleanup only, without broadening the design.

### Tasks
- Review each C function in `gnu/hash.c` and mark its Rust counterpart complete.
- Add focused tests for:
  - collision-heavy buckets
  - first/next traversal ordering as defined by the original table layout
  - bulk entry collection count accuracy
  - maximum bucket length calculation
  - table validation before and after transfer/clear
- Remove any temporary compatibility helpers that became unnecessary during porting.
- Verify visibility is minimal: internal helpers stay private unless cross-file use already exists.

### Exit Criteria
- Every listed C function has a mapped Rust implementation or an intentional ownership-based replacement.
- The module is limited to the `gnu/hash.c` migration scope.
- `cargo test` passes on the branch `007-module_gnu_hash_entry_01-rust-port`.