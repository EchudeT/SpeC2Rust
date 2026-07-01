# spec.md

## Title

Rust Functional Specification for `main_root_mbrtoc32_09`

## Summary

This module provides the `mbrtoc32` conversion behavior for translating a multibyte character sequence into a single 32-bit wide character value while tracking conversion state across calls. The Rust rewrite must preserve the observable behavior of the C module in `mbrtoc32.c`, including handling of complete characters, incomplete input, state-dependent decoding, null input conventions, and error signaling.

## Scope

In scope:

- Functional behavior of `mbrtoc32` as implemented by this module.
- Stateful decoding of at most one multibyte character from byte input.
- Reporting of conversion outcomes through the function result and output parameters.

Out of scope:

- Any API beyond the behavior corresponding to `mbrtoc32`.
- Capabilities not evidenced by `mbrtoc32.c`.

## Feature Specification

### Feature: Convert one multibyte sequence to one `char32_t`

The module exposes behavior equivalent to C `mbrtoc32(char32_t *pwc, const char *s, size_t n, mbstate_t *ps)`.

The Rust version must implement:

- Conversion of the next multibyte character from the byte sequence `s`, limited by `n` available bytes.
- Optional writing of the decoded Unicode scalar value / 32-bit character result to `pwc` when an output location is supplied.
- Use of a caller-provided conversion state `ps`, including support for stateful decoding across multiple calls.
- The null-input convention of `mbrtoc32`, where `s == NULL` is treated as a request tied to the current conversion state rather than fresh byte input.
- Distinct result cases for:
  - successful decoding of a non-null character,
  - successful decoding of the null character,
  - incomplete but potentially valid input,
  - invalid input sequence.

### Behavioral boundaries

The Rust version must preserve these observable boundaries:

- It decodes at most one character per call.
- It does not require output storage in order to consume input and update state.
- It distinguishes incomplete input from invalid input.
- It uses and updates conversion state consistently across split-input scenarios.
- It supports reset/finalization behavior implied by calling with null input and the current state.

## User Scenarios & Testing

### Scenario 1: Decode a complete multibyte character in one call

A caller provides:

- input bytes beginning with a complete multibyte character,
- the available byte count,
- an initial conversion state.

Expected behavior:

- The function returns the number of bytes consumed for that character, unless the decoded character is the null character.
- The decoded 32-bit character is stored when an output pointer is supplied.
- The conversion state is updated to the post-character state.

Testing focus:

- Single-byte character input.
- Multi-byte character input that completes within `n`.

### Scenario 2: Decode with no output destination

A caller wants to advance through input but does not need the decoded value.

Expected behavior:

- The function still performs conversion.
- Input consumption and state transitions match the behavior when an output destination is provided.
- No output write is required.

Testing focus:

- Same byte sequence decoded once with output storage and once without; return values and state progression must match.

### Scenario 3: Handle a null character conversion

A caller provides input that decodes to the multibyte representation of the null character.

Expected behavior:

- The function reports the special null-character success result defined by `mbrtoc32`.
- State after conversion corresponds to completion of that character.

Testing focus:

- Input containing the encoding of `U+0000`.

### Scenario 4: Handle incomplete input across calls

A caller provides only the first part of a valid multibyte sequence, then later provides the remaining bytes using the same conversion state.

Expected behavior:

- First call reports incomplete input without producing an invalid-sequence result.
- Subsequent call with remaining bytes completes the character.
- Final decoded character and total consumption are consistent with decoding the full sequence at once.

Testing focus:

- Split valid multibyte sequences at every possible boundary.

### Scenario 5: Reject an invalid multibyte sequence

A caller provides bytes that are not valid for the current locale/encoding state.

Expected behavior:

- The function reports an encoding error result.
- The conversion state behavior matches `mbrtoc32` error semantics.

Testing focus:

- Invalid leading byte.
- Invalid continuation pattern.
- Invalid sequence relative to current state.

### Scenario 6: Use null input to act on existing state

A caller invokes the function with `s == NULL` and a conversion state that may or may not contain a pending partial character.

Expected behavior:

- The function behaves according to `mbrtoc32` null-input semantics.
- It does not invent fresh input bytes.
- It either completes/reset-checks state or reports that no complete character can be produced from the current state, consistent with the source module behavior.

Testing focus:

- Null input with initial state.
- Null input after partial-sequence state has been established.

