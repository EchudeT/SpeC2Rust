# spec.md

## Overview

This module provides a single memory-resizing function, `rpl_realloc`, corresponding to the behavior implemented in `gnu/realloc.c`. Its purpose is to supply realloc-style resizing with defined handling for zero-size requests and allocation failure.

The Rust rewrite must preserve the observable behavior of this module as a focused allocation utility used by other code that needs to grow, shrink, allocate, or release dynamically managed memory through one entry point.

## Feature Specification

### Functional Scope

The Rust version must implement the behavior of the module’s sole functional entry point:

- Resize an existing allocation to a requested size.
- Accept a null input pointer as an allocation request.
- Treat a zero-size request specially by freeing the input allocation and returning a null pointer.
- Report allocation failure by returning a null pointer for nonzero requested sizes.
- Preserve standard realloc-style usage expectations for successful resizing.

### Behavioral Notes

The module’s behavior is defined at the allocation API boundary:

- When the requested size is greater than zero, the function behaves as a realloc-compatible resize/allocation operation.
- When the requested size is zero, the function does not retain the original allocation; it releases it and returns null.
- The function does not define higher-level ownership tracking, container semantics, or typed allocation behavior.

## User Scenarios & Testing

### Scenario 1: Allocate through a null input pointer

A caller passes a null pointer and a positive size to request a new allocation.

Expected result:

- The function returns either:
  - a non-null pointer to a block suitable for the requested size, or
  - null if allocation fails.

Testing focus:

- Verify that null input with nonzero size is accepted.
- Verify that failure is reported as null.

### Scenario 2: Grow an existing allocation

A caller already holds a dynamically allocated block and requests a larger size.

Expected result:

- On success, the function returns a pointer representing the resized allocation.
- On failure, the function returns null.

Testing focus:

- Verify that a nonzero resize request can be issued for an existing pointer.
- Verify success and failure signaling through the return value.

### Scenario 3: Shrink an existing allocation

A caller requests a smaller nonzero size for an existing allocation.

Expected result:

- The function returns a pointer representing the resized allocation, or null on failure.

Testing focus:

- Verify that nonzero shrink operations are handled through the same interface.
- Verify that return-value semantics remain consistent.

### Scenario 4: Release memory through a zero-size request

A caller passes any previously allocated pointer with size zero.

Expected result:

- The function frees the input allocation and returns null.

Testing focus:

- Verify that the return value is null for zero-size requests.
- Verify that zero-size requests are treated as deallocation rather than retained zero-length storage.

### Scenario 5: Zero-size request with null input

A caller passes a null pointer and size zero.

Expected result:

- The function returns null without producing a usable allocation.

Testing focus:

- Verify that this edge case is accepted and returns null.

## Requirements

### Functional Requirements

#### FR-1: Realloc-compatible resize entry point

The module shall provide one functionally equivalent entry point to `rpl_realloc` that accepts:

- an existing allocation pointer, which may be null, and
- a requested size.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

#### FR-2: Null input as allocation request

When the input pointer is null and the requested size is greater than zero, the module shall behave as an allocation request for the requested size.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

#### FR-3: Nonzero resize behavior

When the requested size is greater than zero, the module shall attempt to provide storage for that size and return:

- a non-null pointer on success, or
- null on allocation failure.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

#### FR-4: Zero-size deallocation behavior

When the requested size is zero, the module shall release the input allocation, if any, and return null.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

#### FR-5: No additional functional surface

The Rust rewrite shall not require or expose additional module-level behaviors beyond the allocation-resize semantics evidenced by this module.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

### Key Entities

#### Entity: Allocation pointer

An opaque memory reference supplied by the caller and returned by the module.

Role:

- Represents either:
  - no allocation when null, or
  - a dynamically managed allocation subject to resize or release.

Relationships:

- Passed into the resize function as the current allocation state.
- Returned from the resize function as the resulting allocation state.

Traceability:

- `rpl_realloc`

#### Entity: Requested size

An unsigned size value indicating the desired allocation extent.

Role:

- Determines whether the operation is:
  - allocation/resize when greater than zero, or
  - deallocation when equal to zero.

Relationships:

- Combined with the input pointer to determine the module’s operation and result.

Traceability:

- `rpl_realloc`

## Success Criteria

### SC-1: Correct zero-size behavior

For calls corresponding to `rpl_realloc(p, 0)`, the Rust version returns null and performs deallocation behavior for the provided allocation state.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

### SC-2: Correct null-input allocation behavior

For calls corresponding to `rpl_realloc(NULL, n)` where `n > 0`, the Rust version accepts the call and returns either a valid allocation result or null on failure.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

### SC-3: Correct nonzero resize signaling

For calls with a non-null input pointer and `n > 0`, the Rust version returns a non-null pointer on successful resize and null on failure.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

### SC-4: Edge-case acceptance

The Rust version handles the `NULL, 0` case by returning null without requiring special caller-side handling beyond the module contract.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`

### SC-5: Scope fidelity

The Rust rewrite remains limited to the single allocation-resize behavior defined by this module and does not introduce unrelated public functionality.

Traceability:

- `gnu/realloc.c`
- `rpl_realloc`