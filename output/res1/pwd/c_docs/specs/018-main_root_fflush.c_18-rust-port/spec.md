# spec.md

## Title

Functional Specification: `main_root_fflush.c_18`

## Metadata

- Project: `pwd`
- Module: `main_root_fflush.c_18`
- Category: `main_cluster`
- Source file: `fflush.c`
- Rust branch target: `018-main_root_fflush.c_18-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides replacement `fflush` behavior for C `FILE *` streams, with special handling for cases where standard flushing behavior is insufficient or inconsistent across environments. Its primary responsibility is to flush stream state while preserving correct file-position behavior for seekable input streams and for global flush requests.

The Rust rewrite must preserve the observable behavior implemented by this module: flushing a specific stream or all streams, distinguishing input-capable versus output-capable cases as required by the source behavior, and maintaining correct underlying file-position state when flushing an input stream that may have buffered unread data.

## Scope

In scope:

- Replacement flush behavior corresponding to `rpl_fflush`.
- Handling of a null stream argument as a request to flush all streams.
- Stream-specific flush behavior for non-null streams.
- Preservation or update of stream/file position state where required by buffered input flushing behavior.
- Temporary disabling and restoration of seek-related optimization flags when needed to obtain correct position updates.

Out of scope:

- Definition of the `FILE` type itself.
- General stream creation, opening, closing, reading, or writing APIs beyond what is required to support this module’s flush behavior.
- New public APIs not evidenced by the source module.

## Feature Specification

### Summary

The module implements a replacement flush operation that behaves compatibly across platform-specific `stdio` implementations. The Rust version must implement the same functional boundary:

- Accept a target stream or a request to flush all streams.
- For a specific stream, perform flush behavior that is correct for the stream’s current mode and buffering state.
- When flushing input-oriented state on seekable streams, ensure the effective file position reflects consumed buffered data rather than leaving the underlying descriptor position stale.
- Apply and then restore any stream-level seek optimization state necessary to make the position update reliable.
- Return success or failure consistent with the underlying flush/seek operations used by the module logic.

### Functional Behavior

#### Global flush request

When the flush entry point is invoked with no specific stream, the module must perform a flush-all operation affecting all open streams, with success or failure reported through the function result.

Traceability: `rpl_fflush` in `fflush.c`.

#### Flush of a specific stream

When invoked for a specific stream, the module must determine whether ordinary flushing is sufficient or whether additional handling is needed for the stream state.

Traceability: `rpl_fflush` in `fflush.c`.

#### Input-stream position correction

For a stream whose buffered input state can cause the underlying file position to diverge from the logical stream position, the module must correct or preserve file-position state during flush so that subsequent operations observe the correct position.

This includes updating cached position information when the source behavior requires it.

Traceability: `rpl_fflush`, `update_fpos_cache` in `fflush.c`.

#### Temporary suppression of seek optimization

Where stream internals use a seek optimization that would interfere with correct position adjustment during flush, the module must temporarily disable that optimization, perform the required action, and restore the prior optimization state afterward.

Traceability: `disable_seek_optimization`, `restore_seek_optimization`, `rpl_fflush` in `fflush.c`.

#### Error propagation

If the underlying flush or positioning work fails, the module must report failure through its return value rather than silently succeeding.

Traceability: `rpl_fflush` in `fflush.c`.

## User Scenarios & Testing

### Scenario 1: Flush all open streams

A caller requests a global flush by passing no specific stream target. The module flushes all open streams and returns success if the operation completes, or failure if the underlying runtime reports an error.

Required test coverage:

- Invoke the Rust replacement with a null-equivalent/all-streams request.
- Verify that buffered output written to multiple open streams is forced out as expected.
- Verify that an underlying flush failure is reflected as a failure result.

Traceability: `rpl_fflush`.

### Scenario 2: Flush a normal output stream

A caller flushes a specific output stream with buffered written data. The module performs the flush and returns the corresponding success or failure status.

Required test coverage:

- Write buffered data to a writable stream.
- Invoke the Rust replacement on that stream.
- Verify the data is externally observable after flush.
- Verify return status matches underlying operation success.

Traceability: `rpl_fflush`.

### Scenario 3: Flush a seekable input stream with unread buffered data

A caller has read from a seekable input stream such that the library may have prefetched data into its input buffer. On flush, the module must ensure the stream’s effective file position is synchronized correctly rather than leaving the descriptor position advanced by unread prefetched bytes.

Required test coverage:

- Open a seekable file for reading.
- Read a portion that causes input buffering.
- Capture the logical stream position before flush.
- Invoke the Rust replacement.
- Verify the effective position after flush matches the logical consumed position expected by the source behavior.

Traceability: `rpl_fflush`, `update_fpos_cache`.

### Scenario 4: Flush a stream requiring temporary seek optimization changes

A caller flushes a stream implementation whose internal seek optimization would otherwise prevent correct repositioning. The module temporarily changes that state, completes the needed position-sensitive operation, and restores the prior state.

Required test coverage:

- Use a test harness or integration setup capable of observing the position-sensitive flush path.
- Verify the operation succeeds with correct final file position.
- Verify state restoration by confirming subsequent stream operations continue to behave normally.

Traceability: `disable_seek_optimization`, `restore_seek_optimization`, `rpl_fflush`.

### Scenario 5: Underlying positioning failure during input flush

A caller flushes a stream for which the module must perform additional seek/position logic, but the underlying positioning operation fails. The module must return failure.

Required test coverage:

- Simulate or induce a failure in the position-adjustment path.
- Invoke the Rust replacement on the affected stream.
- Verify the function reports failure.

Traceability: `rpl_fflush`.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a replacement flush operation equivalent in role to `rpl_fflush` for flushing either a specific stream or all streams.
  - Traceability: `rpl_fflush`

- **FR-2**: When invoked with no specific stream target, the module shall perform a flush-all operation across open streams and return a success/failure status.

- **FR-3**: When invoked with a specific stream, the module shall flush that stream and return a success/failure status reflecting the result of the operation.

- **FR-4**: For a seekable input stream whose buffered state may make the underlying file position inconsistent with the logical stream position, the module shall adjust handling so the effective position remains correct after flush.

- **FR-5**: The module shall update any maintained file-position cache when required by the position-correction path.
  - Traceability: `update_fpos_cache`, `rpl_fflush`

- **FR-6**: When seek optimization flags would interfere with correct position-sensitive flushing, the module shall temporarily disable the optimization before the sensitive operation.
  - Traceability: `disable_seek_optimization`, `rpl_fflush`

- **FR-7**: After completing a position-sensitive flush operation, the module shall restore any previously saved seek optimization state.
  - Traceability: `restore_seek_optimization`, `rpl_fflush`

- **FR-8**: The module shall propagate failure from underlying flush or seek/position operations through its return value.

### Key Entities

- **Stream handle / `FILE`**
  - The central entity operated on by this module.
  - Represents either a specific stream to flush or, when absent, the global flush-all case.
  - Carries buffering, mode, and implementation-specific state relevant to flush and position correction.
  - Traceability: `rpl_fflush`, helper functions in `fflush.c`

- **Seek optimization state**
  - Saved per-stream state used to temporarily alter seek-related behavior during position-sensitive flush processing.
  - Produced before the sensitive operation and consumed during restoration.
  - Traceability: `disable_seek_optimization`, `restore_seek_optimization`

- **Cached file-position state**
  - Stream-associated position information updated when the module corrects logical versus underlying file position during input flush handling.
  - Traceability: `update_fpos_cache`

## Success Criteria

- **SC-1**: A Rust implementation exposes module behavior equivalent to `rpl_fflush` for both specific-stream flush and all-stream flush requests.
  - Verification: integration tests covering null/all-stream and non-null/specific-stream invocation paths.
  - Traceability: `rpl_fflush`

- **SC-2**: Flushing a buffered writable stream causes buffered data to become externally observable and reports success when the underlying operation succeeds.
  - Verification: file-backed output test.

- **SC-3**: Flushing a seekable input stream after buffered reads preserves or restores the correct effective file position consistent with consumed input.
  - Verification: position-before/position-after integration test on a seekable file.
  - Traceability: `rpl_fflush`, `update_fpos_cache`

- **SC-4**: In the position-sensitive path, temporary seek optimization changes do not remain in effect after the operation completes.
  - Verification: tests confirming correct post-flush stream behavior after the path that disables and restores optimization.
  - Traceability: `disable_seek_optimization`, `restore_seek_optimization`

- **SC-5**: When underlying flush or position-adjustment operations fail, the Rust implementation returns failure rather than success.
  - Verification: fault-injection or induced-error tests.

## Notes for Rust Rewrite

- Preserve the source module’s behavioral contract rather than the C-specific internal layout of `FILE`.
- Any Rust-side abstraction used to represent streams must support the same decision points evidenced here: whole-process flush request, per-stream flush, seek-sensitive input handling, and position-cache maintenance where applicable.
- No additional public capability should be introduced beyond the replacement flush behavior defined by this module.