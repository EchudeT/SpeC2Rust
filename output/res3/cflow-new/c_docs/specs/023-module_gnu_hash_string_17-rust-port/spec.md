# spec.md

## Title

Rust Functional Specification for `module_gnu_hash_string_17`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_hash_string_17`
- **Category**: `module_cluster`
- **Source files analyzed**: `gnu/hash.c`
- **Rust branch**: `023-module_gnu_hash_string_17-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides string-to-bucket hash computation for the hash table logic defined in `gnu/hash.c`. The analyzed entry point is `hash_string(const char *string, size_t n_buckets)`, which maps a null-terminated C string to a bucket index constrained by the caller-provided bucket count.

The Rust rewrite must preserve the observable behavior of this hashing function as used by the surrounding hash table module. The scope evidenced by the analysis is limited to computing a bucket index for string keys; no broader API or new capabilities are required by this specification.

## Feature Specification

### Summary

The Rust version must implement the module behavior that accepts a string key and a bucket count and returns a bucket index suitable for use with the hash table structures in `gnu/hash.c`.

### In-Scope Functionality

- Compute a deterministic hash-derived bucket index from a string input.
- Constrain the result to the range defined by the bucket count.
- Support repeated use for hash table operations that depend on stable mapping of equal strings to the same bucket.
- Integrate conceptually with the hash table entities present in `gnu/hash.c`, especially bucketed storage of `hash_entry` items within `hash_table`.

### Out-of-Scope Functionality

The following are not evidenced as responsibilities of this module specification and must not be introduced as required behavior:

- New public APIs beyond the behavior represented by `hash_string`
- Persistence, serialization, or import/export of hash state
- Thread-safety guarantees
- Recovery, retry, or fault-tolerance mechanisms
- Alternative hashing algorithms selectable at runtime
- FFI-specific requirements

## User Scenarios & Testing

### Scenario 1: Bucket selection for string insertion

A caller preparing to insert a string-keyed entry into a hash table provides a string and the table's current number of buckets. The module returns a bucket index that the caller can use to locate the destination bucket chain or slot.

**Expected support in Rust**
- The same input string and bucket count always produce the same bucket index during a run.
- The returned value is valid for indexing within the current bucket set.

**Test focus**
- Verify the result is `>= 0` and `< n_buckets`.
- Verify deterministic output for repeated calls with identical inputs.

### Scenario 2: Bucket selection for string lookup

A caller performing lookup of an existing string key uses the same string and bucket count used for insertion. The module returns the same bucket index so lookup is directed to the correct bucket.

**Expected support in Rust**
- Equal string content maps to the same bucket index.
- The function behavior is content-based rather than dependent on caller object identity.

**Test focus**
- Verify two independently created but equal strings yield the same result for the same `n_buckets`.
- Verify a lookup-path call and insert-path call agree for the same key.

### Scenario 3: Operation across varying bucket counts

A caller may use different table sizes over the lifetime of the hash table logic. For each provided bucket count, the module computes an index bounded by that count.

**Expected support in Rust**
- The mapping remains valid for each supplied bucket count.
- Result bounds change appropriately with the bucket count.

**Test focus**
- Verify result bounds for multiple `n_buckets` values.
- Verify behavior with small bucket counts such as `1`, where the only valid result is `0`.

### Scenario 4: Use with the surrounding hash table structures

The surrounding module contains `hash_table` and `hash_entry` structures that organize entries into buckets. This hashing behavior is used to decide which bucket a string-keyed entry belongs to.

**Expected support in Rust**
- The hashing function can serve as the bucket-selection mechanism for a Rust representation of the same hash table relationships.
- Returned indices are directly usable by caller-managed bucket storage.

**Test focus**
- In integration tests, verify inserted and subsequently queried string keys are routed to the same bucket index.
- Verify distinct strings can be processed without violating index bounds.

## Requirements

### Functional Requirements

