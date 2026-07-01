# spec.md

## Title

Rust Functional Specification for `module_gnu_GL_ATTRIBUTE_12`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_GL_ATTRIBUTE_12`
- Category: `module_cluster`
- Source files:
  - `gnu/error.c`
  - `gnu/hash.c`
- Rust branch: `018-module_gnu_GL_ATTRIBUTE_12-rust-port`
- Generation date: `2026-06-17`

## Overview

This module combines two distinct but related utility areas drawn from GNU-style support code:

1. error-report tail handling for formatted diagnostic emission, including optional system error text and process termination behavior; and
2. generic hash table support, including internal table sizing based on prime-number selection.

The Rust rewrite must preserve the observable behavior provided by these source files and functions, with emphasis on:

- formatting and finishing diagnostic output consistent with the C module’s error-report tail path in `gnu/error.c`;
- hash table behavior driven by the `hash_table` / `hash_entry` model in `gnu/hash.c`;
- prime-based table sizing via `is_prime` and `next_prime` in `gnu/hash.c`.

## Feature Specification

### 1. Diagnostic tail emission

The module must support the behavior represented by `error_tail` in `gnu/error.c`.

This functionality is responsible for completing an error report from:

- an optional formatted message,
- a variable argument list for that message,
- an optional system error number,
- and an optional termination status.

The Rust version must implement equivalent behavior at the functional boundary:

- consume a format string plus arguments to produce the message body;
- when an error number is supplied, include the corresponding system error text in the final diagnostic;
- terminate the diagnostic line consistently;
- when a nonzero status is supplied, perform the same logical “emit then terminate” behavior as the C path.

The Rust version must preserve the distinction between:

- reporting only a caller-supplied message, and
- reporting a caller-supplied message augmented with system error information.

### 2. Generic hash table behavior

The module must support the hash table functionality evidenced by `gnu/hash.c` and its core entities `hash_table` and `hash_entry`.

The Rust version must implement a generic table that supports the same functional role:

- storing entries in hashed buckets or chains,
- locating an existing entry from a lookup key using caller-defined hashing and comparison behavior,
- inserting entries,
- removing entries,
- iterating or traversing entries as needed by the table logic,
- and managing table size changes in response to capacity needs.

The specification is limited to functionality evidenced by the module analysis: a generic hash table with entry management and prime-sized capacity selection. The Rust rewrite must not invent unrelated container features.

### 3. Prime-based capacity selection

The module must support internal prime-number utilities represented by:

- `is_prime` in `gnu/hash.c`
- `next_prime` in `gnu/hash.c`

The Rust version must provide equivalent behavior for table sizing decisions:

- determine whether a candidate size is prime;
- choose the next prime at or above a candidate value for hash table sizing.

This behavior is part of the hash table’s functional correctness, because table capacities are selected according to prime-number rules rather than arbitrary growth alone.

## User Scenarios & Testing

### Scenario 1: Emit a formatted diagnostic without system error text

A caller reports a diagnostic message with formatting arguments and no associated system error number.

Expected behavior:

- the formatted message is emitted correctly;
- no system error description is appended;
- output is properly terminated;
- no termination occurs when status indicates nonfatal reporting.

Test evidence target:
- `gnu/error.c`, `error_tail`

### Scenario 2: Emit a formatted diagnostic with system error text

A caller reports a diagnostic tied to a specific OS error value.

Expected behavior:

- the formatted message is emitted correctly;
- the system error text for the provided error number is appended in the same diagnostic;
- output remains well-formed and terminated.

Test evidence target:
- `gnu/error.c`, `error_tail`

### Scenario 3: Emit a fatal diagnostic

A caller reports an error with a nonzero termination status.

Expected behavior:

- the diagnostic is emitted before termination behavior is triggered;
- the Rust port preserves the logical fatal/nonfatal distinction of the C module.

Test evidence target:
- `gnu/error.c`, `error_tail`

### Scenario 4: Create and use a hash table for insertion and lookup

A caller uses the hash table to store items and later retrieve them by equivalent keys.

Expected behavior:

- inserted entries become discoverable by lookup;
- lookup uses the table’s configured hashing and equality behavior;
- existing entries are distinguishable from absent keys.

Test evidence target:
- `gnu/hash.c`, `hash_table`, `hash_entry`

### Scenario 5: Remove entries from the hash table

A caller deletes a previously inserted key.

Expected behavior:

- the matching entry is removed;
- later lookup for that key reports absence;
- other entries remain accessible.

Test evidence target:
- `gnu/hash.c`, `hash_table`, `hash_entry`

### Scenario 6: Grow or resize the hash table using prime capacities

A caller inserts enough entries that the table must change capacity.

Expected behavior:

- the replacement capacity follows the module’s prime-based sizing rule;
- existing entries remain reachable after resizing;
- no entries are lost or spuriously duplicated.

Test evidence target:
- `gnu/hash.c`, `is_prime`, `next_prime`, `hash_table`

### Scenario 7: Prime helper correctness at boundary values

Internal sizing logic evaluates candidate capacities including small and non-prime values.

Expected behavior:

- primality checks correctly reject non-primes and accept primes;
- next-prime selection returns a prime not smaller than the candidate.

Test evidence target:
- `gnu/hash.c`, `is_prime`, `next_prime`

## Requirements

### Functional Requirements

#### FR-1: Formatted diagnostic completion
The module shall format and emit the trailing portion of an error report from a message template and argument list.

Traceability:
- `gnu/error.c`
- `error_tail`

#### FR-2: Optional inclusion of system error text
When given a nonzero or otherwise valid error number for reporting, the module shall include corresponding system error text in the emitted diagnostic.

Traceability:
- `gnu/error.c`
- `error_tail`

#### FR-3: Fatal vs. nonfatal reporting behavior
The module shall preserve the behavioral distinction between nonfatal diagnostic emission and diagnostic emission followed by termination when a nonzero status is supplied.

Traceability:
- `gnu/error.c`
- `error_tail`

#### FR-4: Hash-based storage of entries
The module shall maintain a hash table that stores entries and organizes them by hashed key placement.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `hash_entry`

#### FR-5: Key-based lookup
The module shall support locating an existing entry by key using the table’s configured hash/equality semantics.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `hash_entry`

#### FR-6: Entry insertion
The module shall support insertion of entries into the hash table.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `hash_entry`

#### FR-7: Entry removal
The module shall support removal of entries from the hash table while preserving access to remaining entries.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `hash_entry`

#### FR-8: Capacity management by prime sizing
The module shall choose hash table capacities according to prime-number selection logic rather than arbitrary size values.

Traceability:
- `gnu/hash.c`
- `is_prime`
- `next_prime`
- `hash_table`

#### FR-9: Prime candidate testing
The module shall determine whether a candidate table size is prime.

Traceability:
- `gnu/hash.c`
- `is_prime`

#### FR-10: Next-prime selection
The module shall compute a prime size greater than or equal to a requested candidate value for use in hash table sizing.

Traceability:
- `gnu/hash.c`
- `next_prime`

#### FR-11: Entry preservation across resizing
When the hash table changes capacity, the module shall preserve the logical set of stored entries and their retrievability.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `hash_entry`
- `next_prime`

### Key Entities

#### `hash_entry`
A hash table entry node representing one stored item and its linkage within the table structure.

Role:
- unit of storage inside the table;
- participates in collision handling and traversal within the table.

Traceability:
- `gnu/hash.c`
- `struct hash_entry`

#### `hash_table`
The owning structure that manages table state, entry organization, and capacity decisions.

Role:
- holds the collection of entries;
- applies hashing and lookup semantics;
- coordinates insertion, removal, search, and resizing;
- uses prime-based size selection.

Traceability:
- `gnu/hash.c`
- `struct hash_table`

#### Prime-sizing helpers
Internal value-oriented helpers used to validate and choose table capacities.

Included entities:
- `is_prime`
- `next_prime`

Role:
- ensure table sizes follow the module’s prime-number rule.

Traceability:
- `gnu/hash.c`

#### Diagnostic tail handler
The internal diagnostic-finishing routine for formatted error reporting.

Included entity:
- `error_tail`

Role:
- completes message formatting,
- appends system error text when applicable,
- and triggers termination behavior for fatal reports.

Traceability:
- `gnu/error.c`

## Success Criteria

### SC-1: Diagnostic formatting parity
Tests demonstrate that formatted diagnostics produced by the Rust module preserve the same observable distinctions as the C module between:
- message-only output, and
- message-plus-system-error output.

Traceability:
- `gnu/error.c`
- `error_tail`

### SC-2: Fatal-path behavior parity
Tests demonstrate that when a nonzero status is used, the Rust module performs “emit then terminate” behavior equivalent at the functional level to the C module, and when status is nonfatal it does not terminate.

Traceability:
- `gnu/error.c`
- `error_tail`

### SC-3: Hash insertion/lookup correctness
For representative key sets, every inserted entry is retrievable by equivalent key, and absent keys are not falsely reported as present.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `hash_entry`

### SC-4: Hash removal correctness
For representative key sets, removing an entry makes it unreachable by lookup without breaking retrieval of remaining entries.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `hash_entry`

### SC-5: Resize preservation
Tests that force capacity growth show that all entries present before resizing remain retrievable afterward.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `next_prime`

### SC-6: Prime helper correctness
Unit tests verify that:
- `is_prime` accepts known prime inputs and rejects known non-prime inputs; and
- `next_prime` returns a prime value not less than the candidate input.

Traceability:
- `gnu/hash.c`
- `is_prime`
- `next_prime`

### SC-7: Prime-capacity rule use
Tests or implementation-level verification show that hash table capacities selected during initialization or growth conform to the module’s prime-based sizing rule.

Traceability:
- `gnu/hash.c`
- `hash_table`
- `is_prime`
- `next_prime`