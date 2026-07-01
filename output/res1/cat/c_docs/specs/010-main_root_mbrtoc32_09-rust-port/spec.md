# spec.md

## Overview

- **Project**: cat
- **Module**: main_root_mbrtoc32_09
- **Category**: main_cluster
- **Source basis**: `mbrtoc32.c`
- **Primary function**: `mbrtoc32`

This module provides conversion from a multibyte character sequence to a single 32-bit character value, with stateful decoding support through `mbstate_t`. The Rust rewrite must preserve the observable behavior of the C module’s `mbrtoc32` interface and its handling of input buffers, conversion state, completion, incompleteness, and error signaling.

## Feature Specification

### Summary

The module implements a `mbrtoc32`-compatible conversion routine that decodes at most one multibyte character from a byte sequence and optionally stores the resulting `char32_t` value. The conversion is state-dependent and must support incremental decoding across calls using caller-provided conversion state.

### In-scope functionality

The Rust version must implement the following module behavior evidenced by `mbrtoc32.c` and the exported `mbrtoc32` function:

- Convert a multibyte input sequence into one Unicode scalar value represented as a 32-bit character result.
- Accept a nullable output pointer equivalent, allowing callers to request conversion progress without storing the decoded character.
- Accept a nullable input pointer equivalent for the state-dependent restart/reset behavior expected by `mbrtoc32`.
- Use caller-supplied conversion state when provided.
- Correctly distinguish:
  - successful conversion of a non-null character,
  - successful conversion of a null character,
  - incomplete multibyte input,
  - invalid multibyte input.
- Report consumed input length using `size_t`-compatible return semantics.
- Preserve and update conversion state according to whether a sequence is completed, incomplete, or invalid.

### Out of scope

The Rust rewrite must not introduce capabilities not evidenced by this module analysis, including:

- additional public conversion APIs,
- bulk string conversion APIs,
- locale management APIs beyond what is required for `mbrtoc32` behavior,
- guarantees unrelated to the original function contract.

## User Scenarios & Testing

### Scenario 1: Decode a complete single-byte character

A caller passes a buffer containing a complete multibyte character that is representable from the first byte sequence examined, along with a valid conversion state.

Expected behavior:

- The function recognizes the character as complete.
- The function returns the number of bytes consumed for that character.
- If an output location is provided, the decoded `char32_t` value is written there.
- Conversion state remains valid for future calls.

### Scenario 2: Decode a complete multibyte character

A caller passes a buffer containing a complete multibyte sequence requiring multiple bytes.

Expected behavior:

- The function consumes exactly the bytes belonging to the first character.
- The function returns the byte count consumed.
- The decoded character is produced when output storage is provided.
- State transitions reflect successful completion.

### Scenario 3: Decode incrementally across calls

A caller provides only part of a multibyte sequence in one call, then additional bytes in a later call while reusing the same `mbstate_t`.

Expected behavior:

- The first call reports incomplete input.
- The conversion state retains the partial sequence.
- The next call completes decoding when enough bytes are provided.
- The final result matches decoding the full sequence in one step.

### Scenario 4: Decode a null character

A caller provides input whose next character is the null wide character.

Expected behavior:

- The function reports successful conversion using the special return behavior defined for null-character conversion.
- If an output location is provided, the written value is the null `char32_t`.
- State after completion is suitable for decoding a subsequent character.

### Scenario 5: Detect invalid input

A caller passes a byte sequence that is not a valid multibyte encoding for the current conversion rules.

Expected behavior:

- The function reports conversion failure using the error return behavior of `mbrtoc32`.
- The conversion state is handled consistently with invalid-sequence semantics.
- No successful character result is produced from that call.

### Scenario 6: Call with no output storage

A caller wants to advance decoding or validate input without storing the resulting character value.

Expected behavior:

- The function still performs conversion.
- Return value and state updates are the same as if output storage had been provided.
- No decoded character write is required.

