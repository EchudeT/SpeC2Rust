# spec.md

## Title

Functional Specification: `module_gnu_calloc.c_22` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_calloc.c_22`
- Category: `module_cluster`
- Source file: `gnu/calloc.c`
- Primary function: `rpl_calloc`
- Target Rust branch: `028-module_gnu_calloc.c_22-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides a replacement calloc-style allocation routine. Its functional role is to allocate memory for an array-like region defined by an element count and an element size, while ensuring the total size calculation is safe and that allocation failure is reported through a null return.

The Rust rewrite must preserve this behavior boundary:

- accept two size inputs representing count and per-element size,
- detect when the requested total size cannot be represented safely,
- avoid performing an invalid allocation in that case,
- otherwise request zero-initialized storage for the computed total size,
- return a null-equivalent failure result when allocation cannot be completed.

This specification is limited to the behavior evidenced by `rpl_calloc` in `gnu/calloc.c`.

## Feature Specification

### Feature: Safe zero-initialized allocation by count and element size

The module exposes one allocation operation that behaves like a guarded replacement for `calloc`.

The Rust version must implement the following observable behavior:

1. It receives:
   - a number of elements, and
   - a size per element.

2. It computes the total requested allocation size as the product of these two inputs.

3. Before attempting allocation, it checks whether the multiplication is representable in the target size domain.

4. If the multiplication would overflow, the operation fails immediately and returns a null-equivalent result.

5. If the multiplication is valid, it attempts to allocate zero-initialized memory for the computed total size.

6. If the underlying allocation fails, the operation returns a null-equivalent result.

7. If allocation succeeds, it returns a handle/pointer result representing the allocated zeroed region.

No additional public behavior is required beyond this guarded calloc-compatible allocation role.

## User Scenarios & Testing

### Scenario 1: Normal allocation for a small array

A caller needs storage for `n` elements of size `s`, where `n * s` is representable and allocation succeeds.

Expected behavior:

- the module accepts the inputs,
- computes the total size,
- performs a zero-initialized allocation,
- returns a non-null success result.

Test coverage:

- use small valid inputs such as a small count and small element size,
- verify that the result indicates success,
- verify that the allocated region is zero-initialized.

### Scenario 2: Zero-sized request

A caller provides a request where the total requested size is zero because one or both inputs are zero.

Expected behavior:

- the module must treat the request through the same guarded path,
- it must not report overflow,
- it must forward the request as a valid zero-size allocation attempt,
- the return value must follow the underlying allocation outcome for that valid request.

Test coverage:

- invoke with `n = 0, s > 0`,
- invoke with `n > 0, s = 0`,
- confirm no overflow failure path is taken.

### Scenario 3: Overflowing total size

A caller requests a number of elements and element size whose product exceeds representable `size_t` range.

Expected behavior:

- the module detects the overflow condition before allocation,
- no allocation is attempted for the invalid size,
- the operation returns a null-equivalent failure result.

Test coverage:

- choose boundary values that make the multiplication overflow,
- verify that the result indicates failure.

### Scenario 4: Allocation failure without overflow

A caller provides valid sizes, but the allocation cannot be satisfied by the allocator.

Expected behavior:

- the module must distinguish this from arithmetic overflow only by path, not by return shape,
- the result is a null-equivalent failure result.

Test coverage:

- exercise through an allocator stub or controlled failure mechanism in tests,
- verify null-equivalent failure on allocator refusal for a non-overflowing size.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide one calloc-style allocation operation corresponding to `rpl_calloc`.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **FR-2**: The operation shall accept two size-domain inputs: element count and element size.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc (size_t n, size_t s)`

- **FR-3**: The operation shall determine the requested allocation extent from the product of the two inputs.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **FR-4**: The operation shall detect when the product of the two inputs cannot be represented in the size domain used for allocation.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **FR-5**: On detected size multiplication overflow, the operation shall fail without performing the allocation for the invalid computed size and shall return a null-equivalent result.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **FR-6**: When the total requested size is representable, the operation shall request zero-initialized storage for that total size.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **FR-7**: If the allocation request succeeds, the operation shall return a non-null success result referencing the allocated storage.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

- **FR-8**: If the allocation request fails, the operation shall return a null-equivalent failure result.
  **Traceability**: `gnu/calloc.c`, `rpl_calloc`

### Key Entities

This module has no custom core data structures evidenced in the input.

The key functional entities are:

- **Element count (`n`)**: the number of elements requested.
- **Element size (`s`)**: the size of each element requested.
- **Total allocation size (`n * s`)**: the derived size used to determine whether allocation can proceed.
- **Allocation result**: a success/failure pointer-like result, where failure is represented by a null-equivalent value.

Relationships:

- `n` and `s` combine multiplicatively to form the total allocation size.
- The representability of the total allocation size determines whether allocation is attempted.
- The allocation attempt determines whether the final result is success or null-equivalent failure.

## Success Criteria

- **SC-1**: For valid non-overflowing inputs where allocation succeeds, the Rust module returns a non-null result.
  **Traceability**: `rpl_calloc`

- **SC-2**: For valid non-overflowing inputs where allocation succeeds, the allocated region is zero-initialized.
  **Traceability**: `rpl_calloc`

- **SC-3**: For input pairs whose product overflows the size domain, the Rust module returns a null-equivalent failure result.
  **Traceability**: `rpl_calloc`

- **SC-4**: For input pairs whose product overflows the size domain, the Rust module does not proceed as though the overflowed total were a valid allocation size.
  **Traceability**: `rpl_calloc`

- **SC-5**: For non-overflowing inputs where the allocator rejects the request, the Rust module returns a null-equivalent failure result.
  **Traceability**: `rpl_calloc`

- **SC-6**: Zero-size requests are accepted as non-overflowing requests and follow the normal allocation path rather than the overflow failure path.
  **Traceability**: `rpl_calloc`

## Non-Goals

The Rust port is not required by this specification to provide:

- additional allocation APIs,
- custom allocator configuration interfaces,
- thread-safety guarantees beyond those implied by the chosen runtime,
- error objects or rich error enums beyond the null-equivalent failure behavior,
- serialization or persistence behavior,
- recovery or retry mechanisms.

## Acceptance Notes

Conformance should be evaluated by tests that exercise:

- successful zero-initialized allocation,
- zero-size requests,
- multiplication overflow detection,
- allocator failure after a valid size computation.