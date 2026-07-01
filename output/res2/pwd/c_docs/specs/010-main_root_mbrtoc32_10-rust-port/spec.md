# spec.md

## Title

Functional Specification: `main_root_mbrtoc32_10`

## Metadata

- **Project**: `pwd`
- **Module**: `main_root_mbrtoc32_10`
- **Category**: `main_cluster`
- **Source file**: `mbrtoc32.c`
- **Primary function**: `mbrtoc32`
- **Rust branch target**: `010-main_root_mbrtoc32_10-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides the `mbrtoc32` conversion behavior for transforming a multibyte character sequence into a single `char32_t` code point while maintaining conversion state through `mbstate_t`.

The Rust rewrite must preserve the observable behavior of the C module’s `mbrtoc32` interface, including stateful decoding, handling of partial input, handling of invalid input, and the ability to operate with caller-provided conversion state.

## Feature Specification

### Summary

The module implements conversion from a multibyte input sequence to one Unicode scalar value represented as `char32_t`, using an `mbstate_t` object to preserve decoding state across calls.

### Functional Scope

The Rust version must implement the following behavior evidenced by the module’s `mbrtoc32` entry point:

1. Accept a destination for the decoded `char32_t`, a byte-sequence pointer, a byte-count limit, and a conversion-state object.
2. Decode at most one multibyte character from the provided input.
3. Support incremental decoding when the complete character is not yet available in the provided bytes.
4. Use the supplied conversion state to continue decoding across multiple calls.
5. Report conversion outcomes through the same result categories expected of `mbrtoc32`:
   - successful conversion of a non-null character,
   - successful conversion of a null character,
   - incomplete but potentially valid multibyte sequence,
   - invalid multibyte sequence.
6. Update the output code point and conversion state consistently with the conversion result.
7. Behave correctly when the caller does not request output storage for the decoded `char32_t`.

### Out of Scope

The Rust rewrite must not introduce additional public functionality beyond the evidenced `mbrtoc32` behavior. In particular, this specification does not require:
- new decoding APIs,
- bulk string conversion,
- encoding functionality,
- thread-safety guarantees beyond what is implied by caller-managed state,
- serialization or persistence of conversion state.

## User Scenarios & Testing

### Scenario 1: Decode a complete multibyte character in one call

A caller provides:
- input bytes containing one complete multibyte character,
- a byte limit sufficient for that character,
- an initialized conversion state,
- and output storage for a `char32_t`.

Expected behavior:
- the function reports successful conversion,
- the decoded `char32_t` is written to output when requested,
- and the state is left ready for the next character.

### Scenario 2: Decode incrementally across multiple calls

A caller receives input in chunks and calls the function repeatedly with the same conversion state.

Expected behavior:
- an initial call with an incomplete sequence reports that more bytes are needed,
- subsequent call(s) with remaining bytes complete the character,
- the final result matches decoding the same bytes in a single complete call.

### Scenario 3: Handle a null character

A caller passes input beginning with the multibyte representation of the null character.

Expected behavior:
- the function reports successful decoding of the null character using the return convention of `mbrtoc32`,
- the output value is the null code point when output storage is provided,
- and the conversion state is in the correct post-conversion condition.

### Scenario 4: Detect invalid multibyte input

A caller provides bytes that do not form a valid multibyte character for this conversion interface.

Expected behavior:
- the function reports an invalid sequence,
- the state and output effects are consistent with failed conversion behavior,
- and the caller can distinguish this case from incomplete input.

### Scenario 5: Probe conversion without storing output

A caller wants to validate and consume one multibyte character without receiving the decoded `char32_t`, and therefore passes no output destination.

Expected behavior:
- the function still performs conversion,
- advances or preserves state according to the input result,
- and returns the same conversion-status category it would have returned if output storage had been provided.

### Testing Expectations

The Rust rewrite must be tested for:
- complete single-character decoding,
- null-character decoding,
- incomplete-sequence reporting,
- invalid-sequence reporting,
- repeated calls with shared state,
- operation with and without output storage,
- equivalence of incremental decoding and single-call decoding for the same valid input.

## Requirements

### Functional Requirements

#### FR-1: Single-character multibyte decoding
The module shall decode at most one multibyte character per call from the supplied byte sequence into a `char32_t` result, as defined by `mbrtoc32`.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-2: Stateful conversion
The module shall use an `mbstate_t` conversion state so that decoding can continue across multiple calls when input is split across buffers.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-3: Bounded input consumption
The module shall respect the caller-provided byte-count limit when determining whether a complete multibyte character is available.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-4: Distinct result classes
The module shall expose the standard `mbrtoc32` outcome classes through its return value:
- decoded non-null character,
- decoded null character,
- incomplete sequence,
- invalid sequence.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-5: Output writing behavior
The module shall write the decoded `char32_t` only when output storage is provided by the caller.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-6: State update semantics
The module shall update conversion state consistently with the outcome of the call, including successful completion of a character and continuation from incomplete input.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-7: Null-input conversion handling
The module shall support the `mbrtoc32` null-character conversion behavior for input representing the terminating null character.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### Key Entities

#### `char32_t`
Represents the decoded output code point for a single successful conversion.

#### `mbstate_t`
Represents the caller-maintained conversion state used to preserve partial decoding information across calls.

#### Multibyte input sequence
Represents the bounded input byte range from which one character is decoded.

### Entity Relationships

- `mbrtoc32` consumes a multibyte input sequence.
- `mbrtoc32` reads and updates `mbstate_t` to preserve conversion progress.
- `mbrtoc32` may produce one `char32_t` when conversion succeeds and output storage is provided.

## Success Criteria

### SC-1: Correct successful decoding
For valid complete inputs representing one character, the Rust module returns the successful `mbrtoc32` result category and produces the expected `char32_t` value when requested.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-2: Correct incremental behavior
For valid characters split across multiple calls, the Rust module reports incomplete input until sufficient bytes are available, then completes decoding correctly using the same state object.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-3: Correct null-character handling
For input representing the null character, the Rust module returns the `mbrtoc32` null-character success result and produces a null `char32_t` when output storage is supplied.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-4: Correct invalid-sequence detection
For invalid multibyte input, the Rust module returns the invalid-sequence result category, distinct from incomplete input.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-5: Respect for absent output storage
When called without output storage, the Rust module still performs conversion-state and return-value behavior correctly without requiring decoded-value storage.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-6: Input bound compliance
Tests demonstrate that the Rust module does not require bytes beyond the caller-provided length to determine its result for the current call.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`