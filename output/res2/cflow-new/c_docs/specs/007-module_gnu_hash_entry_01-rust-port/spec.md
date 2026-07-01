# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_gnu_hash_entry_01`
- **Category**: `module_cluster`
- **Source scope**: `gnu/hash.c`
- **Rust branch target**: `007-module_gnu_hash_entry_01-rust-port`
- **Generation date**: `2026-06-17`

This module provides the entry-level operational behavior of a generic hash table: lookup, traversal, entry collection, validation, clearing, freeing, internal entry allocation, bucket sizing support, and entry transfer between tables. The Rust rewrite must preserve the observable behavior of these operations as evidenced by the analyzed functions and data structures in `gnu/hash.c`.

The module scope evidenced here is limited to:
- inspecting table state and consistency,
- locating entries by key or stored value,
- iterating over stored entries,
- collecting or processing stored entries,
- clearing or destroying table contents,
- internal entry management for insertion/deletion support,
- internal migration of entries between table instances.

No broader capabilities are specified beyond this evidenced behavior.

---

## Feature Specification

### Summary

The Rust version must implement the functional behavior of a generic chained-bucket hash table module whose entries store user-provided element pointers/values and are organized through linked `hash_entry` nodes under a `hash_table`.

The module must support:

- determining the maximum chain length across buckets,
- validating basic table structural consistency,
- safe bucket selection for lookup,
- looking up an entry by key,
- traversing entries in table order,
- copying visible entries into a caller-supplied buffer,
- applying a callback to each stored entry,
- clearing all stored entries while retaining a usable table object,
- freeing all storage owned by the table,
- internally allocating and releasing entry nodes,
- internally finding and optionally deleting a matching entry,
- internally transferring entries from one table to another,
- computing a bucket count candidate from tuning inputs.

### Functional Boundaries

The Rust rewrite must preserve the following observable boundaries:

1. **Hash table inspection**
   - Provide a way to inspect bucket occupancy characteristics through the maximum bucket length.
   - Provide a validity check that reports whether a table's internal structure is acceptable according to the original module's consistency rules.

2. **Entry retrieval**
   - Support lookup of a stored entry matching a provided key/value probe.
   - Return the stored entry payload when found and indicate absence when not found.

3. **Sequential traversal**
   - Support retrieving the first stored entry in the table.
   - Support retrieving the next stored entry following a known entry.
   - Traversal behavior must cover all present entries without inventing entries.

4. **Bulk enumeration**
   - Support copying stored entries into a caller-provided buffer up to a caller-provided capacity.
   - Support invoking a callback once for each stored entry and returning the number of processed entries.

5. **Lifecycle operations**
   - Support clearing all entries from an existing table.
   - Support freeing all table-associated storage handled by this module.

6. **Internal entry-node management**
   - Support internal allocation of entry nodes for table operations.
   - Support internal release of entry nodes when no longer needed.

7. **Internal search/delete support**
   - Support internal matching of entries within a bucket chain.
   - Support optional deletion of a found entry during internal search.

8. **Internal reorganization support**
   - Support internal transfer of entries between two table instances.
   - Preserve represented entries during successful transfer.

9. **Bucket sizing support**
   - Support computation of a bucket-size choice from a candidate size and tuning parameters, consistent with the original module’s role in table sizing decisions.

---

## User Scenarios & Testing

### Scenario 1: Look up an entry in a populated table

A caller has a populated hash table and needs to retrieve the stored element matching a probe value.

**Expected behavior**
- Lookup returns the stored element when a match exists.
- Lookup returns no element when no match exists.
- Lookup does not alter table contents.

**Relevant evidence**
- `hash_lookup`
- `safe_hasher`
- `hash_find_entry`

**Testing focus**
- Existing key returns the exact stored payload.
- Missing key returns absence.
- Repeated lookups produce consistent results.

---

### Scenario 2: Iterate through all entries

