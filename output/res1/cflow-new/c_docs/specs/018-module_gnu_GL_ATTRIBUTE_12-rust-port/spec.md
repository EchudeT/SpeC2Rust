# spec.md

## Title

Rust Functional Specification for `module_gnu_GL_ATTRIBUTE_12`

## Overview

This module cluster covers two evidenced responsibilities from `gnu/error.c` and `gnu/hash.c`:

- formatted error-tail reporting behavior centered on `error_tail`
- hash-table support behavior centered on hash table entry/table structures and prime-capacity selection helpers (`is_prime`, `next_prime`)

The Rust rewrite on branch `018-module_gnu_GL_ATTRIBUTE_12-rust-port` must preserve the functional behavior represented by these responsibilities. The specification is limited to behavior evidenced by the analyzed files and symbols and does not introduce new capabilities beyond them.

## Scope

Included in scope:

- producing the trailing portion of an error report from a format string, variadic arguments, status, and optional error number
- representing hash table state and entries
- selecting prime table sizes for hash-table sizing or growth decisions

Out of scope:

- any public API surface not evidenced by the analyzed module files
- concurrency guarantees
- persistence, serialization, or network behavior
- FFI requirements
- performance targets beyond functional correctness

## Feature Specification

### Feature 1: Error-tail reporting

The module must provide behavior equivalent to the `error_tail` function in `gnu/error.c`.

Observed function role:

- accepts `status`, `errnum`, `message`, and formatted arguments
- requires a non-null message string
- emits the trailing part of an error message using printf-style formatting
- incorporates system error information when an error number is supplied
- may terminate processing when status indicates termination is required

Required Rust behavior:

- accept an already selected message template and formatting arguments
- produce formatted message text corresponding to the template and arguments
- when an error number is present and non-zero, append or include the associated system error text as part of the error output behavior
- honor the status argument’s control effect, including termination behavior when the original C behavior would exit after reporting
- reject or prevent null-message equivalents at the Rust boundary

This feature is limited to the evidenced reporting tail behavior and must not be expanded into a broader logging framework.

### Feature 2: Hash table representation and entry relationships

The module must provide Rust equivalents for the hash table entities evidenced in `gnu/hash.c`:

- `hash_entry`
- `hash_table`

Observed entity role:

- `hash_entry` represents one stored position within the hash structure
- `hash_table` holds table-wide state and references to entries, including capacity-related state
- the table relies on entry relationships and table metadata to manage occupancy and traversal/state updates

Required Rust behavior:

- represent individual entries and table-wide state distinctly
- preserve the relationship that a table owns or manages many entries
- preserve enough table metadata to support prime-capacity decisions and normal table-state transitions evidenced by the source file
- preserve behaviorally meaningful distinctions present in the C structures, including occupied-versus-non-occupied entry state if such state is required by table operations in the file

This specification does not require reproducing the C memory layout exactly, only the functional entity boundaries and relationships.

### Feature 3: Prime-capacity selection for hash tables

The module must provide behavior equivalent to `is_prime` and `next_prime` in `gnu/hash.c`.

Observed function role:

- determine whether a candidate size is prime
- return the next prime size at or above a candidate threshold for hash-table sizing decisions

Required Rust behavior:

- correctly classify candidate sizes for primality for the range of sizes supported by the Rust port
- return a prime size that is not smaller than the requested candidate
- ensure returned sizes are suitable for use as hash-table capacities consistent with the C module’s sizing intent

The Rust rewrite must preserve the functional link between these helpers and hash-table capacity management.

## User Scenarios & Testing

### Scenario 1: Format and emit an error message without a system error

A caller supplies:

- a message template
- formatting arguments
- `errnum` equivalent to no system error
- `status` indicating reporting only

Expected behavior:

- the message is formatted using the supplied arguments
- no system error text is included
- reporting completes without termination

Testing focus:

- formatted substitutions appear correctly
- absence of system error text when no error number is present
- no exit/termination side effect for non-terminating status

Traceability:

- `gnu/error.c`, `error_tail`

### Scenario 2: Format and emit an error message with a system error

A caller supplies:

- a message template
- formatting arguments
- a non-zero error number
- non-terminating status

Expected behavior:

- the message is formatted
- corresponding system error text is included in the resulting error report
- reporting completes without termination

Testing focus:

- message text remains intact
- system error component is included only when requested
- behavior is deterministic for a known error number on the target platform

Traceability:

- `gnu/error.c`, `error_tail`

### Scenario 3: Report an error and terminate when status requires it

A caller supplies a valid message and arguments with a status value that in the C behavior triggers process termination after reporting.

Expected behavior:

- the error report is produced before termination
- the module performs the same control-flow effect expected from the original function’s status handling

Testing focus:

- termination path is observable and testable
- output is produced before termination effect

Traceability:

- `gnu/error.c`, `error_tail`

### Scenario 4: Check whether candidate capacities are prime

A caller evaluates multiple candidate table sizes.

Expected behavior:

- prime candidates are identified as prime
- composite candidates are identified as non-prime
- boundary values relevant to size-based capacity logic behave correctly

Testing focus:

- known small primes and composites
- edge candidates near minimum valid table sizes
- consistency across repeated calls

Traceability:

- `gnu/hash.c`, `is_prime`

### Scenario 5: Select the next usable prime capacity

A caller requests a capacity at or above a target size for table creation or growth.

Expected behavior:

- returned value is prime
- returned value is greater than or equal to the requested candidate
- if the candidate is already prime, the same candidate may be returned

Testing focus:

- composite input returns a larger prime
- prime input returns that prime
- monotonicity with increasing candidate sizes

Traceability:

- `gnu/hash.c`, `next_prime`

### Scenario 6: Maintain hash-table state through entry ownership

