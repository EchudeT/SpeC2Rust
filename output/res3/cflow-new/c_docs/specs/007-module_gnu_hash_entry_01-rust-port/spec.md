# spec.md

## Title

Rust Functional Specification for `module_gnu_hash_entry_01`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_hash_entry_01`
- Category: `module_cluster`
- Source basis: `gnu/hash.c`
- Rust branch: `007-module_gnu_hash_entry_01-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides core hash-table entry management and traversal behavior for the project’s GNU-style hash table implementation. The analyzed portion of the C module covers:

- table integrity checking,
- lookup by key or entry identity rules defined by the table,
- iteration across stored entries,
- extraction and callback-based processing of entries,
- bucket-size evaluation support,
- clearing and freeing table contents,
- internal entry allocation and release,
- entry search with optional deletion,
- transfer of entries between tables.

The Rust rewrite must preserve the observable behavior of these operations for the module boundary evidenced in `gnu/hash.c`. The specification is limited to behavior directly supported by the analyzed functions and referenced data structures.

## Feature Specification

### 1. Hash table query and inspection

The module must support read-only inspection of a hash table’s state through:

- maximum bucket-chain length computation,
- consistency validation of table structure,
- key/entry lookup,
- retrieval of the first stored entry,
- retrieval of the next stored entry after a known entry,
- extraction of stored entries into a caller-provided buffer,
- application of a caller-provided processor across entries.

These features are evidenced by:
`hash_get_max_bucket_length`, `hash_table_ok`, `hash_lookup`, `hash_get_first`, `hash_get_next`, `hash_get_entries`, and `hash_do_for_each`.

### 2. Safe bucket selection for lookup-related operations

The module includes logic that derives the bucket head for a given key in a way suitable for use by lookup operations. The Rust version must preserve the behavioral role of this helper within table lookup and search workflows, including use against the table’s configured hashing behavior.

This feature is evidenced by:
`safe_hasher`.

### 3. Bucket sizing support

The module computes a usable bucket count from a candidate size and tuning parameters. The Rust version must preserve this decision behavior as part of the module’s hash-table capacity logic where this module is responsible for it.

This feature is evidenced by:
`compute_bucket_size` and the referenced `Hash_tuning` relationship.

### 4. Entry lifecycle management within a table

The module manages entry records used to store table members. The Rust version must preserve behavior for:

- allocating an internal entry record,
- releasing an internal entry record,
- clearing all entries from a table while leaving the table object reusable,
- freeing all table-owned storage associated with the table.

This feature is evidenced by:
`allocate_entry`, `free_entry`, `hash_clear`, and `hash_free`.

### 5. Search and conditional deletion

The module must support internal search for an entry within a bucket chain, including an option to delete the found entry as part of the search operation. The Rust version must preserve the distinction between search-only and search-plus-delete behaviors.

This feature is evidenced by:
`hash_find_entry`.

### 6. Entry transfer between tables

The module must support transferring entries from one table to another, including behavior that distinguishes a normal transfer mode from a safe mode. The Rust version must preserve successful transfer semantics and failure signaling.

This feature is evidenced by:
`transfer_entries`.

## User Scenarios & Testing

### Scenario 1: Validate a table before or after use

A caller has a hash table and wants to verify that its internal bucket/entry organization is sound.

Expected support:
- Integrity validation returns a boolean result.
- Validation does not require modifying the table.
- A valid table remains usable for lookup and traversal afterward.

Traceability:
`hash_table_ok`.

Suggested tests:
- Validation succeeds for a populated, well-formed table fixture.
- Validation succeeds for an empty table fixture.
- Validation detects an intentionally malformed fixture if such fixtures are representable in Rust test scope.

### Scenario 2: Look up an entry by key

A caller needs to find the stored element corresponding to a given key or representative entry.

Expected support:
- Lookup returns the stored entry when present.
- Lookup returns no result when absent.
- Lookup behavior follows the table’s configured hashing/equality rules.

Traceability:
`safe_hasher`, `hash_lookup`, `hash_find_entry`.

Suggested tests:
- Present key returns matching stored item.
- Missing key returns no result.
- Colliding keys in the same bucket still return the correct stored item.

### Scenario 3: Iterate through all entries

A caller wants to enumerate every stored entry without copying the whole table.

Expected support:
- Retrieval of the first entry in iteration order.
- Retrieval of the next entry after a previously returned entry.
- Iteration eventually reaches the end.
- Empty tables produce no first entry.

Traceability:
`hash_get_first`, `hash_get_next`.

