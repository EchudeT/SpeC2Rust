# spec.md

## Title

Rust Functional Specification for `main_root_quotearg_colon_12`

## Metadata

- **Project**: `pwd`
- **Module**: `main_root_quotearg_colon_12`
- **Category**: `main_cluster`
- **Source file**: `quotearg.c`
- **Primary source functions**:
  - `quotearg_colon`
  - `quotearg_colon_mem`
- **Rust branch target**: `012-main_root_quotearg_colon_12-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides a narrow quoting service for argument-like text where colon characters must be treated as characters requiring quoting/escaping. It exposes two entry points:

- one for NUL-terminated text
- one for byte sequences with explicit length

The Rust rewrite must preserve the observable quoting behavior of these two entry points, including their dependence on quoting configuration represented by the module’s quoting options data.

## Feature Specification

### Summary

The module produces a quoted representation of input text such that colon-sensitive content is rendered using the module’s established quoting rules. The behavior is specialized from the broader quoting subsystem through preconfigured quoting options rather than through a caller-supplied policy.

### In-scope functionality

The Rust version must implement:

1. **Colon-aware quoting of NUL-terminated input**
   - Accept a string-like input argument.
   - Return a quoted form consistent with the source module’s colon-specific quoting behavior.

2. **Colon-aware quoting of bounded memory input**
   - Accept a pointer-plus-length style input model in Rust terms as a byte slice or equivalent bounded data view.
   - Quote exactly the provided byte range, without relying on a terminating NUL.

3. **Use of quoting configuration**
   - Apply quoting behavior derived from the module’s quoting options structure.
   - Preserve the variant used by these entry points: colon is treated as a character that triggers quoting behavior.

4. **Compatibility with module-managed result storage semantics**
   - Preserve the externally observable behavior expected from these convenience quoting wrappers, including that they return quoted text without requiring the caller to pass an options object.

### Out of scope

The Rust rewrite specification for this module does not require adding:
- new quoting styles
- caller-defined option mutation APIs
- thread-safety guarantees
- serialization formats
- FFI surfaces
- recovery or retry behavior beyond the source module’s normal operation

## User Scenarios & Testing

### Scenario 1: Quote a path-like argument containing a colon

A caller needs a printable representation of an argument that contains `:` and must ensure the colon is treated as quote-worthy by this module’s policy.

**Expected support**
- The Rust module accepts the input through the string-oriented entry point.
- The output reflects the colon-aware quoting rules used by the source function `quotearg_colon`.

**Test focus**
- Input containing at least one colon produces output matching the C module’s behavior.
- Input without a colon still follows the same quoting style selection used by the source function.

### Scenario 2: Quote a non-NUL-terminated memory region

A caller has a bounded region of bytes that may contain colons and may not end with `\0`.

**Expected support**
- The Rust module accepts explicit input length.
- Only the specified bytes are considered.
- Embedded NUL bytes, if present in the specified range, are handled according to the source module’s bounded-memory quoting behavior.

**Test focus**
- A byte slice with an internal NUL and a colon is quoted based on the full provided length.
- Bytes beyond the provided length do not affect the result.

### Scenario 3: Use convenience quoting without constructing options

A caller wants the module’s predefined colon-specific quoting behavior and does not provide any quoting configuration.

**Expected support**
- The Rust entry points internally use the appropriate quoting options/profile corresponding to the source wrappers.
- The caller receives a quoted result directly.

**Test focus**
- Results from the Rust wrappers match the source wrappers for the same inputs.
- No caller-side configuration is required.

### Scenario 4: Repeated calls with different inputs

A caller invokes the quoting wrappers multiple times during program execution.

**Expected support**
- Each call returns a valid quoted representation for its own input.
- Repeated use preserves correctness of per-call output.

**Test focus**
- Sequential calls with varied inputs each match the source behavior.
- Earlier results are not corrupted in ways that differ from the source module’s observable semantics.

## Requirements

### Functional Requirements

#### FR-1: Colon-specific quoting wrapper for string input
The Rust module shall provide behavior equivalent to `quotearg_colon` from `quotearg.c`, producing a quoted representation for NUL-terminated input using the module’s colon-aware quoting configuration.

**Traceability**: `quotearg.c`, function `quotearg_colon`

#### FR-2: Colon-specific quoting wrapper for bounded memory input
The Rust module shall provide behavior equivalent to `quotearg_colon_mem` from `quotearg.c`, producing a quoted representation for an input region defined by data plus explicit length.

**Traceability**: `quotearg.c`, function `quotearg_colon_mem`

#### FR-3: Preconfigured quoting behavior
The Rust implementation shall apply quoting behavior derived from the module’s `quoting_options` data such that the colon character is included in the set of characters treated specially for quoting by these wrappers.

**Traceability**: `quotearg.c`, `struct quoting_options`, functions `quotearg_colon`, `quotearg_colon_mem`

#### FR-4: Correct handling of explicit-length data
For the bounded-memory entry point, the Rust implementation shall process exactly the supplied number of bytes, regardless of whether the input contains embedded NUL bytes.

**Traceability**: `quotearg.c`, function `quotearg_colon_mem`

#### FR-5: Returned quoted text availability
The Rust implementation shall return quoted text in a form that preserves the source wrappers’ practical role as convenience functions returning a completed quoted result to the caller.

**Traceability**: `quotearg.c`, functions `quotearg_colon`, `quotearg_colon_mem`

### Key Entities

#### `quoting_options`
A configuration entity representing the quoting policy used by the quoting subsystem. For this module, it is the source of the predefined behavior used by the colon-oriented wrappers.

**Role in this module**
- Determines how input is quoted.
- Encodes the fact that colon receives special treatment for these entry points.

**Traceability**: `quotearg.c`, `struct quoting_options`

#### Quoted result
The produced quoted text returned by the wrapper functions.

**Role in this module**
- Represents the final externally visible output of `quotearg_colon` and `quotearg_colon_mem`.
- Must correspond to the source module’s quoting rules for the provided input.

**Traceability**: `quotearg.c`, functions `quotearg_colon`, `quotearg_colon_mem`

#### Slot-based internal result storage
The source file includes `slotvec`, indicating that the C implementation manages reusable storage for quoted results internally.

**Role in this module**
- Explains that the wrappers are convenience-oriented and return ready-to-use quoted text.
- The Rust rewrite must preserve observable result behavior, but does not need to reproduce C memory layout or internal storage strategy.

**Traceability**: `quotearg.c`, `struct slotvec`, functions `quotearg_colon`, `quotearg_colon_mem`

## Success Criteria

### Behavioral parity

1. **String-input parity**
   - For representative test inputs, the Rust behavior corresponding to `quotearg_colon` matches the source module’s output.
   - Test set includes:
     - empty input
     - input without colon
     - input with one colon
     - input with multiple colons
     - input containing other characters that may also require quoting

2. **Bounded-memory parity**
   - For representative byte-slice inputs, the Rust behavior corresponding to `quotearg_colon_mem` matches the source module’s output.
     - explicit length shorter than available backing storage
     - embedded NUL within the specified length
     - colon before and after embedded NUL
     - zero-length input

3. **Configuration parity**
   - The Rust implementation demonstrably applies the colon-aware quoting policy associated with the source wrappers rather than generic unconstrained quoting.

### Interface adequacy

4. **Direct-use convenience**
   - Callers can obtain the module’s quoted result through wrapper-level functionality without constructing or passing a quoting options object.

5. **Deterministic per-call results**
   - Repeated calls with the same input produce the same quoted output as the source behavior.
   - Repeated calls with different inputs each produce the correct output for that input.

### Traceable completion criteria

6. **Source coverage**
   - The Rust rewrite fully covers the functional scope evidenced by:
     - `quotearg_colon`
     - `quotearg_colon_mem`
     - their use of `quoting_options`
   - No additional public capability is required for this module specification to be considered complete.