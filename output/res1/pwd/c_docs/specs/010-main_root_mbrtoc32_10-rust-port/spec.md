# spec.md

## Title

Rust Functional Specification for `main_root_mbrtoc32_10`

## Metadata

- **Project**: `pwd`
- **Module**: `main_root_mbrtoc32_10`
- **Category**: `main_cluster`
- **Source file**: `mbrtoc32.c`
- **Primary function**: `mbrtoc32`
- **Rust branch**: `010-main_root_mbrtoc32_10-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides the `mbrtoc32` functionality: conversion of a multibyte character sequence into a single 32-bit character value while tracking conversion state across calls.

The Rust rewrite must preserve the observable behavior of this module as a stateful multibyte-to-`char32` conversion interface, including handling of complete characters, incomplete input, invalid sequences, null-input state handling, and reporting through `mbrtoc32`-style result codes and state transitions.

## Feature Specification

### Summary

The module implements a single-character decoder for multibyte input. It accepts:

- an optional output location for the resulting 32-bit character,
- an optional byte input pointer,
- an input byte count,
- and a conversion state object.

It produces one conversion result at a time and updates conversion state so callers can continue decoding across split input boundaries.

### Functional Scope

The Rust version must implement the following behavior evidenced by the source module:

1. **Stateful decoding of one multibyte character at a time**
   - Convert the next character from a byte sequence into a 32-bit character value.
   - Use caller-provided conversion state to continue decoding when input is split across calls.

2. **Support for restartable conversion**
   - If the next character is incomplete because not enough input bytes are available, preserve enough state to resume on a later call with additional bytes.

3. **Support for stateless use**
   - If the caller supplies a conversion state, use it.
   - If the caller does not supply one, behavior must remain valid through an internal/default conversion state mechanism consistent with `mbrtoc32` semantics.

4. **Null-input state query/reset behavior**
   - Support calls where the input byte pointer is null, using the conversion machinery in the way standard `mbrtoc32` semantics require for examining or finalizing the current shift state.

5. **NUL character handling**
   - Correctly process an input sequence representing the null wide character and report the result using `mbrtoc32` return conventions.

6. **Error signaling for invalid multibyte input**
   - Detect invalid byte sequences.
   - Report failure using `mbrtoc32` error conventions and leave state behavior consistent with that contract.

7. **No overconsumption**
   - Consume only the bytes belonging to the next multibyte character or required by the defined error/incomplete-sequence behavior.

### Out of Scope

The Rust rewrite must not introduce new module responsibilities beyond the evidenced scope above. In particular, this specification does not require:

- exposing new public APIs beyond the module-equivalent functionality,
- batch string conversion,
- encoding output back to multibyte form,
- Unicode normalization or validation beyond what the conversion contract requires,
- thread-safety guarantees beyond the original module contract.

## User Scenarios & Testing

### Scenario 1: Decode a complete single-byte character

A caller passes a valid input buffer whose next character is complete in the provided bytes and a writable output location.

**Expected behavior**
- The function returns the number of bytes consumed for that character.
- The output location receives the decoded 32-bit character value.
- The conversion state is updated to reflect successful completion of that character.

**Test focus**
- Valid ASCII or other single-byte complete input.
- Correct byte count returned.
- Correct output value written.

### Scenario 2: Decode a complete multibyte character

A caller provides bytes that form one complete multibyte character and a conversion state.

**Expected behavior**
- The function returns the exact number of bytes consumed for the character.
- The output location receives the decoded 32-bit character.
- The state after completion is usable for decoding the following character.

**Test focus**
- Valid multibyte sequence.
- Exact consumption count.
- No extra bytes consumed.

### Scenario 3: Resume after partial input

A caller provides only the first part of a valid multibyte sequence, then later provides the remaining bytes using the same conversion state.

**Expected behavior**
- The first call reports an incomplete sequence.
- The state retains the partial conversion progress.
- The later call completes decoding and returns the correct byte consumption/result according to restartable-conversion semantics.

**Test focus**
- Split a valid multibyte sequence across multiple calls.
- Verify incomplete status on the first call.
- Verify successful completion on the second call.

### Scenario 4: Handle invalid input

A caller provides an invalid multibyte sequence.

**Expected behavior**
- The function reports an encoding error using `mbrtoc32` error signaling.
- The conversion state remains consistent with error handling expectations for subsequent use.

**Test focus**
- Invalid leading byte.
- Invalid continuation structure.
- Error code/return value behavior.

### Scenario 5: Process a null character

A caller passes input representing the null wide character.

**Expected behavior**
- The function reports completion using the special `mbrtoc32` return convention for a null character.
- The output location, when provided, receives a null 32-bit character value.
- The state ends in the initial conversion state.

**Test focus**
- Input beginning with `'\0'`.
- Special return value handling.

### Scenario 6: Call with null output pointer

A caller wants to advance or validate conversion without storing the decoded character.

**Expected behavior**
- The function performs conversion normally.
- No output write is required.
- Return value and state updates follow normal conversion rules.

**Test focus**
- `pwc == NULL` equivalent behavior.
- Same return code as when output storage is present.

### Scenario 7: Call with null input pointer for state-dependent behavior

A caller invokes the function with no input pointer to query/finalize behavior associated with the current shift state, as defined by `mbrtoc32` semantics.

**Expected behavior**
- The function behaves consistently with standard null-input `mbrtoc32` usage.
- The state and return value reflect whether the conversion state is initial or requires completion/reset handling.

**Test focus**
- Null input while state is initial.
- Null input after prior stateful decoding activity, if applicable to the active encoding semantics.

### Scenario 8: Zero-length input

A caller passes a non-null input pointer with `n == 0`.

**Expected behavior**
- The function reports that more input is required rather than consuming bytes or producing a character.

**Test focus**
- Zero-length buffer.
- No state corruption.

## Requirements

### Functional Requirements

#### FR-1: Single-character multibyte to 32-bit character conversion
The module shall provide `mbrtoc32` behavior that converts at most one next multibyte character from the input into a 32-bit character result.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-2: Stateful conversion across calls
The module shall use a conversion state object to continue decoding when a character sequence is split across multiple calls.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-3: Incomplete-sequence reporting
When available bytes are insufficient to complete the next character, the module shall report an incomplete conversion result without falsely consuming a complete character.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-4: Invalid-sequence reporting
When the input does not form a valid next multibyte character, the module shall report an error according to `mbrtoc32` conversion semantics.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-5: Null-character recognition
When the next converted character is the null character, the module shall report the null-character completion result required by `mbrtoc32` semantics and place a null 32-bit character in the output when output storage is supplied.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-6: Optional output storage
The module shall support operation when the output character pointer is absent, while preserving the same conversion and state behavior apart from omitting the write.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-7: Null-input handling
The module shall support calls where the input pointer is absent, following `mbrtoc32` semantics for operating on the current conversion state in that case.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-8: Provided-or-internal state usage
The module shall operate with a caller-supplied conversion state and also support the standard behavior used when no state object is supplied.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-9: Bounded consumption
The module shall not require or report consumption beyond the bytes needed to determine the next conversion result under successful, incomplete, null-character, or invalid-input cases.
**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### Key Entities

#### Entity: Multibyte input sequence
A byte sequence supplied by the caller that contains the next character to decode, possibly incomplete.
**Relationship**:
- is consumed by `mbrtoc32`,
- is interpreted relative to the current conversion state.

#### Entity: 32-bit character output
An optional caller-provided location that receives the decoded character value for successful conversion outcomes, including the null character case.
**Relationship**:
- is written by `mbrtoc32` on successful conversion when present.

#### Entity: Conversion state (`mbstate_t`-equivalent)
A state object that stores progress and shift/conversion context across calls.
**Relationship**:
- is read and updated by `mbrtoc32`,
- links partial input from one call to completion in a later call,
- may be caller-provided or replaced by internal/default state behavior when absent.

#### Entity: Conversion result code
The function result representing one of the standard `mbrtoc32` outcomes: bytes consumed, incomplete input, encoding error, or null-character completion.
**Relationship**:
- is returned by `mbrtoc32`,
- determines how the caller proceeds with subsequent input and state handling.

## Success Criteria

1. **Correct complete-character decoding**
   - For valid complete input sequences, the Rust module returns the correct `mbrtoc32`-style consumption count and decoded 32-bit character.
   - **Traceability**: FR-1, FR-9

2. **Correct restart behavior**
   - For valid sequences split across calls, the Rust module first reports incomplete input and then successfully completes decoding when remaining bytes are supplied with the same state.
   - **Traceability**: FR-2, FR-3

3. **Correct invalid-input behavior**
   - For invalid multibyte sequences, the Rust module reports an error result matching `mbrtoc32` semantics.
   - **Traceability**: FR-4

4. **Correct null-character behavior**
   - For input representing the null character, the Rust module returns the null-character completion result and resets/ends in the appropriate initial state.
   - **Traceability**: FR-5

5. **Optional output support**
   - Test cases with and without output storage produce the same conversion status and byte-consumption behavior.
   - **Traceability**: FR-6

6. **Null-input conformance**
   - Calls with absent input behave consistently with `mbrtoc32` null-input semantics for the maintained conversion state.
   - **Traceability**: FR-7, FR-8

7. **State source conformance**
   - Equivalent decoding scenarios succeed both with caller-supplied state and with the module behavior used when no state object is supplied.
   - **Traceability**: FR-8

8. **No overconsumption**
   - Tests verify that reported consumption never exceeds the bytes required for the next conversion outcome.
   - **Traceability**: FR-9

## Acceptance Test Matrix

| Test ID | Scenario | Expected Result | Traceability |
|---|---|---|---|
| AT-1 | Complete single-byte character | Returns consumed byte count, writes correct char32 value | FR-1, FR-9 |
| AT-2 | Complete multibyte character | Returns exact multibyte length, writes correct char32 value | FR-1, FR-9 |
| AT-3 | Partial then resumed multibyte character | First call incomplete; second call completes with preserved state | FR-2, FR-3 |
| AT-4 | Invalid sequence | Returns encoding error result | FR-4 |
| AT-5 | Null character input | Returns null-character completion result and null output value | FR-5 |
| AT-6 | Null output pointer | Same status/consumption as normal conversion, no output write required | FR-6 |
| AT-7 | Null input pointer | Behaves per `mbrtoc32` state/null-input semantics | FR-7, FR-8 |
| AT-8 | No state object supplied | Conversion remains valid using default/internal state behavior | FR-8 |
| AT-9 | Zero-length input | Reports incomplete input / need for more bytes without invalid consumption | FR-3, FR-9 |

## Notes for Rust Port

The Rust rewrite should preserve the C module’s externally observable conversion contract rather than its internal organization. Any Rust representation used internally must still support:

- restartable multibyte decoding,
- `mbrtoc32`-equivalent outcome reporting,
- optional output and optional state behavior,
- correct handling of null input, null character, incomplete input, and invalid sequences.