# spec.md

## Title

Functional Specification: `main_root_close-stream.c_20`

## Metadata

- Project: `cat`
- Module: `main_root_close-stream.c_20`
- Category: `main_cluster`
- Source file: `close-stream.c`
- Primary function: `close_stream(FILE *stream) -> int`
- Rust branch target: `021-main_root_close_stream.c_20-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides a single stream-finalization operation for C `FILE*` output streams. Its purpose is to close a stream while reporting whether the stream encountered an output error either before or during closure.

The Rust rewrite must preserve this observable behavior: callers must be able to finalize a previously opened stream and receive a success or failure result that reflects stream error state and close outcome.

## Feature Specification

### Summary

The module implements a checked stream close operation.

### Functional behavior

The Rust version must implement behavior equivalent to the source module:

- Accept a handle representing an open C-style buffered stream.
- Finalize the stream by attempting to close it.
- Detect whether the stream had entered an error state before closure.
- Return success only when no prior stream error is present and the close operation succeeds.
- Return failure when:
  - the stream already has an error condition before close, or
  - the close operation itself fails.

### Scope boundary

This module is limited to checked stream closure behavior. It does not define stream creation, reading, writing, buffering policy, retry behavior, recovery, or higher-level application output policy.

## User Scenarios & Testing

### Scenario 1: Clean output stream closes successfully

A caller has written data to an output stream and wants to finalize it. The stream has no pending error state, and the underlying close succeeds.

Expected result:
- The checked close operation reports success.

Test guidance:
- Use a writable stream with no induced write or flush error.
- Invoke the module operation once.
- Verify that the result indicates success.

### Scenario 2: Stream already has an output error before close

A caller has written to a stream, but the stream is already marked with an error condition before finalization.

Expected result:
- The checked close operation reports failure even if the subsequent close completes.

Test guidance:
- Create or simulate a stream whose error indicator is set before close.
- Invoke the module operation.
- Verify that the result indicates failure.

### Scenario 3: Close operation fails

A caller finalizes a stream that has no prior error flag, but the underlying close action fails.

Expected result:
- The checked close operation reports failure.

Test guidance:
- Use a test double, fixture, or controlled environment where close fails.
- Invoke the module operation.
- Verify that the result indicates failure.

### Scenario 4: Error state and close are both problematic

A caller finalizes a stream that already has an error condition and whose close also fails.

Expected result:
- The checked close operation reports failure.

Test guidance:
- Simulate both a pre-existing stream error and a close failure.
- Verify that the operation still reports failure.

## Requirements

### Functional Requirements

#### FR-1: Checked stream finalization
The module shall provide an operation that accepts a stream handle and attempts to close that stream.

**Traceability:** `close-stream.c`, `close_stream`

#### FR-2: Pre-close error observation
The operation shall consider whether the stream is already in an error state before closure.

**Traceability:** `close-stream.c`, `close_stream`

#### FR-3: Close outcome observation
The operation shall consider whether the close action itself succeeds or fails.

**Traceability:** `close-stream.c`, `close_stream`

#### FR-4: Failure on prior stream error
If the stream has an error state before close, the operation shall report failure.

**Traceability:** `close-stream.c`, `close_stream`

#### FR-5: Failure on close error
If the close action fails, the operation shall report failure.

**Traceability:** `close-stream.c`, `close_stream`

#### FR-6: Success only when both checks pass
The operation shall report success only when the stream has no prior error condition and the close action succeeds.

**Traceability:** `close-stream.c`, `close_stream`

#### FR-7: Integer-style success/failure result
The operation shall expose a result compatible with the source module’s integer success/failure contract.

**Traceability:** `close-stream.c`, `close_stream`

### Key Entities

#### Stream handle
A buffered C stream handle representing the stream to be finalized.

- Source form: `FILE *`
- Role: input to the checked close operation

**Traceability:** `close-stream.c`, `close_stream`

#### Checked close result
An integer success/failure status returned by the operation.

- Role: communicates whether the stream was closed without any observed prior stream error or close failure

**Traceability:** `close-stream.c`, `close_stream`

#### Relationship
The checked close operation consumes a stream handle as input and produces an integer status as output indicating overall success or failure of finalization.

**Traceability:** `close-stream.c`, `close_stream`

## Success Criteria

### SC-1: Correct success case
When invoked on a stream with no prior error state and a successful close, the Rust implementation returns the success status.

**Traceability:** `close-stream.c`, `close_stream`

### SC-2: Correct prior-error case
When invoked on a stream whose error state is already set before close, the Rust implementation returns the failure status.

**Traceability:** `close-stream.c`, `close_stream`

### SC-3: Correct close-failure case
When invoked on a stream for which close fails, the Rust implementation returns the failure status.

**Traceability:** `close-stream.c`, `close_stream`

### SC-4: Combined failure handling
When both a prior stream error and a close failure are present, the Rust implementation returns the failure status.

**Traceability:** `close-stream.c`, `close_stream`

### SC-5: No extra functional surface
The Rust rewrite exposes only the checked stream-close behavior evidenced by this module and does not require unrelated features to satisfy module parity.

**Traceability:** `close-stream.c`, `close_stream`