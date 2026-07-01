# spec.md

## Title

Functional Specification: `main_root_quoting_options_02`

## Metadata

- Project: `cat`
- Module: `main_root_quoting_options_02`
- Category: `main_cluster`
- Source basis: `quotearg.c`
- Primary function in scope: `quotearg_n_custom_mem`
- Primary data types in scope: `quoting_options`, `slotvec`
- Rust branch target: `003-main_root_quoting_options_02-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides quoting behavior configuration for argument formatting, centered on producing a quoted representation of a memory-backed argument using caller-supplied left and right quote strings.

The Rust rewrite must preserve the module behavior evidenced by `quotearg.c` for:

- creating or deriving quoting behavior from `quoting_options`
- applying custom quote delimiters
- quoting an input argument supplied with an explicit memory length
- supporting indexed quote-result storage behavior associated with `n`
- returning a quoted string result consistent with the selected quoting options

The specification is limited to functionality evidenced by the provided source analysis and does not add new capabilities beyond that scope.

## Feature Specification

### Feature: Custom-delimited quoting of memory arguments

The module formats an input argument into a quoted string using:

- an argument slot index `n`
- a left quote delimiter
- a right quote delimiter
- an input character buffer `arg`
- an explicit input size `argsize`

The Rust version must support quoting where the input is treated as a memory region of known length rather than relying on NUL termination.

### Feature: Quoting behavior controlled by options

The module uses `quoting_options` as the controlling configuration for how quoting is applied. For the in-scope function, the relevant behavior is that custom left and right quote delimiters are incorporated into the quoting configuration used for formatting.

The Rust rewrite must preserve the ability for this function to operate through quoting options rather than bypassing option-based behavior.

### Feature: Indexed result handling

The presence of the `n` parameter and the `slotvec` structure indicates that quoted results are managed in indexed slots. The Rust version must preserve observable behavior that allows repeated calls with an index to obtain a quoted result associated with that index.

This requirement is about functional behavior, not about reproducing the C memory layout.

## User Scenarios & Testing

### Scenario 1: Quote a byte sequence with custom delimiters

A caller has an input buffer and wants it rendered with a caller-defined opening and closing quote string.

- Input:
  - slot index `n`
  - left quote such as `<<`
  - right quote such as `>>`
  - argument bytes
  - explicit byte length
- Expected behavior:
  - the returned string is the quoted form of exactly the provided byte range
  - the output uses the provided left and right delimiters

### Scenario 2: Quote data containing embedded NUL or non-terminated memory

A caller has data that must be quoted based on a provided size, not on C-string termination.

- Input:
  - a pointer to memory
  - `argsize` defining the full region to quote
- Expected behavior:
  - processing covers the exact supplied length
  - embedded terminators do not truncate the logical input range

### Scenario 3: Reuse different result slots

A caller performs multiple quoting operations and uses different slot indices.

- Input:
  - repeated calls with different `n` values
- Expected behavior:
  - each call yields the quoted result for the requested slot index
  - use of one slot index does not corrupt the observable result for a different slot index

### Scenario 4: Apply custom quoting through option-based behavior

A caller relies on this function to set up custom delimiters through the quoting options mechanism rather than through a separate formatting path.

- Expected behavior:
  - custom delimiters are reflected in the resulting quoted output
  - behavior remains aligned with the module’s quoting-options model

### Testing guidance

The Rust version must be tested with cases covering:

- empty input with non-empty custom delimiters
- non-empty input with single-character delimiters
- non-empty input with multi-character delimiters
- input containing embedded `\0` bytes
- repeated calls using the same slot index
- repeated calls using different slot indices
- distinct custom delimiter pairs across calls

## Requirements

### Functional Requirements

#### FR-1: Quote explicit-length input
Traceability: `quotearg.c`, `quotearg_n_custom_mem`

The module shall accept an input argument together with an explicit size and produce a quoted string from that exact memory extent.

#### FR-2: Support caller-supplied left and right quote delimiters
Traceability: `quotearg.c`, `quotearg_n_custom_mem`

The module shall allow the caller to specify both the opening and closing quote strings used in the output.

#### FR-3: Use quoting options as the controlling configuration
Traceability: `quotearg.c`, `quoting_options`, `quotearg_n_custom_mem`

The module shall apply quoting through a `quoting_options`-based configuration model, including custom quote delimiter settings used by the in-scope function.

#### FR-4: Support indexed quote-result selection
Traceability: `quotearg.c`, `quotearg_n_custom_mem`, `slotvec`

The module shall support the function’s indexed operation model via parameter `n`, such that quoted results are associated with a requested slot index.

#### FR-5: Return the quoted result as a string output
Traceability: `quotearg.c`, `quotearg_n_custom_mem`

The module shall provide the quoted representation as a string result corresponding to the input argument and the active custom delimiters.

### Key Entities

#### `quoting_options`
Traceability: `quotearg.c`

Represents the quoting configuration used to control how an argument is rendered. In this module’s evidenced scope, it is the entity through which custom left and right quote strings are applied before producing the quoted result.

Relationship:
- used by the quoting function to determine formatting behavior

#### `slotvec`
Traceability: `quotearg.c`

Represents indexed storage associated with quote result handling. Its presence supports the slot-oriented behavior implied by parameter `n`.

Relationship:
- provides the indexed result association used by quoting operations
- works alongside quoting behavior configured through `quoting_options`

## Success Criteria

### SC-1: Exact-length input handling
Traceability: `quotearg.c`, `quotearg_n_custom_mem`

Given input memory and `argsize`, the Rust version produces output based on exactly that byte range, including cases where the memory contains embedded NUL bytes.

### SC-2: Custom delimiters appear in output
Traceability: `quotearg.c`, `quotearg_n_custom_mem`

For test cases using distinct left and right quote strings, the returned quoted string includes the specified opening and closing delimiters in the correct positions.

### SC-3: Option-based custom quoting is preserved
Traceability: `quotearg.c`, `quoting_options`, `quotearg_n_custom_mem`

Tests exercising the in-scope function confirm that custom quote delimiters are applied through the module’s quoting-options behavior rather than being ignored or replaced by fixed delimiters.

### SC-4: Indexed behavior remains correct across repeated calls
Traceability: `quotearg.c`, `quotearg_n_custom_mem`, `slotvec`

Repeated calls using one or more slot indices return correct quoted outputs for each requested index without observable cross-slot corruption.

### SC-5: Empty and non-empty inputs are both handled
Traceability: `quotearg.c`, `quotearg_n_custom_mem`

The Rust version returns valid quoted output for:
- zero-length input
- ordinary non-empty input
- inputs quoted with multi-character custom delimiters

## Scope Notes

This specification covers only the functionality evidenced by the provided analysis for `quotearg.c`, especially `quotearg_n_custom_mem` and its use of `quoting_options` and slot-based result handling. It does not require additional public APIs or capabilities not supported by the input evidence.