# spec.md

## Overview

This specification defines the functional behavior to preserve when rewriting `gnu/malloc.c` from the `cflow-new` project into Rust for branch `039-module_gnu_malloc.c_33-rust-port`.

The analyzed module exposes one function, `rpl_malloc(size_t n)`, whose purpose is to provide allocation behavior compatible with the project’s expectations around zero-size allocation and allocation failure handling.

The Rust rewrite must preserve the observable behavior of this module as a small allocation wrapper module. It must not add unrelated APIs or capabilities.

## Feature Specification

### Module Purpose

The module provides a replacement memory-allocation entry point used in place of direct `malloc` calls where the project requires normalized behavior.

### Supported Behavior

The Rust version must implement the following behavior represented by `rpl_malloc`:

- Accept a requested allocation size.
- Request heap allocation for that size.
- When the requested size is zero, treat it as a request for a non-zero allocation so that the function does not rely on platform-specific `malloc(0)` behavior.
- Return a pointer/reference/handle representing successfully allocated storage when allocation succeeds.
- On allocation failure, trigger the module’s failure handling behavior rather than silently returning a null result in ordinary use.

### Behavioral Boundaries

This module is limited to allocation request normalization and forwarding. It does not, based on the analyzed input, define:
- a deallocation API,
- resizing/reallocation behavior,
- custom allocator configuration,
- ownership-tracking data structures,
- object initialization beyond allocation semantics.

## User Scenarios & Testing

### Scenario 1: Allocate a positive number of bytes

A caller requests allocation for a non-zero size.

Expected behavior:
- The module attempts to allocate that exact size.
- If allocation succeeds, the caller receives usable allocated storage.
- If allocation fails, the module follows its failure path.

Test coverage:
- Request a small positive size such as 1 or 16.
- Verify the result indicates successful allocation when memory is available.
- Verify the returned storage can be treated as allocated memory by surrounding integration tests.

### Scenario 2: Allocate zero bytes

A caller requests allocation with size `0`.

Expected behavior:
- The module does not depend on implementation-defined or platform-specific zero-size allocator behavior.
- The request is normalized to a non-zero allocation attempt.
- On success, the caller receives a non-failure allocation result.

Test coverage:
- Call the function with `0`.
- Verify behavior matches the module contract for successful allocation rather than exposing raw zero-size allocation semantics.

### Scenario 3: Allocation failure path

A caller requests an allocation that cannot be satisfied.

Expected behavior:
- The module does not silently proceed with an unusable result.
- The module invokes the project-compatible failure behavior for out-of-memory conditions.

Test coverage:
- Use a controlled test seam or failure injection to simulate allocator failure.
- Verify the failure path is taken consistently for both normalized zero-size requests and ordinary positive-size requests.

### Scenario 4: Wrapper use inside larger code

A higher-level module uses this function instead of calling the allocator directly.

Expected behavior:
- The higher-level module receives consistent allocation behavior regardless of whether the original request size is zero or non-zero.
- The wrapper remains a drop-in functional replacement for the C module’s role.

Test coverage:
- Integration test a caller that delegates allocation through this module.
- Verify no special-case caller logic is required for zero-size requests.

## Requirements

### Functional Requirements

#### FR-1: Allocation entry point
The Rust module shall provide the functional equivalent of the C module’s replacement allocation routine represented by `rpl_malloc`.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

#### FR-2: Size-based allocation request
The allocation routine shall accept a caller-provided size value and use it as the basis for the allocation request.

Traceability:
- `rpl_malloc(size_t n)`

#### FR-3: Zero-size normalization
When the requested size is zero, the allocation routine shall convert that request into a non-zero allocation request before attempting allocation.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

#### FR-4: Successful allocation result
When allocation succeeds, the routine shall return a success result representing allocated storage for the request actually issued by the module.

Traceability:
- `rpl_malloc`

#### FR-5: Allocation failure handling
When the underlying allocation attempt fails, the routine shall follow the module’s defined failure behavior rather than providing ordinary successful completion.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

#### FR-6: No additional memory-management operations
The Rust rewrite shall not introduce additional public functionality such as freeing, reallocation, allocator selection, or initialization helpers as part of this module specification.

Traceability:
- Absence of such functions in analyzed module input
- `gnu/malloc.c`
- `rpl_malloc`

### Key Entities

#### Allocation size input
- Represents the requested number of bytes to allocate.
- Provided by the caller to the allocation routine.
- Drives either direct allocation or zero-size normalization.

Traceability:
- `rpl_malloc(size_t n)`

#### Allocated storage result
- Represents the outcome of a successful allocation.
- Returned to the caller by the allocation routine.
- Corresponds to heap storage obtained through the module’s wrapper behavior.

Traceability:
- `void *` return of `rpl_malloc`

#### Failure outcome
- Represents the module’s out-of-memory handling path when allocation cannot be completed.
- Is part of the routine’s observable behavior even if not represented as a normal return value.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

## Success Criteria

### SC-1: Positive-size allocation behavior
For positive allocation sizes exercised by tests, the Rust module returns a successful allocation result when the allocator succeeds.

Traceability:
- `rpl_malloc`

### SC-2: Zero-size request normalization
A test calling the Rust equivalent with size `0` demonstrates that the module does not expose raw zero-size allocation behavior and instead performs a non-zero allocation attempt.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

### SC-3: Failure-path preservation
Under controlled allocation failure, the Rust module follows the specified failure behavior consistently instead of reporting ordinary success.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

### SC-4: Interface scope preservation
The Rust rewrite exposes only the functionality evidenced by the analyzed C module: a replacement allocation routine with the same functional role and no added memory-management surface.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

### SC-5: Caller compatibility at module role level
Integration tests show that code using this module as an allocation wrapper can rely on consistent behavior for both zero and non-zero size requests without adding caller-side zero-size workarounds.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`