# spec.md

## Title

Functional Specification: `main_root_quoting_options_02`

## Metadata

- Project: `pwd`
- Module: `main_root_quoting_options_02`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Primary function in scope: `quotearg_n_custom_mem`
- Rust branch target: `002-main_root_quoting_options_02-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides argument quoting functionality for byte sequences using caller-supplied left and right quote strings. Its in-scope behavior is represented by `quotearg_n_custom_mem`, which quotes an input memory region of explicit length and returns a quoted result associated with a caller-selected slot number.

The Rust rewrite must preserve the observable behavior of this module boundary:

- accept an argument as a memory buffer plus explicit size,
- apply custom quoting delimiters supplied by the caller,
- use quoting configuration derived from `quoting_options`,
- support selection by numeric slot `n`,
- return the quoted representation for the selected slot.

This specification covers only the functional behavior evidenced by the provided source analysis and does not introduce additional capabilities beyond that boundary.

## Feature Specification

### Feature: Quote a sized memory region with custom delimiters

The module shall provide functionality equivalent to `quotearg_n_custom_mem(int n, char const *left_quote, char const *right_quote, char const *arg, size_t argsize)`.

Behavior in scope:

- The caller provides:
  - a slot index `n`,
  - a left quote string,
  - a right quote string,
  - an input memory region `arg`,
  - the exact input size `argsize`.
- The module produces a quoted form of the input memory region using the provided left and right quote strings.
- The operation uses quoting options represented by `quoting_options`.
- The quoted result is associated with the requested slot index and returned to the caller.

### Feature boundaries

Included:

- quoting of memory buffers using explicit length rather than relying on string termination,
- use of caller-provided left and right quote delimiters,
- interaction with quoting configuration data,
- slot-based result selection and retrieval.

Not included unless directly required to preserve this function’s behavior:

- unrelated quoting styles not evidenced by this entry point,
- new public APIs,
- concurrency guarantees,
- persistence, serialization, or external protocol behavior.

## User Scenarios & Testing

### Scenario 1: Quote a simple byte string with custom delimiters

A caller has an input buffer representing a name or path fragment and wants the returned text wrapped in specific quote markers.

- Input:
  - slot index `n`
  - left quote such as `<<`
  - right quote such as `>>`
  - buffer containing the input bytes
  - explicit buffer length
- Expected behavior:
  - the module returns a quoted representation using the supplied delimiters,
  - the returned content corresponds to the exact provided byte length.

Test focus:

- verify the returned result includes the specified left and right quote strings,
- verify the entire input region of `argsize` bytes is processed.

### Scenario 2: Quote input containing embedded NUL or non-terminated data

A caller has a memory region that is not a conventional C string or contains embedded zero bytes and needs quoting based on the provided size.

- Input:
  - any valid slot index
  - custom left/right quote strings
  - buffer containing bytes with embedded NUL or trailing data beyond `argsize`
  - explicit length smaller than the physical allocation
- Expected behavior:
  - quoting uses only the first `argsize` bytes,
  - no dependence on NUL termination is required for correctness.

Test focus:

- verify output is determined by the explicit size, not by the first NUL byte or later bytes in the allocation.

### Scenario 3: Reuse different slots for separate quoted results

A caller needs multiple quoted outputs to coexist logically by selecting different slot indices.

- Input:
  - multiple calls with different `n` values
  - distinct input buffers and/or delimiters
- Expected behavior:
  - each call returns the quoted result for the requested slot,
  - slot selection affects which stored/returned quoted result is used.

Test focus:

- verify calls using different slot indices can be made without collapsing all results into one indistinguishable slot outcome.

### Scenario 4: Apply quoting through quoting options configured for custom style

A caller relies on the module’s `quoting_options` handling to perform custom quoting with the provided delimiters.

- Input:
  - slot index
  - custom left/right quote strings
  - arbitrary byte buffer and size
- Expected behavior:
  - the result reflects quoting behavior driven by `quoting_options`,
  - the custom delimiters provided to the call are the ones used for the returned quoted form.

Test focus:

- verify the operation is consistent with configuration represented by `quoting_options`,
- verify caller-supplied delimiters are honored.

## Requirements

### Functional Requirements

#### FR-1: Explicit-length input handling

The Rust module shall accept an input argument as a byte region plus explicit length and shall base quoting on that explicit length.

Traceability:

- `quotearg.c`
- `quotearg_n_custom_mem`

#### FR-2: Custom quote delimiters

The Rust module shall support caller-specified left and right quote strings for the quoting operation.

Traceability:

- `quotearg.c`
- `quotearg_n_custom_mem`

#### FR-3: Slot-indexed result selection

The Rust module shall support selection of quoted output by numeric slot index `n`, matching the role of the source function’s first parameter.

Traceability:

- `quotearg.c`
- `quotearg_n_custom_mem`
- `slotvec`

#### FR-4: Quoting options participation

The Rust module shall preserve behavior in which quoting is governed by `quoting_options` data used by this module.

Traceability:

- `quotearg.c`
- `quoting_options`
- `quotearg_n_custom_mem`

#### FR-5: Returned quoted representation

The Rust module shall return the quoted representation corresponding to the requested slot and supplied input.

Traceability:

- `quotearg.c`
- `quotearg_n_custom_mem`
- `slotvec`

### Key Entities

#### `quoting_options`

Configuration data representing how quoting is performed. Within this module, it is the options structure through which quoting behavior is determined for the custom-memory quoting operation.

Relationship to other entities:

- guides the behavior of `quotearg_n_custom_mem`,
- works with caller-supplied left and right quote strings to determine the produced quoted form.

Traceability:

- `quotearg.c`
- `struct quoting_options` references throughout the file

#### `slotvec`

Slot-oriented storage associated with numeric slot selection for quoted results.

Relationship to other entities:

- supports the slot parameter `n`,
- holds or indexes data used to return the quoted result associated with a slot.

Traceability:

- `quotearg.c`
- `struct slotvec` references near lines 829-845 and related uses

#### Quoted result

The returned quoted representation produced from the input memory region, explicit size, custom delimiters, and quoting options.

Relationship to other entities:

- produced by `quotearg_n_custom_mem`,
- determined by `quoting_options`,
- associated with a `slotvec` entry selected by `n`.

Traceability:

- `quotearg.c`
- `quotearg_n_custom_mem`

## Success Criteria

### SC-1: Correct delimiter application

For calls corresponding to `quotearg_n_custom_mem`, the Rust version applies the provided left and right quote strings to the returned quoted representation.

Traceability:

- `quotearg_n_custom_mem`

### SC-2: Correct explicit-size behavior

For test cases where the input contains embedded NUL bytes or is not NUL-terminated, the Rust version’s output is determined by `argsize` and not by C-string termination behavior.

Traceability:

- `quotearg_n_custom_mem`

### SC-3: Slot-based behavior preserved

For test cases using different slot indices, the Rust version preserves observable slot-based result selection consistent with the source module’s `n` parameter and `slotvec` usage.

Traceability:

- `quotearg_n_custom_mem`
- `slotvec`

### SC-4: Quoting options role preserved

The Rust version retains the module behavior in which quoting is controlled through `quoting_options` for this custom-memory quoting path.

Traceability:

- `quoting_options`
- `quotearg_n_custom_mem`

### SC-5: Functional equivalence at module boundary

For representative inputs covering simple text, explicit-length binary-like buffers, and multiple slot selections, the Rust rewrite produces the same externally observable quoting results as the C module entry point in scope.

Traceability:

- `quotearg.c`
- `quotearg_n_custom_mem`