Suggested tests:
- Empty table returns no first entry.
- Single-entry table returns one entry then end.
- Multi-bucket and same-bucket chain layouts can be fully traversed.
- Repeated `get_next` calls enumerate each stored entry exactly once for a stable table.

### Scenario 4: Copy available entries into caller storage

A caller provides a buffer and wants the module to place stored entry references into that buffer.

Expected support:
- Up to the buffer capacity is written.
- The function reports how many entries were placed in the buffer.
- Behavior is correct for zero-sized buffers, partially sized buffers, and buffers large enough for all entries.

Traceability:
`hash_get_entries`.

Suggested tests:
- Zero-capacity buffer yields zero copied entries.
- Undersized buffer fills only available slots and reports that number.
- Sufficiently large buffer receives all stored entries.

### Scenario 5: Apply a callback to each stored entry

A caller wants to process all entries using a supplied function and context pointer/state.

Expected support:
- The processor is invoked for entries in table traversal.
- The function returns the count of processed entries.
- The caller-provided context is passed through to each invocation.

Traceability:
`hash_do_for_each`.

Suggested tests:
- Callback count matches return value.
- Callback sees every entry in a populated table.
- Empty table causes zero callback invocations and returns zero.

### Scenario 6: Clear a table for reuse

A caller wants to remove all entries but continue using the same table object afterward.

Expected support:
- All stored entries are removed.
- Subsequent lookup finds nothing from the previous contents.
- Post-clear iteration reports an empty table.
- The table remains valid for future use.

Traceability:
`hash_clear`, `hash_table_ok`.

Suggested tests:
- After clear, entry count by traversal is zero.
- After clear, lookup of previously present entries fails.
- After clear, validation still succeeds.

### Scenario 7: Free a table and its owned entry storage

A caller is done with a table and wants to release its owned resources.

Expected support:
- Entry storage owned by the table is released.
- Any internal auxiliary storage managed by this module is released.
- This is terminal lifecycle behavior for the table object under this module’s contract.

Traceability:
`hash_free`, `free_entry`, referenced `struct obstack`.

Suggested tests:
- Resource-release tests confirm all entries are dropped/freed in Rust ownership terms.
- Freeing a populated table releases the same logical contents as clearing followed by final release.

### Scenario 8: Remove a specific entry during search

Internal module logic needs to search for a matching entry and optionally remove it from the bucket chain.

Expected support:
- Search-only mode leaves the table unchanged.
- Delete mode removes the matching entry if found.
- If no match is found, table contents remain unchanged.
- Deletion updates subsequent lookup and traversal results.

Traceability:
`hash_find_entry`.

Suggested tests:
- Search-only returns match without changing traversal set.
- Delete mode removes an existing entry.
- Delete mode on a missing entry reports absence and leaves contents intact.

### Scenario 9: Rebuild or move entries into another table

Internal module logic needs to transfer entries from one table to another, such as during reorganization.

Expected support:
- Transfer succeeds when destination can accept all entries.
- Failure is reported when transfer cannot be completed.
- Safe/non-safe mode distinctions are preserved.
- Successful transfer preserves discoverability of transferred entries in the destination.

Traceability:
`transfer_entries`, `compute_bucket_size`.

Suggested tests:
- Successful transfer yields all source entries reachable in destination.
- Failed transfer reports failure without silent loss of accounted entries.
- Safe-mode and non-safe-mode paths both execute according to their distinct semantics.

## Requirements

### Functional Requirements

#### FR-1: Table integrity validation
The module shall provide a way to evaluate whether a hash table is internally consistent and return a boolean result.

Traceability:
`hash_table_ok`

#### FR-2: Maximum bucket-chain inspection
The module shall provide a way to compute the maximum number of linked entries present in any bucket of a table.

Traceability:
`hash_get_max_bucket_length`

#### FR-3: Lookup of stored entries
The module shall provide lookup that returns the stored entry corresponding to a provided key or representative entry, or no result when none matches.

Traceability:
`safe_hasher`, `hash_lookup`, `hash_find_entry`

#### FR-4: Ordered table traversal by successor operation
The module shall provide traversal support that can:
- return the first stored entry in the table, and
- return the next stored entry following a previously returned entry,
until no further entry exists.

Traceability:
`hash_get_first`, `hash_get_next`

#### FR-5: Bulk extraction into caller buffer
The module shall copy available stored entries into a caller-provided buffer up to the provided capacity and return the number copied.

Traceability:
`hash_get_entries`

#### FR-6: Callback-based processing
The module shall invoke a caller-provided processor for stored entries in the table and return the number of processed entries.

Traceability:
`hash_do_for_each`

