# spec.md

## Title

Rust Functional Specification for `main_root_quotearg_style_14`

## Summary

This module provides style-based quoting entry points for producing a quoted representation of an input argument. The analyzed C surface for this module consists of two functions in `quotearg.c`:

- `quotearg_style`
- `quotearg_style_mem`

The Rust rewrite must preserve the observable behavior of these entry points: selecting a quoting style, accepting either NUL-terminated text or explicit-length input, and returning a quoted result consistent with the selected style and the module’s quoting option model.

## Scope

In scope:

- Quoting an argument using a specified quoting style.
- Supporting both string input and explicit-size memory input.
- Using the module’s quoting option model as the basis for style selection and output generation.
- Producing quoted output for callers in the same role as the C module’s exported entry points.

Out of scope:

- Defining new quoting styles not evidenced by the source module.
- Adding public configuration APIs beyond the evidenced style-based entry points.
- Extending the module with unrelated formatting, parsing, or escaping features.

## Feature Specification

### Feature: Quote an argument by quoting style

The module exposes a function-level interface that accepts a quoting style and an argument, and returns that argument rendered according to the selected style.

The Rust version must implement this behavior for:

- input treated as a conventional C string
- input treated as a byte sequence with explicit length

The selected style must control how the result is quoted. The Rust rewrite must preserve the distinction between these two input modes:

- `quotearg_style`: quotes an argument provided as a standard string input
- `quotearg_style_mem`: quotes an argument provided with an explicit byte count

### Feature: Support explicit-length quoting

The module must support quoting data where the caller supplies the number of input bytes to consider, rather than relying on termination by a trailing NUL byte.

This is required because the C module exposes a dedicated explicit-length entry point. The Rust rewrite must therefore preserve behavior for inputs that may include embedded NUL bytes or trailing bytes beyond the first NUL when the explicit-length path is used.

### Feature: Apply quoting through the module’s quoting options model

The analyzed file shows that style-based quoting is tied to `struct quoting_options`, and the style entry points are thin, style-selecting front doors into that model. The Rust rewrite must preserve that functional relationship:

- a quoting style determines the effective quoting configuration
- quoted output is produced according to that effective configuration
- style-only entry points do not require the caller to construct options directly

## User Scenarios & Testing

### Scenario 1: Quote a normal path-like argument using a chosen style

A caller has a textual argument and wants a quoted form appropriate to one supported quoting style.

Expected support:

- The caller can provide a string input.
- The caller selects a quoting style.
- The module returns a quoted string representation for that input.

Testing focus:

- verify that non-empty input is transformed according to the chosen style
- verify that the output changes when the style changes, where the underlying style definitions differ

Traceability:

- `quotearg_style`
- `struct quoting_options`

### Scenario 2: Quote an empty argument

A caller passes an empty input and expects the output to reflect the selected quoting style’s treatment of empty data.

Expected support:

- Empty string input is accepted.
- A quoted result is returned without error.

Testing focus:

- verify that empty input produces a deterministic quoted result for each supported style path exposed by the module

Traceability:

- `quotearg_style`
- `quotearg_style_mem`

### Scenario 3: Quote data with embedded NUL using explicit length

A caller needs to quote bytes that cannot be represented correctly by a NUL-terminated string interface alone.

Expected support:

- The caller provides a byte buffer and an explicit size.
- The module uses exactly the provided byte count.
- Embedded NUL does not truncate processing.

Testing focus:

- provide input containing one or more NUL bytes
- verify that the explicit-length API processes the full requested span
- verify that behavior differs from NUL-terminated handling when trailing data exists after the first NUL

Traceability:

- `quotearg_style_mem`

### Scenario 4: Quote only a prefix of a larger buffer

A caller has a larger memory region but wants only the first `n` bytes quoted.

Expected support:

- The explicit-length entry point respects the supplied length exactly.
- Bytes beyond the requested length do not affect the result.

Testing focus:

- quote the same buffer with two different lengths
- verify that the shorter result corresponds only to the requested prefix

Traceability:

- `quotearg_style_mem`

### Scenario 5: Use style-only quoting without direct option management

A caller wants quoting behavior selected only by style, without constructing or mutating a quoting-options object.

Expected support:

- Style-based entry points are sufficient to obtain quoted output.
- Internally, the selected style maps to the module’s quoting option semantics.

