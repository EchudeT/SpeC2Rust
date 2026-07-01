# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_gnu_reallocarray.c_44`
- **Category**: `module_cluster`
- **Source basis**: `gnu/reallocarray.c`
- **Primary interface**: `reallocarray(void *ptr, size_t nmemb, size_t size)`

## Feature Specification

This module provides a single allocation utility that resizes or allocates a memory region based on an element count and an element size.

The module’s functional purpose is:

- to compute the total allocation size from `nmemb * size`
- to detect multiplication overflow before attempting allocation
- to delegate allocation behavior as a reallocation of `ptr` to the computed total size when the multiplication is valid
- to report failure by returning `NULL` when the requested total size cannot be represented safely

The Rust rewrite must implement equivalent observable behavior for this module’s responsibility:

- accept an existing allocation pointer or null-equivalent input
- accept a requested element count and element size
- determine whether `nmemb * size` overflows `size_t`-equivalent bounds
- if no overflow occurs, perform the equivalent of reallocating to the computed total byte size
- if overflow occurs, fail without attempting the allocation and report failure through the function result

No additional public capabilities are required beyond this checked array reallocation behavior.

## User Scenarios & Testing

### Scenario 1: Allocate a new array-sized region
A caller needs memory for `nmemb` elements of `size` bytes each and has no prior allocation.

Expected behavior:

- the caller passes a null pointer with valid `nmemb` and `size`
- the module computes the total size as `nmemb * size`
- if the multiplication does not overflow, the module returns a pointer to a region sized for that total byte count
- if allocation fails, the module returns `NULL`

### Scenario 2: Grow or shrink an existing allocation
A caller already has a pointer and needs to resize it to hold a different number of elements.

Expected behavior:

- the caller passes the existing pointer with new `nmemb` and `size`
- the module computes the new total size
- if the multiplication does not overflow, the module performs the equivalent of `realloc(ptr, nmemb * size)`
- success returns a resulting pointer; failure returns `NULL`

### Scenario 3: Reject impossible array sizes
A caller requests a count and element size whose product exceeds the representable allocation size range.

Expected behavior:

- the module detects the overflow condition before reallocation
- the module returns `NULL`
- the module does not proceed as though the wrapped product were valid

### Scenario 4: Zero-sized total request
A caller passes values that produce a total size of zero.

Expected behavior:

- the module treats the request through the same checked multiplication path
- with no overflow, it forwards the zero total size to the underlying reallocation semantics
- the result follows the platform allocation semantics for zero-size reallocation, as exposed through the function result

### Testing coverage derived from scenarios

The Rust version must be testable for:

- successful allocation from a null input with a non-overflowing product
- successful resize from a non-null input with a non-overflowing product
- overflow detection for products larger than the maximum representable size
- forwarding behavior for zero-total-size requests
- failure signaling through a null-equivalent result on overflow

## Requirements

### Functional Requirements

#### FR-1: Checked total-size computation
The module shall compute the requested allocation size as the product of element count and element size.

**Traceability**: `gnu/reallocarray.c`, `reallocarray`

#### FR-2: Overflow rejection
The module shall detect when `nmemb * size` cannot be represented in the target size type and shall fail the operation instead of using a wrapped or truncated product.

**Traceability**: `gnu/reallocarray.c`, `reallocarray`

#### FR-3: Reallocation on valid size
When the total size computation is valid, the module shall perform the equivalent of reallocating `ptr` to the computed total byte size.

**Traceability**: `gnu/reallocarray.c`, `reallocarray`

#### FR-4: Failure signaling
When overflow is detected, or when the underlying reallocation cannot provide the requested storage, the module shall signal failure by returning `NULL` or the Rust-port equivalent failure result for this interface boundary.

**Traceability**: `gnu/reallocarray.c`, `reallocarray`

#### FR-5: Null-input support
The module shall accept a null input pointer and treat it according to reallocation semantics for allocating a new region of the computed total size.

**Traceability**: `gnu/reallocarray.c`, `reallocarray`

#### FR-6: Existing-pointer support
The module shall accept a non-null input pointer and treat it according to reallocation semantics for resizing an existing region to the computed total size.

**Traceability**: `gnu/reallocarray.c`, `reallocarray`

### Key Entities

#### Entity: Allocation pointer
The module operates on an opaque memory pointer representing either:

- no existing allocation when null
- an existing allocation to be resized when non-null

Relationship:

- this pointer is the subject of the reallocation request

**Traceability**: `reallocarray(void *ptr, size_t nmemb, size_t size)`

#### Entity: Element count
The module accepts a count of elements to allocate space for.

Relationship:

- combined with element size to determine the total byte request

**Traceability**: `reallocarray(void *ptr, size_t nmemb, size_t size)`

#### Entity: Element size
The module accepts the size in bytes of each element.

Relationship:

- combined with element count to determine the total byte request

**Traceability**: `reallocarray(void *ptr, size_t nmemb, size_t size)`

#### Entity: Total allocation size
The module derives a total byte size from the multiplication of count and size.

Relationship:

- if representable, it is passed to reallocation behavior
- if not representable, the request is rejected

**Traceability**: `gnu/reallocarray.c`, `reallocarray`

## Success Criteria

1. **Overflow safety**: For inputs where `nmemb * size` exceeds the maximum representable size, the Rust version rejects the request and returns the interface’s failure result without treating the wrapped product as valid.
   **Traceability**: `gnu/reallocarray.c`, `reallocarray`

2. **Correct valid-size behavior**: For inputs where `nmemb * size` is representable, the Rust version uses that exact total byte size for the reallocation operation.
   **Traceability**: `gnu/reallocarray.c`, `reallocarray`

3. **Null-pointer allocation behavior**: When invoked with a null pointer and a representable total size, the Rust version behaves as a new allocation request through reallocation semantics.
   **Traceability**: `gnu/reallocarray.c`, `reallocarray`

4. **Existing-pointer resize behavior**: When invoked with a non-null pointer and a representable total size, the Rust version behaves as a resize request through reallocation semantics.
   **Traceability**: `gnu/reallocarray.c`, `reallocarray`

5. **Failure reporting consistency**: On overflow and on allocation failure, the Rust version reports failure through the function result rather than returning a success pointer.
   **Traceability**: `gnu/reallocarray.c`, `reallocarray`

6. **Zero-size request handling**: For requests whose computed total size is zero and does not overflow, the Rust version forwards the zero-size request consistently with reallocation semantics rather than treating it as an overflow case.
   **Traceability**: `gnu/reallocarray.c`, `reallocarray`