#### FR-7: Bucket-size computation from tuning input
The module shall compute a usable bucket size from a candidate size and tuning parameters for hash-table capacity decisions governed by this module.

Traceability:
`compute_bucket_size`

#### FR-8: Entry record allocation
The module shall allocate an internal entry record suitable for linking into a hash table managed by this module and signal failure if allocation cannot be completed.

Traceability:
`allocate_entry`

#### FR-9: Entry record release
The module shall release an internal entry record previously managed by the table.

Traceability:
`free_entry`

#### FR-10: Table clearing
The module shall remove all entries from a table and reset the table to an empty, reusable state under the module’s ownership model.

Traceability:
`hash_clear`

#### FR-11: Table final release
The module shall release table-owned storage associated with entries and internal structures when the table is no longer needed.

Traceability:
`hash_free`

#### FR-12: Search with optional deletion
The module shall support internal search for a matching entry within a bucket chain and, when requested, remove the matched entry from the table as part of that operation.

Traceability:
`hash_find_entry`

#### FR-13: Entry transfer between tables
The module shall support transferring entries from one hash table to another and report success or failure.

Traceability:
`transfer_entries`

### Key Entities

#### Hash table
A hash table is the main container entity managed by this module. It owns or references:

- a bucket array,
- linked entry chains within buckets,
- table-specific hashing and comparison behavior used by lookup/search logic,
- table lifecycle state used by clearing and freeing operations,
- auxiliary allocation state referenced by the source structure, including obstack-related storage.

Traceability:
`struct hash_table` references in `gnu/hash.c`, and use across all listed functions.

#### Hash entry
A hash entry is the internal record linked into a bucket chain and used to carry a stored element through lookup, traversal, deletion, and transfer operations.

Traceability:
`struct hash_entry` references across lookup, traversal, lifecycle, and transfer functions.

#### Hash tuning
Hash tuning is configuration input used when computing a usable bucket size from a candidate size.

Traceability:
`compute_bucket_size`, referenced `Hash_tuning`

#### Processor callback
A processor callback is caller-supplied behavior applied to each entry during callback-based traversal, together with caller-provided processor data.

Traceability:
`hash_do_for_each`, referenced `Hash_processor`

#### Source and destination table relationship
For transfer behavior, one table acts as the source of existing entries and another as the destination that receives them. The module is responsible for the success/failure outcome of this movement.

Traceability:
`transfer_entries`

## Success Criteria

### SC-1: Validation correctness
For representative empty and populated valid tables used in tests, table validation returns `true`. For any deliberately malformed testable state representable in the Rust test harness, validation returns `false`.

Traceability:
`hash_table_ok`

### SC-2: Lookup correctness
For test tables with present, absent, and colliding keys:
- present keys resolve to the correct stored entry,
- absent keys produce no result,
- collisions do not cause incorrect matches.

Traceability:
`safe_hasher`, `hash_lookup`, `hash_find_entry`

### SC-3: Traversal completeness
For stable tables under test, repeated use of first/next traversal enumerates every stored entry exactly once and ends with no further result.

Traceability:
`hash_get_first`, `hash_get_next`

### SC-4: Buffer extraction correctness
Given buffers of size `0`, smaller than entry count, and at least equal to entry count, the extraction operation returns exactly the number of entries written and never reports a count greater than buffer capacity.

Traceability:
`hash_get_entries`

### SC-5: Callback processing correctness
For a populated table, callback-based processing invokes the processor once per traversed entry and returns that same invocation count.

Traceability:
`hash_do_for_each`

### SC-6: Clear behavior
After clearing a populated table:
- lookup of formerly present entries fails,
- traversal yields no entries,
- integrity validation still succeeds.

Traceability:
`hash_clear`, `hash_lookup`, `hash_get_first`, `hash_table_ok`

### SC-7: Final release behavior
Rust ownership/resource tests show that freeing a populated table releases all module-owned entry storage and internal owned state without leaving logically reachable contents behind.

Traceability:
`hash_free`, `free_entry`

### SC-8: Search/delete behavior
Internal search tests demonstrate:
- search-only mode preserves table contents,
- delete mode removes exactly the matched entry when present,
- missing-entry deletion leaves contents unchanged.

Traceability:
`hash_find_entry`

### SC-9: Transfer behavior
Transfer tests demonstrate that on success, all transferred entries are reachable from the destination table, and on failure, the operation reports failure explicitly rather than appearing successful.

Traceability:
`transfer_entries`

### SC-10: Bucket-size computation behavior
For controlled tuning inputs and candidate sizes, bucket-size computation produces deterministic usable results consistent with the module’s source behavior.

Traceability:
`compute_bucket_size`