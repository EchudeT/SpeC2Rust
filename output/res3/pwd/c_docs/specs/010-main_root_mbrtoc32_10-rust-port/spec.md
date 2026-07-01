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
- **Generation date**: `2026-06-09`

## Overview

This module provides the `mbrtoc32` functionality for converting a multibyte character sequence into a single 32-bit wide character result while tracking conversion state across calls.

The Rust rewrite must preserve the observable behavior of this module as a stateful multibyte-to-`char32` conversion interface, including handling of complete input, incomplete input, reset behavior, and error signaling as defined by the source module’s `mbrtoc32` entry point.

## Feature Specification

### Summary

The module implements conversion from a byte-oriented multibyte character input into one Unicode scalar value / 32-bit character result per call, using a caller-supplied conversion state.

### In-Scope Functionality

The Rust version must implement the behavior of the module’s `mbrtoc32` function with support for:

- accepting a destination for the converted `char32` result,
- accepting an input byte sequence and byte-count limit,
- accepting and updating conversion state across calls,
- recognizing and reporting successful conversion of one character,
- recognizing and reporting incomplete multibyte input,
- recognizing and reporting invalid multibyte input,
- supporting state-dependent operation when the caller provides persistent conversion state,
- supporting the special reset/probe-style call patterns that are part of `mbrtoc32` behavior.

### Behavioral Boundary

This module is limited to single-character decoding through the `mbrtoc32` interface. It does not define higher-level string iteration, buffering policy beyond the supplied state object, or additional public APIs.

## User Scenarios & Testing

### Scenario 1: Convert one complete multibyte character

A caller provides:

- a valid output location,
- a pointer to input bytes containing one complete multibyte character,
- the available byte count,
- a conversion state object.

Expected behavior:

- the function consumes the bytes belonging to the next character,
- stores the resulting `char32` value when an output location is provided,
- returns a status indicating successful conversion.

### Scenario 2: Decode input incrementally across multiple calls

A caller receives input in fragments and uses the same conversion state object across calls.

Expected behavior:

- when the first fragment does not contain a complete character, the function reports incomplete input without producing a completed character,
- when the remaining bytes are later provided with the same state, the function completes decoding correctly.

### Scenario 3: Detect invalid multibyte input

A caller supplies bytes that do not form a valid multibyte sequence for the active conversion rules.

Expected behavior:

- the function reports an encoding error,
- the failure is distinguishable from incomplete input,
- no successful character result is reported for that call.

### Scenario 4: Use the function without storing the decoded character

A caller needs to validate or advance past one multibyte character but does not need the resulting `char32` value.

Expected behavior:

- the function still performs conversion semantics,
- successful/incomplete/error outcomes remain the same as when an output pointer is supplied,
- omission of the destination does not change the decoding rules.

### Scenario 5: Handle input representing the null character

A caller provides input whose next decoded character is the null wide character.

Expected behavior:

- the function reports the null-character conversion according to `mbrtoc32` semantics,
- state is left consistent for subsequent decoding.

### Scenario 6: Reset or operate with special null-input usage

A caller invokes the function in a way used to query or reset shift/conversion state through the standard `mbrtoc32` calling convention.

Expected behavior:

- the function handles this usage according to `mbrtoc32` semantics,
- any state reset effect is applied consistently.

### Testing Guidance

The Rust rewrite must be testable with cases covering:

- successful decoding of single-byte and multibyte characters,
- decoding with `pwc` present and absent,
- incomplete sequences split across calls,
- invalid sequences,
- null-character input handling,
- state reset/probe call forms,
- repeated sequential calls using the same state object.

## Requirements

### Functional Requirements

#### FR-1: Single-character multibyte decoding
The module shall provide `mbrtoc32` behavior that converts at most one multibyte character from the supplied input and reports the result through its return value and optional output character storage.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-2: Stateful decoding across calls
The module shall accept a conversion-state object and use it to preserve partially processed decoding context across multiple calls when input arrives incrementally.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-3: Bounded input consumption
The module shall limit decoding to the number of bytes indicated by the caller and distinguish between a complete character and an incomplete sequence caused by insufficient available bytes.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-4: Optional output storage
The module shall allow the caller to omit the output character destination while preserving conversion, validation, and return-status behavior.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-5: Error signaling for invalid input
The module shall detect invalid multibyte input sequences and report conversion failure in the manner defined by `mbrtoc32` semantics.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-6: Null-character handling
The module shall support decoding of an input sequence representing the null wide character and report that case according to `mbrtoc32` semantics.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-7: Special null-input/reset semantics
The module shall support the `mbrtoc32` call forms in which the input pointer is used to trigger reset/probe behavior and shall update or clear conversion state accordingly.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### Key Entities

#### `char32` result
The decoded single-character output value produced by a successful conversion when the caller supplies an output destination.

**Relationship**:
- produced from the next multibyte sequence in the input,
- written by `mbrtoc32`,
- omitted when the caller passes no output destination.

#### Input byte sequence
The caller-provided byte buffer and byte-count limit representing the next available multibyte data to decode.

**Relationship**:
- consumed by `mbrtoc32`,
- may contain a complete character, an incomplete prefix, or invalid data.

#### `mbstate_t` conversion state
The caller-provided state object that carries decoding context across calls.

**Relationship**:
- read and updated by `mbrtoc32`,
- enables incremental decoding,
- participates in reset/probe semantics.

#### Return status
The `size_t` result returned by `mbrtoc32`, which communicates conversion outcome.

**Relationship**:
- indicates success, incomplete input, null-character handling, or error according to `mbrtoc32` semantics,
- is the caller’s primary mechanism for determining the result of each invocation.

## Success Criteria

### SC-1: Functional parity of the public entry point
A Rust implementation exposes module behavior equivalent to the source module’s `mbrtoc32` function for all supported call patterns described in this specification.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-2: Correct complete-sequence behavior
For valid complete multibyte input representing one character, the Rust version returns the correct success status, advances by the correct number of bytes for that character, and stores the correct `char32` value when an output destination is provided.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-3: Correct incremental behavior
For test cases where a valid multibyte sequence is split across calls, the Rust version first reports incomplete input and then successfully completes decoding when the remaining bytes are provided with the same conversion state.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-4: Correct invalid-input behavior
For invalid multibyte sequences, the Rust version reports an error outcome distinct from successful conversion and incomplete input.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-5: Correct null-character behavior
For input that decodes to the null character, the Rust version reports the null-character case according to `mbrtoc32` semantics and leaves state consistent for subsequent calls.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-6: Correct optional-output behavior
When invoked without an output destination, the Rust version preserves the same decoding and status behavior as with an output destination, except for not storing the decoded `char32`.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-7: Correct reset/probe behavior
For `mbrtoc32` call forms that use special null-input/reset semantics, the Rust version produces state effects and return behavior consistent with the source module.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

## Out of Scope

The Rust rewrite specification does not require:

- adding new public APIs beyond the module behavior represented by `mbrtoc32`,
- defining whole-string decoding helpers,
- defining locale-management APIs,
- promising concurrency or thread-safety properties beyond source behavior,
- adding serialization, persistence, benchmarking, or recovery features.