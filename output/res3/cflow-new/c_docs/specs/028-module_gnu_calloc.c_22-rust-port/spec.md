# spec.md

## Overview

This module provides a replacement allocation routine, `rpl_calloc`, corresponding to the behavior implemented in `gnu/calloc.c`. Its purpose is to allocate storage for an array of elements while enforcing the key safety property expected from `calloc`: detecting invalid total-size multiplication before allocation and returning zero-initialized memory on success.

The Rust rewrite must preserve the observable behavior of this module as a focused allocation utility with the same functional boundary: accept an element count and element size, compute the total allocation size safely, and either return a successful zeroed allocation or fail without allocating when the request is invalid or cannot be satisfied.

## Scope

In scope for this module:

- Safe computation of total requested allocation size from `(n, s)`.
- Detection of size overflow in the multiplication `n * s`.
- Allocation of zero-initialized storage for the computed size when valid.
- Failure behavior for overflow or allocation failure.

Out of scope for this module:

- Any allocation APIs beyond the replacement calloc behavior evidenced by `rpl_calloc`.
- Data structure management beyond raw allocated storage.
- Ownership abstractions or container semantics not evidenced by the source module.

## Feature Specification

### Feature: Replacement calloc behavior

The module implements a replacement for `calloc` through `rpl_calloc(size_t n, size_t s)`.

Required Rust behavior:

- Accept two size inputs representing:
  - number of elements
  - size of each element
- Determine whether `n * s` is representable as a valid allocation size.
- If the multiplication overflows, report allocation failure and do not allocate memory.
- If the multiplication is valid, request a block of memory of exactly `n * s` bytes.
- On successful allocation, the returned storage must be zero-initialized.
- If allocation cannot be completed, report failure.

This feature exists to preserve the module’s boundary as an allocation helper that protects callers from incorrect wraparound in array-size calculations.

## User Scenarios & Testing

### Scenario 1: Allocate a normal zeroed array buffer

A caller needs storage for `n` elements of size `s`, where `n * s` fits in the platform allocation size range.

Expected behavior:

- The module succeeds.
- The returned memory region has length `n * s`.
- All bytes in the region are initialized to zero.

Test coverage:

- Use representative nonzero values for `n` and `s`.
- Verify successful result.
- Verify every byte in the allocation is zero.

### Scenario 2: Reject an overflowing allocation request

A caller requests an array size where multiplying `n` by `s` exceeds the maximum representable allocation size.

Expected behavior:

- The module fails the request.
- No successful allocation result is returned.

Test coverage:

- Use boundary values that force overflow in `n * s`.
- Verify failure is reported.

### Scenario 3: Propagate failure when allocation is not available

A caller provides valid, non-overflowing sizes, but the underlying allocation cannot be satisfied.

Expected behavior:

- The module fails the request.
- No successful allocation result is returned.

Test coverage:

- Use a test configuration or allocator injection strategy in the Rust implementation sufficient to exercise allocation failure.
- Verify failure is reported for valid inputs when allocation cannot succeed.

### Scenario 4: Handle zero-sized requests consistently with allocator semantics

A caller requests allocation with `n == 0`, `s == 0`, or both, while the multiplication remains valid.

Expected behavior:

- The module forwards the request through the same allocation path used for other valid requests.
- The result follows the platform/allocator success-or-failure semantics for a zero-sized zero-initialized allocation, without overflow misclassification.

Test coverage:

- Exercise `n = 0, s > 0`
- Exercise `n > 0, s = 0`
- Exercise `n = 0, s = 0`
- Verify the request is treated as valid size computation and not as overflow.

## Requirements

### Functional Requirements

#### FR-1: Array allocation request handling
The module shall provide the functionality of `rpl_calloc` by accepting an element count and an element size and treating them as an array allocation request.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

#### FR-2: Overflow-checked size computation
The module shall determine whether the total byte count `n * s` can be represented without overflow before attempting allocation.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

#### FR-3: Failure on invalid total size
If `n * s` overflows, the module shall fail the allocation request and shall not return a successful allocation.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

#### FR-4: Exact-size zeroed allocation on valid input
If `n * s` is valid, the module shall attempt to allocate storage for exactly that total byte count, with the allocated bytes initialized to zero.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

#### FR-5: Failure propagation for unsatisfied allocation
If the allocation attempt for a valid total size cannot be completed, the module shall report failure.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

### Key Entities

This module has no module-specific structured data types evidenced in the input.

Key operational entities are:

- `n`: requested number of elements.
- `s`: requested size of each element.
- `total size`: the computed product `n * s` when representable.
- `allocated zeroed memory block`: the result returned on success.

Relationships:

- `n` and `s` combine to define `total size`.
- `total size` determines whether the request is valid.
- A valid `total size` may produce an allocated zeroed memory block.
- An invalid or unsatisfied request produces failure instead of a successful memory block.

## Success Criteria

### SC-1: Correct success behavior for valid non-overflowing requests
For test cases where `n * s` is representable and allocation succeeds, the Rust module returns a successful allocation representing exactly `n * s` bytes, and the allocated bytes are all zero.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

### SC-2: Correct failure behavior for overflowing requests
For test cases where `n * s` overflows, the Rust module returns failure and does not report successful allocation.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

### SC-3: Correct failure behavior for allocation exhaustion
For test cases where `n * s` is valid but the allocator cannot satisfy the request, the Rust module returns failure.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`

### SC-4: Correct treatment of zero-sized valid requests
For test cases with zero-valued `n` and/or `s` where multiplication does not overflow, the Rust module treats the request as a valid allocation request path rather than as an overflow case.

Traceability:

- `gnu/calloc.c`
- `rpl_calloc`