A caller needs to enumerate all entries currently present in the table using first/next traversal.

**Expected behavior**
- The first call returns the first available stored entry, or absence if the table is empty.
- Successive next calls walk remaining stored entries.
- Traversal reaches each stored entry once in table traversal order as defined by the underlying bucket/chain layout.

**Relevant evidence**
- `hash_get_first`
- `hash_get_next`

**Testing focus**
- Empty table returns no first entry.
- Single-entry table returns that entry then ends.
- Multi-entry table traversal reaches all stored entries with no duplicates or omissions.

---

### Scenario 3: Extract entries into a caller buffer

A caller wants a snapshot of currently stored entry payloads in a supplied array.

**Expected behavior**
- The module writes up to `buffer_size` entries into the buffer.
- The return value is the number of entries copied.
- If the table contains fewer entries than capacity, only present entries are copied.
- If the table contains more entries than capacity, copying stops at capacity.

**Relevant evidence**
- `hash_get_entries`

**Testing focus**
- Zero-capacity buffer yields zero copied entries.
- Exact-capacity buffer receives all entries.
- Smaller buffer receives a prefix/subset bounded by capacity.
- Returned count matches copied elements.

---

### Scenario 4: Apply an operation to every entry

A caller needs to process each stored entry through a callback.

**Expected behavior**
- The callback is invoked once per stored entry encountered.
- The module returns the number of processed entries.

**Relevant evidence**
- `hash_do_for_each`

**Testing focus**
- Empty table yields zero callback invocations.
- Non-empty table invokes callback for each stored entry.
- Returned count equals callback invocation count.

---

### Scenario 5: Validate table integrity during diagnostics

A caller or internal diagnostic path checks whether a table appears structurally valid.

**Expected behavior**
- Valid tables are reported as valid.
- Structurally inconsistent tables are reported as invalid according to module rules.

**Relevant evidence**
- `hash_table_ok`
- `hash_get_max_bucket_length`

**Testing focus**
- Known-good constructed tables validate successfully.
- Deliberately inconsistent internal states are rejected in unit tests for the Rust port’s internal representation.

---

### Scenario 6: Clear a table but keep using it

A caller needs to remove all stored entries while keeping the table object available for later reuse.

**Expected behavior**
- After clear, the table contains no stored entries.
- Post-clear lookup finds nothing.
- Post-clear traversal yields no entries.
- Table object remains usable for subsequent operations supported by the broader hash-table module.

**Relevant evidence**
- `hash_clear`

**Testing focus**
- Clearing an empty table is harmless.
- Clearing a populated table removes all entries.
- Repeated clear calls remain safe and leave the table empty.

---

### Scenario 7: Destroy a table and release owned storage

A caller is finished with the table and needs all table-associated resources released.

**Expected behavior**
- All storage managed by the module for the table and its entry structures is released.
- No remaining entries are accessible through the destroyed table object.

**Relevant evidence**
- `hash_free`
- `free_entry`

**Testing focus**
- Freeing a table that has been populated releases its internal resources.
- Freeing after clear behaves correctly.
- Internal resource accounting tests show no leaks in the Rust implementation.

---

### Scenario 8: Internal entry deletion during match search

Internal table logic needs to find a matching entry and optionally remove it from the bucket chain.

**Expected behavior**
- Match search can return the found stored payload.
- When deletion mode is enabled, the found entry is removed from the table representation.
- When deletion mode is disabled, the table remains unchanged.

**Relevant evidence**
- `hash_find_entry`
- `free_entry`

**Testing focus**
- Search-only mode leaves membership unchanged.
- Delete mode removes exactly the matching entry.
- Neighboring chain links remain traversable after deletion.

---

### Scenario 9: Internal transfer during resize or reorganization

Internal table logic needs to move represented entries from one table instance to another.

