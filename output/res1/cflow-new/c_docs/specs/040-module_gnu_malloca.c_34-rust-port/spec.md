# spec.md

## Title

Functional Specification for `module_gnu_malloca.c_34` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_malloca.c_34`
- Category: `module_cluster`
- Source file: `gnu/malloca.c`
- Source functions:
  - `mmalloca(size_t n)`
  - `freea(void *p)`
- Target branch: `040-module_gnu_malloca.c_34-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides paired allocation and release behavior for temporary memory buffers. The source module exposes one function to obtain a memory region of a requested size and one function to release a region previously obtained from that allocator interface.

The Rust rewrite must preserve the module’s observable behavior as a two-operation allocation facility:

- allocate memory for a requested byte size through `mmalloca`
- release memory obtained from that facility through `freea`

The specification is limited to behavior evidenced by `gnu/malloca.c` and the exported function pair listed above.

## Feature Specification

### Purpose

The module supplies a small allocation interface intended for callers that need a temporary writable memory buffer and a matching release operation.

### Functional Scope

The Rust version must implement the following functional behavior:

1. Accept a requested allocation size in bytes through the allocation entrypoint corresponding to `mmalloca`.
2. Return a pointer/reference-equivalent handle to a contiguous memory region suitable for caller use when allocation succeeds.
3. Support release of memory previously returned by the allocation entrypoint through the release entrypoint corresponding to `freea`.
4. Correctly handle the module’s own allocation origin tracking so that `freea` can safely perform the appropriate release action for memory returned by `mmalloca`.
5. Preserve the pairing contract: memory obtained from this module is released through this module’s release function, not by unrelated deallocation paths.

### Out of Scope

The Rust port specification does not require any capabilities not evidenced by the source module, including:

- additional public allocation APIs
- resizing/reallocation APIs
- zero-initialization guarantees unless explicitly evidenced by the source behavior
- thread-safety guarantees
- persistence, serialization, or recovery features
- user-visible allocation statistics or diagnostics

## User Scenarios & Testing

### Scenario 1: Allocate a temporary buffer and release it

A caller requests a buffer of `n` bytes, writes data into the returned region, and later releases it through the paired release function.

**Expected behavior**
- Allocation succeeds for supported sizes when resources are available.
- The returned region is usable by the caller as temporary storage.
- Releasing through the paired function completes without requiring the caller to know how the memory was obtained internally.

**Test coverage**
- Request a nonzero size.
- Verify a non-null success result under normal conditions.
- Write within the requested range.
- Release using the module’s release function.

### Scenario 2: Use the module without knowing allocation origin details

A caller uses the allocation function but does not need to distinguish whether the module satisfied the request using one internal strategy or another.

**Expected behavior**
- The allocation interface presents a single allocation contract to callers.
- The release interface accepts the returned value from the allocator and performs the correct cleanup path based on module-managed origin information.

**Test coverage**
- Allocate through the public allocator.
- Release only through the public releaser.
- Confirm no caller-side branching or origin tagging is required.

### Scenario 3: Repeated temporary allocations

A caller performs multiple independent allocate/use/release cycles.

**Expected behavior**
- Each successful allocation can be released independently through the release function.
- The module maintains correct pairing behavior across repeated calls.

**Test coverage**
- Perform repeated cycles with varying sizes.
- Ensure each returned allocation can be released exactly through the module release path.

### Scenario 4: Boundary-oriented size requests

A caller requests edge-case sizes relevant to byte-count allocation behavior.

**Expected behavior**
- The module accepts a `size_t` byte count input.
- For boundary sizes supported by the source behavior, the Rust version preserves equivalent observable results.
- Failure cases, if any arise from inability to obtain storage, are reported through the allocator’s return behavior rather than through a different public API.

**Test coverage**
- Exercise small sizes, including minimal positive requests.
- Exercise larger sizes that still fit the test environment.
- Validate release behavior for every successful allocation.

## Requirements

### Functional Requirements

#### FR-1 Allocation entrypoint
The module shall provide an allocation operation corresponding to `mmalloca(size_t n)` that accepts a requested size in bytes and returns memory for caller use on success.

**Traceability:** `gnu/malloca.c`, `mmalloca`

#### FR-2 Release entrypoint
The module shall provide a release operation corresponding to `freea(void *p)` for memory previously returned by the module allocation operation.

**Traceability:** `gnu/malloca.c`, `freea`

#### FR-3 Paired allocation lifecycle
The module shall support the lifecycle in which memory returned by the allocation operation can later be passed to the release operation for cleanup.

**Traceability:** `gnu/malloca.c`, `mmalloca`, `freea`

#### FR-4 Contiguous caller-usable storage
The allocation operation shall provide access to a contiguous region representing the requested byte count for caller use, subject to successful allocation.

**Traceability:** `gnu/malloca.c`, `mmalloca`

#### FR-5 Module-managed release selection
The release operation shall determine the correct release behavior for memory produced by the allocation operation using module-defined allocation metadata or equivalent internal state, without requiring the caller to supply allocation-origin information.

**Traceability:** `gnu/malloca.c`, `mmalloca`, `freea`

#### FR-6 Failure signaling through allocation result
If the module cannot obtain memory for a request, the allocation operation shall signal failure through its return result in a manner equivalent to the source module’s observable contract.

**Traceability:** `gnu/malloca.c`, `mmalloca`

#### FR-7 No additional caller-visible allocation protocol
The Rust port shall preserve the source module’s simple two-call usage model and shall not require additional caller-visible steps beyond allocate and release for the supported lifecycle.

**Traceability:** `gnu/malloca.c`, `mmalloca`, `freea`

### Key Entities

This module does not define standalone exported data structures in the provided analysis input. Its key entities are functional and relational:

#### Entity 1: Allocated memory region
A contiguous temporary storage region returned by the allocation operation for caller use.

**Relationships**
- Produced by `mmalloca`
- Consumed by caller code
- Later released by `freea`

#### Entity 2: Allocation-origin metadata
Internal allocation-state information associated with a returned region, sufficient for the release operation to choose the proper cleanup behavior.

**Relationships**
- Established during `mmalloca`
- Interpreted during `freea`
- Not supplied by the caller as a separate argument

## Success Criteria

### SC-1 Allocation/release compatibility
For every successful allocation produced by the Rust equivalent of `mmalloca`, passing the returned allocation to the Rust equivalent of `freea` shall complete successfully in module tests.

**Traceability:** `mmalloca`, `freea`

### SC-2 Caller-transparent release behavior
Module tests shall demonstrate that callers can allocate and release memory without knowing or specifying the allocation origin or cleanup strategy.

**Traceability:** `mmalloca`, `freea`

### SC-3 Repeated lifecycle support
Module tests shall pass for multiple independent allocate/use/release cycles across varying request sizes supported by the test environment.

**Traceability:** `mmalloca`, `freea`

### SC-4 Writable storage on successful allocation
For successful allocations, tests shall verify that the returned region can be written within the requested bounds before release.

**Traceability:** `mmalloca`

### SC-5 Failure is observable through allocator result
When allocation cannot be completed in a controlled failure test or equivalent validation path, the Rust port shall expose failure through the allocator result rather than through an unrelated public mechanism.

**Traceability:** `mmalloca`

## Notes for Rust Port Alignment

- The Rust rewrite must preserve the source module’s externally observable functional contract, not the original C implementation technique.
- Internal representation may change, provided the paired allocation/release behavior remains equivalent to the source module.
- Because the analyzed input names only two public functions and no exported structs, the Rust-facing design should remain minimal and centered on those evidenced behaviors.