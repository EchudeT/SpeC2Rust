# spec.md

## Overview

This module provides a single write-oriented helper that attempts to transfer an entire buffer to an already-open file descriptor. Its role is to shield callers from partial-write behavior by repeatedly writing until either all requested bytes have been written or progress can no longer continue.

The Rust rewrite must preserve this functional boundary: given a file descriptor, a buffer, and a byte count, it must try to write the requested byte range completely and report how many bytes were actually written.

## Scope

In scope for this module:

- Repeated write attempts for a caller-supplied byte buffer.
- Accumulation of bytes successfully written across multiple underlying write operations.
- Early return when the full requested byte count has been written.
- Early return when the underlying write operation does not make further progress.

Out of scope for this module:

- Opening or closing file descriptors.
- Buffer ownership management beyond reading caller-supplied bytes.
- Read operations.
- Higher-level output formatting or line handling.
- Error reporting beyond the returned byte count.

## Feature Specification

### Feature: Complete-as-possible buffered write

The module exposes functionality equivalent to `full_rw(int fd, const void *buf, size_t count)` from `full-write.c`.

Behavior required for the Rust version:

- Accept an already-open output file descriptor/handle, a byte buffer, and a requested byte count.
- Attempt to write data starting from the beginning of the buffer.
- If an underlying write transfers only part of the remaining data, continue writing the unwritten remainder.
- Track total bytes written across all successful partial writes.
- Stop and return once the total written byte count reaches the requested count.
- If an underlying write fails to transfer bytes or otherwise prevents further progress, stop and return the total number of bytes written so far.
- Support zero-length requests by returning zero written bytes without requiring any transfer.

This module is a low-level utility used by higher-level program flow that needs a "write as much as possible, up to the full buffer" operation.

## User Scenarios & Testing

### Scenario 1: Entire buffer written in one operation

A caller has an open file descriptor and a buffer of `N` bytes. The destination accepts the entire buffer in one write.

Expected behavior:

- The module returns `N`.
- No additional write attempts are needed after the first full transfer.

Test focus:

- Verify returned byte count equals requested count.
- Verify output data exactly matches the input buffer.

### Scenario 2: Buffer written across multiple partial writes

A caller writes a buffer to a destination that accepts only part of the data per write call.

Expected behavior:

- The module continues issuing writes for the remaining data.
- The module returns the full requested count once all bytes are transferred.

Test focus:

- Simulate multiple partial-write results.
- Verify the final returned count equals the original requested count.
- Verify the destination receives the complete byte sequence in order.

### Scenario 3: Partial success followed by inability to continue

A caller writes a buffer, some bytes are written successfully, and then a subsequent write cannot progress.

Expected behavior:

- The module returns the number of bytes written before progress stopped.
- The module does not over-report bytes that were not transferred.

Test focus:

- Simulate one or more successful partial writes followed by a failing or zero-progress write.
- Verify the returned count equals only the successfully transferred prefix.

### Scenario 4: Immediate inability to write

A caller invokes the helper, but the first underlying write does not transfer any bytes.

Expected behavior:

- The module returns `0`.

Test focus:

- Simulate immediate failure or zero-progress behavior.
- Verify no bytes are reported as written.

### Scenario 5: Zero-length request

A caller requests writing zero bytes.

Expected behavior:

- The module returns `0`.
- No data transfer is required.

Test focus:

- Pass an empty range or zero count.
- Verify the result is zero.

## Requirements

### Functional Requirements

#### FR-1: Write-attempt loop
The Rust module shall provide the same functional behavior as the C helper in `full-write.c`: attempt to write a caller-specified byte count to an already-open file descriptor/handle and continue until completion or loss of progress.

Traceability: `full-write.c`, `full_rw`

#### FR-2: Partial-write handling
When an underlying write transfers fewer bytes than requested for the current attempt, the module shall treat that as successful partial progress, advance within the caller buffer, and continue writing the remaining bytes.

Traceability: `full-write.c`, `full_rw`

#### FR-3: Total byte-count reporting
The module shall return the total number of bytes successfully written across all write attempts.

Traceability: `full-write.c`, `full_rw`

#### FR-4: Full-completion return
If the destination accepts all requested bytes, whether in one write or many, the module shall return the original requested byte count.

Traceability: `full-write.c`, `full_rw`

#### FR-5: Early return on no further progress
If the write process cannot continue before all requested bytes are transferred, the module shall stop retrying and return the number of bytes written so far.

Traceability: `full-write.c`, `full_rw`

#### FR-6: Zero-count support
If the requested byte count is zero, the module shall return zero.

Traceability: `full-write.c`, `full_rw`

### Key Entities

#### File descriptor / output handle
An already-open destination supplied by the caller. The module uses it as the target of byte transfer but does not create, own, or manage its lifecycle.

Traceability: `full-write.c`, `full_rw`

#### Input byte buffer
A caller-supplied memory region containing the bytes to be written. The module reads from this buffer sequentially from the start up to the requested count.

Traceability: `full-write.c`, `full_rw`

#### Requested byte count
The number of bytes the caller wants transferred from the input buffer.

Traceability: `full-write.c`, `full_rw`

#### Accumulated written count
The running total of bytes successfully transferred. This determines both completion and the value returned to the caller.

Traceability: `full-write.c`, `full_rw`

## Success Criteria

### SC-1: Complete write behavior
For a destination that accepts all bytes, the Rust implementation returns exactly the requested count.

Traceability: `full-write.c`, `full_rw`

### SC-2: Partial-write continuation
Under tests that force multiple partial underlying writes, the Rust implementation continues until either all requested bytes are written or no further progress is possible.

Traceability: `full-write.c`, `full_rw`

### SC-3: Accurate partial result
When progress stops after some bytes have been written, the Rust implementation returns exactly the number of bytes successfully transferred before stopping.

Traceability: `full-write.c`, `full_rw`

### SC-4: No false success
The Rust implementation never reports more bytes written than the requested count.

Traceability: `full-write.c`, `full_rw`

### SC-5: Zero-count correctness
For a zero-length request, the Rust implementation returns zero.

Traceability: `full-write.c`, `full_rw`

### SC-6: Ordered data transfer
In tests with controlled partial writes, the bytes observed at the destination match the caller buffer prefix actually reported as written, in original order.

Traceability: `full-write.c`, `full_rw`