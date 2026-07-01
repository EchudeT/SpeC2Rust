# spec.md

## Title

Functional Specification: `main_root_mbrtoc32_09`

## Document Control

- Project: `cat`
- Module: `main_root_mbrtoc32_09`
- Category: `main_cluster`
- Source file: `mbrtoc32.c`
- Primary function: `mbrtoc32`
- Rust branch target: `010-main_root_mbrtoc32_09-rust-port`
- Generation date: 2026-06-07

## Overview

This module provides conversion of a multibyte character sequence into a single 32-bit character value while tracking conversion state across calls. The Rust rewrite must preserve the observable behavior of the C module’s `mbrtoc32` functionality, including handling of input presence or absence, bounded input length, and caller-supplied conversion state.

The module’s functional boundary is limited to single-character decoding through the `mbrtoc32` interface. No additional public capabilities are evidenced by the source analysis and therefore none are part of this specification.

## Feature Specification

### Feature Summary

Implement a Rust equivalent of the module behavior exposed by `mbrtoc32`, which:

- accepts an optional destination for the decoded 32-bit character,
- accepts a pointer to input bytes and a maximum byte count,
- accepts a conversion state object,
- converts at most one multibyte character from the input sequence,
- reports the conversion result through a `size_t`-compatible return value,
- updates the conversion state according to the progress of decoding.

### In-Scope Behavior

The Rust version must implement the behavior represented by the C module’s `mbrtoc32` entry point:

- decode one character from a multibyte input sequence,
- write the decoded value when an output location is provided,
- support stateful decoding using caller-provided state,
- support repeated calls that continue an incomplete conversion,
- distinguish between completed conversion, incomplete input, and invalid input through return behavior consistent with the source module’s contract.

### Out of Scope

The Rust version must not introduce module capabilities not evidenced by the input, including:

- new public conversion APIs,
- bulk string conversion,
- independent locale-management APIs,
- serialization of conversion state,
- concurrency guarantees beyond the source module contract.

## User Scenarios & Testing

### Scenario 1: Decode a complete multibyte character in one call

A caller provides:

- an output location for a 32-bit character,
- a non-null byte sequence,
- a byte count sufficient for one complete character,
- a conversion state object.

Expected behavior:

- the function consumes the bytes for exactly one multibyte character,
- stores the resulting 32-bit character in the output location,
- returns a value indicating successful conversion of one character,
- leaves the state ready for the next character conversion.

Test coverage:

- verify output character value,
- verify returned consumed-byte count or success code,
- verify state remains usable for the next call.

### Scenario 2: Continue decoding after incomplete input

A caller provides only part of a multibyte character, then later provides the remaining bytes using the same conversion state.

Expected behavior:

- the first call reports incomplete input without falsely producing a character,
- the conversion state retains partial decoding progress,
- the second call completes the character using the same state,
- the final output matches the intended decoded 32-bit character.

Test coverage:

- verify incomplete-input return on the first call,
- verify no completed character is reported prematurely,
- verify successful completion on the second call,
- verify state transitions are consistent across calls.

### Scenario 3: Detect invalid multibyte input

A caller provides bytes that do not form a valid multibyte character under the module’s conversion rules.

Expected behavior:

- the function reports an invalid-sequence result,
- conversion state behavior matches the source module’s contract for invalid input,
- no successful character conversion is reported.

Test coverage:

- verify invalid-input return behavior,
- verify no valid output character is claimed,
- verify subsequent handling of state follows source-compatible semantics.

### Scenario 4: Query behavior with no output destination

A caller wants the module to parse a character without storing it and passes no output destination.

Expected behavior:

- the function still processes one multibyte character,
- the return value reflects the same conversion status as if an output location had been supplied,
- state advancement remains correct.

Test coverage:

- verify success, incomplete, and invalid cases with a missing output destination,
- verify byte-consumption behavior remains consistent.

### Scenario 5: Handle absent input pointer according to state-based semantics

A caller invokes the function without a new input byte sequence in order to exercise the interface’s state-sensitive behavior.

Expected behavior:

- the function behaves consistently with the source `mbrtoc32` contract for a null input pointer,
- any effect on conversion state or return value matches the C module behavior.

Test coverage:

- verify behavior when input pointer is absent and state is initial,
- verify behavior when input pointer is absent and state is non-initial,
- verify outputs and return values match source-compatible expectations.

