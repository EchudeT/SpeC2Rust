# spec.md

## Title

Functional Specification: `main_root_quotearg_n_07`

## Metadata

- Project: `cat`
- Module: `main_root_quotearg_n_07`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Rust target branch: `008-main_root_quotearg_n_07-rust-port`
- Generation date: 2026-06-06

## Overview

This module provides indexed argument-quoting entry points that return a quoted representation of an input argument string. The supported entry points are:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

The module also depends on quoting configuration represented by `struct quoting_options`, and on per-index storage represented by `struct slotvec`.

The Rust rewrite must preserve the observable behavior of these entry points as a quoting service that:

- accepts a caller-supplied slot index `n`,
- formats an argument according to quoting options,
- returns a pointer-like result corresponding to stored quoted text for that slot,
- supports both NUL-terminated string input and explicit-length memory input,
- supports a custom left/right quote pair for the custom variant.

## Feature Specification

### Summary

The module exposes convenience functions for quoting arguments while reusing storage associated with an integer slot number. It supports three functional modes evidenced by the exported functions:

1. Quote a conventional C string using default quoting behavior.
2. Quote a memory region using default quoting behavior and an explicit byte length.
3. Quote a conventional C string using caller-provided custom quote delimiters.

### Supported behavior

The Rust version must implement the following behavior:

- Quoting by slot index:
  - The caller provides an integer index `n`.
  - The module returns the quoted result associated with that slot.
  - Repeated use of the same slot is supported as part of the module contract implied by slot-based storage.

- Quoting of string input:
  - `quotearg_n` accepts a NUL-terminated argument string and returns its quoted form.

- Quoting of memory input:
  - `quotearg_n_mem` accepts a pointer plus explicit `argsize`.
  - The quoted result must be based on exactly the provided byte range rather than requiring NUL termination.

- Quoting with custom delimiters:
  - `quotearg_n_custom` accepts `left_quote` and `right_quote` strings in addition to the argument.
  - The result must be produced using those custom delimiters through quoting options for that call.

- Use of quoting options:
  - The module must apply quoting configuration through `struct quoting_options`.
  - The custom entry point must derive or use options reflecting the requested custom delimiters.

### Functional boundary

This specification covers only the slot-indexed quoting functions and the data they rely on in this module. It does not require unrelated quoting APIs, policy-setting APIs, or other behaviors not evidenced by the listed functions and structures.

## User Scenarios & Testing

### Scenario 1: Quote a simple argument by slot

A caller needs a quoted form of a command-line argument and uses `quotearg_n` with slot `0`.

Expected support:

- The function accepts the slot index and input string.
- It returns a quoted string result for that input.
- The result is available through the slot-based return path.

Suggested test coverage:

- Call `quotearg_n(0, "file")`.
- Verify that a non-null quoted result is returned.
- Verify that the output is stable enough to be used immediately by the caller as a quoted representation of `"file"`.

### Scenario 2: Quote different arguments into different slots

A caller formats multiple arguments at once and uses distinct slot indices to keep their quoted results separate.

Expected support:

- Different slot numbers can be used independently.
- Each slot yields the quoted result for its own most recent input.

Suggested test coverage:

- Call `quotearg_n(0, "a")` and `quotearg_n(1, "b")`.
- Verify that both results are returned successfully.
- Verify that slot `0` corresponds to the quoting of `"a"` and slot `1` to `"b"`.

### Scenario 3: Quote non-NUL-terminated memory

A caller has a byte range that may contain data not represented as a conventional C string and uses `quotearg_n_mem`.

Expected support:

- The function uses the explicit length.
- The function does not require NUL termination to determine input extent.

Suggested test coverage:

- Provide a buffer with a known prefix and extra trailing bytes.
- Call `quotearg_n_mem(0, buffer, prefix_len)`.
- Verify that the quoted result corresponds only to the specified `prefix_len` bytes.

### Scenario 4: Quote with custom left and right quote strings

A caller needs output wrapped using application-specific quote delimiters and uses `quotearg_n_custom`.

Expected support:

- The function accepts caller-provided left and right quote strings.
- The output uses those delimiters for the quoted result.

Suggested test coverage:

- Call `quotearg_n_custom(0, "<<", ">>", "name")`.
- Verify that the returned quoted representation reflects the custom delimiters.

### Scenario 5: Reuse a slot for a new argument

A caller uses the same slot index for successive formatting operations.

Expected support:

- The later call updates the slot-associated quoted result.
- The returned value corresponds to the latest input for that slot.

Suggested test coverage:

- Call `quotearg_n(0, "first")`.
- Then call `quotearg_n(0, "second")`.
- Verify that the second returned value corresponds to `"second"`.

## Requirements

