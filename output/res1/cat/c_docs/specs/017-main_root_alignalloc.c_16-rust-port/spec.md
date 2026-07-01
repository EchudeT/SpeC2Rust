# spec.md

## Title

Functional Specification: `main_root_alignalloc.c_16` Rust Port

## Metadata

- Project: `cat`
- Module: `main_root_alignalloc.c_16`
- Category: `main_cluster`
- Source file: `alignalloc.c`
- Rust branch: `017-main_root_alignalloc.c_16-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides aligned dynamic memory allocation paired with a matching deallocation operation.

Its functional role is:

- allocate a memory block of a requested size,
- ensure the returned pointer satisfies a requested alignment,
- retain enough internal bookkeeping to later free the allocation correctly using the module’s paired free function.

The Rust rewrite must preserve this behavior and the allocation/free pairing semantics evidenced by the source module.

## Scope

In scope for the Rust port:

- aligned allocation of a caller-requested size,
- returning a pointer aligned to a caller-requested boundary,
- paired deallocation of pointers returned by this module,
- internal tracking sufficient to free the original allocation underlying an aligned returned pointer.

Out of scope:

- any API beyond the allocate/free behavior evidenced by `alignalloc` and `alignfree`,
- guarantees not shown by the source, such as thread-safety properties, zero-initialization, reallocation, custom allocators, error reporting beyond allocation failure behavior, or ownership models beyond the module’s paired use.

## Feature Specification

### Feature: Aligned heap allocation

The module must provide functionality equivalent to an aligned allocator that accepts:

- an alignment value,
- a requested allocation size,

and returns either:

- a pointer to memory whose address satisfies the requested alignment, or
- a null/failure result if allocation cannot be completed.

The returned pointer is not required to be the original base pointer obtained from the underlying allocator. Instead, the module may return an adjusted aligned pointer within a larger reserved allocation, provided that the allocation can later be released correctly by the paired free operation.

Traceability:

- `alignalloc` in `alignalloc.c:74-102`
- alignment helper behavior evidenced by `align_down` in `alignalloc.c:44-49`

### Feature: Paired deallocation of aligned allocations

The module must provide a deallocation operation that accepts a pointer previously returned by the module’s aligned allocator and releases the correct underlying allocation.

This behavior must work even when the returned aligned pointer differs from the allocator’s original base address.

Traceability:

- `alignfree` in `alignalloc.c:106-116`
- bookkeeping access evidenced by `address_of_pointer_to_malloced` in `alignalloc.c:55-66`

### Feature: Internal recovery of original allocated pointer

The module must preserve, alongside each returned aligned pointer, enough metadata to recover the original heap pointer that must be passed to the underlying deallocator.

This metadata is an internal mechanism of the module, but the Rust port must preserve the functional outcome: freeing a pointer returned by aligned allocation must free the correct original allocation.

Traceability:

- `address_of_pointer_to_malloced` in `alignalloc.c:55-66`
- `alignalloc` in `alignalloc.c:74-102`
- `alignfree` in `alignalloc.c:106-116`

## User Scenarios & Testing

### Scenario 1: Request aligned storage for later use

A caller needs a heap allocation of a specified size and requires the returned address to be aligned to a given boundary. The caller invokes the module’s aligned allocation function and receives a pointer suitable for use under that alignment requirement.

The Rust version must support tests that verify:

- successful allocation returns a non-null pointer when the underlying allocation succeeds,
- the returned pointer address is divisible by the requested alignment,
- the allocation can subsequently be released by the module’s paired free function.

Traceability:

- `alignalloc`
- `alignfree`

### Scenario 2: Free an aligned pointer that is not the original base allocation address

A caller receives an aligned pointer from the module and later passes that same pointer to the module’s free function. The free function must release the correct underlying heap allocation even though the aligned pointer may be an adjusted address.

The Rust version must support tests that verify:

- allocation followed by paired free completes without requiring the caller to know any original base address,
- the free operation uses module-retained allocation identity rather than assuming the aligned pointer itself is the original allocation base.

Traceability:

- `address_of_pointer_to_malloced`
- `alignalloc`
- `alignfree`

### Scenario 3: Allocation failure propagation

If the underlying heap allocation cannot be obtained, the caller must receive a failure result rather than an invalid aligned pointer.

The Rust version must support tests that verify:

- failure to allocate is surfaced as a null/failure return,
- no successful-looking pointer is produced on allocation failure.

Traceability:

- `alignalloc`

### Scenario 4: Zero-size request behavior remains paired and safe to release

A caller may request an allocation with size zero if the underlying allocator permits it. If the module returns a non-null pointer for such a request, that pointer must still be valid for paired release through the module’s free function.

The Rust version must support tests that verify:

- zero-size requests follow the same allocate/free pairing rules as nonzero requests,
- any non-null result returned for a zero-size request can be passed to the module’s free function.

Traceability:

- `alignalloc`
- `alignfree`

## Requirements

### Functional Requirements

#### FR-1: Provide aligned allocation
The module shall provide an operation that accepts an alignment value and a requested size and attempts to allocate heap memory satisfying that alignment.

Traceability:

- `alignalloc` (`alignalloc.c:74-102`)

#### FR-2: Return alignment-conforming pointers on success
When allocation succeeds, the module shall return a pointer whose address conforms to the requested alignment.

Traceability:

- `alignalloc` (`alignalloc.c:74-102`)
- `align_down` (`alignalloc.c:44-49`)

#### FR-3: Report allocation failure as failure/null
When the underlying memory reservation cannot be completed, the allocation operation shall return failure rather than a usable pointer.

Traceability:

- `alignalloc` (`alignalloc.c:74-102`)

#### FR-4: Preserve original allocation identity for later free
For each successful aligned allocation, the module shall preserve enough internal information to recover the original allocated pointer associated with the returned aligned pointer.

Traceability:

- `address_of_pointer_to_malloced` (`alignalloc.c:55-66`)
- `alignalloc` (`alignalloc.c:74-102`)

#### FR-5: Provide paired deallocation
The module shall provide a deallocation operation that accepts a pointer previously returned by the module’s allocation operation and releases the correct underlying heap allocation.

Traceability:

- `alignfree` (`alignalloc.c:106-116`)

#### FR-6: Deallocation shall work for adjusted aligned pointers
The deallocation operation shall not require the caller to supply the original base pointer from the underlying allocator; it shall recover and free that base pointer from module-managed metadata associated with the aligned returned pointer.

Traceability:

- `address_of_pointer_to_malloced` (`alignalloc.c:55-66`)
- `alignfree` (`alignalloc.c:106-116`)

### Key Entities

#### Entity: Requested alignment
A caller-provided integer alignment value used to constrain the address of the returned pointer.

Relationship to behavior:

- drives the alignment property required of successful allocation results.

Traceability:

- `alignalloc`
- `align_down`

#### Entity: Requested allocation size
A caller-provided integer size indicating how many bytes of storage are requested.

Relationship to behavior:

- determines the amount of memory the allocation operation must reserve for caller use, subject to additional hidden space needed for alignment bookkeeping.

Traceability:

- `alignalloc`

#### Entity: Returned aligned pointer
The pointer value returned to the caller on successful allocation.

Relationship to behavior:

- must satisfy the requested alignment,
- is the handle later passed back to the paired deallocation operation,
- may differ from the underlying allocator’s original base pointer.

Traceability:

- `alignalloc`
- `alignfree`

#### Entity: Original allocated pointer
The actual pointer obtained from the underlying heap allocation mechanism.

Relationship to behavior:

- must be recoverable from the returned aligned pointer,
- is the pointer that must be released during deallocation.

Traceability:

- `address_of_pointer_to_malloced`
- `alignalloc`
- `alignfree`

#### Entity: Internal pointer bookkeeping location
An internal storage location associated with the returned aligned pointer that holds or provides access to the original allocated pointer.

Relationship to behavior:

- links the aligned pointer visible to the caller with the original pointer required for correct deallocation.

Traceability:

- `address_of_pointer_to_malloced`

## Success Criteria

### SC-1: Alignment correctness
For successful allocations, tests show that each returned pointer address satisfies the requested alignment constraint for the exercised alignment values.

Traceability:

- `alignalloc`
- `align_down`

### SC-2: Paired free correctness
For pointers returned by the allocation operation, tests show that passing the returned pointer to the paired free operation releases the associated allocation without requiring any caller-visible original base pointer.

Traceability:

- `alignalloc`
- `alignfree`
- `address_of_pointer_to_malloced`

### SC-3: Failure behavior correctness
Tests show that when allocation cannot be obtained, the allocation operation returns failure/null and does not return a pointer presented as successful.

Traceability:

- `alignalloc`

### SC-4: Metadata-backed free path preserved
Tests or code inspection confirm that the Rust port preserves the functional ability to recover the original allocation associated with an adjusted aligned pointer and uses that recovered allocation for deallocation.

Traceability:

- `address_of_pointer_to_malloced`
- `alignalloc`
- `alignfree`

### SC-5: Allocate/free pairing works across representative sizes
Tests show that representative allocation sizes, including at least one zero-size request and multiple nonzero sizes, can be handled under the module’s allocate/free pairing semantics.

Traceability:

- `alignalloc`
- `alignfree`

## Acceptance Notes

- The Rust rewrite may change internal implementation details, but it must preserve the externally observable behavior defined above.
- The specification is limited to behavior evidenced by `alignalloc.c` and does not require additional public APIs or stronger guarantees than the source module shows.