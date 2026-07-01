# spec.md

## Title

Functional Specification for `main_root_quotearg_custom_12`

## Metadata

- **Project**: `cat`
- **Module**: `main_root_quotearg_custom_12`
- **Category**: `main_cluster`
- **Source file**: `quotearg.c`
- **Primary source functions**:
  - `quotearg_custom`
  - `quotearg_custom_mem`
- **Primary source data structures**:
  - `struct quoting_options`
  - `struct slotvec`

## Overview

This module provides custom argument quoting services for text and byte-sequence inputs. Its role is to produce a quoted representation of an input argument using caller-supplied left and right quote strings.

The Rust rewrite must preserve the functional behavior evidenced by the source functions in `quotearg.c`:

- quote a NUL-terminated argument with caller-provided quote delimiters;
- quote a memory buffer with caller-provided quote delimiters and explicit input size;
- perform quoting through the module’s quoting-options model, specifically using custom quotation settings.

This module is part of the broader quoting subsystem and depends on the notion of quoting options and slot-based returned quoted strings, but this specification covers only the behavior directly evidenced for the custom-quoting entry points.

## Scope

### In Scope

- Producing quoted output for an input string using explicit left and right quote strings.
- Producing quoted output for an input memory region using explicit left and right quote strings and explicit length.
- Applying custom quote delimiters through the module’s quoting options behavior.

### Out of Scope

- Defining new quoting styles or public APIs beyond those evidenced by `quotearg_custom` and `quotearg_custom_mem`.
- Adding guarantees not evidenced by the source, including thread-safety, serialization, FFI behavior, recovery behavior, or performance targets.
- Specifying unrelated quoting entry points from the same file except where necessary to describe functional relationships.

## Feature Specification

### Feature: Custom-delimited quoting for string input

The module shall provide functionality equivalent to `quotearg_custom(left_quote, right_quote, arg)`.

Behavior:
- Accept a caller-provided left quote string.
- Accept a caller-provided right quote string.
- Accept an input argument treated as a NUL-terminated string.
- Return a quoted representation of the input argument using the supplied left and right quote strings as the enclosing delimiters.
- Use the module’s quoting-options mechanism to apply custom quoting behavior rather than an unrelated fixed quoting style.

Traceability:
- `quotearg.c:1030-1035` — `quotearg_custom`
- `struct quoting_options` references in `quotearg.c`

### Feature: Custom-delimited quoting for memory input

The module shall provide functionality equivalent to `quotearg_custom_mem(left_quote, right_quote, arg, argsize)`.

Behavior:
- Accept a caller-provided left quote string.
- Accept a caller-provided right quote string.
- Accept an input memory region and its explicit size.
- Quote exactly the supplied input extent rather than relying on NUL termination.
- Return a quoted representation using the supplied left and right quote strings as the enclosing delimiters.
- Use the module’s quoting-options mechanism to apply custom quoting behavior.

Traceability:
- `quotearg.c:1037-1043` — `quotearg_custom_mem`
- `struct quoting_options` references in `quotearg.c`

### Feature: Integration with quoting options

The custom quoting entry points shall operate through the quoting-options model represented by `struct quoting_options`.

Behavior:
- The selected quoting behavior for these functions must reflect “custom quotes” semantics, meaning the caller’s left and right quote strings determine the quote delimiters used in the produced output.
- The Rust rewrite must preserve the relationship between the custom-quoting entry points and quoting options, rather than hard-coding unrelated delimiters.

Traceability:
- `struct quoting_options` occurrences in `quotearg.c`
- `quotearg_custom`
- `quotearg_custom_mem`

### Feature: Returned quoted string through module-managed storage model

The source module uses a slot-based storage model (`struct slotvec`) for returned quoted strings. The Rust rewrite must preserve observable functional behavior of returning a quoted result for immediate caller use.

Behavior:
- Each custom quoting call must yield a usable quoted string result corresponding to the provided input and delimiters.
- The Rust implementation may choose Rust-appropriate ownership internally, but externally must preserve the functional contract of producing the quoted text result for the call.

Traceability:
- `struct slotvec` occurrences in `quotearg.c`
- `quotearg_custom`
- `quotearg_custom_mem`

## User Scenarios & Testing

### Scenario 1: Quote a normal argument with custom delimiters

A caller needs to display an argument surrounded by specific quote strings, such as a left delimiter and a right delimiter chosen by the caller.

Input:
- left quote: caller-supplied string
- right quote: caller-supplied string
- argument: NUL-terminated text

Expected result:
- output begins with the supplied left quote;
- output contains the quoted representation of the argument content;
- output ends with the supplied right quote.

Traceability:
- `quotearg_custom`

Suggested test coverage:
- verify the result starts and ends with the exact supplied delimiters;
- verify the input content is represented within the result.

### Scenario 2: Quote an empty string with custom delimiters

A caller quotes an empty NUL-terminated argument.

Input:
- left quote: non-empty caller-supplied string
- right quote: non-empty caller-supplied string
- argument: empty string

Expected result:
- output still includes the left and right delimiters;
- output reflects quoting of empty content rather than omitting the quoted value entirely.

Traceability:
- `quotearg_custom`

Suggested test coverage:
- verify result equals left delimiter + quoted empty content + right delimiter as defined by the module behavior;
- verify delimiters are present even when content is empty.

### Scenario 3: Quote a byte buffer with embedded NUL using explicit size

A caller needs to quote data that cannot be represented safely as a C string because the data contains embedded NUL bytes.

Input:
- left quote: caller-supplied string
- right quote: caller-supplied string
- argument: byte buffer
- argsize: explicit length including bytes after embedded NUL

Expected result:
- quoting consumes exactly `argsize` bytes;
- output corresponds to the full specified buffer, not just bytes before the first NUL;
- output is enclosed by the supplied delimiters.