## Requirements

### Functional Requirements

#### FR-1: Single-character multibyte decoding

The module shall provide behavior equivalent to the C function `mbrtoc32`, converting at most one multibyte character from the supplied input and reporting the result through its return value.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### FR-2: Optional output character storage

The module shall support operation with or without a caller-provided destination for the decoded 32-bit character. When a destination is provided and conversion succeeds, the decoded character shall be written to it.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### FR-3: Bounded input consumption

The module shall honor the caller-supplied maximum input length and shall not require more than the provided bound to determine whether a character is complete, incomplete, or invalid.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### FR-4: Stateful decoding across calls

The module shall use a conversion state object to preserve partial decoding context across calls and support completion of a multibyte character when input arrives in multiple segments.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### FR-5: Incomplete-sequence reporting

When the provided bytes are insufficient to complete one multibyte character, the module shall report an incomplete-conversion result without claiming successful character completion.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### FR-6: Invalid-sequence reporting

When the provided bytes do not form a valid multibyte character under the source module’s rules, the module shall report an invalid-sequence result consistent with the `mbrtoc32` contract.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### FR-7: Null-input handling

The module shall support invocation with no new input byte sequence and shall produce return behavior and state effects consistent with the source `mbrtoc32` interface semantics.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### FR-8: State update on successful conversion

After successful completion of a character, the module shall update the conversion state so that subsequent calls can continue decoding following characters in a source-compatible manner.

Traceability: `mbrtoc32.c`, `mbrtoc32`

### Key Entities

#### `char32_t` equivalent

The decoded output entity is a single 32-bit character value. It is produced by successful conversion and written only when the caller supplies an output destination.

Relationship:
- produced from the input multibyte sequence,
- stored through the output parameter of `mbrtoc32`.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### Input multibyte byte sequence

The input entity is a byte sequence plus an explicit byte-count bound. It represents source data for at most one character conversion per call.

Relationship:
- consumed by `mbrtoc32`,
- interpreted according to the active conversion state,
- may be complete, incomplete, invalid, or absent.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### `mbstate_t` equivalent

The conversion state entity carries decoding context across calls. It enables stateful processing when a character’s byte sequence is split across multiple invocations or when interface semantics depend on prior state.

Relationship:
- supplied by the caller,
- read and updated by `mbrtoc32`,
- governs continuation and reset-compatible behavior.

Traceability: `mbrtoc32.c`, `mbrtoc32`

#### `size_t`-compatible conversion result

The result entity communicates conversion outcome, including successful character consumption and distinguished non-success outcomes defined by the source interface.

Relationship:
- returned by `mbrtoc32`,
- interpreted together with output storage and state changes.

Traceability: `mbrtoc32.c`, `mbrtoc32`

## Success Criteria

### SC-1: Source-compatible successful decoding

For representative valid multibyte inputs that encode a single character within the provided byte bound, the Rust implementation returns the same class of success result as the C module and produces the same decoded 32-bit character value.

Traceability: `mbrtoc32.c`, `mbrtoc32`

### SC-2: Correct handling of segmented input

For test cases where one character is split across multiple calls using the same conversion state, the Rust implementation matches the source module in:
- incomplete result on insufficient input,
- successful completion after remaining bytes are provided,
- final decoded character value.

Traceability: `mbrtoc32.c`, `mbrtoc32`

### SC-3: Correct invalid-input signaling

For representative invalid multibyte sequences, the Rust implementation reports invalid conversion in the same outcome class as the source module and does not report a successful character conversion.

Traceability: `mbrtoc32.c`, `mbrtoc32`

### SC-4: Correct null-output behavior

When no output destination is supplied, the Rust implementation preserves source-compatible return behavior and state progression for successful, incomplete, and invalid cases.

Traceability: `mbrtoc32.c`, `mbrtoc32`

### SC-5: Correct null-input behavior

When invoked without a new input byte sequence, the Rust implementation matches the source module’s observable return behavior and state effects for both initial and non-initial conversion states.

Traceability: `mbrtoc32.c`, `mbrtoc32`

### SC-6: No expansion beyond module boundary

The Rust rewrite exposes only the functionality required to support source-compatible `mbrtoc32` behavior for this module and does not require additional public module features not evidenced by the source analysis.

Traceability: `mbrtoc32.c`, `mbrtoc32`