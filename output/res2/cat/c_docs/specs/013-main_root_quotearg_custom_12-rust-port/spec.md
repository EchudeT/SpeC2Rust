# spec.md

## Title

Functional Specification: `main_root_quotearg_custom_12`

## Document Control

- Project: `cat`
- Module: `main_root_quotearg_custom_12`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Primary functions:
  - `quotearg_custom`
  - `quotearg_custom_mem`
- Rust target branch: `013-main_root_quotearg_custom_12-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides custom quoting of argument text using caller-supplied left and right quote strings.

The Rust rewrite must preserve the functional behavior evidenced by the source module: producing a quoted string from input text, with support for both NUL-terminated string input and explicit-length memory input. The module behavior is driven by quoting configuration represented by `struct quoting_options`, and the custom-quote entry points use that configuration to apply caller-provided quote delimiters.

This specification covers only the functionality evidenced by:
- `quotearg_custom`
- `quotearg_custom_mem`
- related option/state structures used to carry quoting behavior:
  - `struct quoting_options`
  - `struct slotvec`

## Feature Specification

### Feature Summary

The module formats input data as a quoted argument string using custom opening and closing quote delimiters supplied by the caller.

Two input forms are supported:

1. **String input**
   - `quotearg_custom(left_quote, right_quote, arg)`
   - Accepts a conventional C string argument.

2. **Memory-buffer input**
   - `quotearg_custom_mem(left_quote, right_quote, arg, argsize)`
   - Accepts arbitrary byte input with explicit length.

### Required Behavior

The Rust version must implement the following module behavior:

- Accept a left quote string and a right quote string from the caller.
- Apply those quote strings as the enclosing delimiters for the formatted result.
- Quote the provided argument content according to the custom-quote mode implied by these entry points.
- Support input that is either:
  - terminated text, or
  - explicit-length memory that may contain bytes not representable as a normal C string boundary.
- Return the resulting quoted text in the same functional role as the C module: a produced quoted argument string suitable for caller consumption.

### Functional Boundary

This module is limited to **custom quoted argument construction**. The Rust rewrite must not introduce unrelated capabilities beyond those evidenced here.

## User Scenarios & Testing

### Scenario 1: Quote a normal argument with caller-defined delimiters

A caller needs to display an argument surrounded by specific quote strings, such as custom typography or shell-facing markers.

- Input:
  - left quote: caller-provided string
  - right quote: caller-provided string
  - argument: ordinary text string
- Expected behavior:
  - output encloses the argument using the provided left and right quote strings
  - output is returned as a quoted argument string

**Test focus**
- Verify the output starts with the left quote and ends with the right quote.
- Verify the original argument content is represented in the quoted result according to module quoting behavior.

### Scenario 2: Quote a memory buffer with explicit size

A caller has argument data in a buffer where the data length is known independently of termination.

- Input:
  - left quote: caller-provided string
  - right quote: caller-provided string
  - argument buffer: pointer plus explicit size
- Expected behavior:
  - output is generated from exactly the specified number of input bytes
  - left and right quote strings are applied around the formatted content

**Test focus**
- Verify behavior uses the supplied `argsize`.
- Verify the produced output corresponds to the buffer content bounded by the explicit size rather than relying on terminators.

### Scenario 3: Use asymmetric quote strings

A caller provides different opening and closing delimiters.

- Input:
  - left quote != right quote
  - argument text or buffer
- Expected behavior:
  - output uses the exact supplied opening delimiter at the start
  - output uses the exact supplied closing delimiter at the end

**Test focus**
- Verify asymmetric delimiters are preserved exactly.
- Verify no substitution with default quote characters occurs.

### Scenario 4: Use the string and memory forms on equivalent data

A caller uses both entry points on equivalent content to ensure consistent quoting semantics for text content.

- Input:
  - identical quote strings
  - same logical content passed once as a C string and once as a buffer of matching length
- Expected behavior:
  - for equivalent content, both forms produce equivalent quoted output

**Test focus**
- Compare results for matching data where explicit length corresponds to the string length.
- Confirm no behavioral divergence other than input boundary handling.

## Requirements

### Functional Requirements

#### FR-1: Custom quote delimiters
The module shall accept caller-specified `left_quote` and `right_quote` values and use them as the quote delimiters for the produced argument representation.

**Traceability:** `quotearg_custom`, `quotearg_custom_mem`

#### FR-2: Quoting of string arguments
The module shall support quoting an input argument supplied as a string value.

**Traceability:** `quotearg_custom`

#### FR-3: Quoting of explicit-length memory arguments
The module shall support quoting an input argument supplied as memory plus explicit byte length.

**Traceability:** `quotearg_custom_mem`

#### FR-4: Boundary-respecting memory processing
For the explicit-length entry point, the module shall process the argument content according to the provided `argsize` boundary.

**Traceability:** `quotearg_custom_mem`

#### FR-5: Custom quoting mode configuration
The module shall use quoting configuration represented by `struct quoting_options` to drive custom-quote behavior for these entry points.

**Traceability:** `struct quoting_options`, `quotearg_custom`, `quotearg_custom_mem`

#### FR-6: Produced quoted result
The module shall produce and return a quoted argument string as the result of each supported entry point.

**Traceability:** `quotearg_custom`, `quotearg_custom_mem`, `struct slotvec`

### Key Entities

#### `quoting_options`
Configuration entity representing the quoting behavior used when constructing quoted argument output.

For this module, its relevant role is:
- carrying the custom quoting configuration needed by the custom-quote entry points
- associating caller-provided quote delimiters with the quoting operation

**Relationships**
- Used by the custom quoting functions to determine how argument content is enclosed and formatted.

**Traceability:** `struct quoting_options`

#### `slotvec`
Internal result-storage entity associated with retaining produced quoted argument strings.

For this module, its relevant role is:
- supporting storage of generated quoted output returned by quoting entry points

**Relationships**
- Works with quoting operations that produce returned quoted strings.

**Traceability:** `struct slotvec`

## Success Criteria

### Functional Acceptance Criteria

1. **Custom delimiter correctness**
   - Given specific `left_quote` and `right_quote` inputs, the Rust module produces output using those exact delimiters as the enclosing quotes.
   - Traceable to: `quotearg_custom`, `quotearg_custom_mem`

2. **String-input support**
   - The Rust module successfully accepts and quotes string input through the `quotearg_custom` functional equivalent.
   - Traceable to: `quotearg_custom`

3. **Explicit-length input support**
   - The Rust module successfully accepts and quotes memory input through the `quotearg_custom_mem` functional equivalent using the supplied input length.
   - Traceable to: `quotearg_custom_mem`

4. **Equivalent behavior on equivalent content**
   - When identical content is passed through the string form and the memory form with matching boundaries, the resulting quoted output is equivalent.

5. **Configuration-driven behavior preserved**
   - The Rust rewrite preserves the custom-quote functional role of `quoting_options` for these entry points rather than hardcoding unrelated quoting behavior.
   - Traceable to: `struct quoting_options`

6. **Returned quoted result available to caller**
   - Each supported entry point returns a produced quoted string result usable by the caller.
   - Traceable to: `quotearg_custom`, `quotearg_custom_mem`, `struct slotvec`

## Out of Scope

The Rust rewrite specification does not require any capability not evidenced by this module analysis, including:
- new public APIs beyond the functional equivalents of the identified entry points
- promises about concurrency behavior
- serialization formats
- FFI-specific interfaces
- performance or benchmark targets
- recovery or persistence mechanisms