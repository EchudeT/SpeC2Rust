# spec.md

## Title

Functional Specification for `module_gnu_reallocarray.c_44`

## Overview

This module provides a single allocation helper: `reallocarray`.

Its functional role is to resize or allocate a memory region for an array of elements by combining:

- element count (`nmemb`)
- element size (`size`)
- reallocation of an existing pointer (`ptr`)

The module must preserve the key safety behavior of the C implementation: detect multiplication overflow in `nmemb * size` before attempting allocation. If the requested array size cannot be represented, the operation fails instead of proceeding with a wrapped byte count.

The Rust rewrite must implement the same functional boundary: array-oriented reallocation with overflow-checked total-size computation and failure signaling equivalent to the source module’s behavior.

## Scope

### In Scope

- Computing the total byte size from `nmemb` and `size`
- Detecting overflow in that computation
- Reallocating or allocating memory based on the computed total size
- Returning success or failure according to the result of overflow checking and underlying reallocation

### Out of Scope

- Any API beyond the behavior represented by `reallocarray`
- Container abstractions or collection management
- Initialization of allocated memory
- Memory ownership models beyond what is necessary to represent this allocation operation
- Concurrency, persistence, serialization, or recovery behavior

## Feature Specification

### Feature: Overflow-checked array reallocation

The module provides a function-level capability for array allocation growth or resize requests.

Behavior required from the Rust version:

1. Accept an optional existing allocation reference equivalent in role to `ptr`, plus an element count and element size.
2. Compute the requested total byte count as `nmemb * size`.
3. Reject the request if the multiplication overflows the representable allocation size domain.
4. On non-overflowing input, attempt to allocate or reallocate storage for exactly the computed total number of bytes.
5. Return a success value carrying the resulting allocation reference when the operation succeeds.
6. Return a failure result when:
   - the multiplication overflows, or
   - the underlying allocation/reallocation cannot be completed.

### Behavioral Notes

- The module is array-oriented: callers provide count and element width rather than a precomputed total byte count.
- Overflow detection is part of the module’s required behavior, not an optional validation layer.
- The module does not add higher-level semantics such as element construction, zero-filling, or resizing policies.

## User Scenarios & Testing

### Scenario 1: Allocate storage for a new array

A caller needs storage for `nmemb` elements of size `size` and has no prior allocation.

Expected behavior:

- The module computes `nmemb * size`.
- If the product fits, it allocates that many bytes.
- The call succeeds with a pointer/allocation handle to usable storage, or fails if allocation cannot be performed.

Test coverage:

- Successful allocation with small valid inputs
- Failure propagation when allocation cannot be satisfied
- Correct handling when `ptr` is absent/null-equivalent

### Scenario 2: Grow or shrink an existing array allocation

A caller already has an allocation and needs it resized to hold a different number of elements.

Expected behavior:

- The module computes the new total byte count from the new `nmemb` and `size`.
- If the product fits, it requests reallocation of the existing storage to that size.
- The call returns the resulting pointer/allocation handle on success or a failure result on allocation failure.

Test coverage:

- Reallocation from one valid size to a larger valid size
- Reallocation from one valid size to a smaller valid size
- Preservation of failure signaling when reallocation fails

### Scenario 3: Reject an overflowing array size request

A caller passes values for `nmemb` and `size` whose product exceeds the representable allocation size.

Expected behavior:

- The module detects overflow before performing the allocation request.
- The operation fails.
- No successful allocation result is produced from a wrapped or truncated byte count.

Test coverage:

- Inputs where `nmemb * size` overflows `size_t`-equivalent bounds
- Boundary cases near the maximum representable size
- Confirmation that overflow is treated distinctly as failure of the request

### Scenario 4: Accept zero-sized total requests according to allocator behavior

A caller passes inputs whose total byte count is zero.

Expected behavior:

- The module computes a total size of zero without overflow.
- It delegates to reallocation behavior for size zero rather than inventing additional policy.
- The result follows the platform/runtime allocation semantics used by the Rust rewrite.

Test coverage:

- `nmemb == 0`
- `size == 0`
- Existing allocation with resulting total size zero

## Requirements

### Functional Requirements

#### FR-1: Array-size computation
The module shall derive the requested allocation size from the product of `nmemb` and `size`, matching the role of `reallocarray` in `gnu/reallocarray.c`.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

#### FR-2: Overflow detection
The module shall detect when `nmemb * size` cannot be represented in the target allocation size type and shall treat that condition as failure.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

#### FR-3: Allocation/reallocation on valid size
When the total byte count is representable, the module shall perform allocation or reallocation for exactly that byte count using the caller-supplied existing allocation reference role.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

#### FR-4: Failure signaling
The module shall return a failure outcome when overflow is detected or when the underlying allocation operation fails.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

#### FR-5: Existing pointer support
The module shall support both:
- allocation behavior when the input pointer/reference is absent or null-equivalent
- reallocation behavior when the input pointer/reference refers to an existing allocation

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

### Key Entities

#### Entity: Existing allocation reference
Represents the input memory block to be resized. This corresponds to the `ptr` parameter of `reallocarray`.

Relationship:
- Combined with the computed total byte count to determine whether the operation acts as allocation or reallocation.

Traceability:
- `reallocarray`

#### Entity: Element count
Represents the number of array members requested. This corresponds to `nmemb`.

Relationship:
- Multiplied by element size to form the requested total allocation size.

Traceability:
- `reallocarray`

#### Entity: Element size
Represents the size in bytes of each array member. This corresponds to `size`.

Relationship:
- Multiplied by element count to form the requested total allocation size.

Traceability:
- `reallocarray`

#### Entity: Total byte size
Represents the computed allocation request size `nmemb * size` after overflow validation.

Relationship:
- Derived from element count and element size
- Used as the byte count for allocation or reallocation

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

## Success Criteria

### SC-1: Correct overflow handling
For test inputs where `nmemb * size` exceeds the representable allocation-size range, the Rust module fails the request and does not produce a successful allocation result.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

### SC-2: Correct valid-size handling
For test inputs where `nmemb * size` is representable and the allocator succeeds, the Rust module returns success for the requested total byte count.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

### SC-3: Support for both allocation modes
Tests demonstrate correct behavior both when the existing allocation input is absent/null-equivalent and when it refers to an existing allocation to be resized.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

### SC-4: Failure propagation
When the underlying allocation or reallocation operation fails for a non-overflowing request, the Rust module returns failure rather than reporting success.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`

### SC-5: Zero-total requests are not treated as overflow
For inputs producing a total byte count of zero, the Rust module processes the request through normal allocation/reallocation semantics for zero size rather than rejecting it as multiplication overflow.

Traceability:
- `gnu/reallocarray.c`
- `reallocarray`