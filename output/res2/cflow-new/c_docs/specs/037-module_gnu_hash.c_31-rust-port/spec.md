# spec.md

## Title

Rust Functional Specification for `module_gnu_hash.c_31`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_hash.c_31`
- Category: `module_cluster`
- Source file: `gnu/hash.c`
- Target branch: `037-module_gnu_hash.c_31-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides a configurable hash table for storing opaque entry pointers, with support for caller-supplied hashing, equality comparison, optional entry cleanup, table tuning validation/reset, insertion, deletion, rehashing, and statistics reporting.

The Rust rewrite must preserve the module’s observable behavior as a generic hash-table facility driven by user-provided callbacks or equivalent Rust-side strategy objects/functions. The rewritten module must support initialization with explicit tuning, fallback/default tuning reset behavior, raw byte hashing/comparison helpers used when no custom behavior is supplied, dynamic resizing through rehash, insertion of entries, deletion of matching entries, and reporting of table statistics to an output stream.

## Feature Specification

### In-scope functionality

The Rust version must implement the following module behavior evidenced by `gnu/hash.c`:

- Create and initialize a hash table with:
  - a requested candidate capacity,
  - optional tuning parameters,
  - a hash function,
  - a comparator function,
  - an optional entry cleanup function.
- Validate or normalize tuning before table use.
- Provide a reset operation that restores tuning values to the module’s default configuration.
- Support a default raw-data hash helper over a byte sequence.
- Support a default raw-data comparator helper for entry equality.
- Insert an entry into the table and return a pointer/reference outcome consistent with the source behavior of `hash_insert`.
- Delete an entry matching a lookup key and return the removed stored entry if found.
- Rehash an existing table to a new candidate capacity while preserving stored entries.
- Produce human-readable hash-table statistics to a caller-supplied output stream.

### Out-of-scope functionality

The Rust version must not claim or introduce capabilities not evidenced by this module analysis, including:

- new public APIs beyond the behavior necessary to cover the source module,
- thread-safety guarantees,
- persistence or serialization,
- transactional behavior,
- recovery/journaling,
- interoperability layers not present in the source evidence.

## User Scenarios & Testing

### Scenario 1: Create a table with explicit behavior

A caller needs a hash table for entries identified by custom key semantics. The caller initializes the table with custom hash and comparison functions and an optional cleanup function. The table becomes ready for insertion and deletion operations.

**Test expectations**
- Initialization succeeds when required behavior providers are present and tuning is valid or normalized.
- The created table uses the supplied equality and hash behavior for later operations.

### Scenario 2: Restore default tuning before initialization

A caller has a tuning structure that may contain modified values and wants to restore module defaults before creating a table.

**Test expectations**
- Calling the tuning reset operation produces the module’s default tuning state.
- A table initialized with reset tuning is accepted by initialization and subsequent operations.

### Scenario 3: Insert entries and preserve stored object identity

A caller inserts entries into the table, where the stored object is an opaque pointer-like value. The table must accept entries and make them discoverable for later deletion by matching key semantics.

**Test expectations**
- Insertion reports success according to the original `hash_insert` contract.
- A later delete using an equivalent key removes and returns the stored entry.
- The returned deleted entry is the stored entry object, not merely the lookup key.

### Scenario 4: Delete a missing entry

A caller attempts to remove an entry that is not present.

**Test expectations**
- Deletion reports absence without corrupting the table state.
- Existing entries remain retrievable/removable afterward.

### Scenario 5: Rehash to a different candidate capacity

A caller requests table resizing after entries have already been inserted.

**Test expectations**
- Rehash succeeds for valid resize requests supported by the module.
- Entries present before rehash remain present after rehash.
- Equality behavior and deletion results remain unchanged after rehash.

### Scenario 6: Print table statistics

A caller wants diagnostic information about the table’s current distribution and occupancy.

**Test expectations**
- Statistics output is written to the provided stream/output target.
- The operation is non-destructive: it does not remove or alter entries.

### Scenario 7: Use default raw byte semantics

A caller relies on the module’s raw hash/comparison behavior for data treated as raw bytes.

**Test expectations**
- The raw hashing helper deterministically hashes the provided byte sequence.
- The raw comparator reports equality for equivalent raw entries and inequality otherwise.

## Requirements

### Functional Requirements

#### FR-1: Hash table initialization
The module shall provide table initialization that accepts a candidate capacity, optional tuning, a hash function, a comparator function, and an optional entry cleanup function, producing a usable hash table object on success.
**Traceability:** `hash_initialize`, `struct hash_table`

