# spec.md

## Title

Functional Specification: `main_root_quoting_options_02`

## Metadata

- Project: `pwd`
- Module: `main_root_quoting_options_02`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Primary analyzed function: `quotearg_n_custom_mem`
- Primary analyzed data structures: `quoting_options`, `slotvec`
- Rust branch target: `002-main_root_quoting_options_02-rust-port`
- Generation date: 2026-06-07

## Overview

This module provides quoting behavior for byte sequences using configurable quoting options, with the analyzed entry point focused on producing a quoted representation of a memory buffer using caller-supplied left and right quote strings.

The Rust rewrite must preserve the module behavior evidenced by:

- the presence of a configurable `quoting_options` structure,
- support for custom left/right quote delimiters,
- quoting of input provided as a pointer plus explicit size,
- use of a numbered quoting slot (`n`) for result selection/storage semantics,
- interaction with internal slot storage represented by `slotvec`.

This specification is limited to behavior evidenced by the analyzed source elements and does not define new APIs beyond what is traceable to the provided module analysis.

## Feature Specification

### Feature Summary

The module formats an input memory region into a quoted string according to quoting options derived from custom quote delimiters. The function `quotearg_n_custom_mem` accepts:

- a slot index `n`,
- a left quote string,
- a right quote string,
- an input memory region `arg`,
- the size of that region `argsize`,

and returns a character pointer to the quoted result.

### Required Rust Module Behavior

The Rust version must implement behavior equivalent to the analyzed module for the custom-memory quoting path:

1. It must accept an input as bytes plus explicit length semantics, rather than relying on NUL termination.
2. It must apply caller-provided left and right quote delimiters around the quoted representation.
3. It must derive or use quoting configuration represented by a `quoting_options` entity for this operation.
4. It must preserve slot-indexed result behavior associated with parameter `n`, consistent with the existence of `slotvec`.
5. It must produce a returned quoted string value equivalent in content to the C module’s result for the same inputs and quoting configuration.

### Functional Scope Boundaries

Included in scope:

- Custom quoting using explicit left/right quote strings.
- Quoting of memory buffers with explicit size.
- Use of quoting options data.
- Slot-indexed result handling for repeated quoting calls.

Out of scope unless directly required by the rewrite of this module:

- New quoting styles not evidenced by the analyzed entry point.
- New public configuration surfaces beyond those represented by `quoting_options`.
- Thread-safety guarantees.
- Persistence, serialization, networking, or FFI-specific behavior.
- Performance targets or benchmark requirements.

## User Scenarios & Testing

### Scenario 1: Quote a fixed-length byte sequence with custom delimiters

A caller has a byte sequence and wants a quoted representation surrounded by specific left and right delimiters.

- Input:
  - `n = 0`
  - `left_quote = "<"`
  - `right_quote = ">"`
  - `arg` references a byte sequence
  - `argsize` gives the exact number of bytes to quote
- Expected behavior:
  - The module returns a string beginning with `<` and ending with `>`.
  - The quoted result is based only on the first `argsize` bytes of the input.

Test coverage:
- Verify prefix and suffix match the supplied delimiters.
- Verify bytes beyond `argsize` are not considered.

### Scenario 2: Quote input containing embedded NUL bytes

A caller needs quoting for data that is not a C string and may contain zero bytes inside the buffer.

- Input:
  - an `arg` buffer containing one or more embedded `\0` bytes
  - `argsize` covering the full buffer
- Expected behavior:
  - The module processes the full buffer length indicated by `argsize`.
  - Embedded NUL does not truncate processing.

Test coverage:
- Verify output corresponds to all bytes in the provided memory span.
- Verify behavior differs from NUL-terminated string handling by honoring explicit length.

### Scenario 3: Use different quote pairs for different calls

A caller wants the same input quoted with different left/right delimiters in separate invocations.

- Input:
  - same `arg` and `argsize`
  - different `left_quote` / `right_quote` pairs
- Expected behavior:
  - Output changes to reflect the supplied quote pair for each call.
  - The function does not hard-code a single delimiter set.

Test coverage:
- Compare outputs from two delimiter pairs and verify only delimiter-dependent behavior changes as expected.

### Scenario 4: Use multiple slot indices

A caller uses different values of `n` to obtain quoted results associated with different numbered slots.

- Input:
  - multiple calls with distinct `n` values
- Expected behavior:
  - The module supports slot-indexed quoting behavior consistent with `slotvec`.
  - Results for one slot usage must not corrupt the content returned for another active slot use in a way that changes quoted content unexpectedly.

Test coverage:
- Perform calls with `n = 0`, `n = 1`, and another repeated slot.
- Verify each returned result matches the expected quoted content for that call.

