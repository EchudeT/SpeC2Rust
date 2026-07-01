# spec.md

## Title

Functional Specification: `module_gnu_hash_get_13` Rust Port

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_gnu_hash_get_13`
- **Category**: `module_cluster`
- **Source file**: `gnu/hash.c`
- **Rust branch**: `019-module_gnu_hash_get_13-rust-port`
- **Generation date**: `2026-06-17`

## 1. Feature Specification

### 1.1 Purpose

This module provides read-only accessors for basic occupancy and size metrics of a hash table managed by the hash subsystem in `gnu/hash.c`.

The Rust rewrite must preserve the behavior of the C module functions that report:

- the configured number of buckets in a hash table,
- the number of buckets currently in use,
- the number of stored entries.

These functions are observational only: they expose current table state and do not modify the table.

### 1.2 In-Scope Functionality

The Rust version must implement the functional equivalent of these accessors over the Rust representation of the hash table:

- `hash_get_n_buckets`
- `hash_get_n_buckets_used`
- `hash_get_n_entries`

Each accessor must return the corresponding count from the table state represented by the module’s core hash table entity.

### 1.3 Out of Scope

The following are not required by this module specification unless needed internally to support the above accessors:

- hash insertion behavior
- hash removal behavior
- hash iteration behavior
- resizing or rehashing behavior
- allocator or obstack behavior
- new public APIs beyond the evidenced accessor functionality

## 2. User Scenarios & Testing

### 2.1 Scenario: Read configured bucket count

A caller with access to an existing hash table needs to know how many buckets the table currently has.

**Expected behavior**:
- Calling the Rust equivalent of `hash_get_n_buckets` returns the table’s current bucket-count value.
- The call does not alter table contents or occupancy counters.

**Test coverage**:
- Create or obtain a table with a known bucket count.
- Verify the accessor returns that exact count.
- Verify repeated calls return the same value when the table is unchanged.

### 2.2 Scenario: Read number of buckets in use

A caller needs to inspect how many buckets currently contain at least one entry.

**Expected behavior**:
- Calling the Rust equivalent of `hash_get_n_buckets_used` returns the current used-bucket count tracked by the table.
- The call is read-only.

**Test coverage**:
- Obtain a table state with a known used-bucket count.
- Verify the accessor returns that exact count.
- Verify the value reflects current state after the table has been populated elsewhere.

### 2.3 Scenario: Read total entry count

A caller needs to know how many entries are stored in the table.

**Expected behavior**:
- Calling the Rust equivalent of `hash_get_n_entries` returns the current total entry count.
- The call is read-only.

**Test coverage**:
- Obtain a table state with a known entry count.
- Verify the accessor returns that exact count.
- Verify repeated reads are stable when no mutation occurs.

### 2.4 Scenario: Distinguish table metrics

A caller must separately inspect capacity-related and occupancy-related metrics.

**Expected behavior**:
- Bucket count, used-bucket count, and entry count are independently readable.
- The Rust port does not collapse these into a single derived value.

**Test coverage**:
- Use a table state where:
  - bucket count differs from used-bucket count, and
  - used-bucket count differs from entry count.
- Verify each accessor returns its own distinct metric.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Bucket count accessor

The module shall provide a read-only operation equivalent to `hash_get_n_buckets` that returns the current number of buckets stored in the hash table state.

**Traceability**:
- Function: `hash_get_n_buckets` (`gnu/hash.c:144-148`)
- Entity: `struct hash_table` (`gnu/hash.c:55-87`)

#### FR-2: Used-bucket count accessor

The module shall provide a read-only operation equivalent to `hash_get_n_buckets_used` that returns the current number of buckets in use stored in the hash table state.

**Traceability**:
- Function: `hash_get_n_buckets_used` (`gnu/hash.c:150-154`)
- Entity: `struct hash_table` (`gnu/hash.c:55-87`)

#### FR-3: Entry count accessor

The module shall provide a read-only operation equivalent to `hash_get_n_entries` that returns the current number of entries stored in the hash table state.

**Traceability**:
- Function: `hash_get_n_entries` (`gnu/hash.c:156-160`)
- Entity: `struct hash_table` (`gnu/hash.c:55-87`)

#### FR-4: No state mutation during metric reads

The module shall not modify hash table logical state when serving any of the three accessor operations.

**Traceability**:
- Functions: `hash_get_n_buckets`, `hash_get_n_buckets_used`, `hash_get_n_entries` (`gnu/hash.c:144-160`)
- Entity: `struct hash_table` (`gnu/hash.c:55-87`)

#### FR-5: Metric identity preservation

The module shall preserve the distinction between:
- total bucket count,
- buckets-used count,
- total entry count,

as separate observable properties of the table.

**Traceability**:
- Functions: `hash_get_n_buckets`, `hash_get_n_buckets_used`, `hash_get_n_entries` (`gnu/hash.c:144-160`)
- Entity: `struct hash_table` (`gnu/hash.c:55-87`)

### 3.2 Key Entities

#### Hash table

The central entity is the hash table structure represented in C as `struct hash_table`. It owns or tracks the table-wide state from which all three reported metrics are read.

Relevant relationship:
- It maintains table-level counts for bucket capacity and occupancy.
- The accessor functions read these counts from the table.

**Traceability**:
- Entity: `struct hash_table` (`gnu/hash.c:55-87`)

#### Hash entry

The table is composed of linked or associated hash entries represented in C as `struct hash_entry`. While this module does not expose entry operations directly, entry presence is part of the table state that determines occupancy and entry totals.

Relevant relationship:
- Entries are associated with the hash table.
- Entry distribution across buckets underlies the difference between used-bucket count and total entry count.

**Traceability**:
- Entity: `struct hash_entry` (`gnu/hash.c:49-53` and further references in `gnu/hash.c`)

## 4. Success Criteria

### 4.1 Functional correctness

- The Rust port returns the same bucket count as the source module for equivalent table state.
- The Rust port returns the same used-bucket count as the source module for equivalent table state.
- The Rust port returns the same entry count as the source module for equivalent table state.

### 4.2 Read-only behavior

- Tests demonstrate that calling any accessor does not change:
  - bucket count,
  - used-bucket count,
  - entry count,
  - or stored entries.

### 4.3 Distinct metric reporting

- Tests demonstrate that the three accessors can return different values when table state warrants it.
- No accessor returns another metric in place of its own.

### 4.4 Type-appropriate count reporting

- Each accessor returns a count value compatible with the source module’s count semantics (`size_t` in C), using the Rust type chosen for representing table counts in the port.
- Tests cover non-negative count values including zero where table state permits.

### 4.5 Traceable implementation completeness

- The Rust module includes functional equivalents for all three evidenced accessor functions and no required accessor from this module is omitted.