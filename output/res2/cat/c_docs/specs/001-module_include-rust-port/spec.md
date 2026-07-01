# spec.md

## Title

Rust Port Functional Specification: `module_include`

## Summary

This module provides a safe read/write wrapper used by the project to perform file-descriptor I/O while tolerating interrupt-related transient failure. The Rust rewrite must preserve the module’s observable behavior for the provided operation: attempt the underlying read/write action, retry when interrupted, and return the final byte-count result produced by the system call behavior exposed through the module.

## Scope

In scope:

- Port the functionality represented by `safe_rw` from `include/safe-read.c`.
- Preserve the module’s externally visible behavior as a file-descriptor I/O helper.

Out of scope:

- New public APIs beyond the behavior evidenced by the existing module.
- Additional buffering, formatting, async I/O, or higher-level stream abstractions.
- New error-reporting models not required to preserve the original module behavior.

## Feature Specification

### Feature: interrupt-tolerant file-descriptor I/O wrapper

The module supplies a helper that executes a read/write-style operation against a file descriptor and shields callers from transient interruption by retrying when the operation is interrupted by a signal.

The Rust version must implement the same functional boundary:

- Accept a file descriptor, a buffer reference, and a requested byte count.
- Invoke the underlying file-descriptor I/O operation associated with this module.
- If the operation is interrupted and would otherwise fail only for that reason, retry the operation.
- Stop retrying once the operation completes or fails for a reason other than interruption.
- Return the resulting byte count as the module result.

The module is a low-level utility. It does not define higher-level file processing rules; it only provides the retrying I/O behavior evidenced by the existing function.

## User Scenarios & Testing

### Scenario 1: operation succeeds immediately

A caller uses the module helper with a valid file descriptor and buffer, and the underlying operation succeeds on the first attempt.

Expected result:

- The module returns the byte count produced by that successful operation.
- No extra observable behavior is introduced.

Test approach:

- Use a descriptor and conditions where the operation completes without interruption.
- Verify the returned count matches the underlying system result.

### Scenario 2: operation is interrupted before succeeding

A caller invokes the helper, the underlying operation is interrupted by a signal, and a subsequent retry succeeds.

Expected result:

- The module retries automatically after interruption.
- The final return value is the byte count from the successful retry.

Test approach:

- Simulate or induce an interrupt-caused failure on the first attempt.
- Verify the helper does not surface that interrupted attempt as the final result.
- Verify it returns the later successful byte count.

### Scenario 3: operation fails for a non-interrupt reason

A caller invokes the helper but the underlying operation cannot complete for a reason other than interruption.

Expected result:

- The module does not keep retrying indefinitely for non-interrupt failures.
- The module returns the final result corresponding to that failure behavior.

Test approach:

- Use an invalid descriptor or another condition that produces a non-interrupt failure.
- Verify the helper stops and returns the failure result without masking it as success.

### Scenario 4: zero-length request

A caller invokes the helper with a requested byte count of zero.

Expected result:

- The module forwards the request through the same wrapper behavior.
- The result matches the underlying operation’s result for a zero-length request.

Test approach:

- Call the helper with count `0`.
- Verify the observed return matches system-call behavior under the same conditions.

## Requirements

### Functional Requirements

- **FR-1**: The Rust module shall provide the behavior of the existing `safe_rw` helper from `include/safe-read.c`.
  **Traceability**: `safe_rw`

- **FR-2**: The module shall perform a file-descriptor I/O operation using three inputs: file descriptor, buffer, and requested byte count.
  **Traceability**: `safe_rw (int fd, void const *buf, size_t count)`

- **FR-3**: If the attempted operation is interrupted by a signal-related transient condition equivalent to `EINTR`, the module shall retry the operation until it no longer ends for that reason.
  **Traceability**: `safe_rw`

- **FR-4**: If the attempted operation completes without interruption, the module shall return the byte count result from that completion.
  **Traceability**: `safe_rw`

- **FR-5**: If the attempted operation fails for a reason other than interruption, the module shall stop retrying and return the corresponding result of that failed attempt.
  **Traceability**: `safe_rw`

- **FR-6**: The module shall not add higher-level semantics beyond the retry-on-interrupt behavior evidenced by the source module.
  **Traceability**: `include/safe-read.c`, `safe_rw`

### Key Entities

- **Safe read/write helper operation**: The module’s sole evidenced functional entity is the helper represented by `safe_rw`, which mediates a file-descriptor I/O call and applies retry-on-interrupt behavior.
- **File descriptor**: Integer handle identifying the I/O target used by the helper.
- **Buffer input**: Caller-provided memory reference supplied to the helper for the underlying I/O operation.
- **Byte count**: Requested transfer size provided by the caller and returned as the result when the operation completes.

Relationships:

- The helper consumes a file descriptor, buffer input, and byte count.
- The helper returns a byte-count result reflecting the final underlying operation outcome after any interrupt-driven retries.

## Success Criteria

- **SC-1**: A Rust implementation of this module exposes behavior equivalent to `safe_rw` for the supported file-descriptor I/O use in this module.
  **Traceability**: `safe_rw`

- **SC-2**: In tests where the first attempted operation is interrupted and a later attempt succeeds, the Rust module returns the later successful byte count rather than surfacing the interrupted attempt as final.
  **Traceability**: `safe_rw`

- **SC-3**: In tests where the operation fails for a non-interrupt reason, the Rust module stops retrying and returns the corresponding failure result.
  **Traceability**: `safe_rw`

- **SC-4**: In tests where the operation succeeds on the first try, the Rust module returns the same byte count as the underlying operation.
  **Traceability**: `safe_rw`

- **SC-5**: In tests using a zero-length request, the Rust module matches the underlying operation’s observed result for that request size.
  **Traceability**: `safe_rw`