### Scenario 5: Empty input buffer

A caller provides an empty memory region.

- Input:
  - `argsize = 0`
- Expected behavior:
  - The module returns a valid quoted result representing an empty payload surrounded by the chosen delimiters.

Test coverage:
- Verify output contains the delimiters and no payload content.

## Requirements

### Functional Requirements

#### FR-1: Quote explicit-length memory input

The module shall quote input provided as a memory region with an explicit size, using exactly the bytes covered by `argsize`.

**Traceability:** `quotearg_n_custom_mem` signature includes `char const *arg, size_t argsize`.

#### FR-2: Support caller-specified quote delimiters

The module shall support custom left and right quote strings supplied by the caller for the quoting operation.

**Traceability:** `quotearg_n_custom_mem` signature includes `char const *left_quote, char const *right_quote`.

#### FR-3: Use quoting options as the configuration model

The module shall represent quoting configuration through a `quoting_options` entity and use that configuration model for the custom quoting operation.

**Traceability:** multiple analyzed `struct quoting_options` entries; `quotearg_n_custom_mem` associated with `quoting_options` near line 1025.

#### FR-4: Support numbered quoting result slots

The module shall support quoting behavior parameterized by slot index `n`, consistent with slot-based result handling.

**Traceability:** `quotearg_n_custom_mem(int n, ...)`; analyzed `struct slotvec` entries.

#### FR-5: Return a quoted character string result

The module shall return a character-string representation of the quoted input.

**Traceability:** `quotearg_n_custom_mem` return type is `char *`.

#### FR-6: Preserve delimiter-sensitive output behavior across calls

The module shall produce output whose surrounding quote representation reflects the delimiter arguments provided for each invocation.

**Traceability:** `quotearg_n_custom_mem` custom delimiter parameters; `quoting_options` configuration role.

### Key Entities

#### `quoting_options`

A configuration entity representing quoting behavior. For this module scope, it is the configuration model used to define how input is quoted, including custom delimiter-based behavior required by `quotearg_n_custom_mem`.

Relationship to functionality:

- drives quoting behavior,
- is constructed or selected for custom quoting,
- influences the returned quoted string.

**Traceability:** multiple `struct quoting_options` references in `quotearg.c`.

#### `slotvec`

A slot-management entity associated with numbered quoting results.

Relationship to functionality:

- supports the `n` parameter used by `quotearg_n_custom_mem`,
- represents storage or tracking for per-slot quoted results.

**Traceability:** analyzed `struct slotvec` entries near lines 829-845 and later references.

#### Quoted result string

A returned character string containing the quoted representation of the provided memory input.

Relationship to functionality:

- is the observable output of `quotearg_n_custom_mem`,
- reflects input bytes, quoting options, chosen delimiters, and slot selection.

**Traceability:** `quotearg_n_custom_mem` return type `char *`.

## Success Criteria

### Behavioral Correctness

1. For a given input buffer and `argsize`, the Rust module produces quoted content based on exactly that byte range.
   - Traceability: `quotearg_n_custom_mem(arg, argsize)`

2. For caller-provided `left_quote` and `right_quote`, the Rust module’s output uses those delimiters in the resulting quoted representation.
   - Traceability: `quotearg_n_custom_mem(left_quote, right_quote)`

3. For inputs containing embedded NUL bytes, the Rust module processes the full explicit-length buffer rather than truncating at the first NUL.
   - Traceability: explicit memory-size API in `quotearg_n_custom_mem`

4. Distinct slot indices `n` are accepted and yield correct quoted results for each invocation under slot-based behavior.
   - Traceability: `quotearg_n_custom_mem(int n, ...)`; `slotvec`

### Structural Conformance

5. The Rust rewrite includes a configuration model corresponding to `quoting_options` and uses it in the custom quoting path.
   - Traceability: `struct quoting_options`; `quotearg_n_custom_mem`

6. The Rust rewrite preserves slot-oriented result handling corresponding to `slotvec` for the numbered quoting API path.
   - Traceability: `struct slotvec`; `quotearg_n_custom_mem`

### Testability

7. Automated tests can demonstrate:
   - custom delimiter application,
   - explicit-length handling,
   - embedded-NUL handling,
   - empty-input behavior,
   - multi-slot usage behavior.

   **Traceability:** `quotearg_n_custom_mem`; `quoting_options`; `slotvec`

## Acceptance Notes

- The Rust port may adapt memory management to Rust ownership rules, but observable behavior for quoted output must remain equivalent for the module scope defined here.
- This specification intentionally limits itself to the custom-memory quoting behavior and its evidenced supporting entities.
- Any behavior not evidenced by the analyzed function or data structures must not be added as a required feature in this module specification.