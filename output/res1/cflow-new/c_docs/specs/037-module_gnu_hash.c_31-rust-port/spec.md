# spec.md

## Title

Rust Port Functional Specification: `module_gnu_hash.c_31`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_gnu_hash.c_31`
- **Category**: `module_cluster`
- **Source file**: `gnu/hash.c`
- **Rust branch**: `037-module_gnu_hash.c_31-rust-port`
- **Generation date**: 2026-06-11

## Overview

This module provides a general-purpose hash table facility with configurable hashing, comparison, tuning, insertion, deletion, rehashing, tuning reset, and statistics reporting.

The Rust rewrite must preserve the functional behavior evidenced by `gnu/hash.c`, specifically the behavior represented by:

- hash table initialization with caller-supplied behavior hooks
- default handling for raw hashing and raw comparison when applicable
- validation and reset of tuning parameters
- insertion and deletion of entries
- explicit table rehashing
- reporting of table statistics to an output stream

The specification is limited to behavior evidenced by the analyzed module and does not define additional capabilities beyond those boundaries.

## Feature Specification

### Feature Summary

The module manages a hash table whose behavior is partly defined by caller-supplied functions:

- a hash function for mapping entries to buckets
- a comparator for entry equality
- an optional entry data freer for owned entry cleanup
- an optional tuning configuration influencing table sizing and resizing behavior

The table supports lifecycle and mutation operations:

- initialize a table
- reset tuning values to module defaults
- validate tuning before use
- insert entries
- delete entries
- rehash an existing table to a new candidate size
- print table statistics

### In-Scope Functional Behavior

1. **Hash table creation**
   - A caller can request creation of a table with a candidate size and optional tuning.
   - The module must establish a usable table only when supplied configuration is acceptable.
   - The created table must retain the caller-provided hash, comparison, and free behavior for later operations.

2. **Tuning management**
   - The module supports resetting a tuning object to default values.
   - The module validates tuning values before use and rejects invalid configurations.

3. **Entry hashing and comparison**
   - The module contains behavior for raw hashing of unstructured data and raw comparison of entries.
   - The Rust version must preserve equivalent behavior where this module uses those defaults internally.

4. **Insertion**
   - A caller can insert an entry into the table.
   - The operation must use the table’s configured hashing and comparison behavior.

5. **Deletion**
   - A caller can delete an entry matching a lookup key.
   - The operation must return the removed entry when deletion succeeds, preserving the entry-oriented behavior evidenced by the C API.

6. **Rehashing**
   - A caller can request rehashing of an existing table using a candidate size.
   - Rehashing must preserve the table’s stored entries and continue using the table’s configured behavior hooks.

7. **Statistics reporting**
   - A caller can request printing of table statistics to a provided output stream.
   - The Rust rewrite must preserve the ability to emit statistics based on the current table state.

## User Scenarios & Testing

### Scenario 1: Create a table with explicit behavior functions

A caller supplies:

- a desired initial candidate size
- a tuning configuration or defaulted tuning
- a hash function
- a comparator
- an optional entry freer

The module creates a hash table that can later accept inserts and deletes using those supplied functions.

**Expected result**
- Creation succeeds only for valid configuration.
- The resulting table is usable for subsequent operations.

**Traceability**
- `hash_initialize`
- `check_tuning`
- `hash_reset_tuning`

### Scenario 2: Reset tuning before initialization

A caller has a tuning object but wants known-good default values before creating a table.

**Expected result**
- Resetting tuning produces the module’s default tuning state.
- A table initialized with that reset tuning passes validation and can be created if other inputs are acceptable.

**Traceability**
- `hash_reset_tuning`
- `check_tuning`
- `hash_initialize`

### Scenario 3: Insert entries and retrieve duplicate/equality behavior through comparator semantics

A caller initializes a table and inserts entries whose identity is determined by the configured comparator.

**Expected result**
- Insertion uses the configured hash and comparison behavior.
- Entry placement and equality decisions are governed by those hooks rather than by unrelated object identity.

**Traceability**
- `hash_initialize`
- `hash_insert`
- `raw_hasher`
- `raw_comparator`

### Scenario 4: Delete an existing entry

A caller requests deletion of an entry using a lookup value compatible with the table’s comparator.

**Expected result**
- If a matching entry exists, deletion succeeds and returns the removed entry.
- If no matching entry exists, deletion indicates absence consistently with the source behavior.

**Traceability**
- `hash_delete`

### Scenario 5: Rehash a populated table

A caller with an existing table requests rehashing to a new candidate size.

**Expected result**
- Rehash succeeds only when the module can establish the new table organization.
- Existing entries remain represented in the table after a successful rehash.
- Hashing/comparison behavior remains the one configured on the table.

**Traceability**
- `hash_rehash`
- `hash_initialize`

### Scenario 6: Print table statistics

A caller requests a statistics report for a table and provides an output stream.

**Expected result**
- The module writes statistics derived from the current table state to the stream.
- The operation is observational and does not change table contents.

**Traceability**
- `hash_print_statistics`

## Requirements

### Functional Requirements

#### FR-1: Table initialization
The Rust module shall provide hash table initialization equivalent to `hash_initialize`, accepting:
- a candidate size,
- a tuning configuration,
- a hash function,
- a comparator,
- and a data freer callback.

Initialization shall produce a usable table only when configuration is valid.

**Traceability**
- `hash_initialize`
- `check_tuning`
- `struct hash_table`

#### FR-2: Tuning reset
The Rust module shall provide functionality equivalent to `hash_reset_tuning` that resets a tuning object to the module’s default tuning values.

**Traceability**
- `hash_reset_tuning`

#### FR-3: Tuning validation
The Rust module shall enforce validation of tuning values before accepting them for table use, equivalent in role to `check_tuning`.

Invalid tuning shall prevent successful use for table initialization or resizing.

**Traceability**
- `check_tuning`
- `hash_initialize`
- `hash_rehash`

#### FR-4: Configurable hashing and equality
The Rust module shall use table-configured hashing and comparison behavior for table operations, equivalent to the callback-based behavior evidenced in `hash_initialize`, `hash_insert`, and `hash_delete`.

Where raw hashing and raw comparison are used by the module, the Rust rewrite shall preserve equivalent semantics.

**Traceability**
- `hash_initialize`
- `hash_insert`
- `hash_delete`
- `raw_hasher`
- `raw_comparator`

#### FR-5: Entry insertion
The Rust module shall support insertion of an entry into a table, equivalent in role to `hash_insert`.

Insertion shall operate against an initialized table and use the table’s configured hash/comparison behavior.

**Traceability**
- `hash_insert`
- `struct hash_table`
- `struct hash_entry`

#### FR-6: Entry deletion
The Rust module shall support deletion of an entry matching a supplied lookup value, equivalent in role to `hash_delete`.

Deletion shall use the table’s configured hash/comparison behavior and shall return the removed entry value when a deletion occurs.

**Traceability**
- `hash_delete`
- `struct hash_table`
- `struct hash_entry`

#### FR-7: Table rehashing
The Rust module shall support explicit rehashing of an existing table to a candidate size, equivalent to `hash_rehash`.

Successful rehashing shall preserve the table’s represented entries and continue using the table’s configured hash/comparison behavior.

**Traceability**
- `hash_rehash`
- `struct hash_table`
- `struct hash_entry`

#### FR-8: Statistics reporting
The Rust module shall provide statistics reporting equivalent to `hash_print_statistics`, writing a summary of the current table state to a caller-supplied output target.

**Traceability**
- `hash_print_statistics`
- `struct hash_table`

### Key Entities

#### `Hash_table`
The primary table object representing the hash table’s current state.

It is the owner of:
- table sizing and occupancy state,
- entry storage organization,
- configured hash and comparison behavior,
- configured entry-freeing behavior,
- tuning information used for table management.

**Relationships**
- Contains or manages multiple `hash_entry` values.
- Uses `Hash_tuning` to govern validation and resizing-related behavior.
- Is consumed by initialization, insertion, deletion, rehashing, and statistics reporting operations.

**Traceability**
- `struct hash_table`
- `hash_initialize`
- `hash_insert`
- `hash_delete`
- `hash_rehash`
- `hash_print_statistics`

#### `hash_entry`
The internal entry representation used by the table to hold stored values in the hash table structure.

**Relationships**
- Belongs to a `Hash_table`.
- Participates in insertion, deletion, and reorganization during rehashing.

**Traceability**
- `struct hash_entry`
- `hash_insert`
- `hash_delete`
- `hash_rehash`

#### `Hash_tuning`
The tuning configuration object that controls acceptable and default table tuning parameters.

**Relationships**
- Is reset by tuning reset functionality.
- Is validated before table creation or rehash-related use.
- Influences `Hash_table` behavior.

**Traceability**
- `hash_reset_tuning`
- `check_tuning`
- `hash_initialize`

#### Hashing and comparison callbacks
Caller-provided behavior used by the table to hash entries and determine entry equality.

**Relationships**
- Attached to `Hash_table` during initialization.
- Used by insertion, deletion, and rehash-related operations.
- Raw default-style behavior is represented by `raw_hasher` and `raw_comparator`.

**Traceability**
- `hash_initialize`
- `hash_insert`
- `hash_delete`
- `raw_hasher`
- `raw_comparator`

#### Data freer callback
Caller-provided cleanup behavior associated with table-held entries.

**Relationships**
- Attached to `Hash_table` during initialization.
- Defines how entry data is freed when module behavior requires cleanup.

**Traceability**
- `hash_initialize`
- `struct hash_table`

## Success Criteria

### SC-1: Valid initialization behavior
Given valid tuning and required behavior functions, the Rust implementation can create a usable table corresponding to `hash_initialize`.

Given invalid tuning, initialization fails rather than creating a table.

**Traceability**
- `hash_initialize`
- `check_tuning`

### SC-2: Default tuning reset behavior
After invoking the Rust equivalent of `hash_reset_tuning`, the resulting tuning object is accepted by the module’s tuning validation logic for normal table setup.

**Traceability**
- `hash_reset_tuning`
- `check_tuning`

### SC-3: Insert/delete operational correctness
For a table initialized with a supplied hash function and comparator:
- inserting entries succeeds for supported inputs,
- deleting a present entry returns the removed entry,
- deleting an absent entry reports absence consistently with source behavior.

**Traceability**
- `hash_insert`
- `hash_delete`
- `hash_initialize`

### SC-4: Rehash preserves table contents
After inserting entries and then performing a successful rehash, all previously represented entries remain represented in the table and remain subject to the same comparator semantics.

**Traceability**
- `hash_rehash`
- `hash_insert`
- `hash_delete`

### SC-5: Statistics output is available
For an initialized table, the Rust implementation can emit table statistics to a caller-provided output target without mutating table contents.

**Traceability**
- `hash_print_statistics`

### SC-6: Raw behavior equivalence where used
Where the source module relies on raw hashing or raw comparison behavior, the Rust port preserves equivalent results for the same inputs.

**Traceability**
- `raw_hasher`
- `raw_comparator`