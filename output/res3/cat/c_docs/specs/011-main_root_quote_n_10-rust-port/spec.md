# spec.md

## Title

Functional Specification: `main_root_quote_n_10`

## Metadata

- Project: `cat`
- Module: `main_root_quote_n_10`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Primary functions:
  - `quote_n_mem`
  - `quote_n`
- Rust target branch: `011-main_root_quote_n_10-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides indexed quoting of argument text and returns a quoted string through an internal slot-based interface. It supports quoting either:

- a byte sequence with an explicit length, via `quote_n_mem`, or
- a NUL-terminated string, via `quote_n`.

The module’s behavior is tied to quoting configuration represented by `struct quoting_options`, and to per-index storage represented by `struct slotvec`. The Rust rewrite must preserve the observable behavior of these entry points as a quoting service that returns a quoted textual representation for a caller-supplied argument, keyed by a caller-supplied slot index.

## Feature Specification

### In scope

The Rust version must implement the functionality evidenced by the module entry points and related data structures:

1. Accept a slot index `n` and input text.
2. Produce a quoted representation of that text.
3. Support quoting for both:
   - explicit-length byte input (`quote_n_mem`)
   - conventional NUL-terminated string input (`quote_n`)
4. Use the module’s quoting configuration model represented by `quoting_options`.
5. Preserve slot-indexed behavior represented by `slotvec`, so that quoting is performed and returned in the context of the requested slot number.
6. Return quoted text in a form usable by the caller as a string result.

### Out of scope

The Rust version must not claim or introduce functionality not evidenced in the input, including:

- new public quoting APIs beyond the evidenced entry points,
- new configuration surfaces not represented by `quoting_options`,
- persistence, serialization, or external storage of quote slots,
- thread-safety guarantees,
- FFI-specific behavior,
- benchmarking or performance guarantees.

## User Scenarios & Testing

### Scenario 1: Quote a NUL-terminated argument string

A caller has a regular C-style string and needs its quoted form for display or diagnostic use. The caller invokes the `quote_n` equivalent with a chosen slot index and the string.

The Rust version must support:
- accepting a string input interpreted as terminating at the first NUL,
- producing the module’s quoted representation,
- returning the quoted result associated with the requested slot.

### Scenario 2: Quote a byte sequence with explicit length

A caller has argument data whose length is known independently of NUL termination, and needs it quoted without relying on string termination. The caller invokes the `quote_n_mem` equivalent with a slot index, a byte pointer, and a byte count.

The Rust version must support:
- accepting input as a byte sequence plus explicit length,
- quoting exactly the provided span of bytes,
- returning the quoted result for the requested slot.

### Scenario 3: Use multiple slot indices

A caller performs multiple quoting operations and uses different slot indices to obtain quoted results in separate slot contexts.

The Rust version must support:
- accepting different slot numbers across calls,
- producing correct quoted output for each call,
- preserving the module’s slot-oriented result behavior implied by `slotvec`.

### Scenario 4: Apply module-defined quoting rules

A caller relies on the module’s established quoting rules rather than ad hoc escaping. The quoting operation is expected to follow the configuration model represented by `quoting_options`.

The Rust version must support:
- using the same functional quoting model as the source module,
- ensuring output is governed by module quoting options behavior rather than plain passthrough.

### Scenario 5: Quote empty or zero-length input

A caller passes an empty string or a zero-length memory region and still expects a valid quoted result.

The Rust version must support:
- processing empty textual input,
- processing zero-length explicit memory input,
- returning a valid quoted string result rather than failing solely because input length is zero.

### Testing expectations for scenarios

The Rust rewrite should be validated with tests that cover:

- `quote_n` on a simple ASCII string,
- `quote_n_mem` on a byte slice of explicit length,
- `quote_n_mem` where the byte slice contains embedded NUL before the supplied end,
- repeated calls with different slot indices,
- repeated calls to the same slot index,
- empty string and zero-length input handling,
- output differences, if any, caused by quoting configuration represented by `quoting_options`.

## Requirements

### Functional Requirements

#### FR-1: Indexed quoting entry points

The module shall provide Rust functionality corresponding to `quote_n_mem` and `quote_n`, each accepting a caller-supplied slot index and returning a quoted string result.

**Traceability:** `quotearg.c`, functions `quote_n_mem`, `quote_n`

#### FR-2: Explicit-length input quoting

The `quote_n_mem` behavior shall quote input based on an explicit byte length, without requiring NUL termination to determine the end of the input.

**Traceability:** `quotearg.c`, function `quote_n_mem`

#### FR-3: NUL-terminated string quoting

The `quote_n` behavior shall quote input provided as a conventional string, using string termination semantics rather than a caller-supplied length.

**Traceability:** `quotearg.c`, function `quote_n`

#### FR-4: Slot-based result selection

The module shall preserve slot-based behavior for quoted results, using the caller-provided index as the selector for the result context.

**Traceability:** `quotearg.c`, type `struct slotvec`; functions `quote_n_mem`, `quote_n`

#### FR-5: Quoting governed by module options

The module shall implement quoting behavior consistent with the module’s quoting configuration model represented by `struct quoting_options`.

**Traceability:** `quotearg.c`, type `struct quoting_options`; functions `quote_n_mem`, `quote_n`

#### FR-6: Valid handling of empty input

The module shall accept empty textual input for both supported entry styles and return a valid quoted string result.

**Traceability:** `quotearg.c`, functions `quote_n_mem`, `quote_n`

#### FR-7: Returned result is textual quoted output

For each supported entry point, the module shall return the quoted form of the caller’s input as text suitable for immediate caller use.

**Traceability:** `quotearg.c`, functions `quote_n_mem`, `quote_n`

### Key Entities

#### `quoting_options`

Represents the quoting configuration model used by the module to determine how input text is transformed into quoted output.

Role in module:
- defines the quoting behavior applied during formatting,
- provides the configuration context underlying quoted output generation.

**Traceability:** `quotearg.c`, `struct quoting_options`

#### `slotvec`

Represents slot-oriented storage or tracking for quoted results keyed by integer index.

Role in module:
- associates quoted output handling with a slot number,
- supports the indexed behavior of `quote_n_mem` and `quote_n`.

**Traceability:** `quotearg.c`, `struct slotvec`

#### Relationship between entities

- `quote_n_mem` and `quote_n` are the observable entry points.
- Their quoting behavior is governed by `quoting_options`.
- Their indexed result behavior is organized through `slotvec`.

**Traceability:** `quotearg.c`, functions `quote_n_mem`, `quote_n`; types `struct quoting_options`, `struct slotvec`

## Success Criteria

### SC-1: Functional parity for both entry styles

The Rust module provides working equivalents of the two evidenced entry behaviors:
- one for explicit-length input,
- one for NUL-terminated string input.

**Traceability:** `quotearg.c`, functions `quote_n_mem`, `quote_n`

### SC-2: Correct explicit-length behavior

Tests demonstrate that the Rust `quote_n_mem` equivalent quotes exactly the specified input length, including cases where the provided bytes contain embedded NUL before the end of the quoted range.

**Traceability:** `quotearg.c`, function `quote_n_mem`

### SC-3: Correct string-terminated behavior

Tests demonstrate that the Rust `quote_n` equivalent quotes ordinary string input using string termination semantics.

**Traceability:** `quotearg.c`, function `quote_n`

### SC-4: Slot-indexed behavior preserved

Tests using multiple slot indices confirm that the Rust module preserves the source module’s indexed result behavior associated with `slotvec`.

**Traceability:** `quotearg.c`, type `struct slotvec`; functions `quote_n_mem`, `quote_n`

### SC-5: Quoting remains configuration-driven

The Rust rewrite retains the module’s configuration-driven quoting model, with behavior traceable to `quoting_options` rather than hard-coded plain string return.

**Traceability:** `quotearg.c`, type `struct quoting_options`

### SC-6: Empty input is accepted

Tests confirm that empty string input and zero-length explicit-length input both return valid quoted results without module failure.

**Traceability:** `quotearg.c`, functions `quote_n_mem`, `quote_n`

### SC-7: No unsupported surface expansion

The Rust rewrite exposes only the functionality evidenced for this module and does not require new public capabilities beyond the quoted indexed entry behavior and its supporting configuration/slot model.

**Traceability:** `quotearg.c`, functions `quote_n_mem`, `quote_n`; types `struct quoting_options`, `struct slotvec`