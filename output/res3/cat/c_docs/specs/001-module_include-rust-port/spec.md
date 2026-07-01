# spec.md

## Title

Rust Port Functional Specification: `module_include`

## Metadata

- Project: `cat`
- Module: `module_include`
- Category: `module`
- Source branch: `001-module_include-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides a safe retrying read/write helper around a file descriptor operation, exposed through `safe_rw`. Its functional purpose is to perform a single buffer transfer request while handling interruption by signals transparently to the caller.

The Rust rewrite must preserve the observable behavior evidenced by the source module: accepting a file descriptor, a buffer reference, and a byte count; attempting the underlying transfer; retrying when the operation is interrupted; and returning the resulting byte count from the successful underlying call or the final call outcome.

## Scope

In scope for this module:

- The behavior represented by `safe_rw`.
- Support for callers that need a transfer helper resilient to interrupted system calls.

Out of scope:

- Defining broader file I/O abstractions beyond the evidenced helper.
- Adding new public APIs not evidenced by the source module.
- Extending behavior beyond interrupted-call retry semantics.

## Feature Specification

### Feature: Interrupted-call-safe transfer helper

The module shall provide the functional equivalent of `safe_rw(fd, buf, count) -> size_t`.

Behavior required from the Rust version:

- Accept a file descriptor identifying the target of the transfer.
- Accept a caller-provided buffer and requested byte count.
- Attempt the underlying transfer operation for the given descriptor and byte count.
- If the transfer attempt is interrupted by a signal-equivalent condition (`EINTR` behavior), retry the transfer instead of surfacing that interruption immediately.
- Continue retrying until the transfer attempt completes without that interruption condition.
- Return the byte count result produced by the completed underlying call.

### Functional boundary

The evidenced module boundary is narrow:

- It is a helper concerned with retry behavior for interrupted transfer attempts.
- It does not define policy for buffering, parsing, formatting, stream ownership, or multi-operation workflows.
- It does not expose additional stateful entities.

## User Scenarios & Testing

### Scenario 1: Read or write call interrupted by a signal

A caller invokes the module helper during file descriptor I/O. The first underlying transfer attempt is interrupted before completion with an interrupt error condition. The module retries automatically, and a later attempt completes.

Expected support in Rust:

- The caller does not need to implement its own retry loop for interruption.
- The returned value is the result of the successful non-interrupted attempt.

Test guidance:

- Simulate an underlying transfer operation that returns an interrupt condition on the first call and a successful byte count on the second call.
- Verify that the helper retries and returns the successful byte count.

### Scenario 2: Transfer succeeds immediately

A caller invokes the helper and the underlying transfer completes on the first attempt.

Expected support in Rust:

- The helper performs no unnecessary extra retries.
- The return value matches the underlying transfer result.

Test guidance:

- Simulate a successful first attempt.
- Verify exactly one underlying call and the returned byte count.

### Scenario 3: Transfer completes with zero bytes

A caller invokes the helper and the underlying transfer reports zero bytes transferred without interruption.

Expected support in Rust:

- The zero result is returned directly.
- No retry occurs unless the call was interrupted.

Test guidance:

- Simulate a completed call returning `0`.
- Verify the helper returns `0`.

### Scenario 4: Non-interrupt outcome

A caller invokes the helper and the underlying transfer completes with a final outcome other than interruption retry.

Expected support in Rust:

- The helper stops retrying once the outcome is not the interrupt condition.
- The returned result corresponds to that final call outcome as represented by the Rust port’s chosen interface.

Test guidance:

- Simulate a non-`EINTR` terminal outcome.
- Verify that the helper does not loop indefinitely and surfaces the final call result.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide one transfer helper corresponding to `safe_rw` evidenced in `include/safe-read.c`.
  **Traceability**: `safe_rw` in `include/safe-read.c:55-71`.

- **FR-2**: The helper shall take three inputs functionally equivalent to a file descriptor, a buffer reference, and a requested byte count.
  **Traceability**: `safe_rw (int fd, void const *buf, size_t count)` in `include/safe-read.c:55-71`.

- **FR-3**: The helper shall repeatedly invoke the underlying transfer operation when the previous attempt ended with an interruption condition equivalent to `EINTR`.
  **Traceability**: `safe_rw` in `include/safe-read.c:55-71`.

- **FR-4**: The helper shall stop retrying once the underlying transfer no longer reports the interruption condition.
  **Traceability**: `safe_rw` in `include/safe-read.c:55-71`.

- **FR-5**: The helper shall return the transfer result from the first non-interrupted completion of the underlying operation.
  **Traceability**: `safe_rw` in `include/safe-read.c:55-71`.

- **FR-6**: The helper shall not require caller-managed retry logic for interrupted transfer attempts.
  **Traceability**: retry behavior embodied by `safe_rw` in `include/safe-read.c:55-71`.

### Key Entities

This module does not define module-specific structs or persistent state in the provided analysis.

Key functional entities are:

- **File descriptor**: an integer-like handle identifying the transfer target or source.
- **Buffer reference**: caller-provided memory used by the transfer operation.
- **Byte count**: the requested transfer length and returned completion amount.
- **Transfer result**: the outcome returned after retrying interrupted attempts until a non-interrupted completion occurs.

Relationships:

- The caller supplies the file descriptor, buffer reference, and byte count to the helper.
- The helper performs the transfer operation using those inputs.
- The helper returns a transfer result associated with the same request after applying interruption retry semantics.

## Success Criteria

- **SC-1**: A test double that produces one or more interrupt-condition results followed by a successful transfer causes the Rust helper to retry and return the successful byte count.
  **Traceability**: `safe_rw` retry behavior in `include/safe-read.c:55-71`.

- **SC-2**: When the first transfer attempt completes without interruption, the Rust helper returns that result without requiring additional caller action.
  **Traceability**: `safe_rw` completion behavior in `include/safe-read.c:55-71`.

- **SC-3**: When the transfer completes with `0`, the Rust helper returns `0`.
  **Traceability**: `safe_rw` return of underlying non-interrupted result in `include/safe-read.c:55-71`.

- **SC-4**: The Rust port exposes no additional module functionality beyond the evidenced interrupted-transfer helper behavior.
  **Traceability**: only `safe_rw` is identified in `include/safe-read.c`.

- **SC-5**: All documented behavior in the Rust module can be mapped directly to the evidenced source function boundary and inputs/outputs of `safe_rw`.
  **Traceability**: `safe_rw (int fd, void const *buf, size_t count)` in `include/safe-read.c:55-71`.

## Acceptance Notes

- The Rust rewrite may adapt representation details to Rust, but it must preserve the functional contract evidenced by the source module.
- Validation should focus on observable behavior: retry on interruption, stop on non-interruption, and return the resulting transfer count.