**Expected behavior**
- Successful transfer preserves the source entries in the destination representation.
- Transfer result reports success or failure.
- Transfer respects the safe/unsafe transfer mode distinction evidenced by the interface.

**Relevant evidence**
- `transfer_entries`
- `safe_hasher`
- `allocate_entry`

**Testing focus**
- All source entries appear in destination after successful transfer.
- No entries are invented or lost on success.
- Failure-path tests preserve the contract implemented by the Rust port for partial transfer handling, matching original observable outcomes where evidenced.

---

## Requirements

### Functional Requirements

#### FR-1: Table occupancy inspection
The module shall provide maximum bucket-chain length inspection for a hash table and return a `size_t`-equivalent count representing the largest number of linked entries present in any bucket.

**Traceability**
- `hash_get_max_bucket_length`

---

#### FR-2: Table consistency validation
The module shall provide a boolean table validation operation that checks whether the table’s internal structure satisfies the consistency conditions enforced by the original module.

**Traceability**
- `hash_table_ok`

---

#### FR-3: Safe hash-to-bucket resolution
The module shall support internal resolution from a key/probe to a bucket entry position in a way used by lookup and transfer-related operations.

**Traceability**
- `safe_hasher`

---

#### FR-4: Entry lookup
The module shall provide lookup of a stored entry by probe value and return the stored payload when a matching entry exists, otherwise return absence.

**Traceability**
- `hash_lookup`
- `hash_find_entry`

---

#### FR-5: First-entry retrieval
The module shall provide retrieval of the first stored entry currently present in the table, or absence when the table is empty.

**Traceability**
- `hash_get_first`

---

#### FR-6: Successor-entry retrieval
The module shall provide retrieval of the next stored entry following a provided current entry within the table traversal order, or absence when there is no successor.

**Traceability**
- `hash_get_next`

---

#### FR-7: Bulk entry extraction
The module shall provide copying of stored entry payloads into a caller-supplied buffer, limited by caller-supplied capacity, and shall return the number of copied entries.

**Traceability**
- `hash_get_entries`

---

#### FR-8: Per-entry callback processing
The module shall provide iteration over stored entries using a caller-supplied processing callback and shall return the number of entries processed.

**Traceability**
- `hash_do_for_each`

---

#### FR-9: Bucket sizing computation
The module shall support internal computation of a bucket-size decision from a candidate size and tuning input, producing a `size_t`-equivalent result used by hash-table sizing logic.

**Traceability**
- `compute_bucket_size`

---

#### FR-10: Table clearing
The module shall remove all stored entries from a table while preserving the table object for continued validity as an empty table.

**Traceability**
- `hash_clear`

---

#### FR-11: Table destruction
The module shall release all table-associated storage managed by this module, including entry-node storage owned through the table’s internal structures.

**Traceability**
- `hash_free`
- `free_entry`

---

#### FR-12: Internal entry-node allocation
The module shall support internal allocation of a hash entry node for use in table operations.

**Traceability**
- `allocate_entry`

---

#### FR-13: Internal entry-node release
The module shall support internal release of a previously allocated hash entry node.

**Traceability**
- `free_entry`

---

#### FR-14: Internal match search with optional deletion
The module shall support internal search for a matching entry within a bucket chain and, when requested, remove that matching entry from the table structure.

**Traceability**
- `hash_find_entry`

---

#### FR-15: Internal transfer between tables
The module shall support internal transfer of entries from a source table to a destination table and report success or failure.

**Traceability**
- `transfer_entries`

---

### Key Entities

#### `hash_table`
The central table entity that owns or references:
- bucket-organized entry storage,
- linked `hash_entry` chains,
- sizing/tuning-related state,
- allocation-related state used by clear/free/allocation paths.

It is the required input for all table inspection, traversal, enumeration, clearing, freeing, and transfer behaviors evidenced in this module.