#### FR-1: String key hashing
The module shall accept a string key and compute a deterministic hash-derived value for that key, as evidenced by `hash_string` in `gnu/hash.c`.

**Traceability**
- Function: `hash_string` (`gnu/hash.c:359-373`, `gnu/hash.c:382-391`)

#### FR-2: Bucket-bounded result
The module shall return a value constrained to the caller-supplied bucket count so the result can be used as a bucket index in the hash table.

**Traceability**
- Function signature includes `size_t n_buckets`: `hash_string`
- Related entities: `hash_table`, `hash_entry` in `gnu/hash.c`

#### FR-3: Equal-content consistency
For identical string content and the same bucket count, the module shall return the same bucket index each time.

**Traceability**
- Function: `hash_string`
- Related usage context: `hash_table` / `hash_entry` bucketed organization in `gnu/hash.c`

#### FR-4: Compatibility with string-keyed hash table usage
The module shall support the bucket-selection needs of the surrounding hash table logic represented by `hash_table` and `hash_entry`.

**Traceability**
- Function: `hash_string`
- Types: `struct hash_table`, `struct hash_entry` in `gnu/hash.c`

#### FR-5: Null-terminated string semantics preservation
Because the source function accepts `const char *string`, the Rust rewrite shall preserve behavior for string input as content-based text derived from a C-style string contract at the module boundary or equivalent internal adaptation.

**Traceability**
- Function signature: `hash_string (const char *string, size_t n_buckets)`

### Key Entities

#### `hash_entry`
Represents an entry managed by the hash table module. Entries are associated with bucket placement, and the hash result is used to determine which bucket may contain a given string-keyed entry.

**Traceability**
- Type occurrences: `struct hash_entry` in `gnu/hash.c`

#### `hash_table`
Represents the table structure containing bucket-organized entries. The bucket count supplied to `hash_string` corresponds to this table-level organization.

**Traceability**
- Type definition: `struct hash_table` in `gnu/hash.c:55-87`

#### String key input
The hashing input is a C string (`const char *`) representing the key whose content determines bucket placement.

**Traceability**
- Function signature: `hash_string`

### Entity Relationships

- A `hash_table` organizes many `hash_entry` values into buckets.
- `hash_string` maps a string key to one bucket index within the `hash_table` bucket count.
- Callers use that index to select the subset of `hash_entry` values relevant to insertion or lookup.

## Success Criteria

### Functional Correctness

1. **Deterministic mapping**
   - For any given string content and fixed bucket count, repeated calls return the same result.
   - Traceability: `hash_string`

2. **Valid bucket range**
   - For every supported call with bucket count `n_buckets`, the returned value is within the valid bucket index range for that count.
   - Traceability: `hash_string`, `hash_table`

3. **Equal-string agreement**
   - Two inputs with the same string content produce the same bucket index when `n_buckets` is the same.

4. **Hash table integration suitability**
   - In integration tests modeling `hash_table`/`hash_entry` usage, a key used for insertion and then lookup resolves to the same bucket index.
   - Traceability: `hash_string`, `hash_table`, `hash_entry`

### Porting Acceptance

5. **Behavioral preservation**
   - The Rust implementation preserves the externally observable bucket-selection behavior required by the C module’s string hashing entry point.
   - Traceability: `hash_string` in `gnu/hash.c`

6. **No unevidenced feature expansion**
   - The Rust rewrite exposes only the behavior necessary to support the analyzed hashing responsibility and does not require new capabilities not evidenced in `gnu/hash.c`.
   - Traceability: scope limited to analyzed function and related table entities

## Notes for Implementers

- This specification is intentionally limited to the evidenced functional boundary of string bucket hashing within the broader hash table module.
- Internal Rust design may differ from the C implementation, but externally observable behavior defined above must be preserved.
- Where the Rust code interfaces with non-Rust string sources, adaptation must preserve the content-based semantics of the original `const char *` input contract.