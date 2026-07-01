# spec.md

## Title
Rust Functional Specification for `main_root_safe_rw_15`

## Summary
This module provides a single helper function, `safe_rw`, used to perform a file-descriptor-based read/write-style operation in a way that safely handles interrupted system calls. The Rust rewrite must preserve the observable behavior of this helper as defined by the C module.

The source evidence identifies one public function:

- `safe_rw(fd, buf, count) -> size_t`

No module-specific structs or other data entities are identified in the provided analysis.

## Scope
In scope:

- Functional behavior of `safe_rw`
- Return-value behavior for successful, partial, zero-length, and failed operations
- Handling of interruption conditions during the underlying system call

Out of scope:

- Introduction of new public APIs
- New buffering strategies
- Thread-safety guarantees
- Async behavior
- Serialization, FFI wrappers, or recovery features

## Feature Specification

### Feature: Safe retrying read/write helper
The module exposes a helper that performs an I/O transfer against a file descriptor using a caller-provided memory buffer and byte count. Its defining behavior is that interruption by signals must not be exposed as a final operation failure when retrying is appropriate.

The Rust version must implement the same functional boundary:

- Accept a file descriptor, a buffer pointer/reference compatible with the intended operation, and a requested byte count.
- Attempt the underlying I/O transfer.
- If the transfer is interrupted by a signal and the operation did not complete, retry until a non-interrupted result is obtained.
- Return the resulting byte count on success.
- Return the failure indicator used by the C function’s contract when a non-retryable error occurs.
- Support a zero-byte request consistently with the underlying operation contract.

Because the available analysis identifies only the helper function and not separate stateful abstractions, the Rust port should preserve this as a small stateless module-level capability.

## User Scenarios & Testing

### Scenario 1: Successful transfer without interruption
A caller uses `safe_rw` to perform an I/O operation on a valid file descriptor and the underlying system call succeeds immediately.

Expected behavior:

- The function returns the number of bytes reported by the underlying operation.
- No extra transformation of the successful result occurs.

Test coverage:

- Valid descriptor
- Nonzero count
- Underlying operation returns a positive byte count

### Scenario 2: Interrupted system call that should be retried
A caller invokes `safe_rw`, and the first attempt is interrupted by a signal in a way that reports an interrupt error condition.

Expected behavior:

- The function does not return failure immediately for the interrupt condition.
- The function retries the operation.
- If a later retry succeeds, the function returns that successful byte count.

Test coverage:

- Simulated or mocked first attempt yielding interrupt error
- Subsequent attempt yielding success
- Final return value matches successful retry result

### Scenario 3: Non-retryable failure
A caller invokes `safe_rw`, and the underlying operation fails for a reason other than interrupt.

Expected behavior:

- The function returns the module’s failure value without converting it into success.
- The error is not retried.

Test coverage:

- Simulated invalid descriptor or equivalent non-interrupt failure
- Single failure result propagated according to the C contract

### Scenario 4: End-of-file or zero-byte completion
A caller uses `safe_rw` in a case where the underlying operation reports zero bytes transferred.

Expected behavior:

- The function returns zero.
- Zero is treated as a valid operation result, not retried as an interrupt.

Test coverage:

- Underlying operation returns `0`
- Function returns `0`

### Scenario 5: Zero-length request
A caller requests transfer of zero bytes.

Expected behavior:

- The function must behave consistently with the underlying I/O operation contract for a zero count.
- The function must not invent additional behavior beyond forwarding the operation and retrying only if interrupted.

Test coverage:

- `count == 0`
- Return value consistent with platform call result

## Requirements

### Functional Requirements

#### FR-1: File-descriptor-based transfer entry point
The module shall provide a function `safe_rw(fd, buf, count) -> size_t` that accepts:

- a file descriptor,
- a caller-supplied buffer,
- a requested byte count.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-2: Preserve successful transfer result
When the underlying I/O operation completes successfully, the module shall return the exact byte-count result produced by that operation.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-3: Retry on interrupt
When the underlying I/O operation reports interruption by signal in a retryable form, the module shall retry instead of returning that interrupt condition as the final result.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-4: Stop retrying on non-interrupt outcome
The module shall stop retrying once the underlying operation yields either:

- a successful result, or
- a failure that is not the retryable interrupt condition.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-5: Preserve failure signaling for non-retryable errors
For a non-retryable failure, the module shall return the failure indicator defined by the existing C function contract.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

#### FR-6: Support zero-byte results
If the underlying operation reports that zero bytes were transferred, the module shall return zero as the final result.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### Key Entities

#### Entity: `safe_rw`
The module’s sole identified functional entity is the `safe_rw` function.

Attributes:

- Input: file descriptor
- Input: caller-provided buffer
- Input: byte count
- Output: `size_t` transfer result or failure indicator per C contract

Relationship to module behavior:

- It is the only evidenced module interface.
- It encapsulates retry-on-interrupt behavior around an underlying file-descriptor I/O transfer.

## Success Criteria

### SC-1: Interface parity
The Rust module exposes functionality equivalent to the C module’s `safe_rw` entry point, accepting the same categories of input and producing the same category of return result.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-2: Correct interrupt retry behavior
In tests that simulate interruption on one or more initial attempts followed by a successful attempt, the Rust version returns the successful transfer size rather than the interrupt failure.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-3: Correct non-retryable failure behavior
In tests that simulate a non-interrupt failure, the Rust version returns the same failure outcome category required by the C contract and does not continue retrying.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-4: Correct zero-result behavior
In tests where the underlying operation returns zero, the Rust version returns zero.

Traceability:

- `safe_rw` in `include/safe-read.c:55-71`
- `safe_rw` in `safe-read.c:55-71`

### SC-5: No unsupported capability expansion
The Rust rewrite does not introduce additional public module responsibilities beyond the evidenced stateless safe retry helper behavior.

Traceability:

- Module files: `include/safe-read.c`, `safe-read.c`
- Function: `safe_rw`