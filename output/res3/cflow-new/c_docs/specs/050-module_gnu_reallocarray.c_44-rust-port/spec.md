# spec.md

## Overview

This module provides a single allocation helper: `reallocarray`. Its purpose is to resize or allocate an array-sized memory region while guarding against multiplication overflow when computing the total byte count from `nmemb * size`.

The Rust rewrite must preserve the same externally observable behavior of this module: accept an existing allocation pointer or null-equivalent input, compute the requested total size as an array element count multiplied by element size, reject requests whose multiplication would overflow `size_t`, and otherwise delegate to normal reallocation semantics for the computed byte size.

## Scope

In scope for this module:

- Safe computation of total allocation size from element count and element size.
- Detection of multiplication overflow before allocation/reallocation is attempted.
- Reallocation behavior for a caller-supplied pointer and computed total byte count.
- Error signaling for overflow cases through the module’s allocation result behavior.

Out of scope for this module:

- Any allocation APIs other than `reallocarray`.
- Container management, ownership abstractions, or higher-level array types.
- Thread-safety guarantees, custom allocators, recovery strategies, or serialization behavior.

## Feature Specification

### Feature: Overflow-checked array reallocation

The module exposes functionality equivalent to C `reallocarray(ptr, nmemb, size)`.

Behavior required from the Rust version:

- Accept a prior allocation reference equivalent to `ptr`, plus `nmemb` and `size`.
- Compute the requested allocation size as `nmemb * size`.
- Detect when that multiplication cannot be represented in the platform `size_t` range.
- If overflow would occur:
  - fail the operation without attempting a reallocation for the wrapped size;
  - report failure through the function result equivalent to a null return in C.
- If overflow does not occur:
  - perform allocation/reallocation behavior for the computed total byte size;
  - return a result equivalent to the underlying reallocation result.

This module is a boundary utility: it adds overflow checking to array-size allocation requests and otherwise preserves standard reallocation semantics.

## User Scenarios & Testing

### Scenario 1: Allocate a new array by element count and element size

A caller needs storage for `nmemb` elements of `size` bytes and has no existing allocation.

Expected support:

- Passing a null-equivalent pointer with valid `nmemb` and `size`.
- The module computes the total size and allocates that many bytes if representable.
- The result indicates success with a usable allocation, or failure if allocation itself fails.

Test coverage:

- Null-equivalent input pointer, small nonzero `nmemb` and `size`.
- Null-equivalent input pointer with zero-sized total request, matching underlying realloc behavior.

### Scenario 2: Grow or shrink an existing array allocation

A caller has an existing allocation and wants to resize it based on a new array length and element width.

Expected support:

- Passing an existing allocation reference and new `nmemb`/`size`.
- The module computes the new total byte count and requests reallocation to that size.
- The returned result matches underlying reallocation success or failure semantics when no overflow occurs.

Test coverage:

- Resize to a larger total size without overflow.
- Resize to a smaller total size without overflow.

### Scenario 3: Reject an impossible array size

A caller requests an array size whose byte count would overflow `size_t`.

Expected support:

- The module detects overflow before allocation is attempted.
- The operation fails rather than reallocating an incorrect wrapped size.

Test coverage:

- Inputs where `nmemb * size` exceeds the maximum representable `size_t`.
- Boundary case near the overflow threshold.

### Scenario 4: Accept boundary values that do not overflow

A caller requests a total size exactly within representable range.

Expected support:

- The module allows requests where `nmemb * size` is representable, including maximum non-overflowing combinations.
- The operation proceeds to normal reallocation handling.

Test coverage:

- Inputs whose product is exactly the largest representable non-overflowing value.
- Inputs with one factor zero.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide the functionality of `reallocarray(ptr, nmemb, size)` as the module’s public behavior.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **FR-2**: The module shall compute the requested allocation size from the product of `nmemb` and `size`.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **FR-3**: The module shall detect whether `nmemb * size` overflows the representable `size_t` range before performing reallocation.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **FR-4**: If overflow is detected, the module shall fail the request and return a failure result equivalent to a null pointer in C.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **FR-5**: If no overflow is detected, the module shall perform reallocation behavior using the computed total byte size.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **FR-6**: The module shall preserve standard realloc-style handling of the input pointer by accepting either an existing allocation or a null-equivalent input.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

### Key Entities

- **Allocation pointer/input reference**: Represents the existing allocation to be resized, or a null-equivalent value indicating allocation of new storage.
  **Traceability:** `reallocarray(void *ptr, size_t nmemb, size_t size)`.

- **Element count (`nmemb`)**: The requested number of array elements.
  **Traceability:** `reallocarray(void *ptr, size_t nmemb, size_t size)`.

- **Element size (`size`)**: The byte size of each element.
  **Traceability:** `reallocarray(void *ptr, size_t nmemb, size_t size)`.

- **Total byte size**: The computed product `nmemb * size`, used only when representable without overflow.
  **Traceability:** behavior of `reallocarray` in `gnu/reallocarray.c`.

Relationship of entities:

- `nmemb` and `size` combine to form the requested total byte size.
- The allocation pointer and total byte size together determine the reallocation request.
- Overflow validation gates whether the reallocation request is permitted to proceed.

## Success Criteria

- **SC-1**: For all tested inputs where `nmemb * size` is representable in `size_t`, the Rust module produces allocation/reallocation results consistent with a normal reallocation request for that exact total byte count.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **SC-2**: For all tested inputs where `nmemb * size` would overflow `size_t`, the Rust module rejects the request and returns a failure result instead of using a wrapped byte count.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **SC-3**: Tests cover null-equivalent input, existing allocation input, zero-factor requests, non-overflow boundary requests, and overflow boundary requests.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.

- **SC-4**: The Rust rewrite introduces no additional public functionality beyond the overflow-checked array reallocation behavior evidenced by this module.
  **Traceability:** `gnu/reallocarray.c`, function `reallocarray`.