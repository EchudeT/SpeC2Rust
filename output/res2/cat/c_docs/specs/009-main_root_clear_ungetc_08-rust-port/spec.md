# spec.md

## Title
Rust Functional Specification for `main_root_clear_ungetc_08`

## Metadata
- Project: `cat`
- Module: `main_root_clear_ungetc_08`
- Category: `main_cluster`
- Source basis: `fflush.c`
- Rust branch: `009-main_root_clear_ungetc_08-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides behavior for clearing any active `ungetc`-supplied input state on a file stream. It covers two related cases:

1. clearing the pushed-back input state while preserving the stream position, and
2. clearing the pushed-back input state when preservation is not applicable or not used.

The Rust rewrite must preserve this functional boundary: it must remove the effect of prior pushed-back input on a stream in the same circumstances handled by the source module, with special handling for the case where stream position must be preserved.

## Scope
In scope:
- Behavior equivalent to clearing `ungetc`-related buffered state on a stream.
- Behavior equivalent to preserving the stream position during that clearing operation when required.
- Internal helper behavior associated with stream state normalization before later stream operations.

Out of scope:
- General flushing semantics beyond what is needed for `ungetc` state clearing.
- New public APIs beyond the module behavior evidenced by the source functions.
- Any capability not directly supported by the source module functions.

## Feature Specification

### Summary
The module normalizes a `FILE` stream by removing pending pushed-back input created by `ungetc`. It supports a position-preserving path and a more general clearing path. The Rust version must implement equivalent behavior for the stream abstraction used in the port.

### Functional Behavior
- Detect and clear active pushed-back input state associated with a stream.
- Provide a mode of clearing that preserves the stream’s logical file position.
- Provide a general clearing path for stream state cleanup.
- Operate on an existing stream object and update only the state relevant to pushed-back input and stream position consistency.

### Rust Port Intent
The Rust rewrite must expose module behavior that can be used where the C module would clear `ungetc` state on a stream. The Rust implementation may adapt to Rust I/O abstractions, but it must preserve these observable outcomes:
- pushed-back input no longer remains active after clearing;
- position-preserving clearing does not change the effective stream position;
- general clearing leaves the stream in a valid state for subsequent stream use.

## User Scenarios & Testing

### Scenario 1: Clear pushed-back input while preserving file position
A stream has received one or more characters via `ungetc`, and subsequent logic needs that pushed-back state removed without changing the current file position.

Expected behavior:
- The pushed-back characters cease to be available as pending input.
- The stream position after clearing matches the position before clearing.
- Later reads observe the underlying stream content from the preserved position, not the cleared pushed-back characters.

Testing guidance:
- Create a seekable input stream.
- Read from it, apply pushed-back input, invoke the position-preserving clearing behavior, then verify:
  - stream position is unchanged;
  - the next read does not return the previously pushed-back character unless it exists naturally at that position.

### Scenario 2: Clear pushed-back input during stream state cleanup
A stream has active `ungetc` state and requires normalization before another operation that should not observe that pushed-back state.

Expected behavior:
- The pending pushed-back input is discarded.
- The stream remains usable after the cleanup.

Testing guidance:
- Create a stream with active pushed-back input.
- Invoke the general clearing behavior.
- Verify that subsequent stream operations no longer observe the pushed-back character as buffered state.

### Scenario 3: Invoke clearing on a stream without active pushed-back input
A stream is already in a normal state and the clearing logic is applied defensively.

Expected behavior:
- The stream remains valid and usable.
- No spurious pushed-back input appears.
- Stream position is not incorrectly altered by the position-preserving path.

Testing guidance:
- Apply both clearing variants to a stream without prior `ungetc`.
- Verify continued correct reading/position behavior.

## Requirements

### Functional Requirements
1. The module shall support clearing `ungetc`-derived buffered input state from a stream.
   Traceability: `clear_ungetc_buffer` in `fflush.c:49-70`

2. The module shall support clearing `ungetc`-derived buffered input state in a way that preserves the stream position.
   Traceability: `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

3. The module shall leave the stream in a usable post-clear state for subsequent stream operations.
   Traceability: `clear_ungetc_buffer` in `fflush.c:49-70`

4. When the position-preserving behavior is used, the module shall maintain the stream’s effective position before and after the clear operation.
   Traceability: `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

5. Applying the module’s clearing behavior shall remove the effect of previously pushed-back input so that later reads do not consume that cleared `ungetc` state.
   Traceability: `clear_ungetc_buffer` in `fflush.c:49-70`; `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

### Key Entities
- **Stream object**: The module operates on a file stream (`FILE *` in the source). It is the state holder for current position and any active pushed-back input.
- **Pushed-back input state**: Logical buffered input created by prior `ungetc` use. This is the state the module removes.
- **Stream position**: The logical current location in the stream. It must be preserved by the position-preserving clearing behavior.

Relationship summary:
- The stream object contains both current position state and any pushed-back input state.
- The clearing operations act on the stream object to remove pushed-back input state.
- The position-preserving variant must do so without changing the stream position.

## Success Criteria
1. For a stream with active pushed-back input, invoking the general clear behavior results in the pushed-back input no longer being returned by subsequent reads.
   Traceability: `clear_ungetc_buffer` in `fflush.c:49-70`

2. For a seekable stream with active pushed-back input, invoking the position-preserving clear behavior leaves the stream at the same effective position as before invocation.
   Traceability: `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

3. For a stream without active pushed-back input, invoking either clear behavior does not invalidate subsequent normal stream use.
   Traceability: `clear_ungetc_buffer` in `fflush.c:49-70`; `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

4. After either clear behavior completes, the stream state is consistent with the absence of pending `ungetc` data.
   Traceability: `clear_ungetc_buffer` in `fflush.c:49-70`; `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

## Acceptance Notes
- The Rust port may adapt internal design to Rust idioms, but it must not widen the module scope beyond clearing `ungetc`-related stream state.
- Validation should focus on observable stream behavior: pending pushed-back input removal, preservation of position where required, and continued stream usability.