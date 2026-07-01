# spec.md

## Title
Rust Functional Specification for `main_root_quotearg_colon_11`

## Metadata
- Project: `cat`
- Module: `main_root_quotearg_colon_11`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Primary source functions:
  - `quotearg_colon`
  - `quotearg_colon_mem`
- Rust target branch: `012-main_root_quotearg_colon_11-rust-port`
- Generation date: 2026-06-06

## Overview
This module provides a narrow quoting facility specialized for producing quoted argument strings using colon-oriented quoting behavior. The source evidence shows two public entry points:

- `quotearg_colon(char const *arg)`
- `quotearg_colon_mem(char const *arg, size_t argsize)`

The Rust rewrite must preserve the functional behavior of these entry points as wrappers over the module’s quoting system, using the module’s quoting option model to produce a quoted representation suitable for arguments where colon handling is part of the selected quoting behavior.

This specification is limited to the functionality evidenced by the listed functions and the referenced internal entities in `quotearg.c`. It does not define new public APIs or additional capabilities beyond the behavior required to support these two entry points.

## Feature Specification

### Summary
The module shall accept either:
- a NUL-terminated string argument, or
- a byte sequence with an explicit length,

and return the quoted form of that argument using the module’s colon-specific quoting configuration.

### In-Scope Behavior
The Rust version must implement:

1. A quoting operation for NUL-terminated input via `quotearg_colon`.
2. A quoting operation for explicit-length input via `quotearg_colon_mem`.
3. Use of the quoting configuration model represented by `struct quoting_options`.
4. Production of quoted output consistent with the colon-specific variant selected by these functions.
5. Correct handling of embedded data up to the provided size in the explicit-length form.

### Out-of-Scope
This specification does not require:
- defining additional public quoting variants not evidenced here,
- exposing internal option mutation APIs unless needed internally for parity,
- promising behavior unrelated to the two identified functions.

## User Scenarios & Testing

### Scenario 1: Quote a standard argument string
A caller has a conventional C-style argument string and needs a quoted representation using the module’s colon-specific quoting behavior.

- Input: a valid NUL-terminated string.
- Operation: call `quotearg_colon`.
- Expected result: a returned quoted string representing the full input under colon-specific quoting rules.

**Test focus**
- Non-empty string input returns a quoted result.
- Empty string input returns a valid quoted representation for the empty input.
- Strings containing colon characters are quoted according to the colon-specific behavior of this module.

### Scenario 2: Quote a buffer with explicit size
A caller has data that may not be terminated at the first NUL byte, or the caller only wants to quote a prefix of a buffer.

- Input: pointer plus `argsize`.
- Operation: call `quotearg_colon_mem`.
- Expected result: only the first `argsize` bytes are considered when generating the quoted result.

**Test focus**
- A buffer containing embedded NUL bytes is processed according to the explicit length, not truncated at the first NUL.
- A buffer with trailing bytes beyond `argsize` ignores those trailing bytes.
- Zero-length input produces the quoted representation of an empty byte sequence.

### Scenario 3: Maintain behavior alignment between string and sized forms
A caller uses either API depending on available input form and expects equivalent results when both describe the same byte sequence.

- Input: a NUL-terminated string `s`, and a length equal to the byte count before its terminator.
- Operation: call both `quotearg_colon(s)` and `quotearg_colon_mem(s, strlen(s))`.
- Expected result: both produce equivalent quoted content.

**Test focus**
- Representative ASCII input yields identical output through both entry points when the effective data is the same.
- Inputs containing colons and other characters relevant to quoting still match across both forms when lengths align.

### Scenario 4: Use within the broader quoting subsystem
A caller depends on these functions as convenience entry points into the module’s quoting machinery rather than constructing quoting options manually.

- Input: arbitrary argument content.
- Operation: call the colon-specific wrapper functions.
- Expected result: the wrapper functions apply the proper preselected quoting configuration without requiring the caller to manage `quoting_options`.

**Test focus**
- Wrapper functions produce output consistent with the module’s colon-specific preset.
- No caller-supplied options are required to obtain the intended behavior.

## Requirements

### Functional Requirements

#### FR-1: Colon-specific quoting entry point for NUL-terminated strings
The Rust module shall provide functionality equivalent to `quotearg_colon` that accepts a NUL-terminated argument and returns its quoted representation using the module’s colon-specific quoting behavior.

**Traceability**
- `quotearg.c:991-995`
- Function: `quotearg_colon`

#### FR-2: Colon-specific quoting entry point for explicit-length input
The Rust module shall provide functionality equivalent to `quotearg_colon_mem` that accepts an input buffer and explicit byte length and returns its quoted representation using the same colon-specific quoting behavior.

