# spec.md

## Title

Rust Functional Specification for `module_gnu_hash_string_17`

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_hash_string_17`
- Category: `module_cluster`
- Source files: `gnu/hash.c`
- Primary traced function: `hash_string`
- Primary traced entities: `struct hash_entry`, `struct hash_table`
- Rust branch target: `023-module_gnu_hash_string_17-rust-port`
- Generation date: `2026-06-11`

## 1. Feature Specification

### 1.1 Overview

This module provides string-to-bucket hashing behavior used by the hash table logic in `gnu/hash.c`. The evidenced public behavior for this module is the computation of a bucket index from:

- a NUL-terminated input string, and
- a caller-supplied bucket count.

The Rust rewrite must preserve the observable behavior of `hash_string` as used by the surrounding hash table module. Its responsibility is limited to producing a valid bucket position for string keys so that table operations can place or locate entries consistently.

### 1.2 In-Scope Functionality

The Rust version must implement:

- Hashing of a C-style string key into a bucket index.
- Use of the provided bucket count to constrain the result to the table’s bucket domain.
- Deterministic behavior: the same input string and bucket count produce the same bucket index.
- Behavior suitable for integration with the module’s hash table entities represented by `struct hash_table` and `struct hash_entry`.

### 1.3 Out-of-Scope Functionality

The following are not evidenced by the provided module slice and must not be added to the specification:

- New public APIs beyond the behavior traced to `hash_string`.
- Thread-safety guarantees.
- Persistence, serialization, or external interchange formats.
- Recovery, journaling, or fault-tolerance features.
- Alternative hashing modes or configurable hash algorithms.
- FFI requirements.

## 2. User Scenarios & Testing

### 2.1 Scenario: Place a string key into a hash table bucket

A caller managing a hash table has a string key and a table with a known number of buckets. The caller uses this module to compute the bucket index for that key and then uses that index to access the corresponding bucket chain in the table.

The Rust version must support this by returning a bucket index valid for the supplied bucket count.

**Testing focus**
- Given a non-empty string and positive bucket count, the returned index is within the bucket range.
- Repeated calls with the same inputs return the same index.

### 2.2 Scenario: Look up an existing string key consistently

A caller previously inserted or associated a string key with a specific bucket index. Later, the same key is hashed again for lookup. The module must return the same bucket index so lookup can search the correct bucket chain.

**Testing focus**
- Hashing the same string with the same bucket count across multiple calls yields identical results.
- Different calls do not depend on mutable external state.

### 2.3 Scenario: Distinguish table layouts by bucket count

A caller may use the same string key with different hash table sizes. The bucket count supplied to the hashing function determines the bucket domain for the current table.

**Testing focus**
- For the same string, results are valid for each supplied bucket count.
- When bucket counts differ, the result remains constrained to the corresponding range.

### 2.4 Scenario: Operate with general string content

A caller supplies ordinary C strings, including empty strings or strings with varying character sequences. The module must hash the full string as provided up to its terminating NUL and produce a valid bucket index.

**Testing focus**
- Empty string input produces a valid bucket index when bucket count is valid.
- Representative strings with different contents produce deterministic outputs.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: String bucket hashing
The module shall accept a NUL-terminated string and a bucket count and compute a bucket index for that string.

**Traceability:** `gnu/hash.c`, `hash_string`

#### FR-2: Result range constraint
The module shall return a bucket index that is within the domain defined by the supplied bucket count.

**Traceability:** `gnu/hash.c`, `hash_string`; integration context: `struct hash_table`

#### FR-3: Deterministic mapping
For identical input string content and identical bucket count, the module shall return the same bucket index on repeated invocations.

**Traceability:** `gnu/hash.c`, `hash_string`

#### FR-4: String-key compatibility with hash table use
The module shall provide hashing behavior suitable for use with the hash table entities in this file, so that string-keyed entries can be assigned to and searched within table buckets consistently.

**Traceability:** `gnu/hash.c`, `hash_string`, `struct hash_table`, `struct hash_entry`

#### FR-5: Full-string evaluation
The module shall base the hash on the supplied string content as a C string, meaning processing proceeds over the string content terminated by NUL.

**Traceability:** `gnu/hash.c`, `hash_string (const char *string, size_t n_buckets)`

### 3.2 Key Entities

#### `hash_table`
The central table entity that owns or references the bucket structure into which entries are organized. Its relationship to this module is that the bucket count supplied to `hash_string` corresponds to the table’s bucket space.

**Traceability:** `gnu/hash.c:55-87`, `struct hash_table`

#### `hash_entry`
The entry entity stored in or linked from hash table buckets. Its relationship to this module is indirect: `hash_string` selects the bucket in which an entry for a given string key belongs or should be searched.

**Traceability:** `gnu/hash.c:49-53` and subsequent references to `struct hash_entry`

#### String key
The input key material consumed by `hash_string`. It is represented as a NUL-terminated character sequence and is the basis for bucket selection.

**Traceability:** `gnu/hash.c`, `hash_string (const char *string, size_t n_buckets)`

#### Bucket count
The size parameter defining the valid output range for hashing and the effective bucket domain of the associated hash table.

**Traceability:** `gnu/hash.c`, `hash_string (const char *string, size_t n_buckets)`; related table context: `struct hash_table`

## 4. Success Criteria

### 4.1 Behavioral Correctness

- The Rust module provides a functionally equivalent string-to-bucket hashing behavior for the role served by `hash_string` in `gnu/hash.c`.
- For every tested input string and valid bucket count, the returned value is a valid bucket index for that bucket count.
- For repeated invocations with the same string and bucket count, the returned value is identical.

**Traceability:** `gnu/hash.c`, `hash_string`

### 4.2 Integration Readiness

- The Rust rewrite can be used by the rewritten hash table logic to assign and locate string-keyed entries using bucket indices derived from the same key input.
- The hashing behavior is compatible with the module’s identified table/entry model (`hash_table` and `hash_entry`) and does not require capabilities beyond those evidenced in the source file.

**Traceability:** `gnu/hash.c`, `hash_string`, `struct hash_table`, `struct hash_entry`

### 4.3 Test Completion Criteria

The port is considered successful when all of the following are demonstrated:

1. Tests confirm deterministic results for repeated calls with identical inputs.
2. Tests confirm output range validity for representative bucket counts.
3. Tests cover empty and non-empty string inputs.
4. Tests demonstrate that the same key can be re-hashed consistently for placement and lookup scenarios within a table model based on the identified entities.

**Traceability:** `gnu/hash.c`, `hash_string`, `struct hash_table`, `struct hash_entry`