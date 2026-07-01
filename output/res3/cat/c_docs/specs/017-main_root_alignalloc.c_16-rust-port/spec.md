# spec.md

## Title

Functional Specification: `main_root_alignalloc.c_16` Rust Port

## Metadata

- Project: `cat`
- Module: `main_root_alignalloc.c_16`
- Category: `main_cluster`
- Source file: `alignalloc.c`
- Rust branch: `017-main_root_alignalloc.c_16-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides aligned dynamic memory allocation paired with a matching deallocation operation.

Its functional role is:

- allocate a memory block whose returned pointer satisfies a requested alignment,
- preserve enough allocation metadata to later release that block correctly,
- free memory previously returned by the module.

The Rust rewrite must preserve the observable behavior of this module as a small allocation utility. The specification covers only the functionality evidenced by `alignalloc.c`.

## Scope

### In scope

- Aligned allocation of dynamically allocated memory.
- Correct freeing of memory obtained from this module.
- Internal handling needed to recover the original allocated address when freeing an aligned pointer.
- Alignment adjustment behavior needed to return an aligned pointer within a larger allocated region.

### Out of scope

- Any API beyond aligned allocation and matching free.
- Guarantees not evidenced in the source analysis, including thread-safety, zero-initialization, reallocation, custom allocators, serialization, recovery, or benchmarking behavior.
- Ownership models or public abstractions beyond what is needed to provide equivalent module functionality.

## Functional Surface

The module behavior is evidenced by these operations:

- `alignalloc(alignment, size) -> pointer or null`
- `alignfree(ptr)`

Internal helper behavior exists to support alignment adjustment and recovery of the original allocated pointer, but these helpers are not part of the required public surface unless needed internally by the Rust rewrite.

## Feature Specification

### Feature: aligned allocation

The module must allocate dynamic memory for a requested payload size and return a pointer aligned to the requested alignment boundary.

Behavioral expectations evidenced by the source module:

- The returned pointer, when non-null, is suitable as the aligned result of the request.
- The returned pointer may differ from the original allocator-returned base address.
- The module must retain enough hidden metadata so the original allocation can later be released.

### Feature: matching deallocation

The module must provide a deallocation operation that accepts a pointer previously returned by the aligned allocation operation and releases the underlying allocation correctly.

Behavioral expectations evidenced by the source module:

- Deallocation uses stored hidden metadata to locate the actual malloc-returned pointer.
- The aligned pointer itself is not assumed to be directly valid for the underlying free operation.
- The module must support freeing the exact values returned by its allocation routine.

### Feature: alignment adjustment support

The module must perform alignment adjustment on an allocated region so that the exposed pointer satisfies the requested boundary.

Behavioral expectations evidenced by the source module:

- Alignment is applied by moving within an allocated region rather than requiring the allocator to natively return the requested alignment.
- Metadata storage must coexist with the aligned result without preventing later recovery of the original allocation address.

## User Scenarios & Testing

### Scenario 1: request aligned memory for later direct use

A caller needs a dynamically allocated memory region whose starting address satisfies a specific alignment value. The caller requests an aligned block and receives either:

- a non-null pointer that satisfies the requested alignment and can be treated as the usable block base, or
- a null result indicating allocation failure.

The Rust version must support this scenario.

#### Test expectations

- For successful allocation, the returned address modulo the requested alignment is zero.
- The usable pointer is non-null when allocation succeeds.
- For allocation failure, the result is null or the Rust-equivalent failure outcome chosen for the port, provided the externally observable allocation-failure behavior remains equivalent for callers of this module.

### Scenario 2: free previously aligned memory

A caller has retained a pointer returned by aligned allocation and later passes it back for deallocation.

The Rust version must support this scenario.

#### Test expectations

- Freeing a pointer previously returned by aligned allocation completes using the matching deallocation path.
- The deallocation operation releases the underlying allocation rather than attempting to free only the adjusted aligned address.

### Scenario 3: alignment is achieved by internal pointer adjustment

A caller requests aligned memory even when the underlying allocator may not directly return the requested alignment. The module internally over-allocates and adjusts the returned address to satisfy alignment.

The Rust version must support this scenario.

#### Test expectations

- The returned pointer is aligned even when internal metadata must be stored alongside the allocation.
- Internal recovery of the original allocation pointer remains possible after alignment adjustment.

### Scenario 4: multiple independent allocations

A caller obtains multiple aligned allocations, potentially with different sizes and alignments, and frees each one independently.

The Rust version must support this scenario.

#### Test expectations

- Each successful allocation returns a pointer aligned according to its own request.
- Freeing one allocation does not affect the ability to free another allocation returned by the module.
- Original allocation recovery is correct for each independently allocated block.

## Requirements

### Functional Requirements

#### FR-1: Provide aligned allocation
The module shall provide an allocation operation that accepts an alignment value and a size value and attempts to allocate dynamic memory for that request.

Traceability: `alignalloc.c`, `alignalloc`

#### FR-2: Return an aligned usable pointer
When allocation succeeds, the module shall return a pointer whose address satisfies the requested alignment boundary.

Traceability: `alignalloc.c`, `alignalloc`, `align_down`

#### FR-3: Preserve deallocation metadata
For each successful allocation, the module shall preserve sufficient hidden information to recover the original allocator-returned address associated with the exposed aligned pointer.

Traceability: `alignalloc.c`, `alignalloc`, `address_of_pointer_to_malloced`

#### FR-4: Provide matching deallocation
The module shall provide a deallocation operation that accepts a pointer previously returned by the module’s allocation operation and releases the underlying dynamic allocation.

Traceability: `alignalloc.c`, `alignfree`

#### FR-5: Recover original allocated address during free
The deallocation operation shall determine the original allocator-returned pointer from metadata associated with the aligned pointer before releasing memory.

Traceability: `alignalloc.c`, `alignfree`, `address_of_pointer_to_malloced`

#### FR-6: Support alignment adjustment within an allocated region
The module shall support producing the aligned result by adjusting a pointer within a dynamically allocated region rather than requiring the underlying allocator to directly provide the final aligned address.

Traceability: `alignalloc.c`, `alignalloc`, `align_down`

#### FR-7: Pair allocation and deallocation consistently
Any pointer value returned as a successful result of the module’s allocation operation shall be acceptable input to the module’s deallocation operation.

Traceability: `alignalloc.c`, `alignalloc`, `alignfree`

### Key Entities

#### Entity: aligned user pointer
The aligned user pointer is the externally returned pointer representing the usable start of the allocated block.

Relationships:

- produced by the allocation operation,
- derived from an underlying allocator-returned region through alignment adjustment,
- consumed later by the deallocation operation.

Traceability: `alignalloc.c`, `alignalloc`, `alignfree`

#### Entity: original allocated pointer
The original allocated pointer is the address returned by the underlying dynamic allocator before alignment adjustment.

Relationships:

- stored indirectly as hidden metadata associated with the aligned user pointer,
- recovered during deallocation,
- is the actual pointer that must be released.

Traceability: `alignalloc.c`, `alignalloc`, `alignfree`, `address_of_pointer_to_malloced`

#### Entity: hidden pointer metadata location
The hidden pointer metadata location is the internal storage area associated with the aligned pointer that contains or identifies the original allocated pointer.

Relationships:

- written during allocation,
- located relative to the aligned pointer,
- read during deallocation to recover the original allocated pointer.

Traceability: `alignalloc.c`, `address_of_pointer_to_malloced`, `alignalloc`, `alignfree`

#### Entity: alignment value
The alignment value is the requested boundary used to determine the returned aligned address.

Relationships:

- provided as input to allocation,
- used to compute the aligned result,
- defines the correctness condition for the returned pointer.

Traceability: `alignalloc.c`, `alignalloc`, `align_down`

## Success Criteria

### SC-1: Alignment correctness
For successful allocations, test cases shall verify that each returned pointer address is evenly divisible by the requested alignment value.

Traceability: `alignalloc.c`, `alignalloc`, `align_down`

### SC-2: Allocation/free pairing correctness
For every pointer returned by a successful allocation call in test coverage, passing that same pointer to the module’s deallocation operation shall complete through the matching release path without requiring the caller to know the original allocator-returned address.

Traceability: `alignalloc.c`, `alignalloc`, `alignfree`, `address_of_pointer_to_malloced`

### SC-3: Independent allocation correctness
When multiple allocations are made and later freed independently, each pointer shall preserve its own alignment and deallocation pairing behavior.

Traceability: `alignalloc.c`, `alignalloc`, `alignfree`

### SC-4: Metadata recovery correctness
Tests shall demonstrate that deallocation relies on recoverable internal metadata associated with the aligned pointer rather than assuming the aligned pointer itself is the original allocation base.

Traceability: `alignalloc.c`, `address_of_pointer_to_malloced`, `alignfree`

### SC-5: Functional parity with source module scope
The Rust port shall implement the aligned allocation and matching free behavior evidenced by `alignalloc.c` and shall not require callers to use capabilities beyond that source module’s functional scope.

Traceability: `alignalloc.c`, `alignalloc`, `alignfree`

## Notes for the Rust Port

- The Rust rewrite may use Rust-appropriate internal representations, but it must preserve the same functional boundaries and observable module behavior.
- Internal helper routines evidenced in the C module may remain internal in Rust as long as the required behavior is preserved.
- This specification does not require exposing additional public types or APIs beyond what is necessary to represent the source module’s behavior.