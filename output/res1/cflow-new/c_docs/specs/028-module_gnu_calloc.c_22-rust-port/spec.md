# spec.md

## Title

Functional Specification for `module_gnu_calloc.c_22` Rust Port

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_gnu_calloc.c_22`
- **Category**: `module_cluster`
- **Source file**: `gnu/calloc.c`
- **Primary function**: `rpl_calloc`
- **Target Rust branch**: `028-module_gnu_calloc.c_22-rust-port`
- **Generation date**: 2026-06-11

## Overview

This module provides a replacement calloc-style allocation function. Its functional role is to allocate memory for `n` elements of size `s` and return a pointer to a zero-initialized block when the requested total size is valid and allocation succeeds.

The Rust rewrite must preserve this observable behavior at the module boundary:

- accept an element count and element size,
- detect requests whose total size cannot be represented safely,
- return failure for invalid or unsuccessful allocations,
- return a usable allocation whose full extent is zero-initialized when successful.

No additional public functionality is evidenced for this module and none is required.

## Scope

### In Scope

- The behavior of the replacement allocation function corresponding to `rpl_calloc`.
- Validation of the requested allocation size as derived from `n * s`.
- Success and failure behavior of the allocation request.
- Zero-initialization of successfully allocated memory.

### Out of Scope

- Any allocator configuration API.
- Resizing, freeing, or ownership-management APIs beyond what is needed internally by the Rust rewrite.
- Any behavior not evidenced by `gnu/calloc.c` and `rpl_calloc`.

## Feature Specification

### Feature: Replacement calloc behavior

The module shall provide the functionality of a replacement calloc-like operation for callers that need allocation of `n` objects of size `s`.

Observed functional boundary from the source analysis:

- Input: two `size_t` values, `n` and `s`.
- Output: a pointer result that indicates either successful allocation or failure.

The Rust version must implement the following observable behavior:

1. Compute the requested total allocation size from the element count and element size.
2. Reject requests whose multiplication is not safely representable as a valid total size.
3. On successful allocation, provide memory whose entire allocated range is initialized to zero bytes.
4. On failure, report failure through the module’s result mechanism corresponding to a null return in the C behavior.

### Feature: Overflow-protected allocation request

The module must protect the allocation request against invalid total-size computation. The Rust rewrite must preserve the behavior that an allocation request is not allowed to proceed as if successful when `n * s` would overflow the size domain represented by the source interface.

### Feature: Allocation result compatibility

The module’s Rust-facing design may differ internally, but its externally observable behavior must remain compatible with the source module’s semantics:

- valid request + successful allocation => zeroed allocation returned,
- overflow or allocation failure => failure returned.

## User Scenarios & Testing

### Scenario 1: Allocate zero-initialized storage for an array

A caller requests storage for `n` elements of size `s`, where `n * s` is representable and allocation succeeds.

**Expected behavior**
- The module returns success.
- The returned memory region has length `n * s`.
- Every byte in the region is zero.

**Testing guidance**
- Use several non-zero `(n, s)` pairs.
- Verify that all bytes in the allocated region are zero before any caller writes to it.

### Scenario 2: Handle zero-sized allocation requests consistently with allocator semantics

A caller requests allocation where `n == 0`, `s == 0`, or both, and the request is passed through the module.

**Expected behavior**
- The module behaves consistently with the underlying replacement calloc semantics for zero-sized requests.
- The call must not be treated as an overflow solely because one factor is zero.
- The result must still follow the module’s success/failure contract.

**Testing guidance**
- Exercise `(0, nonzero)`, `(nonzero, 0)`, and `(0, 0)`.
- Confirm that the module does not misclassify these cases as multiplication overflow.

### Scenario 3: Reject impossible total allocation sizes

A caller provides `n` and `s` such that `n * s` exceeds the representable allocation size range.

**Expected behavior**
- The module returns failure.
- No success value is produced for the request.

**Testing guidance**
- Construct boundary cases near the maximum representable size.
- Verify that overflow cases are rejected deterministically.

### Scenario 4: Propagate allocation failure

A caller makes a valid request, but the allocation cannot be satisfied.

**Expected behavior**
- The module returns failure.
- The module does not report success with an invalid allocation.

**Testing guidance**
- Use fault injection or a controlled allocator failure path in Rust tests.
- Verify that failure is surfaced through the module’s result contract.

## Requirements

### Functional Requirements

- **FR-1**: The module shall accept an element count and element size as allocation inputs, corresponding to `rpl_calloc(size_t n, size_t s)` in `gnu/calloc.c`.
- **FR-2**: The module shall determine the requested total allocation size from the product of the two inputs.
- **FR-3**: The module shall detect when the requested total size cannot be represented safely from the multiplication of the inputs and shall return failure for such requests.
- **FR-4**: The module shall attempt allocation only for requests that pass the total-size validity check.
- **FR-5**: For a successful allocation, the module shall return memory whose allocated extent is initialized to zero.
- **FR-6**: If allocation does not succeed, the module shall return failure.
- **FR-7**: The module shall preserve calloc-style observable semantics for zero-sized requests, subject to allocator success/failure behavior.

### Key Entities

This module has no core custom data structure evidenced in the source analysis.

The key entities are:

- **Allocation request**
  - Composed of:
    - element count `n`
    - element size `s`
  - Relationship:
    - these inputs define the requested total allocation size.

- **Allocation result**
  - Represents either:
    - successful zero-initialized memory, or
    - failure corresponding to the null-result behavior of the C module.

## Success Criteria

- **SC-1**: For valid non-overflowing requests that succeed, tests demonstrate that the module returns a successful allocation and that all bytes in the allocated region are zero.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **SC-2**: For requests where `n * s` exceeds the representable size range, tests demonstrate that the module returns failure and never reports success.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **SC-3**: For valid requests where allocation is forced to fail, tests demonstrate that the module returns failure.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **SC-4**: For zero-sized request combinations, tests demonstrate that the module follows calloc-style success/failure behavior without treating the request as multiplication overflow.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **SC-5**: The Rust port exposes no extra module functionality beyond the replacement calloc behavior evidenced by the source module.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`