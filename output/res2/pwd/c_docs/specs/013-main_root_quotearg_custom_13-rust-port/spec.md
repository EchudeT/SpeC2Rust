# spec.md

## Title

Functional Specification: `main_root_quotearg_custom_13`

## Document Control

- **Project**: `pwd`
- **Module**: `main_root_quotearg_custom_13`
- **Category**: `main_cluster`
- **Source file**: `quotearg.c`
- **Rust branch**: `013-main_root_quotearg_custom_13-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides quoting functionality for arguments using caller-supplied opening and closing quote strings. It exposes two entry points:

- `quotearg_custom`
- `quotearg_custom_mem`

The Rust rewrite must preserve the functional behavior of these entry points as a specialized interface over the module’s quoting configuration machinery, represented by `struct quoting_options`, and its quoted-result storage behavior, represented by `struct slotvec`.

## Feature Specification

### Summary

The module produces a quoted representation of input data using custom left and right quote delimiters provided by the caller.

It supports:

- quoting a NUL-terminated string argument,
- quoting a memory region with explicit length,
- applying custom quote delimiters through the module’s quoting options model,
- returning the quoted result through the module’s established result-allocation/storage behavior.

### In-Scope Functionality

The Rust version must implement the following module behavior:

1. Accept caller-provided left and right quote strings.
2. Quote input using those exact custom delimiters.
3. Support both:
   - string input terminated in the conventional C style, and
   - byte input with explicit size.
4. Route quoting through the module’s quoting-options model rather than treating custom quoting as unrelated behavior.
5. Return a quoted string result consistent with the source module’s public behavior for these entry points.

### Out of Scope

The Rust version specification for this module does not require any functionality not evidenced by the identified functions and related data structures. In particular, this specification does not define new APIs, new quoting styles, persistence, serialization, concurrency guarantees, or behavior outside custom-quote argument rendering.

## User Scenarios & Testing

### Scenario 1: Quote a regular string with custom delimiters

A caller has a normal string argument and wants it wrapped according to custom quoting delimiters.

- **Input**: left quote, right quote, and a NUL-terminated argument string.
- **Action**: call `quotearg_custom`.
- **Expected result**: the returned string is the quoted form of the input using the supplied left and right quote strings.

#### Test expectations

- The result begins with the supplied left quote.
- The result ends with the supplied right quote.
- The content corresponds to the input argument as processed by the module’s quoting behavior.
- The function accepts the string form of input without requiring the caller to provide a length.

### Scenario 2: Quote a bounded memory buffer

A caller needs to quote data that is not represented solely by a terminating NUL byte, or that must be processed only up to a specified size.

- **Input**: left quote, right quote, pointer to data, and explicit size.
- **Action**: call `quotearg_custom_mem`.
- **Expected result**: the returned string is the quoted form of exactly the specified memory region using the supplied left and right quote strings.

#### Test expectations

- The function processes the number of bytes specified by `argsize`.
- Embedded or trailing bytes beyond the specified region do not affect the result.
- The result uses the supplied custom delimiters.

### Scenario 3: Different custom delimiter pairs produce different quoted output

A caller invokes the module multiple times with the same argument but different custom delimiters.

- **Input**: one argument value, multiple left/right quote pairs.
- **Action**: call either custom quoting function repeatedly.
- **Expected result**: each returned value reflects the specific delimiter pair used for that call.

#### Test expectations

- Changing the left quote changes the opening delimiter in output.
- Changing the right quote changes the closing delimiter in output.
- For identical input content, outputs differ when delimiter pairs differ.

### Scenario 4: String and explicit-memory entry points agree on equivalent input

A caller uses both APIs on equivalent data: one as a standard string and the other as a memory buffer of the same logical content length.

- **Input**: identical content passed once as a NUL-terminated string and once as a buffer with matching explicit size.
- **Action**: call both `quotearg_custom` and `quotearg_custom_mem`.
- **Expected result**: both results represent the same quoted content.

#### Test expectations

- When `argsize` matches the string length, both entry points produce equivalent quoted output.
- Differences are permitted only when the explicit-size input intentionally includes or excludes bytes differently from the string form.

## Requirements

### Functional Requirements

#### FR-1: Custom quote delimiters

The module shall accept both a left quote string and a right quote string as caller inputs for custom quoting.

**Traceability**: `quotearg_custom`, `quotearg_custom_mem`, `struct quoting_options`

#### FR-2: Quoting of NUL-terminated string input

The module shall provide an operation that accepts a conventional string argument and returns its quoted form using the supplied custom delimiters.

**Traceability**: `quotearg_custom`

#### FR-3: Quoting of explicit-length memory input

The module shall provide an operation that accepts a memory region and explicit byte count and returns its quoted form using the supplied custom delimiters.

**Traceability**: `quotearg_custom_mem`

#### FR-4: Explicit-length input governs processed content

For the explicit-memory operation, the module shall determine the quoted input extent from the provided size argument.

**Traceability**: `quotearg_custom_mem`

#### FR-5: Custom quoting is expressed through quoting options

The module shall represent custom quoting through the quoting-options structure used by the module’s quoting subsystem.

**Traceability**: `struct quoting_options`, `quotearg_custom`, `quotearg_custom_mem`

#### FR-6: Returned result is a quoted string representation

Each public entry point in this module shall return a string result containing the quoted representation of the supplied input.

**Traceability**: `quotearg_custom`, `quotearg_custom_mem`

#### FR-7: Result generation follows the module’s established quoted-result storage model

The module shall integrate custom quoting with the existing result storage/slot model used by the quoting subsystem.

**Traceability**: `struct slotvec`, `quotearg_custom`, `quotearg_custom_mem`

### Key Entities

#### `quoting_options`

The core configuration entity for the quoting subsystem.

For this module, it is the mechanism that carries the custom quoting configuration, including the caller-supplied left and right quote strings, into the quoting operation.

**Relationships**:
- Used by both public custom-quoting entry points.
- Governs how input is rendered into quoted output.

#### `slotvec`

The module’s quoted-result storage entity.

For this module, it represents the established storage model through which quoted string results are managed and returned.

**Relationships**:
- Serves the result-handling path used by the custom-quoting entry points.
- Works in conjunction with `quoting_options` to produce returned quoted strings.

## Success Criteria

### Functional Correctness

1. **Custom delimiter fidelity**
   For both public entry points, given left and right quote strings, the returned output uses those supplied delimiters for the quoted result.

   **Traceability**: `quotearg_custom`, `quotearg_custom_mem`, `struct quoting_options`

2. **String-input support**
   The Rust version provides behavior equivalent to `quotearg_custom` for quoting NUL-terminated string input.

   **Traceability**: `quotearg_custom`

3. **Explicit-memory support**
   The Rust version provides behavior equivalent to `quotearg_custom_mem` for quoting explicit-length memory input.

   **Traceability**: `quotearg_custom_mem`

4. **Length-bounded processing**
   For the explicit-memory entry point, tests demonstrate that the quoted result depends on the specified byte length and not on bytes beyond that bound.

   **Traceability**: `quotearg_custom_mem`

5. **Entry-point consistency**
   When given equivalent logical input, the string-based and explicit-memory entry points produce equivalent quoted output.

   **Traceability**: `quotearg_custom`, `quotearg_custom_mem`

6. **Integration with quoting configuration model**
   The Rust rewrite expresses custom quoting behavior through a Rust representation corresponding to the source module’s quoting-options concept, rather than as unrelated ad hoc formatting.

   **Traceability**: `struct quoting_options`

7. **Integration with result storage behavior**
   The Rust rewrite preserves externally visible result behavior consistent with the source module’s quoted-result storage model.

   **Traceability**: `struct slotvec`

### Acceptance Evidence

The module rewrite is acceptable when:

- unit or integration tests cover each user scenario in this document,
- both public entry points are implemented on the Rust branch,
- tests verify delimiter-sensitive output,
- tests verify explicit-size behavior for memory input,
- no additional public functionality beyond the evidenced module scope is required to satisfy the port.