**Traceability**
- `quotearg.c:997-1001`
- Function: `quotearg_colon_mem`

#### FR-3: Shared quoting behavior across both entry points
The Rust implementation shall ensure that both entry points use the same colon-oriented quoting configuration, differing only in how input length is determined.

**Traceability**
- `quotearg.c:991-1001`
- Functions: `quotearg_colon`, `quotearg_colon_mem`
- Type family: `struct quoting_options`

#### FR-4: Explicit-length processing must respect provided size
For the explicit-length entry point, the Rust implementation shall quote exactly the bytes designated by the supplied size, including any embedded NUL bytes within that range.

**Traceability**
- `quotearg.c:997-1001`
- Function: `quotearg_colon_mem`

#### FR-5: Quoting behavior must be driven by the module’s quoting option model
The Rust rewrite shall preserve the role of the module’s quoting configuration structure in determining output behavior for the colon-specific wrappers.

**Traceability**
- `quotearg.c` references to `struct quoting_options`
- Functions: `quotearg_colon`, `quotearg_colon_mem`

#### FR-6: Returned result must be a quoted string result suitable for caller consumption
Each entry point shall produce a string result representing the quoted form of the supplied input, not merely validate or classify the input.

**Traceability**
- `quotearg.c:991-1001`
- Return type evidence: `char *`
- Functions: `quotearg_colon`, `quotearg_colon_mem`

### Key Entities

#### `quoting_options`
The module’s core quoting configuration structure. It defines the quoting behavior selected by the wrapper functions and is the governing configuration entity behind the colon-specific quoting mode.

**Role**
- Encodes how quoting is performed.
- Provides the behavioral basis for both public entry points.

**Traceability**
- `quotearg.c` multiple references to `struct quoting_options`

#### `slotvec`
An internal storage-related structure used by the broader quoting subsystem to manage quoted result storage.

**Role**
- Supports result management within the quoting subsystem.
- Relevant because the public functions return string results produced by the shared quoting machinery.

**Traceability**
- `quotearg.c:829-833` and related references to `struct slotvec`

#### Relationship between entities
- `quotearg_colon` and `quotearg_colon_mem` are convenience entry points.
- These entry points rely on `quoting_options` to select colon-specific behavior.
- The resulting quoted string is produced through the subsystem that also references `slotvec` for internal result storage management.

## Success Criteria

### SC-1: String-form quoting parity
For any test input representable as a NUL-terminated string, the Rust implementation’s `quotearg_colon` behavior shall match the source module’s observable quoted output for that same input.

**Traceability**
- `quotearg.c:991-995`
- Function: `quotearg_colon`

### SC-2: Sized-form quoting parity
For any byte sequence and explicit size, the Rust implementation’s `quotearg_colon_mem` behavior shall match the source module’s observable quoted output for the same bytes and size.

**Traceability**
- `quotearg.c:997-1001`
- Function: `quotearg_colon_mem`

### SC-3: Cross-entry consistency
When `quotearg_colon` input and `quotearg_colon_mem` input represent the same byte sequence, both Rust entry points shall produce equivalent quoted output.

**Traceability**
- `quotearg.c:991-1001`
- Functions: `quotearg_colon`, `quotearg_colon_mem`

### SC-4: Embedded-NUL correctness in explicit-length mode
Tests using buffers with embedded NUL bytes shall demonstrate that the Rust explicit-length entry point processes data through the provided byte count rather than terminating early at the first NUL.

**Traceability**
- `quotearg.c:997-1001`
- Function: `quotearg_colon_mem`

### SC-5: Colon-specific behavior preservation
Tests containing colon characters shall demonstrate that the Rust output preserves the source module’s colon-specific quoting behavior selected by these wrappers.

**Traceability**
- `quotearg.c:991-1001`
- Functions: `quotearg_colon`, `quotearg_colon_mem`
- Type family: `struct quoting_options`

### SC-6: Empty-input handling
Both entry points shall successfully produce the correct quoted result for empty input cases:
- empty NUL-terminated string for `quotearg_colon`
- zero-length buffer for `quotearg_colon_mem`

**Traceability**
- `quotearg.c:991-1001`
- Functions: `quotearg_colon`, `quotearg_colon_mem`

## Acceptance Notes
- Conformance should be validated against the behavior of the source `quotearg.c` implementation for the two identified functions.
- The Rust port may reorganize internals, but observable behavior for the specified entry points must remain aligned with the source module.
- Internal entities only need to be preserved to the extent necessary to implement equivalent functionality and output behavior.