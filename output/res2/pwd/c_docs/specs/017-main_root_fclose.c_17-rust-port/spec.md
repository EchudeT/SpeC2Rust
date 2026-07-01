# spec.md

## Title
Functional Specification for `main_root_fclose.c_17`

## Metadata
- Project: `pwd`
- Module: `main_root_fclose.c_17`
- Category: `main_cluster`
- Source file: `fclose.c`
- Rust branch: `017-main_root_fclose.c_17-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides close-time handling for C `FILE *` streams, with behavior centered on a replacement close routine that preserves meaningful error reporting when a stream is closed. The Rust rewrite must preserve the module’s observable behavior: it must attempt to close a stream, avoid exposing close-time exceptions from the internal non-throwing close path, and return a status that reflects whether prior stream errors or close-time errors occurred.

The module contains:
- an internal helper that performs a non-throwing close of a stream;
- a replacement public close routine that evaluates stream state and close outcome to determine the final result.

## Feature Specification

### Summary
The Rust version must implement the module behavior represented by the replacement close operation for stream handles.

### Supported functionality
1. Accept an open stream handle corresponding to a C `FILE *` stream abstraction.
2. Inspect stream error state before closure so that write or stream failures that happened earlier are not lost when closing.
3. Attempt stream closure through a path that does not surface exceptions beyond the function’s status result.
4. Return success only when:
   - the stream had no prior error state, and
   - the close operation itself succeeds.
5. Return failure when either:
   - the stream already indicates an error before close, or
   - the close operation fails.
6. Ensure the stream is closed exactly once through this module’s operation.

### Rust port boundary
The Rust rewrite must reproduce the functional contract of the C module. It does not need to introduce broader file I/O abstractions, new public APIs, recovery features, or additional stream lifecycle management beyond the close behavior evidenced here.

## User Scenarios & Testing

### Scenario 1: Closing a healthy stream
A caller finishes using a stream that has not experienced any prior I/O error and requests closure through the replacement close routine.

Expected result:
- the stream is closed;
- the routine reports success.

Test evidence:
- create or obtain a writable or readable stream with no error flag set;
- invoke the Rust replacement close behavior;
- verify a success return value.

### Scenario 2: Closing a stream with a prior stream error
A caller closes a stream after an earlier operation has already placed the stream into an error state.

Expected result:
- the stream is still closed;
- the routine reports failure, even if the underlying close action itself succeeds.

Test evidence:
- arrange a stream state that reflects a prior error before close;
- invoke the Rust replacement close behavior;
- verify a failure return value tied to the pre-close error condition.

### Scenario 3: Close operation itself fails
A caller closes a stream whose final flush or close step fails.

Expected result:
- the routine reports failure.

Test evidence:
- use a stream setup where close returns an error;
- invoke the Rust replacement close behavior;
- verify a failure return value.

### Scenario 4: Prior error and close failure both exist
A caller closes a stream that already has an error state and whose close operation also fails.

Expected result:
- the routine reports failure;
- no success is reported merely because one failure source masks another.

Test evidence:
- arrange both a pre-close stream error and a failing close path;
- invoke the Rust replacement close behavior;
- verify failure.

### Scenario 5: Internal non-throwing close path
The module’s internal helper is used as part of the replacement close behavior.

Expected result:
- closure is attempted through a non-throwing helper path;
- the outward observable result is an integer-like success/failure status, not propagated exceptions.

Test evidence:
- exercise the replacement close path under both successful and failing close conditions;
- verify outcomes are represented by status returns only.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide a replacement stream-close routine corresponding to `rpl_fclose` in `fclose.c`.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **FR-2**: The replacement close routine shall evaluate whether the stream was already in an error state before attempting closure.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **FR-3**: The replacement close routine shall attempt to close the provided stream handle.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **FR-4**: The close attempt used by the module shall be performed through an internal non-throwing helper corresponding to `fclose_nothrow`.
  **Traceability**: `fclose_nothrow` (`fclose.c:33-41`), `rpl_fclose` (`fclose.c:57-112`)

- **FR-5**: The replacement close routine shall return success only if the stream had no prior error indication and the close attempt succeeds.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **FR-6**: The replacement close routine shall return failure if the stream had a prior error indication, even when the close attempt succeeds.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **FR-7**: The replacement close routine shall return failure if the close attempt fails.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **FR-8**: The module shall expose behavior through status return values consistent with C close semantics for this routine.
  **Traceability**: `fclose_nothrow` (`fclose.c:33-41`), `rpl_fclose` (`fclose.c:57-112`)

### Key Entities
- **Stream handle**: The file-stream object represented in C as `FILE *`. It is the sole input entity operated on by this module.
  **Relationship**: Passed to the replacement close routine; passed internally to the non-throwing close helper.

- **Pre-close error state**: The error indication associated with the stream before closure is attempted.
  **Relationship**: Read by the replacement close routine to influence the final return status.

- **Close result status**: The success/failure outcome produced by the internal close attempt and returned by the replacement close routine.
  **Relationship**: Combines with the pre-close error state to determine the final function result.

## Success Criteria
- **SC-1**: For a stream with no prior error and a successful close, the Rust implementation returns success.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **SC-2**: For a stream with a prior error but a successful close, the Rust implementation returns failure.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **SC-3**: For a stream with no prior error and a failing close, the Rust implementation returns failure.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **SC-4**: For a stream with both a prior error and a failing close, the Rust implementation returns failure.
  **Traceability**: `rpl_fclose` (`fclose.c:57-112`)

- **SC-5**: The Rust implementation performs closure via an internal helper boundary equivalent in role to `fclose_nothrow`, with observable outcomes reported as function status values rather than thrown exceptions.
  **Traceability**: `fclose_nothrow` (`fclose.c:33-41`), `rpl_fclose` (`fclose.c:57-112`)

- **SC-6**: Tests covering the supported scenarios pass using the Rust branch `017-main_root_fclose.c_17-rust-port`.
  **Traceability**: `fclose.c`