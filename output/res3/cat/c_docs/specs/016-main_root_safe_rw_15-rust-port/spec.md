# spec.md

## Title

Functional Specification: `main_root_safe_rw_15`

## Document Metadata

- Project: `cat`
- Module: `main_root_safe_rw_15`
- Category: `main_cluster`
- Rust Branch: `016-main_root_safe_rw_15-rust-port`
- Source Basis:
  - `include/safe-read.c`
  - `safe-read.c`
- Primary Function:
  - `safe_rw(int fd, void const *buf, size_t count) -> size_t`
- Generation Date: 2026-06-09

## Overview

This module provides a single safe read/write helper routine, `safe_rw`, used to perform an I/O transfer request on a file descriptor with bounded retry behavior around interrupt-related failure conditions.

The Rust rewrite must preserve the observable behavior of this helper as a small, single-call I/O wrapper:

- it attempts the requested transfer on the provided file descriptor,
- it tolerates transient interruption conditions by retrying,
- it stops retrying once a non-interruption result is obtained,
- and it returns the transfer result in the same result domain as the C module.

No additional public capabilities are evidenced for this module.

## Feature Specification

### Feature: Safe single-operation file descriptor I/O wrapper

The module exposes one functional unit: a helper that performs one read/write-style system call against a file descriptor and protects callers from spurious interruption failures.

The Rust version must implement equivalent functionality for:

- accepting a file descriptor handle value,
- accepting a caller-provided buffer pointer/reference,
- accepting a requested byte count,
- invoking the underlying transfer operation once or repeatedly as needed,
- retrying only when the attempted operation fails due to interruption,
- returning immediately for all other outcomes.

### Behavioral Scope

Within the evidence of the source module, the behavior is limited to the transfer wrapper itself. The Rust rewrite must therefore preserve these boundaries:

- It is not responsible for opening or closing file descriptors.
- It does not allocate or own transfer buffers.
- It does not define higher-level stream semantics.
- It does not guarantee completion of the full requested byte count.
- It does not convert the operation into buffered, vectored, asynchronous, or multi-step I/O.

## User Scenarios & Testing

### Scenario 1: Read or write attempt succeeds immediately

A caller needs to perform an I/O transfer on an already-open file descriptor. The first underlying system call completes without interruption.

Expected support in the Rust version:

- perform the transfer once,
- return the resulting byte count directly,
- do not introduce extra retries after success.

Test focus:

- verify that the returned value equals the underlying successful transfer result,
- verify no retry occurs when interruption is not reported.

### Scenario 2: Transfer is interrupted and then succeeds

A caller performs an I/O transfer, but the first attempt is interrupted by a signal and reports an interruption error. A later retry succeeds.

Expected support in the Rust version:

- detect the interruption condition,
- retry the transfer,
- return the successful result from the retry.

Test focus:

- simulate one or more interruption failures before success,
- verify the wrapper continues retrying only across interruption cases,
- verify the final returned value is the successful transfer result.

### Scenario 3: Transfer fails for a non-interruption reason

A caller performs an I/O transfer, and the operation fails for a reason other than interruption.

Expected support in the Rust version:

- stop retrying,
- return the failure result immediately in the module’s return domain.

Test focus:

- simulate a permanent error condition,
- verify only interruption-triggered failures are retried,
- verify non-interruption failure is surfaced without looping.

### Scenario 4: Zero-length transfer request

A caller requests a transfer of zero bytes.

Expected support in the Rust version:

- pass through the request through the same wrapper behavior,
- return the underlying operation result without special higher-level semantics beyond the module’s wrapper contract.

Test focus:

- invoke the wrapper with `count == 0`,
- verify the wrapper remains well-defined and returns the system-call result domain consistently.

## Requirements

### Functional Requirements

#### FR-1: Provide a safe read/write wrapper
The module shall provide one helper function, `safe_rw`, that accepts:

- a file descriptor,
- a buffer reference corresponding to caller-supplied storage,
- a requested transfer byte count.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-2: Attempt the underlying transfer operation
The module shall perform the underlying read/write-style transfer operation using the caller-supplied descriptor, buffer, and count.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-3: Retry on interruption
If the underlying transfer attempt fails because it was interrupted, the module shall retry the operation until an outcome other than interruption is obtained.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-4: Do not retry on other outcomes
If the underlying transfer attempt either succeeds or fails for a reason other than interruption, the module shall stop retrying and return that outcome.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-5: Preserve return-domain compatibility
The Rust rewrite shall expose behavior equivalent to the C module’s observable return contract for this helper: the result must correspond to the transfer outcome produced after any interruption retries, within the same success/failure meaning used by the source module.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### Key Entities

#### Entity: `safe_rw`
The sole functional entity in this module is the transfer helper `safe_rw`.

Relationship to other inputs:

- consumes a file descriptor identifying the I/O target,
- consumes a caller-managed buffer participating in the transfer,
- consumes a requested byte count,
- produces a transfer result after applying interruption retry behavior.

#### Entity: File descriptor
An external operating-system file descriptor value supplied by the caller.

Relationship to `safe_rw`:

- serves as the target of the transfer operation,
- is not created, validated for ownership, or closed by this module.

#### Entity: Transfer buffer
Caller-supplied memory referenced by the wrapper during the transfer.

Relationship to `safe_rw`:

- provides the data source or destination for the underlying I/O call,
- remains caller-owned and outside module lifecycle management.

#### Entity: Transfer count
A caller-supplied byte count for the requested operation.

Relationship to `safe_rw`:

- defines the requested transfer size,
- is forwarded as part of the underlying transfer attempt.

## Success Criteria

### SC-1: Immediate success passthrough
When the underlying transfer operation succeeds on the first attempt, the Rust version returns the same successful transfer result without additional retry attempts.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-2: Interruption retry correctness
When the underlying transfer operation reports one or more interruption failures followed by a non-interruption outcome, the Rust version retries through the interruption failures and returns the first later non-interruption outcome.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-3: Non-interruption failure passthrough
When the underlying transfer operation fails for a reason other than interruption, the Rust version returns that failure outcome without retrying indefinitely or masking it as success.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-4: Interface scope preservation
The Rust rewrite exposes only the evidenced module behavior of the source helper and does not require additional caller responsibilities beyond providing the descriptor, buffer, and count.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-5: Scenario coverage
The Rust implementation is verifiably usable for:
- immediate successful transfer,
- interrupted-then-successful transfer,
- non-interruption failure,
- zero-length request handling within the same wrapper contract.

Traceability:
- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`