Traceability:
- `quotearg_custom_mem`

Suggested test coverage:
- pass a buffer containing embedded NUL and trailing bytes;
- verify behavior differs from NUL-terminated handling by including the full specified extent.

### Scenario 4: Distinguish string-input and memory-input semantics

A caller has the same underlying bytes and uses both APIs.

Input:
- same byte content
- string API for one call
- memory API with explicit length for another call

Expected result:
- the string API interprets input as NUL-terminated text;
- the memory API interprets input as a sized region;
- when embedded NUL exists before the explicit end, the memory-input result reflects additional data that the string-input form would not consume.

Traceability:
- `quotearg_custom`
- `quotearg_custom_mem`

Suggested test coverage:
- compare outputs for identical raw bytes under both APIs;
- verify the distinction is observable when explicit length exceeds first NUL position.

### Scenario 5: Use asymmetric custom quote strings

A caller provides different left and right quote strings.

Input:
- left quote and right quote are distinct strings
- argument: ordinary text or byte buffer

Expected result:
- the result uses the exact left quote at the beginning and exact right quote at the end;
- the implementation does not normalize them into matching or fixed delimiters.

Traceability:
- `quotearg_custom`
- `quotearg_custom_mem`

Suggested test coverage:
- verify exact preservation of different opening and closing delimiters.

## Requirements

### Functional Requirements

#### FR-1: Custom quote delimiters for string input

The Rust module shall provide behavior equivalent to `quotearg_custom` for quoting a NUL-terminated argument using caller-specified left and right quote strings.

Traceability:
- `quotearg.c:1030-1035`

#### FR-2: Custom quote delimiters for memory input

The Rust module shall provide behavior equivalent to `quotearg_custom_mem` for quoting an input memory region of explicit size using caller-specified left and right quote strings.

Traceability:
- `quotearg.c:1037-1043`

#### FR-3: Explicit-length processing for memory input

For the memory-input variant, the Rust module shall process the argument according to the supplied size rather than stopping at the first NUL byte.

Traceability:
- `quotearg_custom_mem`

#### FR-4: NUL-terminated processing for string input

For the string-input variant, the Rust module shall treat the argument as NUL-terminated input rather than as an arbitrary sized buffer.

Traceability:
- `quotearg_custom`

#### FR-5: Delimiter preservation

The Rust module shall preserve the exact caller-supplied left and right quote strings as the enclosing delimiters of the quoted result.

Traceability:
- `quotearg_custom`
- `quotearg_custom_mem`

#### FR-6: Quoting-options-based custom behavior

The Rust module shall preserve the functional role of `struct quoting_options` in expressing custom quotation behavior for these entry points.

Traceability:
- `struct quoting_options`
- `quotearg_custom`
- `quotearg_custom_mem`

#### FR-7: Returned quoted result per invocation

Each invocation of the Rust module’s equivalent functionality shall produce a quoted string result corresponding to that invocation’s input and delimiters.

Traceability:
- `quotearg_custom`
- `quotearg_custom_mem`
- `struct slotvec`

### Key Entities

#### `quoting_options`

Represents quotation behavior configuration within the source module.

Functional role in this module:
- carries or enables the selection of custom-quote behavior;
- links caller-supplied quote delimiters to the quoting operation performed by the custom entry points.

Relationships:
- used by the custom quoting functions to determine how quoting is applied;
- conceptually parameterizes the produced quoted result.

Traceability:
- `struct quoting_options` occurrences in `quotearg.c`

#### `slotvec`

Represents the source module’s slot-based storage mechanism for quoted results.

Functional role in this module:
- supports returning quoted string results from quoting operations.

Relationships:
- associated with the output behavior of the custom quoting functions;
- not itself a user-facing feature requirement beyond enabling per-call quoted results.

Traceability:
- `struct slotvec` occurrences in `quotearg.c`

## Success Criteria

### SC-1: Correct delimiter application for string input

For representative string inputs, the Rust rewrite produces results that begin with the supplied left quote and end with the supplied right quote when using the `quotearg_custom` equivalent.

Traceability:
- FR-1
- FR-5

### SC-2: Correct delimiter application for memory input

For representative byte-buffer inputs, the Rust rewrite produces results that begin with the supplied left quote and end with the supplied right quote when using the `quotearg_custom_mem` equivalent.

Traceability:
- FR-2
- FR-5

### SC-3: Distinct handling of NUL-terminated vs explicit-size input

Given input containing an embedded NUL byte before the end of a provided buffer, tests show that:
- the string-input behavior is based on NUL termination; and
- the memory-input behavior includes the full explicitly sized input.

Traceability:
- FR-3
- FR-4

### SC-4: Support for asymmetric quote delimiters

Tests with different opening and closing delimiter strings confirm that the Rust rewrite preserves the exact provided left and right delimiters without replacing them with fixed or symmetric quotes.

Traceability:
- FR-5

### SC-5: Functional preservation of custom quoting behavior

The Rust rewrite demonstrates that the custom quoting entry points operate through the module’s custom-quotation behavior model corresponding to `quoting_options`, rather than ignoring caller-provided delimiters.

Traceability:
- FR-6

### SC-6: Result returned for each invocation

For repeated calls with different inputs and/or delimiters, each call yields a quoted result matching that call’s parameters.

Traceability:
- FR-7

## Acceptance Notes

- Conformance is established by behavior matching the evidenced responsibilities of `quotearg_custom` and `quotearg_custom_mem`.
- Internal Rust design may differ from the C source, but no externally claimed behavior may exceed what is supported by the cited source functions and data structures.
- Where broader quoting behavior exists elsewhere in `quotearg.c`, this module rewrite is only required to preserve the custom-quoting functionality evidenced for this module.