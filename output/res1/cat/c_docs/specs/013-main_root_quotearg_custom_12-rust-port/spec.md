# spec.md

## Title

Rust Functional Specification: `main_root_quotearg_custom_12`

## Overview

This module covers the custom-quotation entry points implemented in `quotearg.c`:

- `quotearg_custom`
- `quotearg_custom_mem`

Its purpose is to produce a quoted representation of an input argument using caller-provided left and right quote strings. The Rust rewrite must preserve the externally observable behavior of these custom quoting operations as defined by the existing module boundaries and the associated `quoting_options` usage in `quotearg.c`.

This specification is limited to the functionality evidenced by the analyzed module scope. It does not define new APIs or behaviors beyond custom quoting of string and memory inputs with caller-supplied quote delimiters.

## Feature Specification

### Summary

The module provides custom quoting for arguments using explicit left and right quote delimiters supplied by the caller.

Two usage forms are supported:

- quoting a NUL-terminated string argument
- quoting a memory buffer with an explicit byte length

The Rust version must implement equivalent behavior for both forms, including use of custom quote delimiters and handling of argument content according to the module’s quoting behavior.

### In-Scope Behavior

The Rust rewrite must support:

1. Accepting a caller-supplied left quote string.
2. Accepting a caller-supplied right quote string.
3. Accepting an input argument either as:
   - a conventional string input, or
   - a byte sequence with explicit size.
4. Producing a quoted result that uses the supplied left and right quote strings around the formatted argument.
5. Applying the module’s custom quoting behavior through the quoting-options pathway evidenced by `struct quoting_options`.

### Out of Scope

The Rust rewrite specification does not require any capability not evidenced by this module slice, including:

- defining additional public quoting styles beyond the custom-quote entry points in scope
- adding configuration formats or persistence
- adding thread-safety guarantees
- adding FFI-specific APIs
- adding error recovery semantics not evidenced by the current module interface

## User Scenarios & Testing

### Scenario 1: Quote a standard string with custom delimiters

A caller has a normal string argument and wants it represented with specific opening and closing quote strings.

- Input:
  - left quote: caller-defined string
  - right quote: caller-defined string
  - argument: NUL-terminated string
- Expected behavior:
  - the module returns a quoted representation of the argument
  - the result uses the provided left quote before the quoted argument content
  - the result uses the provided right quote after the quoted argument content

#### Test expectations

- The output begins with the supplied left quote.
- The output ends with the supplied right quote.
- The interior corresponds to quoting of the supplied argument content rather than omission or truncation.
- Different left/right quote inputs produce correspondingly different output delimiters.

### Scenario 2: Quote a byte buffer with explicit size

A caller has argument data that must be processed using a specified byte count rather than relying on NUL termination.

- Input:
  - left quote: caller-defined string
  - right quote: caller-defined string
  - argument: memory buffer
  - argument size: explicit byte length
- Expected behavior:
  - the module quotes exactly the provided byte range
  - the output is wrapped in the supplied left and right quote strings

#### Test expectations

- The quoted content reflects the explicit buffer length.
- Embedded NUL bytes or absence of a terminating NUL do not cause premature termination when using the memory-sized entry point.
- The output still starts and ends with the caller-supplied quote delimiters.

### Scenario 3: Distinguish string-based and length-based behavior

A caller uses both entry points on similar data and expects each function to honor its own input contract.

- Expected behavior:
  - the string entry point treats input as a normal string argument
  - the memory entry point treats input as byte data of explicit size

#### Test expectations

- For byte data containing an early NUL, the memory-sized form processes data through the specified size, while the string form is only required to operate on string-style input.
- The delimiter behavior is consistent across both entry points.

### Scenario 4: Use asymmetric custom delimiters

A caller supplies different opening and closing quote strings.

- Input example pattern:
  - left quote differs from right quote
- Expected behavior:
  - the module preserves the distinction and uses each delimiter in its proper position

#### Test expectations

