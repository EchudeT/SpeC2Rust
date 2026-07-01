# spec.md

## Title

Rust Functional Specification for `module_gnu_GL_ATTRIBUTE_12`

## Summary

This module cluster covers two observed responsibilities from `gnu/error.c` and `gnu/hash.c`:

- formatted error tail emission used by the GNU-style error reporting path
- generic hash table behavior, including table sizing through prime-number selection

The Rust rewrite for branch `018-module_gnu_GL_ATTRIBUTE_12-rust-port` must preserve the functional behavior evidenced by these sources, with emphasis on:

- producing the formatted trailing portion of an error report from a message template and variadic arguments
- providing hash table storage and lookup behavior centered on `hash_entry` and `hash_table`
- selecting hash table capacities using primality checks and next-prime computation

This specification intentionally limits itself to behavior evidenced by the analyzed files and symbols.

## Feature Specification

### 1. Error message tail formatting and emission

From `gnu/error.c`, the module contains an internal error-reporting routine that accepts:

- an exit status
- an error number
- a message format string
- a variable argument list

The Rust version must implement the same functional role for this module boundary:

- accept a formatted message template with arguments
- emit the message tail in the GNU-style error reporting flow
- incorporate the supplied error number in the same reporting path when nonzero
- honor the supplied status in the same reporting flow, including the possibility that the reporting path terminates processing when a nonzero status is used

This specification does not require exposing the C variadic interface directly; it requires preserving the observable behavior of formatted error-tail reporting.

### 2. Hash table storage behavior

From `gnu/hash.c`, the module defines `hash_entry` and `hash_table` and uses them throughout the file as the core storage model for a generic hash table.

The Rust version must implement the same functional role:

- maintain a table of entries
- support lookup of entries by hash/equality semantics associated with the table
- support insertion into the table
- support removal or vacancy handling as required by the table’s entry model
- preserve the distinction between active entries and non-active slots represented in the original entry structure usage
- maintain table state needed to manage occupancy and sizing

Only behavior evidenced by the presence and repeated use of `hash_entry` and `hash_table` is required; no additional public capabilities are assumed.

### 3. Prime-based capacity selection

From `gnu/hash.c`, the module includes:

- `is_prime(size_t candidate) -> bool`
- `next_prime(size_t candidate) -> size_t`

The Rust version must preserve this sizing behavior:

- determine whether a candidate table size is prime
- compute the next prime size suitable for hash table sizing at or above a candidate
- use prime sizing consistently with the hash table’s capacity management behavior

The Rust rewrite must keep these functions behaviorally correct for all size values used by the table-management logic.

## User Scenarios & Testing

### Scenario 1: Formatted error reporting with message arguments

A caller in the program reports an error using a format string and arguments, optionally with an OS/library error number.

Expected module behavior:

- the formatted message tail is produced correctly from the template and arguments
- when an error number is provided, the report path includes its associated error text in the same overall error output behavior
- when no error number is provided, only the formatted message tail is emitted
- status handling matches the original reporting path semantics

Tests should verify:

- plain message formatting
- formatting with multiple argument types supported by the Rust rewrite
- behavior with `errnum == 0`
- behavior with nonzero `errnum`
- behavior difference between zero and nonzero status, if the Rust design represents termination separately

### Scenario 2: Creating and populating a hash table

A caller initializes a hash table and inserts multiple entries.

Expected module behavior:

- inserted entries become discoverable through lookup
- repeated insertions update table occupancy correctly
- table storage remains valid as the number of entries grows
- table sizing follows the module’s prime-capacity policy

Tests should verify:

- insertion of one entry
- insertion of multiple distinct entries
- successful lookup after insertion
- capacity chosen or grown according to prime-based sizing logic

### Scenario 3: Handling collisions and entry states

A caller inserts keys that map to the same or nearby table positions.

Expected module behavior:

- all reachable inserted entries remain findable according to the table’s equality semantics
- non-active slots and active slots are distinguished correctly
- subsequent operations continue to behave correctly after collisions occur

Tests should verify:

- two or more colliding keys can coexist if distinct
- lookup distinguishes present and absent keys under collision conditions
- entry-state handling does not cause false positives for removed or vacant positions

### Scenario 4: Table growth through next-prime selection

A caller inserts enough entries to require a larger table.

Expected module behavior:

- the table computes a new capacity using next-prime logic
- existing active entries remain reachable after resizing
- occupancy/accounting remains correct after growth

Tests should verify:

- growth is triggered by increased load
- new capacity is prime
- all pre-growth entries remain accessible post-growth

### Scenario 5: Prime helper correctness

A maintainer or internal caller relies on prime helpers for sizing decisions.

Expected module behavior:

- `is_prime` correctly classifies prime and non-prime candidates used in table sizing
- `next_prime` returns a prime not smaller than the requested candidate, consistent with the original sizing purpose

Tests should verify:

- small known primes and composites
- boundary candidates used by table initialization and resizing
- monotonic behavior of `next_prime` over increasing candidate values

## Requirements

