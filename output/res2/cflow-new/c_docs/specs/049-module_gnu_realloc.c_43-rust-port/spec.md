# spec.md

## Title
Functional Specification for `module_gnu_realloc.c_43` Rust Rewrite

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_realloc.c_43`
- Category: `module_cluster`
- Source file: `gnu/realloc.c`
- Primary function: `rpl_realloc`
- Rust branch: `049-module_gnu_realloc.c_43-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides a replacement reallocation routine with defined behavior for edge-case allocation sizes. Its functional role is to accept an existing allocation pointer and a requested new size, then return a pointer representing resized storage or report allocation failure through a null result.

The Rust rewrite must preserve the observable behavior of the C module, especially its handling of zero-size reallocation requests. The module’s scope is limited to reallocation behavior and does not define broader memory-management policy beyond the behavior evidenced by `rpl_realloc`.

## Feature Specification

### Summary
The module implements a single reallocation service that:
- accepts an existing pointer, which may be null,
- accepts a requested size,
- performs reallocation compatible with standard `realloc`-style use,
- avoids passing a zero-size request through as a zero-size reallocation by substituting a nonzero allocation request,
- returns a pointer to resized storage on success,
- returns a null pointer on allocation failure.

### Required Rust Functionality
The Rust version must implement equivalent module behavior for the single supported operation:

1. **Reallocate an existing allocation**
   - When given a non-null pointer and a positive size, request resized storage for that allocation and return the resulting pointer.

2. **Handle null input pointer as allocation request**
   - When given a null pointer and a positive size, behave as an allocation request for that size and return the resulting pointer or null on failure.

3. **Handle zero-size requests in a defined way**
   - When given a requested size of zero, the module must not treat the request as a direct zero-size reallocation.
   - Instead, it must perform reallocation using a nonzero size so that the operation yields either a valid allocation result or null on failure.

4. **Preserve failure signaling**
   - If the underlying resize/allocation attempt cannot be satisfied, the module must return null.

## User Scenarios & Testing

### Scenario 1: Allocate through a null pointer
A caller has no existing allocation and calls the module with a null pointer and a positive requested size.

Expected behavior:
- The module returns a non-null pointer if allocation succeeds.
- The module returns null if allocation fails.

Test coverage:
- Null pointer with size greater than zero.
- Verify returned pointer is suitable for subsequent reallocation or release by the owning allocation system.

### Scenario 2: Grow an existing allocation
A caller has an existing allocated block and requests a larger size.

Expected behavior:
- The module returns a pointer to storage representing the resized allocation.
- On failure, the module returns null.

Test coverage:
- Non-null pointer with larger positive size.
- Verify success path returns non-null when allocation succeeds.

### Scenario 3: Shrink an existing allocation
A caller has an existing allocated block and requests a smaller positive size.

Expected behavior:
- The module returns a pointer to storage representing the resized allocation.
- On failure, the module returns null.

Test coverage:
- Non-null pointer with smaller positive size.

### Scenario 4: Request size zero
A caller requests reallocation with size zero, either from a null pointer or an existing allocation.

Expected behavior:
- The module converts the zero-size request into a nonzero reallocation request.
- The operation returns either a valid pointer or null on allocation failure.
- The module does not expose direct zero-size reallocation behavior to callers.

Test coverage:
- Null pointer with size zero.
- Non-null pointer with size zero.
- Verify behavior is consistent with nonzero fallback handling.

### Scenario 5: Propagate allocation failure
A caller requests reallocation and the underlying allocator cannot satisfy the request.

Expected behavior:
- The module returns null.

Test coverage:
- Simulated or injected allocation failure for positive size.
- Simulated or injected allocation failure for zero-size request after nonzero substitution.

## Requirements

### Functional Requirements

#### FR-1: Reallocation entry point
The module shall provide one reallocation operation corresponding to `rpl_realloc`, accepting:
- an input pointer that may be null, and
- a requested size.

Traceability:
- `gnu/realloc.c`
- `rpl_realloc`

#### FR-2: Null-pointer allocation semantics
When the input pointer is null and the requested size is greater than zero, the module shall behave as an allocation request for that size and return the resulting pointer or null on failure.

Traceability:
- `gnu/realloc.c`
- `rpl_realloc`

#### FR-3: Existing-allocation resize semantics
When the input pointer is non-null and the requested size is greater than zero, the module shall request resized storage for the existing allocation and return the resulting pointer or null on failure.

Traceability:
- `gnu/realloc.c`
- `rpl_realloc`

#### FR-4: Zero-size normalization
When the requested size is zero, the module shall normalize the request to a nonzero size before performing the reallocation operation.

Traceability:
- `gnu/realloc.c`
- `rpl_realloc`

#### FR-5: Failure signaling
If the reallocation attempt fails, the module shall return a null pointer.

Traceability:
- `gnu/realloc.c`
- `rpl_realloc`

### Key Entities

#### Entity: Allocation Pointer
An opaque pointer value representing either:
- no current allocation when null, or
- an existing allocation eligible for resizing when non-null.

Relationship:
- Supplied by the caller as the object to allocate or resize.
- Returned by the module as the result of the operation.

Traceability:
- `rpl_realloc` parameter `p`
- `rpl_realloc` return value

#### Entity: Requested Size
A size value indicating the desired allocation extent.

Relationship:
- Controls whether the operation is treated as a normal positive-size resize or a zero-size request requiring normalization.

Traceability:
- `rpl_realloc` parameter `n`

## Success Criteria

### Behavioral Acceptance Criteria
1. A call with a null pointer and positive size returns either:
   - a non-null pointer on success, or
   - null on failure.

   Traceability:
   - `rpl_realloc`

2. A call with a non-null pointer and positive size returns either:
   - a pointer representing resized storage on success, or

   Traceability:

3. A call with size zero does not execute as a direct zero-size reallocation request; it is handled through a nonzero-size substitute request.

   Traceability:

4. Allocation failure is reported exclusively through a null return result.

   Traceability:

### Test Completion Criteria
1. Automated tests cover all four input classes:
   - null pointer, positive size
   - non-null pointer, positive size
   - null pointer, zero size
   - non-null pointer, zero size

2. Automated tests verify null return on forced allocation failure.

3. The Rust rewrite exhibits behavior equivalent to the C module for all tested input classes listed above.