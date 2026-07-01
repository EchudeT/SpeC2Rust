# spec.md

## Overview

- **Project**: `cat`
- **Module**: `main_root_clear_ungetc_08`
- **Category**: `main_cluster`
- **Source basis**: `fflush.c`
- **Rust target branch**: `009-main_root_clear_ungetc_08-rust-port`
- **Generation date**: `2026-06-09`

## Feature Specification

This module provides internal behavior for clearing any pending `ungetc` state associated with a C `FILE *` stream.

The analyzed module contains two internal functions with distinct behavioral roles:

- `clear_ungetc_buffer_preserving_position(FILE *fp)`: clears pushed-back input state while preserving the stream position.
- `clear_ungetc_buffer(FILE *fp)`: clears pushed-back input state, including the case where preserving position may not be possible or may require a different handling path.

### Functional intent

In C stdio, `ungetc` can place bytes back onto an input stream so they are returned by subsequent reads. This module exists to discard that pushed-back state so the stream no longer reports those unread pushed-back bytes as pending input.

The Rust rewrite must preserve this functional boundary:

- It must support clearing pending pushed-back input state for a stream abstraction corresponding to the original `FILE *`.
- It must support a mode or behavior that clears such state while preserving the stream's logical position.
- It must support the broader clearing operation represented by the general clear function.

### Out of scope

No additional public CLI behavior, formatting behavior, I/O transformation, thread-safety guarantees, persistence, FFI surface, or error recovery features are evidenced by this module analysis and therefore are not part of this specification.

## User Scenarios & Testing

### Scenario 1: Discard pushed-back input before continuing stream processing

A stream has received one or more bytes through `ungetc`. Before further processing continues, the internal pushed-back state must be discarded so future reads operate on the stream without returning those pushed-back bytes.

**Expected result**:
- The pending `ungetc` state is removed.
- Subsequent reads are not satisfied from the discarded pushed-back buffer.

### Scenario 2: Clear pushed-back input without changing the current stream position

A caller needs to remove pushed-back bytes but must preserve the stream position as observed by the rest of the program.

**Expected result**:
- Pushed-back bytes are cleared.
- The effective file position after the operation matches the position expected for the underlying stream state, excluding the discarded pushed-back bytes.

### Scenario 3: Clear stream pushback state as part of broader stream normalization

Before a flush-like or state-resetting operation proceeds, any `ungetc` state on the stream must be cleared so the stream state is consistent.

**Expected result**:
- The stream no longer retains pending pushback state.
- The clearing operation completes for streams where pushback state exists and does not introduce extra observable behavior for streams where no pushback exists.

### Testing guidance

The Rust version should be tested with stream fixtures that can model pushed-back input and stream position.

Minimum test coverage should include:

- Clearing when pushed-back data exists.
- Clearing when no pushed-back data exists.
- Clearing while preserving position.
- Verifying that post-clear reads do not return previously pushed-back bytes.
- Verifying that stream position is preserved for the position-preserving path.

## Requirements

### Functional Requirements

#### FR-1: Clear pending pushed-back input state
The module shall provide behavior equivalent to clearing any pending `ungetc` buffer associated with a stream.

**Traceability**: `clear_ungetc_buffer` in `fflush.c`

#### FR-2: Support position-preserving clear behavior
The module shall provide behavior equivalent to clearing pushed-back input state while preserving the stream position.

**Traceability**: `clear_ungetc_buffer_preserving_position` in `fflush.c`

#### FR-3: Operate on stream objects corresponding to C `FILE *`
The module shall operate on a stream abstraction representing the same role as the original C `FILE *` input.

**Traceability**: both analyzed functions in `fflush.c`

#### FR-4: Be safe when no pushed-back input is present
When the stream has no pending `ungetc` state, the clear operation shall complete without requiring any additional externally visible effect beyond leaving the stream with no pushed-back state.

**Traceability**: implied by the purpose of `clear_ungetc_buffer` as a clearing operation in `fflush.c`

### Key Entities

#### Stream handle
A stream object corresponding to C `FILE *`. This is the only evidenced entity directly manipulated by this module.

#### Pushed-back input state
Internal stream-associated state representing bytes previously returned to the input stream via `ungetc`.

#### Relationship
The pushed-back input state is attached to a stream handle. The module's behavior acts on that relationship by removing the pushed-back state, optionally while preserving stream position.

## Success Criteria

### SC-1: Pushed-back data is removed
For a stream with pending pushed-back bytes, after the Rust module's clear behavior is applied, subsequent reads shall not return the discarded pushed-back bytes.

**Traceability**: `clear_ungetc_buffer` in `fflush.c`

### SC-2: Position-preserving clear maintains stream position
For the position-preserving behavior, the stream position observed after the operation shall match the expected position-preserving outcome for the underlying stream.

**Traceability**: `clear_ungetc_buffer_preserving_position` in `fflush.c`

### SC-3: No-op safety when no pushback exists
For a stream with no pending pushed-back input, invoking the clear behavior shall leave the stream usable and without newly introduced pushed-back state.

**Traceability**: `clear_ungetc_buffer` in `fflush.c`

### SC-4: Functional parity with analyzed module boundary
The Rust rewrite shall implement only the stream pushback-clearing responsibilities evidenced by the analyzed functions, with no reduction of the two distinct behaviors: general clear and position-preserving clear.

**Traceability**: both analyzed functions in `fflush.c`