### Functional Requirements

#### FR-1: Error tail reporting
The module shall provide the internal error-reporting behavior evidenced by `error_tail` in `gnu/error.c`, accepting a message template plus arguments and producing the formatted trailing portion of an error report.

**Traceability:** `gnu/error.c`, `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD ... error_tail`

#### FR-2: Error-number-aware reporting
The module shall support error reporting behavior that varies based on the supplied error number, such that a nonzero error number participates in the emitted error report and zero does not add error-text content.

**Traceability:** `gnu/error.c`, `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD ... error_tail`

#### FR-3: Status-driven reporting flow
The module shall preserve the status-dependent behavior of the error-reporting path, including the distinction between non-terminating and terminating use as evidenced by the `status` parameter to `error_tail`.

**Traceability:** `gnu/error.c`, `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD ... error_tail`

#### FR-4: Hash table entry storage
The module shall represent hash table contents using an entry-oriented model corresponding to `struct hash_entry` and maintain those entries within a table corresponding to `struct hash_table`.

**Traceability:** `gnu/hash.c`, `struct hash_entry`, `struct hash_table`

#### FR-5: Hash table lookup and insertion behavior
The module shall support core hash table operations required by the table structure usage in `gnu/hash.c`, specifically storing entries and locating matching entries within the table.

**Traceability:** `gnu/hash.c`, `struct hash_entry`, `struct hash_table`

#### FR-6: Entry-state handling
The module shall preserve the functional distinction between active entries and non-active table positions evidenced by repeated `hash_entry` usage throughout `gnu/hash.c`.

**Traceability:** `gnu/hash.c`, `struct hash_entry`, `struct hash_table`

#### FR-7: Capacity sizing by primality
The module shall determine hash table capacities using primality-based sizing logic.

**Traceability:** `gnu/hash.c`, `is_prime`, `next_prime`, `struct hash_table`

#### FR-8: Prime classification
The module shall correctly determine whether a candidate size is prime.

**Traceability:** `gnu/hash.c`, `is_prime`

#### FR-9: Next-prime computation
The module shall compute a prime size at or above a requested candidate for use in table sizing.

**Traceability:** `gnu/hash.c`, `next_prime`

#### FR-10: Resize safety for existing entries
When table capacity changes under the module’s sizing logic, existing active entries shall remain logically present and reachable.

**Traceability:** `gnu/hash.c`, `struct hash_table`, `next_prime`

### Key Entities

#### `hash_entry`
Core per-slot or per-entry record used by the hash table. Its repeated use across `gnu/hash.c` indicates that it represents the state of a table position and participates directly in lookup, insertion, and occupancy management.

**Relationship:** owned or managed by `hash_table`.

**Traceability:** `gnu/hash.c`, `struct hash_entry`

#### `hash_table`
Primary table state object for the generic hash table. It groups the entry storage and the metadata required for hash-based access and sizing decisions.

**Relationship:** contains or references multiple `hash_entry` records; relies on prime-sizing helpers for capacity management.

**Traceability:** `gnu/hash.c`, `struct hash_table`

#### Error-reporting tail operation
Internal formatting-and-emission operation for the trailing portion of an error report.

**Relationship:** consumes status, error number, message template, and arguments to produce observable error output behavior.

**Traceability:** `gnu/error.c`, `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD ... error_tail`

#### Prime-sizing helpers
Internal helper operations that classify primes and select the next prime size.

**Relationship:** support `hash_table` capacity selection and growth behavior.

**Traceability:** `gnu/hash.c`, `is_prime`, `next_prime`

## Success Criteria

1. A Rust test exercising the error-reporting path with formatted arguments produces the same message content as the C module for equivalent inputs.
   - **Traceability:** `gnu/error.c`, `error_tail`

2. A Rust test exercising error reporting with `errnum == 0` and with a nonzero `errnum` demonstrates the same inclusion or omission of error-text content as the C module.

3. A Rust test or equivalent behavioral check demonstrates preservation of the status-dependent reporting flow represented by the `status` parameter.

4. A Rust hash table test confirms that inserted entries can be found afterward and absent entries are not falsely reported as present.
   - **Traceability:** `gnu/hash.c`, `struct hash_entry`, `struct hash_table`

5. A Rust collision test confirms that multiple distinct colliding entries remain retrievable according to table matching semantics.

6. A Rust growth test confirms that after capacity expansion, all previously active entries remain reachable.
   - **Traceability:** `gnu/hash.c`, `struct hash_table`, `next_prime`

7. A Rust unit test suite for prime helpers confirms that `is_prime` correctly classifies representative prime and composite values used by sizing logic.
   - **Traceability:** `gnu/hash.c`, `is_prime`

8. A Rust unit test suite confirms that `next_prime(candidate)` returns a prime result that is not smaller than `candidate`.
   - **Traceability:** `gnu/hash.c`, `next_prime`

9. No functionality beyond the evidenced module scope is required for acceptance.
   - **Traceability:** `gnu/error.c`, `gnu/hash.c`