### Functional Requirements

#### FR-1: Indexed quoting entry points

The module shall provide indexed quoting operations corresponding to `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom` in `quotearg.c`.

Traceability:
- `quotearg_n` [quotearg.c:925-929]
- `quotearg_n_mem` [quotearg.c:931-935]
- `quotearg_n_custom` [quotearg.c:1012-1018]

#### FR-2: Default quoting for NUL-terminated strings

The module shall accept a NUL-terminated argument string and produce a quoted representation for the specified slot through `quotearg_n`.

Traceability:
- `quotearg_n` [quotearg.c:925-929]

#### FR-3: Default quoting for explicit-length memory

The module shall accept an argument pointer plus explicit `size_t` length and produce a quoted representation for the specified slot through `quotearg_n_mem`.

Traceability:
- `quotearg_n_mem` [quotearg.c:931-935]

#### FR-4: Custom-delimiter quoting

The module shall accept caller-provided `left_quote` and `right_quote` strings and produce a quoted representation using those delimiters for the specified slot through `quotearg_n_custom`.

Traceability:
- `quotearg_n_custom` [quotearg.c:1012-1018]
- `struct quoting_options` references in `quotearg.c` including [952], [1006], [1025], [1047]

#### FR-5: Slot-based result association

The module shall associate quoted output storage with the caller-supplied slot index, as evidenced by the use of `struct slotvec`.

Traceability:
- `struct slotvec` [quotearg.c:829-833]
- `struct slotvec` references [839], [840], [845], [878]

#### FR-6: Quoting behavior shall be governed by quoting options

The module shall apply quoting configuration using `struct quoting_options` when producing quoted output, including the custom-quote case.

Traceability:
- `struct quoting_options` definition [quotearg.c:57-74]
- `struct quoting_options` references [782], [784], [795], [808], [810], [874], [952], [960], [979], [1006], [1025], [1047]

### Key Entities

#### `quoting_options`

A configuration entity that represents how an argument is quoted. Within this module, it is the policy object used to control quoting behavior, including the custom-quote variant.

Relationships:
- Consumed by the quoting entry points directly or indirectly.
- Parameterizes how input text is turned into quoted output.

Traceability:
- `struct quoting_options` [quotearg.c:57-74]
- additional references throughout `quotearg.c`

#### `slotvec`

A slot-associated storage entity used to retain quoted results by integer index.

Relationships:
- Maps or corresponds slot numbers used by `quotearg_n*` functions to stored quoted output.
- Supports repeated use of indexed quoting functions.

Traceability:
- `struct slotvec` [quotearg.c:829-833]
- additional references [839], [840], [845], [878]

#### Quoted result

The returned char-pointer result from each public entry point.

Relationships:
- Produced from input data plus quoting options.
- Stored or managed per slot through `slotvec`.

Traceability:
- `quotearg_n` [quotearg.c:925-929]
- `quotearg_n_mem` [quotearg.c:931-935]
- `quotearg_n_custom` [quotearg.c:1012-1018]

## Success Criteria

### Behavioral correctness

- The Rust module exposes behaviorally equivalent replacements for `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom`.
- For the same input bytes, slot index, and custom delimiters where applicable, the Rust version produces quoted output consistent with the C module’s observable behavior.

Traceability:
- `quotearg_n` [quotearg.c:925-929]
- `quotearg_n_mem` [quotearg.c:931-935]
- `quotearg_n_custom` [quotearg.c:1012-1018]

### Input mode coverage

- The Rust version successfully handles both:
  - NUL-terminated string input via `quotearg_n`
  - explicit-length memory input via `quotearg_n_mem`

Traceability:
- `quotearg_n` [quotearg.c:925-929]
- `quotearg_n_mem` [quotearg.c:931-935]

### Custom quoting coverage

- The Rust version successfully applies caller-specified left and right quote strings when using the custom entry point.

Traceability:
- `quotearg_n_custom` [quotearg.c:1012-1018]
- `struct quoting_options` references associated with custom handling

### Slot behavior coverage

- Tests demonstrate that at least two distinct slot indices can be used without conflating their most recent quoted results.
- Tests demonstrate that reusing one slot updates the result for that slot.

Traceability:
- `struct slotvec` [quotearg.c:829-833]
- `quotearg_n` [quotearg.c:925-929]
- `quotearg_n_mem` [quotearg.c:931-935]
- `quotearg_n_custom` [quotearg.c:1012-1018]

### Traceable structure preservation

- The Rust rewrite includes clear Rust representations for the functional roles of:
  - quoting options
  - slot-associated result storage

Traceability:
- `struct quoting_options` [quotearg.c:57-74]
- `struct slotvec` [quotearg.c:829-833]