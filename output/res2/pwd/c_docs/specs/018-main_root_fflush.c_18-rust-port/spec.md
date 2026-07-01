# spec.md

## Title

Functional Specification: `main_root_fflush.c_18` Rust Port

## Metadata

- Project: `pwd`
- Module: `main_root_fflush.c_18`
- Category: `main_cluster`
- Source file: `fflush.c`
- Rust branch: `018-main_root_fflush.c_18-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides replacement `fflush` behavior for C `FILE*` streams, with special handling for cases where normal flushing is not sufficient to preserve expected stream-position behavior. Its responsibility is to flush pending stream state while preserving or updating the file-position state associated with the stream, including handling the special case where the caller requests a flush of all open output streams.

The Rust rewrite must implement equivalent observable behavior for the module-defined flush operation, including:

- flushing a specific stream when provided,
- flushing all relevant output streams when no specific stream is provided,
- preserving correct file-position effects for seekable streams,
- avoiding unwanted seek optimizations during the operation when needed,
- restoring temporary stream state changes before returning.

This specification is limited to behavior evidenced by `fflush.c` and its analyzed functions.

## Scope

In scope:

- Behavior equivalent to `rpl_fflush`.
- Temporary disabling and later restoration of seek-related optimization state on a stream when required by the flush operation.
- Updating cached file-position state where the source module does so as part of flush behavior.

Out of scope:

- Any API beyond the module’s replacement flush behavior.
- New stream abstractions or broader I/O management features.
- Guarantees not evidenced by the source analysis, including concurrency guarantees, serialization, recovery, or performance claims.

## Feature Specification

### Summary

The module defines a replacement flush routine that supplements standard stream flushing with stream-position management. It exists to ensure that, after flushing, the stream's effective file position remains coherent with the stream state, especially for seekable streams and cases involving buffered reads or writes.

### Functional Behavior

1. **Specific-stream flush**
   - When called with a non-null stream, the module flushes that stream.
   - If the stream requires position-sensitive handling, the operation must preserve the correct file position semantics across the flush.
   - Any temporary changes made to stream seek-related optimization flags during the flush must be restored before return.

2. **Global flush**
   - When called with a null stream, the module performs the equivalent of flushing all open output streams supported by the underlying runtime behavior.
   - The Rust port must preserve this null-argument semantic distinction.

3. **Seek-optimization control**
   - For streams where internal seek optimization would interfere with correct flush/position behavior, the module temporarily disables that optimization before the sensitive operation.
   - After the operation completes, the prior optimization state is restored.

4. **File-position cache maintenance**
   - When the flush operation determines a concrete resulting stream position, the module updates stream position cache state to match that position.
   - This behavior is part of preserving correct observable stream semantics after flushing.

5. **Return behavior**
   - The module reports success or failure through an integer-style status result compatible with C `fflush` semantics.
   - The Rust rewrite must preserve the same success/failure outcomes at the module boundary it exposes for this port.

## User Scenarios & Testing

### Scenario 1: Flush a writable stream with buffered output

A caller has written data to a buffered stream and invokes the module flush operation for that stream.

Expected result:

- Buffered output is forced to the underlying file or device according to flush semantics.
- The operation returns a success status when the underlying flush succeeds.
- No persistent unintended changes remain in stream optimization state after the call.

### Scenario 2: Flush a seekable stream where stream position matters

A caller uses a stream whose file position must remain coherent after flush. The module handles any necessary position-sensitive steps around the flush.

Expected result:

- The stream is flushed.
- The effective stream position after flushing matches the source module’s behavior.
- If seek optimization had to be disabled to achieve correctness, it is restored afterward.

### Scenario 3: Flush all output streams

A caller invokes the module with a null stream argument to request a global flush.

Expected result:

- The operation performs the equivalent of flushing all open output streams supported by the original module behavior.
- The function returns success or failure consistently with the source module semantics.

### Scenario 4: Flush operation fails

A caller invokes the module on a stream for which flushing cannot complete successfully.

Expected result:

- The operation reports failure through its return status.
- Temporary internal state changes used to support the operation are not left applied after the function returns.

### Scenario 5: Flush on a stream requiring file-position cache update

A caller flushes a stream where the flush path determines a specific resulting file offset.

Expected result:

- The flush succeeds or fails according to the underlying operation.
- On the path where the source module updates cached file-position state, the Rust version applies equivalent state maintenance so later position-dependent behavior remains coherent.

## Requirements

### Functional Requirements

- **FR-1: Replacement flush operation**
  - The module shall provide behavior equivalent to the source module’s replacement flush routine for C stream objects.
  - Traceability: `rpl_fflush` in `fflush.c`.

- **FR-2: Support null-stream global flush semantics**
  - When invoked without a specific stream target, the module shall perform the source-equivalent flush-all behavior for open output streams.

- **FR-3: Support targeted stream flush semantics**
  - When invoked with a specific stream target, the module shall flush that stream and return success or failure according to source-equivalent semantics.

- **FR-4: Temporary seek-optimization suppression**
  - The module shall temporarily suppress seek optimization on a stream when required by the source behavior to carry out a correct flush/position-sensitive operation.
  - Traceability: `disable_seek_optimization` and use within `rpl_fflush` in `fflush.c`.

- **FR-5: Restoration of prior seek-optimization state**
  - After any temporary suppression of seek optimization, the module shall restore the prior optimization-related state before returning.
  - Traceability: `restore_seek_optimization` and use within `rpl_fflush` in `fflush.c`.

- **FR-6: File-position cache maintenance**
  - The module shall update cached file-position state in the cases where the source module computes and records a resulting stream position as part of flushing.
  - Traceability: `update_fpos_cache` and use within `rpl_fflush` in `fflush.c`.

- **FR-7: Observable result compatibility**
  - The module shall preserve the source module’s observable success/failure behavior for the flush operation.

### Key Entities

- **Stream handle (`FILE *`)**
  - The primary entity operated on by the module.
  - It represents either a specific stream to flush or, when absent in the flush call, triggers global flush semantics.
  - Traceability: all functions in `fflush.c`.

- **Seek optimization state**
  - Per-stream state that can be temporarily disabled and later restored to prevent incorrect seek-related behavior during flushing.
  - Relationship: associated with a stream handle and managed around the main flush operation.
  - Traceability: `disable_seek_optimization`, `restore_seek_optimization` in `fflush.c`.

- **File-position cache**
  - Stream-associated cached position state maintained by the module when a flush path establishes a concrete file offset.
  - Relationship: updated for a given stream as part of the replacement flush behavior.
  - Traceability: `update_fpos_cache` in `fflush.c`.

- **Saved flags / saved optimization state**
  - Temporary saved state captured before seek optimization is disabled so it can be restored after the flush operation.
  - Relationship: derived from a stream handle, consumed during restoration.

## Success Criteria

- **SC-1**
  - A targeted flush on a valid stream produces success/failure results matching the source module behavior.
  - Traceability: `rpl_fflush`.

- **SC-2**
  - A null-stream invocation performs source-equivalent global flush behavior and returns a source-compatible status.

- **SC-3**
  - In execution paths where seek optimization is temporarily disabled, the prior optimization state is restored before function return, including failure paths exercised by tests.
  - Traceability: `disable_seek_optimization`, `restore_seek_optimization`, `rpl_fflush`.

- **SC-4**
  - In execution paths where the source module updates cached stream position, the Rust port leaves the stream in an equivalent position-coherent state after flushing.
  - Traceability: `update_fpos_cache`, `rpl_fflush`.

- **SC-5**
  - Tests covering writable streams, seek-sensitive streams, null-stream global flush, and flush failure scenarios all pass against behavior derived from the source module.
  - Traceability: `rpl_fflush` and supporting helper functions in `fflush.c`.

## Acceptance Notes

- The Rust port should be judged on equivalence of externally observable flush and stream-position behavior, not on reproducing C-level internal implementation details verbatim.
- Any internal Rust design is acceptable if it satisfies the requirements and success criteria above and does not introduce unevidenced functionality.