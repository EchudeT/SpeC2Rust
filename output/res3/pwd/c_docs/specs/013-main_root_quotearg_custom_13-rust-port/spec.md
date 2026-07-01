# spec.md

## Title

Functional Specification: `main_root_quotearg_custom_13`

## Summary

This module provides custom-quote argument formatting entry points from `quotearg.c` for the `pwd` project. Its exposed behavior is to produce a quoted representation of an input argument using caller-supplied left and right quote strings.

The Rust rewrite must preserve the functional behavior evidenced by:

- `quotearg_custom`
- `quotearg_custom_mem`

and the related option state represented by:

- `struct quoting_options`
- `struct slotvec`

## Scope

In scope for this module:

- Accepting caller-provided left and right quote delimiters.
- Quoting a NUL-terminated input string.
- Quoting a byte sequence with an explicit length.
- Applying custom quoting through the module’s quoting option state.
- Returning a quoted result through the same functional boundary as the C module.

Out of scope:

- Defining new quoting styles or public APIs not evidenced by the module input.
- Extending behavior beyond the custom-quote entry points and their immediately related option/state handling.
- Adding guarantees not stated by the analyzed C module.

## Feature Specification

### Feature: Custom-quoted argument rendering

The module renders an input argument as a quoted string using a caller-specified opening quote and closing quote.

Two input forms are supported:

1. A conventional string argument.
2. A memory buffer paired with an explicit byte length.

For both forms, the module must apply custom quote delimiters taken from the call arguments rather than fixed built-in delimiters.

### Feature: Explicit-length input handling

The module supports quoting data provided with a byte count. The Rust version must preserve the distinction between:

- quoting text read as a conventional string, and
- quoting bytes limited by an explicit `argsize`.

This distinction is required because the module exposes both `quotearg_custom` and `quotearg_custom_mem`.

### Feature: Quoting option application

The custom-quote operations are mediated by quoting option state represented by `struct quoting_options`. The Rust version must preserve the behavior whereby custom left and right quote strings are applied through module option configuration associated with the custom quoting path.

### Feature: Reusable result storage behavior

The presence of `struct slotvec` in the same module indicates that quoted results are produced through module-managed result storage rather than requiring the caller to supply an output buffer. The Rust rewrite must preserve the observable behavior of returning a quoted result from the custom quoting entry points.

## User Scenarios & Testing

### Scenario 1: Quote a regular argument with custom delimiters

A caller has a normal C-style string argument and wants it rendered using specific opening and closing quote strings.

Example scenario:

- left quote: `<<`
- right quote: `>>`
- argument: `abc`

Expected supported behavior:

- the module returns a quoted representation of the argument using `<<` and `>>`.

Testing focus:

- verify that the output begins with the supplied left quote,
- verify that the output ends with the supplied right quote,
- verify that the argument content is represented between them.

### Scenario 2: Quote a byte buffer with custom delimiters

A caller has input data and an explicit length and wants only that many bytes considered for quoting.

Example scenario:

- left quote: `[`
- right quote: `]`
- buffer: bytes containing `abcXYZ`
- size: `3`

Expected supported behavior:

- the module quotes only the first 3 bytes of the provided buffer.

Testing focus:

- verify that output uses `[` and `]`,
- verify that only the prefix limited by `argsize` is included in the quoted result.

### Scenario 3: Distinguish string-input and explicit-length behavior

A caller uses both entry points with the same underlying data and expects behavior to follow the chosen input model.

Expected supported behavior:

- `quotearg_custom` treats the input as a conventional string argument,
- `quotearg_custom_mem` treats the input as a byte sequence bounded by the provided size.

Testing focus:

- use data where explicit length changes the included content,
- confirm different results when length-bounded quoting excludes later bytes.

### Scenario 4: Use asymmetric custom quote strings

A caller supplies different left and right delimiters.

Example scenario:

- left quote: `(`
- right quote: `)>`

Expected supported behavior:

- the module uses the exact left and right quote strings as provided, without requiring symmetry.

Testing focus:

- verify exact delimiter preservation on both sides.

## Requirements

### Functional Requirements

#### FR-1: Custom quote delimiters
The module shall accept a caller-supplied left quote string and right quote string for custom quoting operations.

Traceability:

- `quotearg_custom`
- `quotearg_custom_mem`

#### FR-2: String-based quoting entry point
The module shall provide behavior equivalent to quoting a conventional string argument using custom left and right quote strings.

Traceability:

- `quotearg_custom`

#### FR-3: Explicit-length quoting entry point
The module shall provide behavior equivalent to quoting an argument buffer using custom left and right quote strings and an explicit byte count.

Traceability:

- `quotearg_custom_mem`

#### FR-4: Length-bounded input handling
For the explicit-length entry point, the module shall limit the quoted input to the provided `argsize` rather than requiring NUL-terminated input.

Traceability:

- `quotearg_custom_mem`

#### FR-5: Exact delimiter application
The module shall use the caller-provided left and right quote strings as the delimiters for the produced quoted result.

Traceability:

- `quotearg_custom`
- `quotearg_custom_mem`
- `struct quoting_options`

#### FR-6: Quoting option mediation
The module shall preserve the use of quoting option state sufficient to support the custom quoting behavior exposed by these entry points.

Traceability:

- `struct quoting_options`
- `quotearg_custom`
- `quotearg_custom_mem`

#### FR-7: Returned quoted result
The module shall return the quoted representation through the module’s result-returning interface rather than requiring a caller-provided destination buffer in these entry points.

Traceability:

- `quotearg_custom`
- `quotearg_custom_mem`
- `struct slotvec`

### Key Entities

#### `quoting_options`
Option state used by the module to describe quoting behavior. For this module scope, it is the entity that carries or governs the custom quoting configuration used by the custom-quote entry points.

Relationship to functionality:

- custom left/right quote behavior is applied through this option state.

Traceability:

- `struct quoting_options`
- `quotearg_custom`
- `quotearg_custom_mem`

#### `slotvec`
Module-managed storage associated with returned quoted results.

Relationship to functionality:

- supports the result-returning behavior of quoting operations in this module.

Traceability:

- `struct slotvec`
- custom quoting entry points in `quotearg.c`

## Success Criteria

### SC-1: Correct custom delimiter usage
For both custom quoting entry points, tests show that the returned result uses the exact caller-supplied left and right quote strings.

Traceability:

- `quotearg_custom`
- `quotearg_custom_mem`

### SC-2: Correct string-entry behavior
Given a conventional string input, the Rust rewrite returns a quoted result consistent with the custom quoting behavior of `quotearg_custom`.

Traceability:

- `quotearg_custom`

### SC-3: Correct explicit-length behavior
Given a buffer and explicit size, the Rust rewrite returns a quoted result that includes only the bytes within the provided length.

Traceability:

- `quotearg_custom_mem`

### SC-4: Distinct handling of the two input forms
Tests demonstrate that string-based quoting and explicit-length quoting can produce different results when the provided byte length excludes data that would otherwise be part of the full string.

Traceability:

- `quotearg_custom`
- `quotearg_custom_mem`

### SC-5: Option-backed custom quoting preserved
The Rust rewrite preserves sufficient quoting option behavior to support the custom-quote functionality represented by `struct quoting_options` in this module.

Traceability:

- `struct quoting_options`
- `quotearg.c`

### SC-6: Result is returned from the module boundary
Tests confirm that each custom quoting entry point returns a quoted result directly from the module boundary, matching the observable contract of the C entry points.

Traceability:

- `quotearg_custom`
- `quotearg_custom_mem`
- `struct slotvec`