- The output contains the exact supplied left delimiter at the start.
- The output contains the exact supplied right delimiter at the end.
- The module does not normalize or replace asymmetric delimiters with built-in defaults.

## Requirements

### Functional Requirements

#### FR-1: Custom quote delimiters
The module shall accept a left quote string and a right quote string and use them as the surrounding delimiters for the produced quoted argument representation.

**Traceability:** `quotearg_custom`, `quotearg_custom_mem`

#### FR-2: String-input quoting
The module shall support quoting an argument provided as a conventional string input through the string-based custom quoting entry point.

**Traceability:** `quotearg_custom`

#### FR-3: Explicit-length input quoting
The module shall support quoting an argument provided as a memory buffer together with an explicit byte length through the memory-based custom quoting entry point.

**Traceability:** `quotearg_custom_mem`

#### FR-4: Length-respecting behavior
For the explicit-length entry point, the module shall base processing on the provided argument size rather than requiring NUL termination of the input buffer.

**Traceability:** `quotearg_custom_mem`

#### FR-5: Quoting-options driven custom behavior
The module shall implement the custom-quote behavior through the quoting-options model represented by `struct quoting_options`, so that custom left and right quote strings are part of the effective quoting configuration.

**Traceability:** `struct quoting_options`, references to `struct quoting_options` within `quotearg.c` near the custom quoting functions

#### FR-6: Returned quoted result
The module shall produce and return a quoted result value for each supported entry point.

**Traceability:** `quotearg_custom`, `quotearg_custom_mem`

### Key Entities

#### `quoting_options`
Represents the quoting configuration used to control how an argument is rendered. Within this module scope, it is the configuration entity that carries or determines the custom quotation behavior, including the use of caller-provided quote delimiters.

**Relationship to functions:**
- `quotearg_custom` uses custom quote settings associated with quoting options.
- `quotearg_custom_mem` uses custom quote settings associated with quoting options.

#### Quoted argument result
The output of each entry point is a quoted representation of the supplied argument content. The result is the observable product of applying custom delimiters and module-defined quoting behavior to the input.

**Relationship to functions:**
- produced by `quotearg_custom`
- produced by `quotearg_custom_mem`

#### Argument input
The module operates on two input forms:

1. string argument
2. memory buffer plus explicit size

These are alternative sources for the content to be quoted.

**Relationship to functions:**
- string form: `quotearg_custom`
- memory-sized form: `quotearg_custom_mem`

## Success Criteria

### Behavioral Correctness

1. For string-based custom quoting, the Rust implementation returns a quoted result wrapped with the exact caller-supplied left and right quote strings.
   - **Traceability:** `quotearg_custom`

2. For memory-based custom quoting, the Rust implementation returns a quoted result wrapped with the exact caller-supplied left and right quote strings.
   - **Traceability:** `quotearg_custom_mem`

3. For the explicit-length entry point, test cases with non-NUL-terminated buffers and buffers containing embedded NUL bytes demonstrate that processing respects the provided byte length.

4. Distinct custom delimiter pairs produce correspondingly distinct outputs, including asymmetric delimiter pairs.
   - **Traceability:** `quotearg_custom`, `quotearg_custom_mem`

### Interface Coverage

5. The Rust rewrite provides functionality corresponding to both module entry points in scope:
   - string-based custom quoting
   - memory-sized custom quoting
   - **Traceability:** `quotearg_custom`, `quotearg_custom_mem`

### Configuration Fidelity

6. The Rust rewrite preserves the role of quoting configuration for custom delimiter behavior, with semantics traceable to `struct quoting_options`.
   - **Traceability:** `struct quoting_options`

### Regression-Oriented Validation

7. A test suite demonstrates the scenarios listed in this specification:
   - standard string with custom delimiters
   - explicit-size buffer quoting
   - distinction between string-based and length-based inputs
   - asymmetric custom delimiters
   - **Traceability:** `quotearg_custom`, `quotearg_custom_mem`, `struct quoting_options`