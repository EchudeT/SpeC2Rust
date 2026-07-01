# spec.md

## Title
Rust Functional Specification for `main_root_full-write.c_27`

## Document Metadata
- Project: `cat`
- Module: `main_root_full-write.c_27`
- Category: `main_cluster`
- Source files: `full-write.c`
- Primary function: `full_rw`
- Target Rust branch: `028-main_root_full_write.c_27-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides a single write-oriented helper that attempts to transfer a requested number of bytes from a caller-provided memory buffer to a file descriptor. Its purpose is to continue issuing write operations until one of the following occurs:

- the full requested byte count has been written, or
- progress can no longer be made because a write operation fails or returns zero.

The Rust rewrite must preserve this functional boundary: given a writable file descriptor-like sink and a byte buffer with a requested length, the module must repeatedly attempt output and report how many bytes were successfully written in total.

## Feature Specification

### Feature: Complete-or-best-effort buffered write loop
The module performs repeated write attempts for a contiguous caller-supplied buffer.

Behavior the Rust version must implement:

- Accept a destination represented by a file descriptor-compatible output target.
- Accept a byte buffer and a requested byte count.
- Attempt to write from the beginning of the buffer.
- If a write succeeds for fewer bytes than requested, continue writing the remaining unwritten suffix.
- Accumulate and return the total number of bytes successfully written.
- Stop retrying when the total written count reaches the requested byte count.
- Stop retrying if a write attempt returns zero bytes.
- Stop retrying if a write attempt fails.
- Return the total successfully written byte count achieved before termination.

### Functional boundary
This module is limited to transfer accounting and repeated write attempts. It does not define higher-level formatting, buffering policy beyond retrying partial writes, or application-level error reporting semantics other than returning the total bytes written.

## User Scenarios & Testing

### Scenario 1: Entire buffer is written in one operation
A caller passes a writable destination and a buffer of `N` bytes. The first write call accepts all `N` bytes.

Expected result:
- The module returns `N`.
- No additional write attempts are required.

### Scenario 2: Buffer is written across multiple partial writes
A caller passes a writable destination and a buffer of `N` bytes. The destination accepts only part of the buffer on the first call and additional parts on later calls.

Expected result:
- The module continues writing the unwritten remainder.
- The module returns `N` once all bytes are written.

### Scenario 3: Write fails after partial progress
A caller passes a writable destination and a buffer of `N` bytes. One or more writes succeed, then a later write fails.

Expected result:
- The module stops further attempts after the failure.
- The return value equals the number of bytes written before the failure.

### Scenario 4: First write fails immediately
A caller passes a writable destination and a buffer of `N` bytes. The first write attempt fails.

Expected result:
- The module returns `0`.
- No bytes are reported as written.

### Scenario 5: Write returns zero before completion
A caller passes a writable destination and a buffer of `N` bytes. A write operation returns zero before all requested bytes have been transferred.

Expected result:
- The module stops retrying.
- The return value equals the bytes written before the zero-length result, or `0` if this occurs on the first attempt.

### Scenario 6: Zero-length request
A caller requests writing zero bytes.

Expected result:
- The module returns `0`.
- No write attempts are required.

### Testing guidance
The Rust rewrite should be tested with controllable output targets that simulate:
- full write in one call,
- multiple partial writes,
- immediate failure,
- failure after partial progress,
- zero-byte write result,
- zero-length request.

Each test should assert the returned total byte count and, where observable, that no bytes beyond the returned count were attempted after termination conditions were met.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide one function corresponding to `full_rw` from `full-write.c`.
  Traceability: `full-write.c`, `full_rw`

- **FR-2**: The function shall accept an output destination, a caller-provided buffer, and a requested byte count.
  Traceability: `full-write.c`, `full_rw`

- **FR-3**: The function shall attempt to write up to the requested byte count from the provided buffer to the destination.
  Traceability: `full-write.c`, `full_rw`

- **FR-4**: If a write transfers fewer bytes than requested and remains positive, the function shall continue with the remaining portion of the buffer.
  Traceability: `full-write.c`, `full_rw`

- **FR-5**: The function shall maintain a running total of successfully written bytes and return that total.
  Traceability: `full-write.c`, `full_rw`

- **FR-6**: The function shall stop when the running total reaches the requested byte count.
  Traceability: `full-write.c`, `full_rw`

- **FR-7**: The function shall stop when a write attempt fails.
  Traceability: `full-write.c`, `full_rw`

- **FR-8**: The function shall stop when a write attempt returns zero bytes.
  Traceability: `full-write.c`, `full_rw`

- **FR-9**: If no bytes are written before termination, the function shall return `0`.
  Traceability: `full-write.c`, `full_rw`

### Key Entities
- **Output destination**: A writable file descriptor-style sink supplied by the caller.
  Relationship: Receives bytes from the caller buffer through repeated write attempts.
  Traceability: `full-write.c`, `full_rw`

- **Input buffer**: A caller-owned contiguous memory region containing bytes to be written.
  Relationship: The function writes an initial prefix, then any remaining suffix after partial writes.
  Traceability: `full-write.c`, `full_rw`

- **Requested byte count**: The maximum number of bytes the function is asked to transfer from the buffer.
  Relationship: Defines the completion target and bounds the returned total.
  Traceability: `full-write.c`, `full_rw`

- **Written byte total**: The cumulative number of bytes successfully transferred.
  Relationship: Advances after each successful positive write and becomes the function result.
  Traceability: `full-write.c`, `full_rw`

## Success Criteria
- **SC-1**: For a destination that accepts all requested bytes in one write, the Rust module returns the full requested count.
  Traceability: `full-write.c`, `full_rw`

- **SC-2**: For destinations that produce partial positive writes whose sum reaches the request, the Rust module returns the full requested count.
  Traceability: `full-write.c`, `full_rw`

- **SC-3**: For a destination that fails after writing `K` bytes in total, where `0 <= K < requested`, the Rust module returns exactly `K`.
  Traceability: `full-write.c`, `full_rw`

- **SC-4**: For a destination whose first write fails, the Rust module returns `0`.
  Traceability: `full-write.c`, `full_rw`

- **SC-5**: For a destination that returns zero before completion after writing `K` bytes, the Rust module returns exactly `K`.
  Traceability: `full-write.c`, `full_rw`

- **SC-6**: For a zero-length request, the Rust module returns `0`.
  Traceability: `full-write.c`, `full_rw`

- **SC-7**: In all supported scenarios, the returned value never exceeds the requested byte count.
  Traceability: `full-write.c`, `full_rw`