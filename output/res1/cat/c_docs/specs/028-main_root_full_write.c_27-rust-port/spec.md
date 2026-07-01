# spec.md

## Title
`main_root_full-write.c_27` Functional Specification

## Metadata
- Project: `cat`
- Module: `main_root_full-write.c_27`
- Category: `main_cluster`
- Source file: `full-write.c`
- Primary function: `full_rw`
- Rust branch: `028-main_root_full_write.c_27-rust-port`
- Generation date: `2026-06-06`

## Overview
This module provides a bounded write helper for a file descriptor. Its purpose is to attempt to write up to a requested byte count from a caller-provided buffer and continue writing until either the full requested amount has been written or further progress is not possible.

The Rust rewrite must preserve this observable behavior: given a file descriptor-like output target, a byte buffer, and a requested count, the module returns the number of bytes actually written, which may be less than requested if the underlying write operation cannot complete the full transfer.

## Feature Specification

### Summary
The module implements one focused capability:

- perform a write operation that may internally repeat partial writes until the requested byte count has been fully written or the write process stops making progress.

### Required behavior
The Rust version must implement behavior equivalent to the C module's exported functionality:

- Accept an output handle, an input byte region, and a requested byte count.
- Attempt to write bytes from the provided buffer to the output handle.
- If an underlying write transfers only part of the remaining data, continue writing the unwritten remainder.
- Stop when either:
  - the full requested count has been written, or
  - the underlying write operation indicates that no more bytes were written for the current attempt.
- Return the total number of bytes successfully written.

### Behavioral boundaries
The Rust rewrite must not broaden module scope beyond the evidenced function of repeated full-write behavior. In particular, this specification does not require any additional public APIs, buffering policies, retry policies beyond repeated partial-write handling, or higher-level stream management.

## User Scenarios & Testing

### Scenario 1: Complete write in one underlying call
A caller provides:
- a valid writable output handle,
- a buffer containing data,
- a count equal to the data length.

If the underlying write accepts all requested bytes immediately, the module returns the full count.

**Test expectation:** returned byte count equals requested count.

### Scenario 2: Complete write across multiple partial writes
A caller writes to an output handle whose underlying write operation accepts only part of the buffer on each call.

The module must continue issuing writes for the remaining bytes until the total requested count has been written.

**Test expectation:** returned byte count equals requested count, and the output contains the full requested data in order.

### Scenario 3: Early stop after partial progress
A caller writes data and one or more underlying write calls succeed partially, but a later call writes zero additional bytes or otherwise stops progress.

The module must stop and return the number of bytes written so far.

**Test expectation:** returned byte count is greater than zero and less than requested count.

### Scenario 4: No bytes written
A caller invokes the module and the first underlying write makes no progress.

The module must return zero.

**Test expectation:** returned byte count is `0`.

### Scenario 5: Zero-length request
A caller requests that zero bytes be written.

The module must report that zero bytes were written.

**Test expectation:** returned byte count is `0`, with no required data transfer.

## Requirements

### Functional Requirements
- **FR-1:** The module shall provide a function corresponding to `full_rw` from `full-write.c` that attempts to write up to a caller-specified byte count from a caller-provided buffer to an output descriptor.
- **FR-2:** The module shall accumulate and return the total number of bytes successfully written across one or more underlying write attempts.
- **FR-3:** When an underlying write completes only part of the remaining request, the module shall continue writing from the next unwritten byte.
- **FR-4:** The module shall stop attempting additional writes once the accumulated written byte count reaches the requested count.
- **FR-5:** The module shall stop attempting additional writes if an underlying write does not add any newly written bytes for the current operation.
- **FR-6:** The module shall support requests for zero bytes and return zero written bytes for such requests.

### Key Entities
- **Output descriptor / writable handle:** The destination to which bytes are written. In the source module this is represented by an integer file descriptor argument.
- **Input byte buffer:** The source memory region containing the bytes to be written.
- **Requested byte count:** The maximum number of bytes the caller asks the module to write from the input buffer.
- **Returned written byte count:** The total number of bytes the module reports as successfully written before completion or early stop.

### Entity relationships
- The caller supplies an output descriptor, input byte buffer, and requested byte count to the module function.
- The module consumes bytes from the input buffer in sequence and attempts to transfer them to the output descriptor.
- The returned written byte count reflects how much of the requested prefix of the input buffer was actually transferred.

## Success Criteria
- **SC-1:** For a writable target that accepts the full request in one operation, the Rust implementation returns exactly the requested byte count.
- **SC-2:** For a target that performs deterministic partial writes but eventually accepts all requested data, the Rust implementation returns exactly the requested byte count and preserves byte order in the written output.
- **SC-3:** For a target that stops making progress before the full request is completed, the Rust implementation returns the exact number of bytes successfully written before progress stopped.
- **SC-4:** For a zero-length request, the Rust implementation returns `0`.
- **SC-5:** The Rust implementation's externally observable behavior for total bytes written is traceably equivalent to the behavior of `full_rw` in `full-write.c`.