## Requirements

### Functional Requirements

#### FR-1: Single-character conversion
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

The module shall process byte input as a request to decode no more than one multibyte character per invocation.

#### FR-2: Bounded input consumption
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

The module shall limit decoding to at most `n` bytes supplied by the caller and shall not require access beyond those bytes.

#### FR-3: Optional character output
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

The module shall support calls where the destination for the decoded `char32_t` value is absent, while still performing conversion semantics and state updates.

#### FR-4: Stateful decoding
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

The module shall accept and use a conversion state object corresponding to `mbstate_t` so that partial multibyte sequences can be continued across calls.

#### FR-5: Null-input semantics
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

The module shall support the `s == NULL` calling form and apply conversion/reset behavior based on the current conversion state, consistent with `mbrtoc32`.

#### FR-6: Successful non-null character result
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

When a valid non-null character is decoded, the module shall report success by returning the number of bytes consumed for that character.

#### FR-7: Successful null character result
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

When the input decodes to the null wide character, the module shall report the distinct success result defined for `mbrtoc32` null-character conversion.

#### FR-8: Incomplete-sequence result
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

When the supplied bytes are insufficient to complete a potentially valid character, the module shall report the incomplete-input result rather than an invalid-sequence result.

#### FR-9: Invalid-sequence result
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

When the supplied bytes do not form a valid multibyte character for the current state, the module shall report an encoding error result.

#### FR-10: State progression consistency
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

The module shall update conversion state consistently so that decoding the same valid sequence in one call or across multiple calls yields the same final character result and completed state.

### Key Entities

#### Entity: Input byte sequence
Traceable to: `mbrtoc32(char32_t *pwc, const char *s, size_t n, mbstate_t *ps)`

A caller-supplied pointer-plus-length view of candidate multibyte input. It may represent fresh bytes for decoding or be null to trigger null-input semantics.

Relationship:

- Consumed by the conversion operation.
- Interpreted relative to the active conversion state.

#### Entity: Conversion state
Traceable to: `mbrtoc32(char32_t *pwc, const char *s, size_t n, mbstate_t *ps)`

A caller-supplied state object corresponding to `mbstate_t` that carries any partial decoding context between invocations.

Relationship:

- Updated by each call.
- Determines whether partial input can be resumed or whether null input has pending state to resolve.

#### Entity: Output character slot
Traceable to: `mbrtoc32(char32_t *pwc, const char *s, size_t n, mbstate_t *ps)`

An optional destination corresponding to `char32_t *pwc` for the decoded single character.

Relationship:

- Written only when a character result is produced and output storage is supplied.

#### Entity: Conversion result code
Traceable to: `mbrtoc32.c`, function `mbrtoc32`

The returned `size_t` value signaling one of the `mbrtoc32` outcome classes: bytes consumed for success, special null-character success, incomplete input, or invalid sequence.

Relationship:

- Interprets the outcome of processing input under the given state.
- Drives caller decisions about retrying with more input, handling errors, or advancing input.

## Success Criteria

### SC-1: Correct success return for complete characters
Traceable to: FR-1, FR-2, FR-6

For representative valid single-byte and multibyte inputs, the Rust module returns the same success class and consumed-byte count as the C module for one-character decoding.

### SC-2: Correct null-character behavior
Traceable to: FR-7

For input encoding the null character, the Rust module returns the distinct null-character success result and leaves the state consistent with completed conversion.

### SC-3: Correct incomplete-input behavior
Traceable to: FR-4, FR-8, FR-10

For valid multibyte sequences split before completion, the Rust module reports incomplete input on the prefix call and completes successfully when the remaining bytes are later provided with the same state.

### SC-4: Correct invalid-input behavior
Traceable to: FR-9

For representative malformed sequences, the Rust module reports the invalid-sequence result matching the source module’s behavior class.

### SC-5: Optional output support
Traceable to: FR-3

For the same valid input and initial state, calls with and without an output destination produce identical return values and equivalent final conversion state.

### SC-6: Null-input semantics preserved
Traceable to: FR-5

For calls made with null input under both initial and partially progressed state, the Rust module exhibits the same outcome class as the C module.

### SC-7: Split-vs-unsplit consistency
Traceable to: FR-10

For any tested valid character sequence that can be decoded both in a single call and across multiple calls, the Rust module yields the same decoded `char32_t` value and equivalent completed state.