### Scenario 7: Use internal/default state behavior

A caller omits explicit state or uses the default state path expected by `mbrtoc32`.

Expected behavior:

- The function remains usable without requiring external state allocation in the simple case.
- Repeated calls behave consistently with the C module’s default-state semantics.

### Test coverage expectations

The Rust rewrite must include tests covering:

- successful one-byte conversion,
- successful multi-byte conversion,
- null-character conversion,
- incomplete sequence handling,
- invalid sequence handling,
- stateful continuation across multiple calls,
- operation with absent output destination,
- operation with default/internal state behavior if applicable to the C semantics.

## Requirements

### Functional Requirements

#### FR-1: Single-character multibyte decoding
The module shall provide `mbrtoc32` behavior that decodes at most one character from the supplied multibyte input sequence and returns a `size_t`-compatible status/length result.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-2: Optional output character storage
The module shall support calls where the decoded character destination is absent, while still performing conversion and returning the same conversion status that would apply if storage were present.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-3: Stateful conversion
The module shall use conversion state supplied by the caller to support decoding across multiple calls.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `mbstate_t`

#### FR-4: Incomplete input reporting
When the supplied bytes do not complete a valid character and additional bytes are required, the module shall report the incomplete-input condition without falsely reporting success.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-5: Invalid input reporting
When the supplied bytes form an invalid multibyte sequence under the module’s conversion rules, the module shall report the invalid-sequence condition.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-6: Null-character handling
When the next decoded character is the null wide character, the module shall return the distinct success result associated with null-character conversion and optionally store the null `char32_t` output.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `char32_t`

#### FR-7: Input consumption reporting
For successful non-null conversions, the module shall report how many input bytes were consumed for the decoded character.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### FR-8: State-consistent restart/default behavior
The module shall support the `mbrtoc32` calling patterns involving conversion state and the `s` parameter semantics expected by the C function contract, including behavior when callers rely on default/internal state handling.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `mbstate_t`

### Key Entities

#### Entity: multibyte input sequence
A contiguous byte sequence supplied through the `s` and `n` parameters. It is the source from which at most one character is decoded.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

#### Entity: output 32-bit character
A `char32_t` destination that receives the decoded character when output storage is provided.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `char32_t`

#### Entity: conversion state
An `mbstate_t` object that stores any state necessary to continue decoding across calls or to represent the current shift/partial-sequence state.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `mbstate_t`

#### Relationship of entities
The function reads bytes from the multibyte input sequence, interprets them using the conversion state, and may produce one output 32-bit character. The conversion state links successive calls when input is incomplete or when stateful decoding is required.

## Success Criteria

### SC-1: Correct successful conversion results
For complete valid input representing one non-null character, the Rust implementation returns the correct consumed-byte count and, when requested, the correct `char32_t` value.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-2: Correct null-character result
For valid input representing the null character, the Rust implementation produces the null output value when requested and returns the distinct null-conversion result expected of `mbrtoc32`.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `char32_t`

### SC-3: Correct incomplete-input behavior
For partial valid multibyte sequences, the Rust implementation reports incompleteness and allows completion by reusing the same conversion state with subsequent input.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `mbstate_t`

### SC-4: Correct invalid-input behavior
For invalid multibyte sequences, the Rust implementation reports the invalid-sequence condition and does not misreport successful conversion.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-5: State-preserving incremental decoding
A sequence decoded over multiple calls with shared state yields the same final character result as the equivalent complete sequence decoded in one call.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`, `mbstate_t`

### SC-6: Optional-output equivalence
Calls made without output character storage produce the same status and input-consumption behavior as corresponding calls with output storage, apart from the absence of a write.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`

### SC-7: Contract-compatible API behavior
The Rust rewrite exposes behaviorally equivalent `mbrtoc32` module functionality required by the original C module, with no missing supported call pattern evidenced by the source function signature and semantics.

**Traceability**: `mbrtoc32.c`, `mbrtoc32`