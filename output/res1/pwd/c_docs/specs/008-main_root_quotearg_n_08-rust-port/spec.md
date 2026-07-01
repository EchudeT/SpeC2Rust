# spec.md

## Overview

This module provides indexed argument quoting services built on the `quotearg.c` quoting subsystem. The analyzed entry points are:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

Its role is to return quoted representations of input arguments, using per-call or temporary quoting options, and to support indexed result slots so callers can request multiple quoted strings without manually managing every output buffer.

The Rust rewrite must preserve the observable behavior of these entry points and the option-driven quoting behavior they rely on.

## Feature Specification

### Purpose

The module produces quoted string representations of input arguments for use by higher-level program logic such as user-visible messages, diagnostics, and command output formatting.

### In-Scope Functionality

The Rust version must implement:

1. Indexed quoting of NUL-terminated string arguments through `quotearg_n`.
2. Indexed quoting of byte sequences with explicit length through `quotearg_n_mem`.
3. Indexed quoting using caller-specified left and right quotation delimiters through `quotearg_n_custom`.
4. Use of quoting configuration represented by `struct quoting_options`.
5. Storage/reuse behavior associated with slot-based results represented by `struct slotvec`, insofar as it affects observable indexed-return behavior.

### Behavioral Summary

- The module accepts an argument plus a slot index `n`.
- It returns a quoted form of that argument.
- Different indices refer to independently usable result slots.
- Memory-sized input is supported so quoting can operate on data that is not limited to a terminating NUL.
- Custom quotation delimiters can be applied for the custom variant.
- Quoting behavior is governed by quoting options, including the custom-quote case.

### Out of Scope

The Rust rewrite spec does not require any capabilities not evidenced by the analyzed module input. In particular, this spec does not introduce new public APIs, persistence, serialization, thread-safety guarantees, FFI requirements, or performance targets.

## User Scenarios & Testing

### Scenario 1: Quote a normal argument by index

A caller needs a quoted form of a standard C-style string and requests it through `quotearg_n` with a chosen slot index.

Expected support:
- The returned value is the quoted representation of the provided argument.
- Reusing the same index remains valid for obtaining that slot’s current quoted output.
- Using different indices allows separate quoted results to be obtained without requiring the caller to merge or manually manage one shared output buffer.

Test focus:
- Quote the same input at index 0 and verify stable quoted content.
- Quote two different inputs at indices 0 and 1 and verify each returned value corresponds to the requested input.

### Scenario 2: Quote data with an explicit byte length

A caller needs to quote input that may contain embedded NUL bytes or is otherwise length-delimited, and uses `quotearg_n_mem`.

Expected support:
- The function processes exactly `argsize` bytes from the input.
- The quoted result reflects the full provided byte range rather than stopping at the first NUL byte.

Test focus:
- Provide a byte buffer containing an embedded NUL and a nonzero `argsize` spanning past it.
- Verify output corresponds to the entire byte sequence covered by `argsize`.

### Scenario 3: Quote using custom delimiters

A caller needs the argument quoted with explicit left and right quote strings and uses `quotearg_n_custom`.

Expected support:
- The returned quoted result uses the caller-supplied left and right quotation delimiters.
- Custom delimiter selection affects quoting behavior for that call.

Test focus:
- Supply distinct left and right quote strings and verify both appear in the result in the expected positions around the quoted argument.

### Scenario 4: Repeated quoting through slot-based indexing

A caller issues multiple quoting requests over time and depends on indexed slot behavior to keep results separated by slot number.

Expected support:
- Slot index selection affects which stored/returned quoted result is associated with the call.
- The Rust implementation preserves externally visible indexed behavior.

Test focus:
- Call quoting functions repeatedly with multiple slot indices.
- Verify each slot yields the quoted content associated with its latest request.

### Scenario 5: Quoting behavior driven by quoting options

A caller indirectly relies on `struct quoting_options` behavior through the provided entry points, especially for custom quoting.

Expected support:
- Option state relevant to the selected quoting mode is applied consistently to the produced output.

Test focus:
- For custom quoting, verify that the effective options reflect the supplied custom delimiters.
- Where default quoting applies, verify output is consistent across repeated calls for the same input and slot usage pattern.

