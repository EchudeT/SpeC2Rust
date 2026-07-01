# spec.md

## Title

Functional Specification: `module_gnu_malloca.c_34` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_malloca.c_34`
- **Category**: `module_cluster`
- **Source file**: `gnu/malloca.c`
- **Rust branch**: `040-module_gnu_malloca.c_34-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides paired temporary-memory allocation and release behavior through two functions:

- `mmalloca(size_t n)`
- `freea(void *p)`

The module’s functional role is to obtain a memory region of a requested size and later release that region through the matching module-provided deallocation function. The Rust rewrite must preserve this allocation/deallocation behavior as a module boundary, including the requirement that memory returned by the allocation function is releasable by the paired release function.

This specification covers only the behavior evidenced by `gnu/malloca.c` and does not introduce additional APIs or capabilities.

## Feature Specification

### Feature: Temporary memory acquisition with paired release

The module shall provide functionality to request a memory block for a caller-specified size and return a pointer-like handle to that block.

The Rust version must implement behavior equivalent to:

- accepting a size request,
- attempting to provide a usable memory region for that size,
- returning a null/empty failure result when allocation cannot be provided,
- allowing the returned region to be released later through the module’s matching release operation.

### Feature: Unified release path for module-allocated memory

The module shall provide a release operation that accepts a previously returned allocation result from the module’s allocation function and relinquishes the associated storage.

The Rust version must implement a release path that:

- accepts memory previously obtained from the module allocation function,
- safely completes the module’s ownership of that allocation,
- does not require the caller to know which internal allocation strategy was used.

### Feature boundaries

The Rust port must not require callers to interact with internal storage strategy details. The externally relevant contract is limited to allocation by `mmalloca` and release by `freea`.

No additional public behaviors are required beyond those evidenced by the two source functions.

## User Scenarios & Testing

### Scenario 1: Allocate temporary storage for a nonzero size and release it

A caller needs a temporary memory area of size `n > 0`.

**Expected behavior**
- The allocation operation accepts the requested size.
- If successful, it returns a non-null usable result.
- The caller may later pass that returned result to the release operation.
- The release operation completes without requiring additional metadata from the caller.

**Testing focus**
- Verify successful allocation for representative nonzero sizes.
- Verify that each successful returned result can be passed to the paired release function.

### Scenario 2: Request zero-sized allocation and release the result if one is returned

A caller requests temporary storage with size `0`.

**Expected behavior**
- The module handles the request without undefined module-level behavior.
- The result is either a valid releaseable allocation result or a failure result, consistent with the module’s allocation contract.
- If a non-failure result is returned, it is accepted by the release operation.

**Testing focus**
- Verify that zero-sized requests are handled deterministically by the Rust port.
- Verify that any successful result from a zero-sized request is releasable through the paired release function.

### Scenario 3: Allocation failure path

A caller requests memory and the module cannot provide it.

**Expected behavior**
- The allocation operation indicates failure via its failure result.
- The caller is able to distinguish failure from success.

**Testing focus**
- Verify that failure is surfaced as an allocation failure result rather than as silent success.
- Verify that no success-only assumptions are required by the API contract.

### Scenario 4: Multiple independent allocations

A caller performs multiple allocations and later releases each returned region.

**Expected behavior**
- Each successful allocation result is independently releasable.
- Releasing one allocation does not invalidate the release contract for another allocation returned by the module.

**Testing focus**
- Verify that multiple successful allocations can be released individually.
- Verify correct behavior when allocations differ in requested size.

## Requirements

### Functional Requirements

#### FR-1: Allocation request handling
The module shall provide an allocation operation corresponding to `mmalloca(size_t n)` that accepts a requested size and attempts to obtain memory for that request.

**Traceability**: `gnu/malloca.c`, function `mmalloca`

#### FR-2: Success/failure allocation result
The allocation operation shall communicate whether allocation succeeded or failed through its return value, with failure represented as an empty/null-equivalent result.

**Traceability**: `gnu/malloca.c`, function `mmalloca`

#### FR-3: Release of module-allocated memory
The module shall provide a release operation corresponding to `freea(void *p)` that accepts a value previously returned by the allocation operation and releases the associated storage.

**Traceability**: `gnu/malloca.c`, function `freea`

#### FR-4: Paired contract
Any successful allocation result returned by the module allocation operation shall be valid input to the module release operation.

**Traceability**: `gnu/malloca.c`, functions `mmalloca`, `freea`

#### FR-5: Strategy transparency to callers
The release contract shall not depend on the caller knowing how the allocation was internally obtained; the caller-facing behavior is only the paired use of allocate then release.

**Traceability**: `gnu/malloca.c`, functions `mmalloca`, `freea`

#### FR-6: Independent handling of separate allocations
Separate successful allocation results produced by distinct allocation calls shall be independently releasable.

**Traceability**: `gnu/malloca.c`, functions `mmalloca`, `freea`

### Key Entities

#### Entity: Allocation result
The core entity is the memory reference returned by the allocation function.

**Properties**
- Represents either a successful memory allocation or a failure result.
- Is produced by the allocation operation.
- May be consumed by the release operation if allocation succeeded.

**Relationships**
- Created by `mmalloca`.
- Released by `freea`.

#### Entity: Requested size
The allocation request size is the input value that determines how much memory the caller asks the module to obtain.

**Properties**
- Provided by the caller to the allocation operation.
- May be zero or nonzero.

**Relationships**
- Consumed by `mmalloca`.
- Determines the size associated with the returned allocation result on success.

## Success Criteria

### SC-1: Allocation API parity
The Rust module exposes behaviorally equivalent allocation and paired release operations for the functionality represented by `mmalloca` and `freea`.

**Traceability**: `gnu/malloca.c`, functions `mmalloca`, `freea`

### SC-2: Successful allocations are releasable
In tests covering representative allocation sizes, every successful allocation result returned by the Rust allocation operation can be passed to the Rust release operation without requiring caller knowledge of internal allocation strategy.

**Traceability**: `gnu/malloca.c`, functions `mmalloca`, `freea`

### SC-3: Failure is observable
When allocation cannot be provided, the Rust allocation operation returns a failure result that is distinguishable from success.

**Traceability**: `gnu/malloca.c`, function `mmalloca`

### SC-4: Zero-size handling is defined by the module contract
A zero-size allocation request is handled consistently by the Rust module, and any successful result from that request is accepted by the release operation.

**Traceability**: `gnu/malloca.c`, functions `mmalloca`, `freea`

### SC-5: Multiple allocations remain independently releasable
Tests with multiple successful allocations of differing requested sizes show that each returned allocation result can be released individually.

**Traceability**: `gnu/malloca.c`, functions `mmalloca`, `freea`

## Out of Scope

The Rust port specification does not require or imply:

- any new public API beyond the allocation and release behavior evidenced here,
- exposure of internal allocation strategy,
- thread-safety guarantees,
- serialization,
- persistence,
- recovery behavior,
- FFI requirements,
- performance targets or benchmarks.

## Notes for Port Validation

Validation should be based on observable behavior at the module boundary:

- request allocation by size,
- inspect success or failure,
- release successful results through the paired release operation,
- verify the paired contract across zero-size, nonzero-size, failure, and multiple-allocation scenarios.