Testing focus:

- verify that quoting can be performed solely from style plus input
- verify consistency between repeated calls with the same style and same input

Traceability:

- `quotearg_style`
- `quotearg_style_mem`
- `struct quoting_options`

## Requirements

### Functional Requirements

#### FR-1: Style-selected quoting

The Rust module shall provide behavior equivalent to `quotearg_style`, accepting a quoting style and a string argument and returning the quoted representation of that argument.

Traceability:

- `quotearg.c`
- `quotearg_style`

#### FR-2: Explicit-length style-selected quoting

The Rust module shall provide behavior equivalent to `quotearg_style_mem`, accepting a quoting style, an input memory region, and an explicit size, and returning the quoted representation of exactly that input span.

Traceability:

- `quotearg.c`
- `quotearg_style_mem`

#### FR-3: Distinct handling for string and explicit-length inputs

The Rust module shall preserve the semantic distinction between NUL-terminated string input and explicit-length input. Behavior for explicit-length input shall not rely on early termination at the first embedded NUL byte.

Traceability:

- `quotearg_style`
- `quotearg_style_mem`

#### FR-4: Quoting behavior derived from quoting style

The Rust module shall determine output behavior from the provided quoting style through the module’s quoting-options model.

Traceability:

- `quotearg_style`
- `quotearg_style_mem`
- `struct quoting_options`

#### FR-5: Deterministic output for the same style and input

For a given quoting style and identical effective input bytes, the Rust module shall produce the same quoted result on repeated calls within the same build and environment assumptions as the original module.

Traceability:

- `quotearg_style`
- `quotearg_style_mem`
- `struct quoting_options`

### Key Entities

#### `quoting_style`

A style selector used by the public entry points to choose how quoting is performed.

Relationship to module behavior:

- supplied directly by callers
- interpreted through the quoting-options model
- controls the form of the returned quoted output

Traceability:

- `quotearg_style`
- `quotearg_style_mem`

#### `quoting_options`

The module’s core configuration entity for quoting behavior, evidenced by repeated `struct quoting_options` references throughout `quotearg.c`.

Relationship to other entities:

- represents effective quoting configuration
- is the configuration model underlying style-based quoting
- provides the functional bridge between a style value and the generated output

Traceability:

- `struct quoting_options`
- `quotearg_style`
- `quotearg_style_mem`

#### `slotvec`

A storage-related entity present in the module and associated with quoted argument result handling.

Relationship to module behavior:

- participates in management of quoted result storage within the C module
- is relevant to preserving returned-result behavior, but does not itself define new externally visible quoting features

Traceability:

- `struct slotvec`
- `quotearg.c`

## Success Criteria

### SC-1: Style API parity

A Rust implementation exposes module behavior covering both analyzed entry points: style-based quoting for string input and style-based quoting for explicit-length input.

Measured by:

- both documented usage scenarios for `quotearg_style` and `quotearg_style_mem` are implementable in tests

Traceability:

- `quotearg_style`
- `quotearg_style_mem`

### SC-2: Explicit-length correctness

Given an input buffer containing embedded NUL bytes, the explicit-length path quotes the full specified byte range rather than stopping at the first NUL.

Measured by:

- tests with embedded NUL and non-NUL trailing bytes confirm that all bytes within the requested length affect the result

Traceability:

- `quotearg_style_mem`

### SC-3: Prefix-length correctness

When quoting the same underlying buffer with different explicit lengths, the Rust implementation produces results corresponding only to the requested prefixes.

Measured by:

- tests comparing outputs for length `n` and `n + k` show that the shorter call excludes bytes beyond `n`

Traceability:

- `quotearg_style_mem`

### SC-4: Style-driven behavior preservation

For supported quoting styles, output differences observed in the original module due to style selection are preserved by the Rust rewrite.

Measured by:

- fixture-based comparison tests across multiple supported style values show matching per-style behavior against the port’s accepted reference expectations

Traceability:

- `quotearg_style`
- `quotearg_style_mem`
- `struct quoting_options`

### SC-5: Stable repeated results

Repeated calls with the same quoting style and same effective input produce identical quoted output.

Measured by:

- deterministic repeat-call tests pass for both string and explicit-length entry paths

Traceability:

- `quotearg_style`
- `quotearg_style_mem`