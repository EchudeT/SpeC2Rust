# spec.md

## Title
Functional Specification: `main_root_safe_rw_15`

## Metadata
- Project: `cat`
- Module: `main_root_safe_rw_15`
- Category: `main_cluster`
- Rust branch: `016-main_root_safe_rw_15-rust-port`
- Source basis: `include/safe-read.c`, `safe-read.c`
- Generation date: 2026-06-07

## Overview
This module provides a single safe read/write wrapper function, `safe_rw`, that performs an I/O operation on a file descriptor and shields callers from interruption by signals. Its role is to ensure that when the underlying system call is interrupted with `EINTR`, the operation is retried instead of being exposed directly to the caller as a spurious failure.

The Rust rewrite must preserve this behavior and provide the same functional boundary: attempt the underlying file-descriptor I/O operation, retry on interruption, and return the resulting byte count or failure result as represented by the module’s interface.

## Scope
In scope:
- Retrying an underlying file-descriptor I/O operation when it fails due to interruption (`EINTR`)
- Returning the final result of the I/O operation once it succeeds or fails for a reason other than interruption
- Supporting the module’s existing call pattern that accepts:
  - a file descriptor
  - a buffer pointer/reference
  - a byte count

Out of scope:
- Defining higher-level stream abstractions
- Buffer ownership management beyond accepting caller-provided input
- Adding new public APIs or new error-reporting models not evidenced by this module
- Changing semantics beyond preserving the existing safe retry behavior

## Feature Specification

### Feature: Interrupted-system-call-safe I/O wrapper
The module exposes `safe_rw`, a wrapper around a file-descriptor read/write-style operation. Its defining behavior is that interruptions caused by signals must not be reported immediately to the caller when the interruption is represented by `EINTR`. Instead, the operation must be attempted again until one of the following occurs:
- the operation completes and returns a normal result, or
- the operation fails for a reason other than interruption

The Rust version must implement the same observable behavior:
- accept the same conceptual inputs: file descriptor, buffer, byte count
- perform the wrapped I/O operation
- retry transparently on `EINTR`
- stop retrying once a non-`EINTR` result is obtained
- return the final result in a form equivalent to the existing module contract

### Behavioral Notes
- The module’s purpose is narrow: it is a retrying wrapper, not a general I/O framework.
- The module does not define persistent state or additional data structures.
- The module’s contract is based on safe handling of interrupted system calls, not on transforming the data being read or written.

## User Scenarios & Testing

### Scenario 1: Read or write completes without interruption
A caller invokes `safe_rw` with a valid file descriptor, buffer, and count. The underlying I/O operation succeeds immediately.

Expected support:
- The Rust module returns the same successful byte count produced by the underlying operation.
- No extra retries occur beyond what is needed to obtain that result.

Suggested test:
- Use a controllable file descriptor where the wrapped operation succeeds immediately.
- Verify that the returned byte count matches the underlying operation result.

### Scenario 2: Operation is interrupted once by a signal
A caller invokes `safe_rw`, and the first underlying I/O attempt fails with `EINTR`. A subsequent attempt succeeds.

Expected support:
- The Rust module does not surface the interrupted attempt as a terminal failure.
- The Rust module retries and returns the successful result from the later attempt.

Suggested test:
- Mock or simulate an underlying operation that returns interruption once, then success.
- Verify that the final result is success and that interruption is not returned to the caller.

### Scenario 3: Operation is interrupted multiple times
A caller invokes `safe_rw`, and multiple consecutive attempts fail with `EINTR` before a later attempt succeeds.

Expected support:
- The Rust module continues retrying for each interrupted attempt.
- The module returns the first non-`EINTR` result.

Suggested test:
- Simulate repeated interruption responses followed by success.
- Verify that the call eventually returns success after all interruptions are retried.

### Scenario 4: Operation fails for a non-interruption reason
A caller invokes `safe_rw`, and the underlying operation fails with an error other than `EINTR`.

Expected support:
- The Rust module does not retry indefinitely on non-`EINTR` failures.
- The Rust module returns the failure result corresponding to that non-`EINTR` condition.

Suggested test:
- Simulate an underlying operation that fails with a non-`EINTR` error.
- Verify that the module returns failure without converting it into success.

### Scenario 5: Zero-length request
A caller invokes `safe_rw` with a byte count of zero.

Expected support:
- The Rust module forwards the operation semantics consistently with the underlying I/O contract.
- The result must match what the wrapped operation would return for a zero-length request, subject only to the same interruption-retry rule.

Suggested test:
- Invoke the wrapper with count zero on a valid descriptor.
- Verify that the result matches the underlying system behavior.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide the functionality represented by `safe_rw` as the module’s public behavior.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **FR-2**: The module shall accept as input a file descriptor, a caller-supplied buffer reference, and a byte count.
  Traceability: `safe_rw` signature in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **FR-3**: The module shall invoke the underlying file-descriptor I/O operation using the caller-supplied inputs.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **FR-4**: If an I/O attempt is interrupted and reports `EINTR`, the module shall retry the operation instead of returning that interrupted result immediately.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **FR-5**: The module shall continue retrying while the result condition remains interruption by `EINTR`.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **FR-6**: The module shall stop retrying and return once the underlying operation produces a result that is not an `EINTR` interruption.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **FR-7**: On success, the module shall return the byte-count result produced by the completed I/O operation.
  Traceability: `safe_rw` return type and behavior in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **FR-8**: On failure for any reason other than interruption by `EINTR`, the module shall return the corresponding failure result without additional interruption-specific retry handling.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

### Key Entities
- **`safe_rw` operation interface**: The module’s sole functional entity. It binds together:
  - a file descriptor identifying the I/O target
  - a caller-provided buffer used by the underlying I/O operation
  - a byte count specifying the requested transfer size
  - a returned size/result value representing the final non-`EINTR` outcome

Relationships:
- The file descriptor, buffer, and count are inputs to one wrapped I/O attempt.
- The wrapper may repeat attempts when interruption occurs.
- The returned result corresponds to the first non-`EINTR` outcome from those attempts.

## Success Criteria
- **SC-1**: A Rust implementation exposes module behavior equivalent to `safe_rw` and accepts the same conceptual inputs: file descriptor, buffer, and byte count.
  Traceability: `safe_rw` signature in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **SC-2**: When the underlying I/O operation succeeds on the first attempt, the Rust implementation returns the same successful result without alteration.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **SC-3**: When the underlying operation returns `EINTR` one or more times before succeeding, the Rust implementation retries and returns the eventual successful result rather than surfacing `EINTR` as the terminal result.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **SC-4**: When the underlying operation fails for a reason other than `EINTR`, the Rust implementation returns that failure result and does not continue retrying under the interruption rule.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`

- **SC-5**: Tests cover at least the following observable cases: immediate success, single `EINTR` then success, repeated `EINTR` then success, and non-`EINTR` failure.
  Traceability: `safe_rw` in `include/safe-read.c:55-71`, `safe-read.c:55-71`