# spec.md

## Title
Rust Functional Specification for `module_gnu_malloc.c_33`

## Document Metadata
- Project: `cflow-new`
- Module: `module_gnu_malloc.c_33`
- Category: `module_cluster`
- Source file: `gnu/malloc.c`
- Primary source function: `rpl_malloc`
- Target Rust branch: `039-module_gnu_malloc.c_33-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides a replacement memory-allocation entry point for callers that need allocation behavior normalized by the project rather than relying directly on the platform `malloc` call.

The Rust rewrite must preserve the observable behavior of the source module’s allocation wrapper:
- accept a requested allocation size,
- return a pointer-like allocation result for successful requests,
- handle zero-size requests according to the module’s replacement semantics rather than passing through implementation-defined behavior unchanged.

No other public functionality is evidenced in the source input for this module.

## Feature Specification

### Summary
The module defines one functional capability: a replacement allocator routine that requests heap storage for a caller-supplied size and returns the result in a form equivalent to C allocation behavior.

### Required Rust Behavior
The Rust version must implement behavior equivalent to `rpl_malloc(size_t n)`:

1. It accepts a requested allocation size.
2. For non-zero sizes, it performs an allocation attempt for that size.
3. For a zero-size request, it must apply the module’s replacement rule so that the request is converted into a real allocation attempt rather than relying on raw zero-size allocator behavior.
4. It returns success or failure in a way that preserves the source module’s caller-visible semantics.

### Functional Boundary
This module’s responsibility is limited to allocation request normalization and delegation to heap allocation. It does not, based on the provided source analysis, define:
- deallocation behavior,
- reallocation behavior,
- ownership abstractions beyond the allocation result,
- custom allocation accounting,
- initialization of allocated memory.

## User Scenarios & Testing

### Scenario 1: Caller requests a positive number of bytes
A caller needs dynamic storage for a non-zero size and invokes the module allocation routine.

Expected behavior:
- the module attempts to allocate that exact number of bytes;
- if allocation succeeds, the caller receives a usable non-failure result;
- if allocation fails, the caller receives a failure result matching the module’s allocation contract.

Test coverage:
- request size `1`;
- request a typical small size;
- request a larger size that is still expected to succeed in the test environment.

### Scenario 2: Caller requests zero bytes
A caller invokes the module with `0` as the requested size.

Expected behavior:
- the module does not leave the outcome to raw allocator-specific zero-size semantics alone;
- it converts the request into the module-defined normalized allocation attempt;
- the result follows the same success/failure contract as other allocation requests.

Test coverage:
- verify the zero-size path is handled distinctly from a direct unchecked pass-through;
- verify the call completes without undefined module behavior.

### Scenario 3: Caller handles allocation failure
A caller requests memory and the underlying allocation cannot be satisfied.

Expected behavior:
- the module reports failure through its return value;
- it does not fabricate a successful allocation result.

Test coverage:
- induce or simulate allocator failure;
- verify failure is observable to the caller through the return contract.

## Requirements

### Functional Requirements

#### FR-1 Allocation entry point
The Rust module shall provide the replacement allocation behavior corresponding to source function `rpl_malloc`.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

#### FR-2 Positive-size allocation
When the requested size is greater than zero, the module shall attempt to allocate that many bytes.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

#### FR-3 Zero-size normalization
When the requested size is zero, the module shall normalize the request into a non-zero allocation attempt rather than preserving allocator-dependent zero-size behavior unchanged.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

#### FR-4 Return-value-based success/failure reporting
The module shall report allocation success or failure through the function result, preserving the caller-visible contract of the C routine.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

### Key Entities

#### Entity: Allocation request size
- Represents the number of bytes requested by the caller.
- Relationship: this input determines whether the module follows the normal allocation path or the zero-size normalization path.

Traceability:
- `rpl_malloc(size_t n)`

#### Entity: Allocation result
- Represents either a successful allocated memory reference equivalent to a C pointer result or an allocation failure result.
- Relationship: produced from the allocation attempt driven by the request size.

Traceability:
- `void *` result of `rpl_malloc`

## Success Criteria

### SC-1 Behavioral equivalence for non-zero requests
For representative positive sizes, the Rust implementation returns a success result when allocation succeeds and a failure result when allocation fails, matching the source routine’s observable contract.

Traceability:
- `rpl_malloc`

### SC-2 Behavioral equivalence for zero-size requests
For input size `0`, the Rust implementation follows the module’s normalization behavior and does not simply expose unchecked allocator-specific zero-length semantics.

Traceability:
- `rpl_malloc`

### SC-3 No unsupported public behavior added
The Rust rewrite exposes only the allocation functionality evidenced by this module and does not require callers to use additional public APIs to obtain equivalent behavior.

Traceability:
- `gnu/malloc.c`
- `rpl_malloc`

### SC-4 Scenario-based test pass
Tests covering positive-size allocation, zero-size normalization, and allocation failure observation all pass against the Rust implementation.

Traceability:
- `rpl_malloc`