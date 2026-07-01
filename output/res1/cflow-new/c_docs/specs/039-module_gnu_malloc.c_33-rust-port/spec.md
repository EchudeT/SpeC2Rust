# spec.md

## Title
Rust Functional Specification for `module_gnu_malloc.c_33`

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_malloc.c_33`
- Category: `module_cluster`
- Source file: `gnu/malloc.c`
- Primary source function: `rpl_malloc`
- Rust branch: `039-module_gnu_malloc.c_33-rust-port`
- Generation date: `2026-06-11`

## Overview
This module provides a replacement memory-allocation entry point for callers that need `malloc`-compatible behavior with an explicit guarantee for zero-size requests.

The source evidence shows one public functional boundary: `rpl_malloc(size_t n) -> void *`. Its purpose is to allocate dynamic memory for `n` bytes and return a pointer suitable for use by the caller. The module must preserve the replacement behavior implied by the function name and source role: zero-length allocation requests must not be passed through as a raw zero-size allocation outcome without adjustment.

The Rust rewrite must implement the same functional behavior at the module boundary: accept a requested allocation size, perform allocation with special handling for a request size of zero, and return either a usable allocation result or an allocation-failure result consistent with the source behavior.

## Scope
In scope:
- Replacement allocation behavior for a requested byte count.
- Special-case handling of zero-size allocation requests.
- Returning an allocation result to the caller.

Out of scope:
- General allocator design beyond this function’s behavior.
- Reallocation, deallocation, alignment-specialized APIs, or allocator statistics.
- Additional public APIs not evidenced by the source module.

## Feature Specification

### Feature: Replacement allocation with zero-size normalization
The module supplies a replacement allocation routine that behaves like a `malloc`-style allocator for requested byte counts, while normalizing a zero-byte request into a non-zero allocation attempt.

#### Functional intent
A caller requests dynamic storage for `n` bytes. If `n` is non-zero, the module allocates that amount. If `n` is zero, the module must instead behave as though a minimum non-zero size were requested, so that the replacement function does not rely on implementation-defined or undesirable zero-size allocation behavior.

#### Rust rewrite expectations
The Rust version must:
- Expose equivalent module functionality for allocating a requested number of bytes.
- Treat a zero-size request as a request for one byte before attempting allocation.
- Return a successful allocation result when allocation succeeds.
- Return a failure result when allocation cannot be obtained.

The rewrite must not add unrelated allocation features or widen the public contract beyond this observed behavior.

## User Scenarios & Testing

### Scenario 1: Allocate a positive number of bytes
A caller requests a positive byte count such as 16. The module attempts to allocate that exact amount and returns a success result when memory is available.

#### Test expectations
- Input: positive size `n > 0`
- Expected behavior: allocation is attempted for `n` bytes
- Expected result: non-failure allocation result on success

### Scenario 2: Allocate zero bytes
A caller requests `0` bytes. The module must not preserve the request as zero for allocation behavior; it must normalize the request to a one-byte allocation attempt.

#### Test expectations
- Input: size `0`
- Expected behavior: allocation is attempted as though the size were `1`
- Expected result: behavior is consistent with a non-zero allocation request, subject to allocator success or failure

### Scenario 3: Allocation failure propagation
A caller requests memory but the allocation cannot be satisfied.

#### Test expectations
- Input: any size after normalization
- Expected behavior: the module reports allocation failure through its return value
- Expected result: failure is observable to the caller and is not converted into a fabricated successful allocation

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide replacement allocation functionality corresponding to source function `rpl_malloc` in `gnu/malloc.c`.
- **FR-2**: The module shall accept a requested allocation size expressed as a byte count.
- **FR-3**: When the requested size is greater than zero, the module shall attempt allocation for that same size.
- **FR-4**: When the requested size is zero, the module shall normalize the request to a one-byte allocation attempt.
- **FR-5**: The module shall return an allocation result representing success when memory is obtained.
- **FR-6**: The module shall return an allocation-failure result when memory cannot be obtained.

### Key Entities
This module has no module-specific persistent data structures evidenced in the source input.

Key functional entities:
- **Allocation request size**: input byte count supplied by the caller.
- **Allocated memory result**: returned allocation outcome, representing either a usable allocated region or allocation failure.

Relationship:
- The allocation request size determines the byte count used for the allocation attempt, with a zero request first transformed to one byte.

## Success Criteria
- **SC-1**: A Rust test covering a positive input size verifies that the module attempts allocation for the requested size and returns success when allocation succeeds. Traceable to `rpl_malloc`.
- **SC-2**: A Rust test covering input size `0` verifies that the module applies zero-size normalization to `1` before allocation behavior is determined. Traceable to `rpl_malloc`.
- **SC-3**: A Rust test covering allocation failure verifies that the module returns a failure result rather than reporting success. Traceable to `rpl_malloc`.
- **SC-4**: The Rust module exposes no additional required public functionality beyond the replacement allocation behavior evidenced by `gnu/malloc.c`. Traceable to the module file set and sole listed function.

## Traceability
- `gnu/malloc.c`
  - `rpl_malloc(size_t n) -> void *`: entire documented functional boundary for this module