#### FR-2: Tuning reset
The module shall provide an operation that resets a tuning object to the module’s default tuning values.
**Traceability:** `hash_reset_tuning`

#### FR-3: Tuning validation for operational use
The module shall validate or otherwise reject unsupported tuning states before or during table setup/use so that the resulting table configuration is internally acceptable to the module.
**Traceability:** `check_tuning`, `hash_initialize`, `struct hash_table`

#### FR-4: Default raw hashing helper
The module shall provide a helper that computes a hash value from raw byte data and a specified byte count.
**Traceability:** `raw_hasher`

#### FR-5: Default raw comparison helper
The module shall provide a helper that compares two entries using raw-data equality semantics.
**Traceability:** `raw_comparator`

#### FR-6: Entry insertion
The module shall support insertion of an entry into an initialized table and return a result consistent with the source module’s insert contract.
**Traceability:** `hash_insert`, `struct hash_table`, `struct hash_entry`

#### FR-7: Entry deletion
The module shall support deletion of an entry matching a supplied lookup key and return the removed stored entry when a match exists.
**Traceability:** `hash_delete`, `struct hash_table`, `struct hash_entry`

#### FR-8: Table rehashing
The module shall support rehashing an existing table to a candidate capacity while preserving the table’s stored entries and operational semantics.
**Traceability:** `hash_rehash`, `struct hash_table`, `struct hash_entry`

#### FR-9: Statistics reporting
The module shall provide a statistics-reporting operation that writes table statistics to a caller-provided output stream.
**Traceability:** `hash_print_statistics`, `struct hash_table`

#### FR-10: Optional entry cleanup integration
The module shall retain and use caller-supplied entry cleanup behavior where applicable to table-managed entry lifecycle events supported by this module.
**Traceability:** `hash_initialize`, `struct hash_table`

### Key Entities

#### Hash table
The central entity is the hash table, which owns or manages the table state required for hashing, comparison, entry storage, resizing behavior, statistics generation, and optional cleanup behavior.

**Traceability:** `struct hash_table`

#### Hash entry
The table stores entries through an internal entry representation. This entity links stored opaque entry values to the table’s collision-management and lookup/deletion behavior.

**Traceability:** `struct hash_entry`

#### Tuning configuration
A tuning configuration controls allowable or preferred table operating parameters and can be reset to defaults or validated for acceptability before use.

**Traceability:** `hash_reset_tuning`, `check_tuning`

#### Behavior callbacks
The module depends on caller-provided hashing, comparison, and optional cleanup behaviors to interpret opaque entry values and manage lifecycle-sensitive operations.

**Traceability:** `hash_initialize`

#### Output stream target
Statistics reporting writes to a caller-provided stream-like target.

**Traceability:** `hash_print_statistics`

## Success Criteria

1. A Rust caller can create a table with supplied hashing and comparison behavior, and the resulting table accepts insert/delete operations without violating the source module’s contract.
   **Traceability:** `hash_initialize`, `hash_insert`, `hash_delete`

2. Resetting tuning produces a default configuration that is accepted by initialization.
   **Traceability:** `hash_reset_tuning`, `hash_initialize`

3. Invalid or unsupported tuning configurations are not silently treated as valid operational state.
   **Traceability:** `check_tuning`, `hash_initialize`

4. Inserting an entry and then deleting it via an equivalent lookup key returns the stored entry and leaves the table in a consistent state.
   **Traceability:** `hash_insert`, `hash_delete`, `struct hash_entry`

5. Deleting a non-existent entry reports absence and does not damage existing table contents.
   **Traceability:** `hash_delete`, `struct hash_table`

6. Rehashing preserves the set of stored entries and continued correctness of comparison-based lookup/deletion behavior.
   **Traceability:** `hash_rehash`, `hash_delete`, `struct hash_table`

7. Statistics reporting writes to the provided output target and does not mutate table contents.
   **Traceability:** `hash_print_statistics`, `struct hash_table`

8. The raw hashing helper returns deterministic results for identical byte sequences, and the raw comparator distinguishes equal from unequal raw entries consistently.
   **Traceability:** `raw_hasher`, `raw_comparator`

9. The Rust rewrite exposes no extra required behavior beyond the evidenced functional scope of this module.
   **Traceability:** `gnu/hash.c` analyzed functions and structures