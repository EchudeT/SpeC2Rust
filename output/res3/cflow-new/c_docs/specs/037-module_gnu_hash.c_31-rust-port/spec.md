# spec.md

## Title

Rust Functional Specification for `module_gnu_hash.c_31`

## Document Control

- Project: `cflow-new`
- Module: `module_gnu_hash.c_31`
- Category: `module_cluster`
- Source file: `gnu/hash.c`
- Rust branch: `037-module_gnu_hash.c_31-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides a configurable hash table for storing opaque entries referenced by pointer-like values. The module supports table creation with caller-supplied hashing and equality behavior, insertion and deletion of entries, table retuning reset support, resizing through rehashing, and reporting of table occupancy statistics.

The Rust rewrite must preserve the observable behavior evidenced by the source module:

- initialize a hash table with optional tuning and optional custom hash/comparison/free callbacks,
- fall back to raw byte/pointer-style hashing and comparison when custom behavior is not supplied,
- validate and normalize tuning before use,
- insert entries into the table and delete matching entries,
- rehash an existing table to a new candidate capacity while preserving stored entries,
- print table statistics to a provided output stream,
- reset a tuning object to module defaults.

This specification is limited to the behavior evidenced by:
`hash_print_statistics`, `hash_reset_tuning`, `raw_hasher`, `raw_comparator`, `check_tuning`, `hash_initialize`, `hash_rehash`, `hash_insert`, and `hash_delete`, along with the `hash_table`, `hash_entry`, and tuning-related data they operate on.

## Feature Specification

### 1. Configurable hash table lifecycle

The module shall provide a hash table type that can be initialized for later insert, delete, rehash, and statistics operations.

Initialization behavior shall include:

- accepting a requested candidate size,
- accepting optional tuning data,
- accepting caller-defined hash and comparator behavior,
- accepting caller-defined entry cleanup behavior,
- producing a usable table only when tuning and allocation preconditions are satisfied.

When custom hash or comparator functions are absent, the module shall use built-in raw hashing and raw comparison behavior.

### 2. Tuning reset and validation

The module shall support restoring a tuning structure to default values through a dedicated reset operation.

The module shall validate tuning values before using them for table sizing or load management decisions. Invalid tuning shall cause initialization or later tuning checks to fail rather than silently proceeding with unusable parameters.

### 3. Entry insertion

The module shall support inserting an entry into an initialized table.

Insertion behavior shall use the table’s configured hash and comparison behavior to locate the target bucket/slot and determine whether an equivalent entry is already present.

The Rust version must preserve the module’s set-like semantics evidenced by a dedicated insert operation that returns a pointer-like result rather than exposing indexed placement details.

### 4. Entry deletion

The module shall support removing an entry matching a supplied lookup value from an initialized table.

Deletion behavior shall use the table’s configured hash and comparison behavior and return the removed stored entry when deletion succeeds, or a null/none-equivalent result when no matching entry is present.

### 5. Rehashing and capacity change

The module shall support rebuilding an existing table around a new candidate capacity.

Rehashing shall preserve existing logical contents while changing the table’s internal placement according to the configured hash behavior and the new capacity decision. Rehashing shall report success or failure.

### 6. Statistics reporting

The module shall support printing statistics about the current table state to a caller-provided output stream.

The Rust rewrite must preserve the existence of this observable reporting behavior and ensure it reads the current table state without modifying table contents.

## User Scenarios & Testing

### Scenario 1: Create a table with caller-defined entry semantics

A caller creates a table for opaque entries, providing custom hash and equality functions and optionally a cleanup function. The caller then uses the returned table for later operations.

The Rust version must support:

- successful creation when valid tuning and allocation conditions are met,
- defaulting behavior when tuning is absent,
- custom hash/comparison behavior being used by later insert/delete/rehash operations,
- failure signaling when tuning is invalid or construction cannot complete.

### Scenario 2: Create a table using built-in raw behavior

A caller creates a table without supplying custom hasher or comparator functions. The module uses built-in raw behavior for hashing and equality.

The Rust version must support:

- initialization without custom callbacks,
- use of the module’s default raw hashing behavior,
- use of the module’s default raw comparison behavior,
- consistent lookup semantics across insertion and deletion under those defaults.

### Scenario 3: Reset tuning before initialization

A caller has a tuning object and wants known default settings before passing it to table creation.

The Rust version must support:

- resetting tuning to module defaults,
- using the reset tuning object successfully in later initialization,
- deterministic default values after reset.

### Scenario 4: Insert entries and detect stored-equivalent behavior

A caller inserts one or more entries into the table, including values that compare equal under the configured comparator.

The Rust version must support tests that verify:

- entries can be inserted into an initialized table,
- insertion uses the configured hash and comparator,
- equivalent-entry handling matches the source module’s observable insert semantics,
- table contents remain valid after repeated insertions.

### Scenario 5: Delete an existing entry

A caller deletes an entry previously stored in the table.

The Rust version must support tests that verify:

- deletion finds an entry using the configured hash/comparison rules,
- the removed stored entry is returned,
- subsequent deletion of the same logical entry reports absence,
- deletion does not corrupt remaining table contents.

### Scenario 6: Delete a non-existent entry

A caller requests deletion for a value not present in the table.

The Rust version must support tests that verify:

- the operation completes without content corruption,
- the result indicates that no matching entry was removed.

### Scenario 7: Rehash after population

A caller populates a table, then requests rehashing with a different candidate size.

The Rust version must support tests that verify:

- rehash reports success or failure,
- on success, all previously stored entries remain logically present,
- post-rehash deletion and any further insertion still use the same hash/comparator semantics,
- rehash does not change entry equivalence rules.

### Scenario 8: Print statistics for inspection

A caller asks the module to print table statistics to an output stream.

The Rust version must support tests that verify:

- statistics output can be directed to a supplied stream/writer,
- the operation does not mutate table contents,
- output is produced for an initialized table.

## Requirements

### Functional Requirements

#### FR-1: Table initialization
The module shall provide an operation that creates and returns a hash table from a candidate size, optional tuning, optional hasher, optional comparator, and optional entry-freeing behavior.
**Traceability:** `hash_initialize`, `hash_table`

#### FR-2: Default behavior when callbacks are absent
If no hasher is supplied, the module shall use built-in raw hashing behavior. If no comparator is supplied, the module shall use built-in raw comparison behavior.
**Traceability:** `hash_initialize`, `raw_hasher`, `raw_comparator`

#### FR-3: Tuning reset
The module shall provide an operation that resets a tuning object to the module’s default tuning values.
**Traceability:** `hash_reset_tuning`

#### FR-4: Tuning validation
The module shall validate tuning data before relying on it for table behavior. Invalid tuning shall cause the relevant operation to fail rather than proceeding with invalid configuration.
**Traceability:** `check_tuning`, `hash_initialize`, `hash_rehash`

#### FR-5: Insert operation
The module shall provide an operation that inserts an entry into an initialized hash table using the table’s configured hash and comparison behavior.
**Traceability:** `hash_insert`, `hash_table`

#### FR-6: Delete operation
The module shall provide an operation that removes and returns an entry matching a supplied lookup value from an initialized hash table, or indicates absence when no match exists.
**Traceability:** `hash_delete`, `hash_table`

#### FR-7: Rehash operation
The module shall provide an operation that rebuilds an existing hash table using a new candidate capacity and reports whether the rebuild succeeded.
**Traceability:** `hash_rehash`, `hash_table`

#### FR-8: Content preservation across successful rehash
A successful rehash shall preserve the logical set of stored entries so that subsequent table operations observe the same contents under the same comparator semantics.
**Traceability:** `hash_rehash`, `hash_insert`, `hash_delete`, `hash_table`

#### FR-9: Statistics reporting
The module shall provide an operation that prints table statistics to a caller-supplied output stream without changing table contents.
**Traceability:** `hash_print_statistics`, `hash_table`

#### FR-10: Stored cleanup behavior retention
The table shall retain caller-supplied entry-freeing behavior as part of table state for use by module-managed entry lifecycle paths.
**Traceability:** `hash_initialize`, `hash_table`

### Key Entities

#### `Hash_table`
The central table state object. It owns or references the current hash storage, active tuning, and the function behaviors used for hashing, comparing, and freeing stored entries. It is the required context for statistics, insertion, deletion, and rehashing.
**Traceability:** `struct hash_table`, `hash_initialize`, `hash_rehash`, `hash_insert`, `hash_delete`, `hash_print_statistics`

#### `Hash_entry`
The per-slot or per-entry representation used by the table to hold stored entry references and manage occupancy state within the table.
**Traceability:** `struct hash_entry`, `struct hash_table`

#### `Hash_tuning`
A configuration object controlling table tuning parameters. It can be reset to defaults and is validated before use.
**Traceability:** `hash_reset_tuning`, `check_tuning`, `hash_initialize`

#### Hasher behavior
A function behavior that maps an entry or lookup value plus table sizing context into a hash value used for placement and lookup. The module supports caller-supplied hashing and a built-in raw fallback.
**Traceability:** `raw_hasher`, `hash_initialize`, `hash_insert`, `hash_delete`, `hash_rehash`

#### Comparator behavior
A function behavior that determines whether two entries are considered equivalent for lookup, insertion, and deletion. The module supports caller-supplied comparison and a built-in raw fallback.
**Traceability:** `raw_comparator`, `hash_initialize`, `hash_insert`, `hash_delete`

#### Data freer behavior
A caller-supplied cleanup behavior retained by the table for entry lifecycle management where the module performs removal or teardown work.
**Traceability:** `hash_initialize`, `struct hash_table`

## Success Criteria

1. A Rust implementation can initialize a table with valid inputs and returns failure for invalid tuning or unconstructable state.
   **Traceability:** `hash_initialize`, `check_tuning`

2. Resetting a tuning object produces deterministic default settings suitable for later initialization.
   **Traceability:** `hash_reset_tuning`, `hash_initialize`

3. When custom hasher/comparator functions are omitted, the Rust implementation uses built-in default raw behavior for hashing and equality.
   **Traceability:** `hash_initialize`, `raw_hasher`, `raw_comparator`

4. Insertion into an initialized table succeeds according to the source module’s observable semantics and uses the configured hash/comparator behavior.
   **Traceability:** `hash_insert`, `hash_table`

5. Deleting a present entry returns the stored entry, and deleting an absent entry returns a null/none-equivalent result.
   **Traceability:** `hash_delete`

6. A successful rehash preserves the table’s logical contents so that previously inserted entries remain findable/removable under unchanged equivalence rules.
   **Traceability:** `hash_rehash`, `hash_delete`, `hash_table`

7. Rehash reports success or failure explicitly and does not silently lose logical contents on success.
   **Traceability:** `hash_rehash`

8. Statistics printing produces output to a supplied stream/writer and does not mutate table contents.
   **Traceability:** `hash_print_statistics`

9. The Rust module’s key entities map cleanly to the source concepts of table state, entry state, tuning, hashing, comparison, and optional cleanup behavior without adding unsupported externally visible capabilities.
   **Traceability:** `struct hash_table`, `struct hash_entry`, `hash_reset_tuning`, `hash_initialize`