## Requirements

### Functional Requirements

#### FR-1 Indexed argument quoting
The module shall provide a function corresponding to `quotearg_n` that accepts a slot index and a NUL-terminated argument and returns that argument’s quoted representation.

Traceability:
- `quotearg.c`
- `quotearg_n`

#### FR-2 Length-delimited argument quoting
The module shall provide a function corresponding to `quotearg_n_mem` that accepts a slot index, an input pointer/reference, and an explicit byte length, and returns the quoted representation of exactly that byte range.

Traceability:
- `quotearg.c`
- `quotearg_n_mem`

#### FR-3 Custom-delimiter quoting
The module shall provide a function corresponding to `quotearg_n_custom` that accepts a slot index, a left quote string, a right quote string, and an argument, and returns a quoted representation using those delimiters.

Traceability:
- `quotearg.c`
- `quotearg_n_custom`

#### FR-4 Slot-indexed result separation
The module shall preserve observable slot-based behavior associated with indexed quoting requests, so that different slot indices act as distinct result channels for quoted output.

Traceability:
- `quotearg.c`
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`
- `struct slotvec`

#### FR-5 Quoting-options-driven behavior
The module shall apply quoting behavior through the quoting configuration represented by `struct quoting_options`, including behavior needed for the custom quotation case.

Traceability:
- `quotearg.c`
- `quotearg_n_custom`
- `struct quoting_options`

#### FR-6 Support for repeated calls
The module shall support repeated quoting requests across the analyzed entry points while preserving correct quoted output for each call’s input and slot selection.

Traceability:
- `quotearg.c`
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`
- `struct slotvec`

### Key Entities

#### `quoting_options`
This structure represents quoting configuration. It is the policy entity that determines how an input argument is transformed into a quoted output. The custom-quoting entry point relies on this configuration to express caller-supplied quote delimiters.

Relationship:
- Consumed by the quoting operations behind the public entry points.
- Defines behavior applied when producing quoted output.

Traceability:
- `struct quoting_options` entries in `quotearg.c`
- `quotearg_n_custom`

#### `slotvec`
This structure represents slot-based storage or tracking for indexed quoted results. It underpins the distinction between different `n` values passed to the quoting entry points.

Relationship:
- Associates slot index values with stored/returned quoted results.
- Supports repeated use of indexed quoting APIs.

Traceability:
- `struct slotvec` entries in `quotearg.c`
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

## Success Criteria

### SC-1 API-equivalent functional coverage
The Rust module implements behaviorally equivalent support for the three analyzed entry points:
- indexed quoting of NUL-terminated strings,
- indexed quoting of explicit-length byte sequences,
- indexed quoting with custom left/right delimiters.

Measured by:
- Presence of Rust functionality covering each of the three entry-point behaviors.
- Tests exercising one case for each behavior.

Traceability:
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

### SC-2 Correct explicit-length handling
For inputs containing embedded NUL bytes, the Rust version quotes the full byte range specified by the provided length rather than truncating at the first NUL.

Measured by:
- At least one test using `quotearg_n_mem` behavior with embedded NUL data and verifying the result reflects bytes beyond the NUL.

Traceability:
- `quotearg_n_mem`

### SC-3 Correct custom quote application
When custom left and right quote strings are supplied, the Rust version produces output that uses those custom delimiters.

Measured by:
- At least one test that verifies the selected left and right quote strings appear as the effective quotation delimiters in the result.

Traceability:
- `quotearg_n_custom`
- `struct quoting_options`

### SC-4 Indexed slot behavior preservation
The Rust version preserves externally visible separation between different slot indices.

Measured by:
- Tests using at least two distinct indices and verifying that results correspond to the correct slot-specific request pattern.

Traceability:
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`
- `struct slotvec`

### SC-5 Repeat-call consistency
Across repeated invocations of the quoting APIs, the Rust version returns quoted results consistent with the current call inputs and selected slot indices.

Measured by:
- Tests performing repeated calls on the same and different indices and verifying expected outputs after each call.

Traceability:
- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`
- `struct slotvec`