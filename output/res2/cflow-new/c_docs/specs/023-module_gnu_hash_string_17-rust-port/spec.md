# spec.md

## Title

Rust Functional Specification for `module_gnu_hash_string_17`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_hash_string_17`
- **Category**: `module_cluster`
- **Source file**: `gnu/hash.c`
- **Rust branch**: `023-module_gnu_hash_string_17-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides string-to-bucket hashing behavior used by the project’s hash-table logic. The analyzed module evidence identifies `hash_string(const char *string, size_t n_buckets)` as the relevant functional boundary for this port, with supporting table and entry structures present in the same source file.

The Rust rewrite must implement the same observable behavior of mapping a C-style string input to a bucket index constrained by a caller-supplied bucket count. The port scope is limited to this hashing functionality and its role as a helper for the surrounding hash table module behavior already evidenced in `gnu/hash.c`.

## Feature Specification

### Feature: Deterministic string bucket hashing

The module shall accept a string and a bucket-count value and compute the bucket index for that string within the range defined by the bucket count.

#### Behavior
- The same input string and same bucket count shall always produce the same bucket index during a process run.
- The produced index shall be suitable for selecting a bucket in a hash table managed by the module’s surrounding data structures.
- The result shall be constrained by the provided bucket count, matching the source module’s contract of returning `size_t` bucket positions for `n_buckets`.
- The Rust version shall preserve the source module’s externally observable semantics for valid inputs used by the project.

#### Port scope
- Included:
  - String hashing to bucket index as represented by `hash_string`.
  - Compatibility with the hash table’s use of bucket arrays and hash entries as evidenced by `struct hash_table` and `struct hash_entry`.
- Excluded:
  - New hash algorithms.
  - New public APIs beyond what is needed to preserve this module behavior.
  - Features not evidenced by the analyzed source.

## User Scenarios & Testing

### Scenario 1: Select a bucket for inserting a string key
A caller has a string key and a hash table with a known number of buckets. The caller uses this module to compute the destination bucket index before linking or storing a `hash_entry`.

**Expected support**
- The Rust module returns a valid bucket index for the provided bucket count.
- Repeating the call with the same key and bucket count returns the same result.

**Testing approach**
- Use representative strings, including empty and non-empty strings.
- Verify the returned value is within the valid bucket range.
- Verify determinism across repeated invocations.

### Scenario 2: Select a bucket for looking up an existing string key
A caller needs to find the bucket that may contain entries matching a given string. The caller hashes the same string with the current bucket count.

**Expected support**
- The Rust module returns the same bucket index used for insertion when the string and bucket count are unchanged.

**Testing approach**
- For a set of keys, compute the bucket index twice and verify equality.
- Verify lookup hashing remains consistent across module calls.

### Scenario 3: Operate with varying bucket counts
A caller may use different table sizes at different times. The same string may therefore map to different bucket indices when the bucket count changes.

**Expected support**
- The Rust module honors the supplied bucket count each time it is called.
- The result always remains valid for the specific bucket count used in that call.

**Testing approach**
- Hash the same string with multiple bucket counts.
- Verify every result falls within the corresponding valid range.

### Scenario 4: Integrate with existing hash table entities
The surrounding module maintains hash entries and hash tables. This hashing function is used as a helper to choose a bucket associated with those entities.

**Expected support**
- The Rust rewrite remains usable as the bucket-selection function for table operations that organize `hash_entry` instances within a `hash_table`.

**Testing approach**
- Create a minimal table model in tests with a chosen bucket count.
- Use the hash result to place and then retrieve keys from the modeled bucket positions.

## Requirements

### Functional Requirements

#### FR-1: String hashing function
The Rust module shall provide the functional equivalent of `hash_string(const char *string, size_t n_buckets)` from `gnu/hash.c`.

**Traceability**
- Function: `hash_string` at `gnu/hash.c:359-373`
- Function: `hash_string` at `gnu/hash.c:382-391`

#### FR-2: Bucket-bounded result
For valid calls, the hashing function shall return a bucket selector derived from the input string and constrained by the caller-provided bucket count.

**Traceability**
- Function signature and return role: `hash_string` at `gnu/hash.c:359-373`, `gnu/hash.c:382-391`
- Related entity: `struct hash_table` in `gnu/hash.c:55-87`

#### FR-3: Deterministic mapping
For identical string content and identical bucket count, the hashing function shall return the same bucket index on repeated calls.

**Traceability**
- Function role: `hash_string` at `gnu/hash.c:359-373`, `gnu/hash.c:382-391`
- Related entities: `struct hash_entry`, `struct hash_table` in `gnu/hash.c`

#### FR-4: Compatibility with hash table usage
The hashing behavior shall remain suitable for use by the module’s hash table logic that organizes entries into buckets.

**Traceability**
- Function: `hash_string` at `gnu/hash.c:359-373`, `gnu/hash.c:382-391`
- Types: `struct hash_entry` and `struct hash_table` in `gnu/hash.c:49-87`

### Key Entities

#### `hash_entry`
Represents an entry managed by the hash table logic in `gnu/hash.c`. The available evidence shows that bucket hashing exists to support placement or lookup of entries associated with string keys.

**Relationship**
- Entries are organized within a hash table.
- The hashing function selects the bucket relevant to an entry’s string key.

**Traceability**
- Type occurrences: `struct hash_entry` in `gnu/hash.c`, including definition area at `gnu/hash.c:49-53`

#### `hash_table`
Represents the table structure that owns or references buckets and entries. The hashing function uses the caller-supplied bucket count corresponding to this table organization.

**Relationship**
- Owns the bucket space into which `hash_entry` instances are grouped.
- Supplies the bucket-count context required by `hash_string`.

**Traceability**
- Type definition area: `struct hash_table` at `gnu/hash.c:55-87`

#### `hash_string`
Computes the bucket index for a string under a specified bucket count.

**Relationship**
- Connects string keys to `hash_table` bucket positions.
- Supports operations involving `hash_entry` placement and retrieval.

**Traceability**
- Function: `gnu/hash.c:359-373`
- Function: `gnu/hash.c:382-391`

## Success Criteria

### SC-1: Functional equivalence for valid inputs
For valid module inputs used by the project, the Rust implementation produces the same bucket index behavior as the source `hash_string` contract for a given string and bucket count.

**Traceability**
- `hash_string` at `gnu/hash.c:359-373`, `gnu/hash.c:382-391`

### SC-2: Range correctness
Tests demonstrate that every returned bucket index is valid for the provided bucket count in all covered scenarios.

**Traceability**
- `hash_string` at `gnu/hash.c:359-373`, `gnu/hash.c:382-391`
- `struct hash_table` at `gnu/hash.c:55-87`

### SC-3: Determinism
Tests demonstrate that repeated hashing of the same string with the same bucket count yields the same result.

**Traceability**
- `hash_string` at `gnu/hash.c:359-373`, `gnu/hash.c:382-391`

### SC-4: Table-integration suitability
Tests using a minimal bucketed table model demonstrate that the Rust hashing function can be used consistently for both bucket selection during insertion and bucket selection during lookup.

**Traceability**
- `hash_string` at `gnu/hash.c:359-373`, `gnu/hash.c:382-391`
- `struct hash_entry` at `gnu/hash.c:49-53`
- `struct hash_table` at `gnu/hash.c:55-87`

## Out of Scope

The Rust port specification does not require:
- Any new public API beyond preserving the evidenced hashing behavior.
- New data persistence, serialization, concurrency, or recovery features.
- Any guarantees not directly supported by the analyzed source evidence.