A caller creates or uses a hash table instance that manages multiple entries.

Expected behavior:

- the table structure holds entry-related state consistently
- capacity-related state remains compatible with prime-capacity selection
- entry state and table state remain synchronized during normal use

Testing focus:

- table can be initialized with coherent metadata
- entry collections are associated with exactly one table state container
- capacity metadata can be validated against prime-selection helpers

Traceability:

- `gnu/hash.c`, `struct hash_entry`, `struct hash_table`

## Requirements

### Functional Requirements

#### FR-1: Formatted error-tail generation
The Rust module shall implement error-tail reporting behavior that accepts a status indicator, an optional system error number, a non-null message template, and formatting arguments, and produces the corresponding formatted error report content.

Traceability:

- `gnu/error.c`, `error_tail`

#### FR-2: Conditional inclusion of system error text
The Rust module shall include system error text in the error report only when a non-zero or otherwise present error number is supplied.

Traceability:

- `gnu/error.c`, `error_tail`

#### FR-3: Status-controlled termination behavior
The Rust module shall preserve the control-flow effect of the `status` parameter used by the original error-tail logic, including termination after reporting when status requires it.

Traceability:

- `gnu/error.c`, `error_tail`

#### FR-4: Non-null message requirement
The Rust module shall enforce the original function’s requirement that the message input be present and valid.

Traceability:

- `gnu/error.c`, `error_tail`, `_GL_ARG_NONNULL ((3))`

#### FR-5: Hash entry representation
The Rust module shall represent hash entries as a distinct entity corresponding to `struct hash_entry`.

Traceability:

- `gnu/hash.c`, `struct hash_entry`

#### FR-6: Hash table representation
The Rust module shall represent hash-table state as a distinct entity corresponding to `struct hash_table`, with a relationship to managed entries.

Traceability:

- `gnu/hash.c`, `struct hash_table`, `struct hash_entry`

#### FR-7: Prime classification
The Rust module shall implement primality testing for size candidates consistent with the behavior of `is_prime`.

Traceability:

- `gnu/hash.c`, `is_prime`

#### FR-8: Next-prime selection
The Rust module shall implement next-prime selection for size candidates consistent with the behavior of `next_prime`, returning a prime not less than the candidate.

Traceability:

- `gnu/hash.c`, `next_prime`

#### FR-9: Capacity-selection compatibility
The Rust module shall keep hash-table capacity state compatible with prime-based sizing decisions represented by the prime helper functions.

Traceability:

- `gnu/hash.c`, `struct hash_table`, `is_prime`, `next_prime`

### Key Entities

#### Entity: Error-tail reporting operation
A reporting operation corresponding to `error_tail` consumes:

- status
- optional system error number
- message template
- formatting arguments

Relationship to other entities:

- independent from hash-table entities
- produces error report output and may trigger termination depending on status

Traceability:

- `gnu/error.c`, `error_tail`

#### Entity: `hash_entry`
Represents one entry slot or stored element state within the hash-table subsystem.

Relationship to other entities:

- managed by a `hash_table`
- participates in table occupancy/storage state

Traceability:

- `gnu/hash.c`, `struct hash_entry`

#### Entity: `hash_table`
Represents the overall hash-table state container.

Relationship to other entities:

- aggregates or manages multiple `hash_entry` instances
- relies on size/capacity values that must remain compatible with prime-capacity helpers

Traceability:

- `gnu/hash.c`, `struct hash_table`, `struct hash_entry`

#### Entity: Prime-capacity helper logic
Represents the sizing logic embodied by `is_prime` and `next_prime`.

Relationship to other entities:

- used to validate or determine `hash_table` capacities
- operates on size-type candidates and returns classification or selected capacity values

Traceability:

- `gnu/hash.c`, `is_prime`, `next_prime`

## Success Criteria

### SC-1: Correct formatted reporting without system error
Given a valid message template, arguments, and no system error number, the Rust module produces the expected formatted error report and does not include system error text.

Traceability:

- FR-1, FR-2
- `gnu/error.c`, `error_tail`

### SC-2: Correct formatted reporting with system error
Given a valid message template, arguments, and a known system error number, the Rust module produces an error report that includes both the formatted message and the corresponding system error text.

Traceability:

- FR-1, FR-2
- `gnu/error.c`, `error_tail`

### SC-3: Correct termination behavior
When invoked with a status value that requires termination, the Rust module reports the error and then exhibits the expected termination control flow.

Traceability:

- FR-3
- `gnu/error.c`, `error_tail`

### SC-4: Message validity enforcement
The Rust module prevents invocation of the error-tail behavior with an absent or invalid message input.

Traceability:

- FR-4
- `gnu/error.c`, `error_tail`

### SC-5: Accurate primality classification
For a test set of known prime and composite size values, the Rust implementation classifies each candidate identically to the original module’s intended prime-testing behavior.

Traceability:

- FR-7
- `gnu/hash.c`, `is_prime`

### SC-6: Accurate next-prime selection
For a test set of candidate sizes, the Rust implementation always returns a prime value that is greater than or equal to the candidate, and returns the candidate itself when the candidate is prime.

Traceability:

- FR-8
- `gnu/hash.c`, `next_prime`

### SC-7: Coherent table-entry modeling
The Rust implementation provides distinct hash-table and hash-entry entities, and tests can verify that table state manages associated entries coherently.

Traceability:

- FR-5, FR-6
- `gnu/hash.c`, `struct hash_entry`, `struct hash_table`

### SC-8: Prime-compatible capacity state
Tests that construct or evolve hash-table capacity state can verify that the state remains compatible with the prime-capacity helper logic.

Traceability:

- FR-9
- `gnu/hash.c`, `struct hash_table`, `is_prime`, `next_prime`