# spec.md

## Title

Functional Specification for `module_gnu_hash_get_13` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_hash_get_13`
- Category: `module_cluster`
- Source file: `gnu/hash.c`
- Rust branch: `019-module_gnu_hash_get_13-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides read-only accessors for hash table occupancy metadata. Its responsibility is limited to reporting:

- the total number of buckets in a hash table,
- the number of buckets currently used, and
- the total number of entries stored.

The Rust rewrite must preserve this narrowly scoped behavior for the hash table type represented in `gnu/hash.c`. The module does not define hash table mutation behavior as part of its own functional boundary; it only exposes already-maintained counts from the underlying table state.

## Feature Specification

### Supported Features

The Rust version must implement three metadata query operations over the hash table entity defined by this module context:

1. **Bucket count query**
   - Return the total number of buckets configured in a hash table.

2. **Used bucket count query**
   - Return how many buckets are currently occupied.

3. **Entry count query**
   - Return how many entries are currently stored in the table.

### Behavioral Scope

- These operations are observer functions only.
- They must read the current state of the hash table and return the corresponding count value.
- They must not alter the hash table, its entries, or any allocator-related state.
- Returned values must correspond directly to the table fields maintained by the underlying hash table structure in `gnu/hash.c`.

### Out of Scope

The following are not part of this module’s evidenced functional scope and must not be added to the specification:

- hash insertion, deletion, lookup, resizing, or iteration APIs,
- creation or destruction semantics beyond what is necessary to access an existing table instance,
- concurrency guarantees,
- serialization, persistence, or recovery behavior,
- cross-language or FFI requirements.

## User Scenarios & Testing

### Scenario 1: Retrieve total bucket capacity metadata

A caller that already owns or references a valid hash table needs to know how many buckets the table currently has.

**Expected behavior**
- The module returns the table’s total bucket count.
- The call does not mutate table state.

**Test coverage**
- Given a table with a known bucket count, the accessor returns that exact count.
- Repeated calls return the same result if the table is unchanged.

### Scenario 2: Retrieve occupied bucket metadata

A caller needs to inspect how many buckets are in use to understand current occupancy.

**Expected behavior**
- The module returns the count of used buckets recorded in the table.
- The result reflects current table state at the time of the call.

**Test coverage**
- Given a table with a known used-bucket count, the accessor returns that count.
- If the underlying table state changes elsewhere before the call, the returned value reflects the updated recorded count.

### Scenario 3: Retrieve total entry metadata

A caller needs the number of stored entries for reporting or consistency checks.

**Expected behavior**
- The module returns the total number of entries recorded in the table.
- The call is non-mutating.

**Test coverage**
- Given a table with a known entry count, the accessor returns that exact count.
- Repeated reads on an unchanged table remain stable.

### Scenario 4: Use all three accessors together for table status inspection

A caller inspects a hash table by collecting all three metrics from the same table instance.

**Expected behavior**
- All three accessors can be called independently on the same table reference.
- Each returns its own corresponding count without interfering with the others.

**Test coverage**
- For a fixture table with known values for total buckets, used buckets, and entries, all three returned values match the fixture state.
- Calling the accessors in any order yields the same per-field results for an unchanged table.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a query that returns the total number of buckets for a hash table.
  **Traceability**: `hash_get_n_buckets` in `gnu/hash.c:144-148`; `struct hash_table` in `gnu/hash.c:55-87`.

- **FR-2**: The module shall provide a query that returns the number of used buckets for a hash table.
  **Traceability**: `hash_get_n_buckets_used` in `gnu/hash.c:150-154`; `struct hash_table` in `gnu/hash.c:55-87`.

- **FR-3**: The module shall provide a query that returns the number of entries for a hash table.
  **Traceability**: `hash_get_n_entries` in `gnu/hash.c:156-160`; `struct hash_table` in `gnu/hash.c:55-87`.

- **FR-4**: Each query shall operate as a read-only observer over an existing hash table instance and shall not modify the table state.
  **Traceability**: Function signatures accept `const Hash_table *` in `gnu/hash.c:144-160`.

- **FR-5**: Each query shall return a count value represented as an unsigned size-based quantity corresponding to the source module’s count return type.
  **Traceability**: Return type `size_t` in `gnu/hash.c:144-160`.

### Key Entities

- **Hash table**
  - The primary entity queried by this module.
  - It stores count metadata for bucket capacity, bucket usage, and entry count.
  - Relationships:
    - Owns or references hash entries.
    - Maintains the aggregate counts returned by the three accessor functions.
  - **Traceability**: `struct hash_table` in `gnu/hash.c:55-87`.

- **Hash entry**
  - Element type associated with the hash table’s stored contents.
  - Included here because table counts describe aggregate state over these entries and their bucket placement.
  - **Traceability**: `struct hash_entry` declarations referenced throughout `gnu/hash.c`, including `gnu/hash.c:49-53`.

## Success Criteria

- **SC-1**: The Rust module exposes equivalents of the three source accessors for total buckets, used buckets, and entry count, with one observable result per query matching the source module’s behavior.
  **Traceability**: `hash_get_n_buckets`, `hash_get_n_buckets_used`, `hash_get_n_entries` in `gnu/hash.c:144-160`.

- **SC-2**: For test fixtures with known hash table metadata values, each Rust accessor returns the exact corresponding stored count.
  **Traceability**: `struct hash_table` in `gnu/hash.c:55-87`; accessor functions in `gnu/hash.c:144-160`.

- **SC-3**: Invoking any of the Rust accessors does not change the observed count values of an otherwise unchanged table instance.
  **Traceability**: `const Hash_table *` observer signatures in `gnu/hash.c:144-160`.

- **SC-4**: The Rust port preserves the source module’s functional scope by limiting behavior to metadata retrieval only, with no additional public capabilities required by this specification.
  **Traceability**: Entire evidenced module surface from `gnu/hash.c:144-160`.