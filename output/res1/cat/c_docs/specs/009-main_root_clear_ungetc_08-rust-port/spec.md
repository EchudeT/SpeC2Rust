# spec.md

## Overview

This module provides internal stream-state handling for clearing any buffered `ungetc` data on a C `FILE *` stream. Its purpose is to remove pushed-back input while preserving correct stream position semantics. The Rust rewrite must implement the same functional behavior for the equivalent internal stream abstraction used by the port.

## Scope

In scope for this module:

- Clearing pushed-back input state associated with a stream.
- Supporting a variant that preserves the current effective read position when clearing that state.
- Supporting a full clear operation for the stream’s `ungetc` buffer state.

Out of scope:

- General stream I/O operations.
- Public user-facing file APIs beyond this internal behavior.
- Any guarantees not evidenced by the analyzed functions.

## Feature Specification

### Summary

The module defines internal behavior for removing `ungetc`-originated buffered data from a stream. It includes:

- A behavior that clears pushed-back input while preserving the stream position as observed by later operations.
- A behavior that clears the stream’s `ungetc` buffer state.

The Rust version must preserve these semantics for the port’s corresponding stream object.

### Required Behavior

1. The module must support a stream-targeted operation that clears buffered `ungetc` state.
2. The module must support a stream-targeted operation that clears buffered `ungetc` state without changing the effective file position that should remain in effect after the clear.
3. The preserving operation must be usable as part of the full clear behavior where position-sensitive cleanup is required.
4. The module must act only on the provided stream object’s `ungetc`-related state.

## User Scenarios & Testing

### Scenario 1: Stream has pushed-back input that must be discarded

A caller has a stream with data previously returned to the stream through `ungetc`-equivalent behavior. Before subsequent processing, the module is invoked to clear this pushed-back state.

Expected result:

- The pushed-back input is no longer pending on the stream.
- Later reads no longer consume the discarded pushed-back bytes or characters.

### Scenario 2: Stream position must remain consistent while clearing pushed-back input

A caller needs to remove pushed-back input but must preserve the stream’s current logical position so that later position-dependent behavior remains correct.

Expected result:

- The `ungetc` state is cleared.
- The stream position after clearing matches the position that should logically apply once pushed-back state is removed.

### Scenario 3: Stream has no active pushed-back input

A caller invokes the clear behavior on a stream that has no `ungetc` data buffered.

Expected result:

- The operation completes without introducing new stream state.
- The stream remains usable and position semantics are unchanged.

### Testing Guidance

The Rust rewrite should be tested with cases that verify:

- Clearing removes all pending pushed-back input from a stream-like object.
- Position-preserving clear does not alter the resulting logical position.
- Invoking clear on a stream with no pushed-back data is harmless.
- Effects are limited to the targeted stream instance.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide an internal operation that clears `ungetc` buffer state for a given stream.
  **Traceability**: `clear_ungetc_buffer` in `fflush.c`.

- **FR-2**: The module shall provide an internal operation that clears `ungetc` buffer state while preserving stream position semantics for the given stream.
  **Traceability**: `clear_ungetc_buffer_preserving_position` in `fflush.c`.

- **FR-3**: After the clear operation completes, previously pushed-back input shall no longer be available from the stream’s `ungetc` buffer state.
  **Traceability**: `clear_ungetc_buffer` in `fflush.c`.

- **FR-4**: After the position-preserving clear operation completes, the stream shall retain the effective position expected after removing pushed-back input.
  **Traceability**: `clear_ungetc_buffer_preserving_position` in `fflush.c`.

- **FR-5**: The module shall operate on the stream object supplied by the caller and shall not require unrelated module-global entities.
  **Traceability**: Both functions accept `FILE *fp` in `fflush.c`.

### Key Entities

- **Stream object**: The C module operates on a `FILE *` stream. In the Rust rewrite, this maps to the port’s internal stream representation that carries read position and any pushed-back input state.
  **Traceability**: Function parameters in `fflush.c`.

- **Ungetc buffer state**: Internal per-stream state representing input that has been pushed back and can be consumed again unless cleared.
  **Traceability**: Implied by `clear_ungetc_buffer` and `clear_ungetc_buffer_preserving_position` in `fflush.c`.

- **Stream position state**: Internal per-stream state needed to preserve correct logical position when clearing pushed-back input.
  **Traceability**: Implied by `clear_ungetc_buffer_preserving_position` in `fflush.c`.

## Success Criteria

- **SC-1**: A test with pending pushed-back input demonstrates that the clear operation removes that pending input from the targeted stream.
  **Traceability**: `clear_ungetc_buffer` in `fflush.c`.

- **SC-2**: A test with pending pushed-back input demonstrates that the position-preserving clear operation leaves the stream at the correct logical position after the clear.
  **Traceability**: `clear_ungetc_buffer_preserving_position` in `fflush.c`.

- **SC-3**: A test invoking clear on a stream without pushed-back input shows no incorrect state change and no loss of usability.
  **Traceability**: `clear_ungetc_buffer` in `fflush.c`.

- **SC-4**: A test using multiple stream instances shows that clearing one stream’s `ungetc` state does not affect another stream.
  **Traceability**: Both functions operate on a provided `FILE *fp` in `fflush.c`.