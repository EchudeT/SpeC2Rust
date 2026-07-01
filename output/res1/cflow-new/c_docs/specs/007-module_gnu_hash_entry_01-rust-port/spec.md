# spec.md

## Title

Rust Functional Specification for `module_gnu_hash_entry_01`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_hash_entry_01`
- Category: `module_cluster`
- Source basis: `gnu/hash.c`
- Rust branch target: `007-module_gnu_hash_entry_01-rust-port`
- Generation date: `2026-06-11`

## 1. Feature Specification

### 1.1 Purpose

This module provides hash-table entry management and table introspection behavior for an existing generic hash table abstraction. The covered functionality is limited to:

- validating table structural consistency,
- inspecting bucket-chain distribution,
- looking up stored entries by key/equivalence,
- iterating through stored entries,
- collecting entries into caller-provided storage,
- applying a callback to each stored entry,
- clearing and freeing stored contents and table-owned storage,
- internal entry allocation/freeing,
- internal entry search with optional deletion,
- internal transfer of entries between tables,
- computing a bucket-size choice from a candidate size and tuning data.

The Rust rewrite must preserve the observable behavior of these capabilities as evidenced by the analyzed functions in `gnu/hash.c`.

### 1.2 Functional Scope

The Rust module must implement behavior corresponding to the following source-backed operations:

- table inspection:
  - maximum bucket-chain length,
  - structural validity check;
- entry access:
  - safe bucket selection for a key,
  - entry lookup,
  - first-entry retrieval,
  - next-entry retrieval,
  - bulk entry extraction,
  - callback-based traversal;
- table mutation and lifecycle:
  - clear all entries,
  - free table-managed resources;
- internal support behavior:
  - allocate a hash entry wrapper,
  - free a hash entry wrapper,
  - find an entry within a bucket chain, with optional deletion,
  - transfer entries from one table representation to another,
  - compute a usable bucket size from tuning constraints.

### 1.3 Behavioral Summary

#### Lookup behavior

The module supports searching the table for an entry matching a provided query object. If a matching entry exists, lookup returns the stored entry object; otherwise it reports absence. Lookup behavior depends on the table's configured hashing and comparison rules, not object identity alone.

#### Iteration behavior

The module supports traversing entries in table order derived from bucket layout and per-bucket chains:

- retrieving the first stored entry,
- retrieving the next stored entry after a known current entry,
- collecting entries into a caller buffer up to the provided capacity,
- invoking a caller callback for each stored entry and reporting how many entries were processed.

The Rust rewrite must preserve the module’s ability to iterate across all currently stored entries without requiring the caller to know internal bucket structure.

#### Validation and inspection behavior

The module can:

- report the longest bucket chain currently present in the table,
- determine whether the table satisfies its expected internal consistency conditions.

The Rust rewrite must preserve these observability points so that callers can inspect table state and validate invariants.

#### Clearing and freeing behavior

The module distinguishes between:

- clearing a table’s entries while leaving the table object reusable, and
- freeing the table and its owned storage.

If the table is configured with entry deallocation/destructor behavior, that behavior must be honored during removal of stored entries.

#### Internal movement behavior

The module includes internal support for transferring entries from one table representation to another, used when entries must be reinserted into a different bucket layout. The Rust rewrite must preserve the semantics that successful transfer retains all source entries in the destination representation without loss or duplication.

### 1.4 Out of Scope

The Rust rewrite specification does not require any capability not evidenced by the analyzed module content. In particular, this spec does not introduce:

- new public APIs,
- concurrency guarantees,
- persistence or serialization,
- recovery or journaling,
- foreign-function interfaces,
- performance targets beyond behavioral correctness.

## 2. User Scenarios & Testing

### 2.1 Scenario: Validate a populated table

A caller has a hash table containing multiple entries and needs to confirm that the table remains internally consistent after a series of insertions or internal maintenance operations.

Expected support:

- the caller can request a boolean validity result,
- the validation reflects actual table structure,
- a valid table reports success,
- an invalid structure is detectable.

Suggested tests:

- validate an empty table,
- validate a single-entry table,
- validate a multi-bucket table with collisions,
- validate after clear,
- validate after internal transfer/rebuild.

### 2.2 Scenario: Inspect collision distribution

A caller needs to observe how unevenly entries are distributed by asking for the maximum bucket-chain length.

Expected support:

- the result is zero for an empty table,
- the result equals the largest chain length among all buckets,
- the result updates after entry removal or clear.

Suggested tests:

- empty table returns `0`,
- one entry returns `1`,
- several entries in distinct buckets return `1`,
- several colliding entries return the expected longest chain length.

### 2.3 Scenario: Find an existing entry by query key

A caller supplies a query object equivalent to a stored entry and needs the stored object back.

Expected support:

- lookup returns the stored entry when a match exists,
- lookup reports absence when no match exists,
- lookup does not remove or alter the entry,
- matching uses the table’s configured lookup semantics.

Suggested tests:

- lookup in empty table,
- lookup matching first item in a bucket chain,
- lookup matching later item in a bucket chain,
- lookup for absent key among present collisions.

### 2.4 Scenario: Traverse all entries

A caller needs to enumerate every stored entry without direct access to bucket internals.

Expected support:

- retrieving the first entry from a non-empty table,
- retrieving successive entries until traversal completes,
- each stored entry is reachable exactly once in a stable table state,
- empty table traversal reports no first entry.

Suggested tests:

- `get_first` on empty and non-empty tables,
- repeated `get_next` covers all entries,
- traversal over multiple buckets,
- traversal over collision chains.

### 2.5 Scenario: Copy out entries into caller storage

A caller has a preallocated buffer and wants the module to place stored entry pointers/references into that buffer.

Expected support:

- up to `buffer_size` entries are written,
- the returned count equals the number written,
- no out-of-bounds write occurs,
- when capacity is at least the table size, all entries are returned.

Suggested tests:

- zero-capacity buffer,
- partial-capacity buffer,
- exact-capacity buffer,
- capacity larger than entry count.

### 2.6 Scenario: Apply processing to every entry

A caller wants to run a callback over every stored entry and receive a processed count.

Expected support:

- callback is invoked once per visited entry,
- the returned count matches processed entries,
- empty tables produce zero processed entries.

Suggested tests:

- empty table with callback,
- single-entry table,
- multi-entry table with collisions,
- callback accumulates values and count matches accumulation.

### 2.7 Scenario: Remove all entries but keep the table reusable

A caller wants to clear current contents and then continue using the same table object.

Expected support:

- all stored entries are removed,
- per-entry destruction/free behavior is applied where configured,
- subsequent lookup finds nothing,
- subsequent traversal yields no entries,
- table validation still succeeds for the cleared reusable table.

Suggested tests:

- clear empty table,
- clear populated table,
- clear followed by reuse with newly inserted entries,
- destructor/free callback invocation count equals removed entry count when configured.

### 2.8 Scenario: Release the table and owned resources

A caller is finished with the table and wants all module-owned storage released.

Expected support:

- freeing a populated table releases entry wrappers and table storage,
- configured entry cleanup behavior is honored before final release,
- no remaining entries are observable after free through valid ownership paths.

Suggested tests:

- free empty table,
- free populated table,
- free table after clear,
- resource cleanup callback counts match expectations.

### 2.9 Scenario: Internal rebuild or resize transfers entries

During internal table maintenance, entries are moved from a source table representation to a destination table representation.

Expected support:

- all source entries are transferred on success,
- destination contains the same logical entry set,
- no entry is duplicated or dropped,
- safe transfer mode preserves correct behavior in the presence of the module’s guarded hashing path.

Suggested tests:

- transfer zero entries,
- transfer one entry,
- transfer multiple entries including collisions,
- validate destination after transfer,
- verify lookups succeed for all previously present entries.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Table validity check
The module shall provide a way to determine whether a hash table satisfies the internal consistency conditions expected by this module’s entry and bucket structure.

Traceability: `hash_table_ok` in `gnu/hash.c`.

#### FR-2: Maximum bucket-chain inspection
The module shall provide a way to compute the greatest number of entries linked within any single bucket chain of a table.

Traceability: `hash_get_max_bucket_length` in `gnu/hash.c`.

#### FR-3: Safe bucket selection for lookup support
The module shall support an internal guarded hashing path that maps a query key to a bucket-chain head suitable for subsequent lookup operations.

Traceability: `safe_hasher` in `gnu/hash.c`.

#### FR-4: Entry lookup
The module shall return the stored entry matching a provided query object when such an entry exists, and shall report absence otherwise.

Traceability: `hash_lookup` and `hash_find_entry` in `gnu/hash.c`.

#### FR-5: First-entry retrieval
The module shall return the first available stored entry in table traversal order, or absence if the table has no entries.

Traceability: `hash_get_first` in `gnu/hash.c`.

#### FR-6: Next-entry retrieval
Given an entry currently present in the table, the module shall return the subsequent stored entry in traversal order, or absence when no later entry exists.

Traceability: `hash_get_next` in `gnu/hash.c`.

#### FR-7: Bulk entry extraction
The module shall write stored entries into a caller-provided buffer up to the supplied capacity and shall return the number of entries written.

Traceability: `hash_get_entries` in `gnu/hash.c`.

#### FR-8: Per-entry callback traversal
The module shall invoke a caller-supplied processor for stored entries and shall return the number of entries processed.

Traceability: `hash_do_for_each` in `gnu/hash.c`.

#### FR-9: Bucket-size computation from tuning input
The module shall compute a bucket-size choice from a candidate size and hash tuning parameters.

Traceability: `compute_bucket_size` in `gnu/hash.c`.

#### FR-10: Table clearing
The module shall remove all stored entries from a table while leaving the table representation reusable.

Traceability: `hash_clear` in `gnu/hash.c`.

#### FR-11: Table freeing
The module shall release a table’s owned storage and entry-management resources.

Traceability: `hash_free` in `gnu/hash.c`.

#### FR-12: Entry-wrapper allocation
The module shall support internal allocation of a hash entry wrapper suitable for storing one logical entry in the table.

Traceability: `allocate_entry` in `gnu/hash.c`.

#### FR-13: Entry-wrapper release
The module shall support internal release of a previously allocated hash entry wrapper.

Traceability: `free_entry` in `gnu/hash.c`.

#### FR-14: Search with optional deletion
The module shall support internal search for a matching entry within a target bucket chain and, when requested, removal of that matching entry from the chain.

Traceability: `hash_find_entry` in `gnu/hash.c`.

#### FR-15: Entry transfer between table representations
The module shall support internal transfer of all entries from a source table representation to a destination table representation and shall report success or failure.

Traceability: `transfer_entries` in `gnu/hash.c`.

### 3.2 Key Entities

#### Hash table
A hash table is the central entity holding:

- bucketized entry chains,
- entry count/state needed for traversal and validation,
- hashing/comparison behavior used for lookup,
- cleanup behavior applied during clear/free,
- tuning or sizing context relevant to bucket computation,
- entry-storage management state.

Traceability: `struct hash_table` in `gnu/hash.c`.

#### Hash entry
A hash entry is the per-stored-item wrapper used to link a logical stored value into a bucket chain and traversal structure.

Traceability: `struct hash_entry` in `gnu/hash.c`.

#### Bucket chain
A bucket chain is the linked sequence of hash entries associated with one bucket index. Lookup, max-chain inspection, validation, deletion, and transfer all operate over bucket chains.

Traceability: `hash_get_max_bucket_length`, `hash_table_ok`, `hash_find_entry`, `transfer_entries` in `gnu/hash.c`.

#### Tuning data
Tuning data provides constraints or preferences used when computing an appropriate bucket size.

Traceability: `compute_bucket_size` parameter `const Hash_tuning *tuning` in `gnu/hash.c`.

#### Entry processor
An entry processor is a caller-supplied function applied to each stored entry during traversal-based processing.

Traceability: `hash_do_for_each` parameter `Hash_processor processor` in `gnu/hash.c`.

#### Entry-storage manager
The module uses table-associated storage management for entry wrapper allocation and release.

Traceability: `allocate_entry`, `free_entry`, `hash_free`, and `struct hash_table` references including storage state in `gnu/hash.c`.

#### Entity relationships

- A hash table owns zero or more bucket chains.
- Each bucket chain contains zero or more hash entries.
- Each hash entry wraps one logical stored entry value.
- Lookup and deletion operate by selecting a bucket chain from the hash table, then scanning hash entries within that chain.
- Clear and free traverse the hash table’s hash entries and apply cleanup/release behavior.
- Transfer reads hash entries from a source hash table and inserts them into a destination hash table.
- Bucket-size computation depends on tuning data associated with sizing decisions for the hash table.

## 4. Success Criteria

### 4.1 Behavioral Correctness

1. For any tested table state, the Rust implementation’s lookup result matches the source module’s presence/absence and matched-entry behavior for the same logical contents.
   - Traceability: `hash_lookup`, `hash_find_entry`.

2. For empty and populated tables, first/next traversal in Rust visits the same logical set of entries exactly once per stable traversal.
   - Traceability: `hash_get_first`, `hash_get_next`.

3. Bulk extraction returns a count not exceeding the caller-provided capacity and returns all entries when capacity is sufficient.
   - Traceability: `hash_get_entries`.

4. Callback traversal returns the number of processed entries and invokes the processor once per visited entry.
   - Traceability: `hash_do_for_each`.

5. Maximum bucket length reported by Rust equals the actual longest bucket chain for the tested table contents.
   - Traceability: `hash_get_max_bucket_length`.

6. Table validation correctly distinguishes structurally acceptable tested states from intentionally corrupted or invalid test constructions where such states are representable in the Rust test harness.
   - Traceability: `hash_table_ok`.

### 4.2 Lifecycle Correctness

7. After clear, lookup finds no prior entries, iteration yields no entries, and the table remains reusable in subsequent tests.
   - Traceability: `hash_clear`, `hash_lookup`, `hash_get_first`.

8. During clear and free, configured entry cleanup behavior is invoked for removed entries in testable configurations.
   - Traceability: `hash_clear`, `hash_free`.

9. Free releases the table from further valid use under ownership rules of the Rust design, with no retained accessible entries through the freed table object.
   - Traceability: `hash_free`.

### 4.3 Internal Support Correctness

10. Internal entry transfer succeeds in tests by preserving the full logical entry set from source to destination without duplication or loss.
    - Traceability: `transfer_entries`.

11. Search-with-delete removes only the matched entry from the targeted bucket chain and leaves all non-matching entries accessible.
    - Traceability: `hash_find_entry`.

12. Bucket-size computation produces results consistent with source-backed tuning-based sizing rules for representative test inputs derived from the C behavior.
    - Traceability: `compute_bucket_size`.

### 4.4 Completion Criterion

The Rust port is complete for this module when all requirements in Section 3 are implemented and automated tests cover the scenarios in Section 2 with passing results on the target branch `007-module_gnu_hash_entry_01-rust-port`.