**Traceability**
- `struct hash_table`
- `hash_get_max_bucket_length`
- `hash_table_ok`
- `hash_lookup`
- `hash_get_first`
- `hash_get_next`
- `hash_get_entries`
- `hash_do_for_each`
- `hash_clear`
- `hash_free`
- `allocate_entry`
- `free_entry`
- `hash_find_entry`
- `transfer_entries`

---

#### `hash_entry`
The linked node entity representing one stored table member within a bucket chain. It carries the stored payload and chain linkage used by lookup, traversal, deletion, clearing, freeing, and transfer.

**Traceability**
- `struct hash_entry`
- `safe_hasher`
- `hash_get_max_bucket_length`
- `hash_table_ok`
- `hash_lookup`
- `hash_get_first`
- `hash_get_next`
- `hash_get_entries`
- `hash_do_for_each`
- `hash_clear`
- `hash_free`
- `allocate_entry`
- `free_entry`
- `hash_find_entry`
- `transfer_entries`

---

#### `Hash_tuning`
A tuning/configuration entity used as input to bucket-size computation and therefore part of the module’s sizing behavior boundary.

**Traceability**
- `compute_bucket_size`

---

#### `Hash_processor`
A callback entity supplied by a caller for per-entry processing during iteration across the table.

**Traceability**
- `hash_do_for_each`

---

## Success Criteria

### SC-1: Correct lookup behavior
For test tables with known contents, lookup returns the matching stored payload for present probes and absence for missing probes.

**Traceability**
- `hash_lookup`
- `hash_find_entry`

---

### SC-2: Complete traversal behavior
For empty, single-entry, and multi-entry tables, first/next traversal visits exactly the entries currently stored in the table and terminates with absence.

**Traceability**
- `hash_get_first`
- `hash_get_next`

---

### SC-3: Correct bounded extraction
For a caller buffer of capacity `N`, bulk extraction copies no more than `N` entries and the returned count equals the number of copied payloads.

**Traceability**
- `hash_get_entries`

---

### SC-4: Correct per-entry processing count
For tables with known entry counts, callback iteration invokes the callback once per visited stored entry and returns the same count.

**Traceability**
- `hash_do_for_each`

---

### SC-5: Clear produces an empty reusable table
After clearing a populated table, lookup returns absence for formerly present entries, traversal yields no entries, and table validation remains successful for the empty state.

**Traceability**
- `hash_clear`
- `hash_lookup`
- `hash_get_first`
- `hash_table_ok`

---

### SC-6: Free releases module-managed resources
In Rust-port resource accounting or leak-check tests, destroying tables after representative usage leaves no unreleased module-managed entry or table storage.

**Traceability**
- `hash_free`
- `free_entry`
- `allocate_entry`

---

### SC-7: Validation distinguishes valid and invalid states
Validation returns true for correctly constructed table states used by normal operations and false for intentionally malformed internal test states that violate module consistency rules.

**Traceability**
- `hash_table_ok`

---

### SC-8: Internal delete removes exactly one matching entry
In internal tests of search/delete behavior, deletion removes the matching entry, preserves remaining entries, and updates subsequent lookup/traversal results accordingly.

**Traceability**
- `hash_find_entry`

---

### SC-9: Successful transfer preserves represented entries
When entries are transferred from a source table to a destination table in internal tests, a successful transfer results in destination visibility of all transferred entries with no invented entries.

**Traceability**
- `transfer_entries`

---

### SC-10: Bucket occupancy metric is accurate
For constructed tables with known chain distributions, the reported maximum bucket length equals the longest actual bucket chain length.

**Traceability**
- `hash_get_max_bucket_length`

---

## Notes

- This specification covers only behavior evidenced by the analyzed portion of `gnu/hash.c`.
- Internal functions listed above need not become public Rust APIs unless required by the surrounding port architecture; however, their evidenced behavior must be preserved within the Rust rewrite where they participate in module functionality.
- No additional guarantees are specified beyond the analyzed module behavior.