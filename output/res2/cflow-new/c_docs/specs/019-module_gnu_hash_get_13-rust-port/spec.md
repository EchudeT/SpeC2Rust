# spec.md

## Overview

This specification defines the Rust rewrite requirements for the `module_gnu_hash_get_13` module from `cflow-new`, based on `gnu/hash.c`.

The module scope is limited to read-only accessors over hash table state. It exposes functions that report:

- the total number of buckets in a hash table,
- the number of buckets currently in use,
- the number of entries currently stored.

The Rust version must preserve this observable behavior for the hash table representation used by the rewritten module cluster.

## Feature Specification

### Summary

The module provides query functionality for an existing hash table object. Its responsibility is to return stored table metrics without modifying the table.

### In Scope

The Rust version must implement accessor behavior corresponding to these functions from `gnu/hash.c`:

- `hash_get_n_buckets`
- `hash_get_n_buckets_used`
- `hash_get_n_entries`

These accessors operate on the hash table data structure and return size/count metadata already maintained by that structure.

### Out of Scope

This module specification does not require the Rust rewrite to introduce any behavior not evidenced by the source analysis, including:

- hash insertion, deletion, lookup, or iteration APIs beyond what is needed to support these accessors,
- resizing policy behavior as a feature of this module,
- thread-safety guarantees,
- serialization or persistence,
- foreign-function interfaces.

## User Scenarios & Testing

### Scenario 1: Retrieve total bucket capacity

A caller with access to a hash table needs to know how many buckets the table currently has allocated.

**Expected behavior:**
- The module accepts a reference to an existing hash table.
- It returns the table’s total bucket count as a non-negative size value.
- The query does not change the table state.

**Testing approach:**
- Construct or obtain a table with a known bucket count.
- Call the bucket-count accessor.
- Verify the returned value matches the table’s stored total bucket count.
- Verify repeated calls return the same result when the table is unchanged.

### Scenario 2: Retrieve number of used buckets

A caller needs to know how many buckets currently contain one or more entries.

**Expected behavior:**
- The module returns the count of buckets marked as used in the current table state.
- The query does not alter entry placement, counts, or any other table state.

**Testing approach:**
- Obtain a table state with a known number of occupied buckets.
- Call the used-bucket accessor.
- Verify the returned value matches the stored used-bucket count.
- Verify the result remains unchanged across repeated read-only calls.

### Scenario 3: Retrieve total entry count

A caller needs the total number of stored entries for reporting or decision-making.

**Expected behavior:**
- The module returns the current entry count from the table.
- The query is read-only.

**Testing approach:**
- Obtain a table with a known number of entries.
- Call the entry-count accessor.
- Verify the returned value matches the stored entry count.
- Verify repeated queries do not mutate the table.

### Scenario 4: Distinguish table metrics correctly

A caller needs all three metrics and relies on them being independently reported rather than conflated.

**Expected behavior:**
- Total bucket count, used bucket count, and total entry count are returned from their respective accessors.
- Different metrics may have different values and must not be substituted for one another.

**Testing approach:**
- Use a table where:
  - total buckets > used buckets,
  - entry count may be equal to or greater than used buckets depending on collisions.
- Query all three accessors.
- Verify each accessor returns its corresponding metric.

## Requirements

### Functional Requirements

#### FR-1: Total bucket count query

The Rust module shall provide a function equivalent in behavior to `hash_get_n_buckets` that returns the total number of buckets recorded in a hash table.

**Traceability:** `gnu/hash.c`, `hash_get_n_buckets`, `struct hash_table`

#### FR-2: Used bucket count query

The Rust module shall provide a function equivalent in behavior to `hash_get_n_buckets_used` that returns the number of buckets currently recorded as used in a hash table.

**Traceability:** `gnu/hash.c`, `hash_get_n_buckets_used`, `struct hash_table`

#### FR-3: Entry count query

The Rust module shall provide a function equivalent in behavior to `hash_get_n_entries` that returns the number of entries currently recorded in a hash table.

**Traceability:** `gnu/hash.c`, `hash_get_n_entries`, `struct hash_table`

#### FR-4: Read-only behavior

Each query function shall observe hash table state without modifying the hash table’s logical contents or its stored count fields.

**Traceability:** `gnu/hash.c`, `hash_get_n_buckets`, `hash_get_n_buckets_used`, `hash_get_n_entries`, `struct hash_table`

#### FR-5: Size-valued results

Each query function shall return its metric as a size/count value corresponding to C `size_t` semantics in the Rust rewrite.

**Traceability:** `gnu/hash.c`, function signatures of `hash_get_n_buckets`, `hash_get_n_buckets_used`, `hash_get_n_entries`

### Key Entities

#### Hash table

The central entity is the hash table structure (`struct hash_table`). For this module, the relevant aspect of that structure is that it stores count metadata for:

- total buckets,
- used buckets,
- total entries.

The accessor functions read these metrics from the hash table.

**Traceability:** `gnu/hash.c:55-87`, `struct hash_table`

#### Hash entry

The hash table is associated with hash entry nodes (`struct hash_entry`), which represent stored entries in the broader hash implementation. For this module, entry structures matter only indirectly because the table exposes an aggregate entry count and bucket usage count derived from table state.

**Traceability:** `gnu/hash.c:49-53`, repeated `struct hash_entry` references in `gnu/hash.c`

## Success Criteria

### SC-1: Correct total bucket reporting

For a table with a known stored bucket count, the Rust accessor corresponding to `hash_get_n_buckets` returns exactly that count.

### SC-2: Correct used bucket reporting

For a table with a known stored used-bucket count, the Rust accessor corresponding to `hash_get_n_buckets_used` returns exactly that count.

### SC-3: Correct entry count reporting

For a table with a known stored entry count, the Rust accessor corresponding to `hash_get_n_entries` returns exactly that count.

### SC-4: No state mutation from queries

After any sequence of calls to the three accessors on an unchanged table, the table’s observable counts remain unchanged.

### SC-5: Metric separation preserved

In tests where total bucket count, used bucket count, and entry count differ, each Rust accessor returns the correct distinct metric rather than another count.