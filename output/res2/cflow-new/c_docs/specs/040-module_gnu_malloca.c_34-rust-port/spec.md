# spec.md

## Title

Functional Specification for `module_gnu_malloca.c_34` Rust Port

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_malloca.c_34`
- Category: `module_cluster`
- Source file: `gnu/malloca.c`
- Rust branch: `040-module_gnu_malloca.c_34-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides paired temporary-memory allocation and release behavior through two public functions:

- `mmalloca(size_t n)`
- `freea(void *p)`

Its functional role is to supply callers with a writable memory block of the requested size and a matching release operation that can safely dispose of memory obtained from the allocator. The Rust rewrite must preserve this paired-allocation behavior and the caller-visible distinctions between successful allocation, allocation failure, and release through the companion free function.

## Scope

In scope for the Rust version:

- Allocation of a memory block sized according to a caller-provided byte count.
- Returning a pointer-like handle representing either successful allocation or failure.
- Releasing memory previously returned by the allocation function through the companion deallocation function.
- Correct handling of all memory origins used by the module’s allocation strategy, as observable through the public API contract that `freea` accepts values returned by `mmalloca`.

Out of scope:

- Any API beyond the two functions evidenced in the source module.
- General-purpose allocator replacement.
- New ownership models or public abstractions not required to preserve the module’s behavior.
- Additional diagnostics, recovery interfaces, or concurrency guarantees not evidenced by the module input.

## Feature Specification

### Feature: Temporary memory allocation with paired release

The module must provide a function equivalent to `mmalloca` that accepts a requested allocation size in bytes and attempts to produce usable writable storage for that many bytes.

Behavioral expectations:

- The caller supplies a byte count.
- On success, the function returns a non-null memory reference usable by the caller as an allocated region.
- On failure, the function indicates allocation failure in the same observable manner as the C module’s public behavior.
- The returned memory is eligible for release only through the module’s paired release function.

### Feature: Unified release of allocator-returned memory

The module must provide a function equivalent to `freea` that accepts a value previously returned by `mmalloca` and releases any resources associated with it.

Behavioral expectations:

- The release function must accept the allocation result form produced by the allocator.
- It must correctly release memory allocated through the module, regardless of which internal allocation path produced it.
- It must not require the caller to know the internal origin of the allocation in order to release it.

## User Scenarios & Testing

### Scenario 1: Allocate a temporary work buffer and release it

A caller needs a temporary byte buffer of size `n` for intermediate computation.

Expected behavior:

1. The caller requests `n` bytes through the allocation function.
2. If allocation succeeds, the caller writes to the returned region.
3. The caller passes the same returned value to the release function.
4. The memory is released without the caller needing to distinguish allocation origin.

Test coverage:

- Request a small positive size.
- Verify that a success result provides writable storage.
- Verify that releasing the returned value completes without requiring any extra metadata from the caller.

### Scenario 2: Handle allocation failure

A caller requests memory and must branch correctly if the request cannot be satisfied.

Expected behavior:

1. The caller requests a size that may fail.
2. The allocator signals failure through its return value.
3. The caller can detect failure without invoking undefined module behavior.

Test coverage:

- Exercise a failing allocation path where feasible.
- Verify that failure is observable to the caller and distinct from successful allocation.

### Scenario 3: Release memory from any allocation path used by the module

A caller uses the allocator without knowledge of whether the module satisfied the request from one internal strategy or another.

Expected behavior:

1. The caller obtains memory from the allocator.
2. The caller later releases it using only the paired release function.
3. Release succeeds correctly without requiring the caller to branch on allocation origin.

Test coverage:

- Validate that all successful `mmalloca` returns are accepted by `freea`.
- Validate behavior across differing requested sizes sufficient to cover the allocator’s supported paths, if such path distinction remains present in the Rust port.

## Requirements

### Functional Requirements

- **FR-1**: The Rust module shall provide functionality equivalent to `mmalloca(size_t n)` that accepts a requested allocation size in bytes and attempts to obtain storage of that size.
  **Traceability**: `gnu/malloca.c`, `mmalloca`.

- **FR-2**: On successful allocation, the Rust module shall return a value representing usable allocated storage for the requested byte count.
  **Traceability**: `gnu/malloca.c`, `mmalloca`.

- **FR-3**: On unsuccessful allocation, the Rust module shall return a failure indication observable to the caller through the allocation function’s return result.
  **Traceability**: `gnu/malloca.c`, `mmalloca`.

- **FR-4**: The Rust module shall provide functionality equivalent to `freea(void *p)` for releasing memory returned by the module’s allocation function.
  **Traceability**: `gnu/malloca.c`, `freea`.

- **FR-5**: The Rust module shall allow callers to release allocator-returned memory using the paired release function without requiring caller knowledge of the internal allocation path that produced the memory.
  **Traceability**: `gnu/malloca.c`, `mmalloca`, `freea`.

- **FR-6**: The Rust module shall preserve the pairing contract that memory released by the module’s deallocation function is memory previously returned by the module’s allocation function.
  **Traceability**: `gnu/malloca.c`, `mmalloca`, `freea`.

### Key Entities

- **Allocation request size**
  The byte-count input supplied to the allocator function. It determines the amount of storage requested.
  **Traceability**: `mmalloca(size_t n)`.

- **Allocated memory reference**
  The returned pointer-like value representing either successful allocation of writable storage or allocation failure. This value is the link between allocation and release operations.
  **Traceability**: `mmalloca`, `freea`.

- **Paired release operation**
  The deallocation action applied to an allocation result previously produced by the allocator. It defines the module’s lifecycle boundary for allocated memory.
  **Traceability**: `freea`.

## Success Criteria

- **SC-1**: For successful allocation requests, the Rust port returns a usable memory reference corresponding to the requested size and suitable for caller writes within that size.
  **Traceability**: `mmalloca`.

- **SC-2**: For allocation failures, the Rust port exposes failure through the allocator’s return result in a way test code can distinguish from success.
  **Traceability**: `mmalloca`.

- **SC-3**: Every value successfully returned by the Rust equivalent of `mmalloca` is accepted by the Rust equivalent of `freea` and released through that paired API without requiring caller-side origin tracking.
  **Traceability**: `mmalloca`, `freea`.

- **SC-4**: Module-level tests cover at least one successful allocate-and-release flow and at least one allocation-failure observation flow.
  **Traceability**: `mmalloca`, `freea`.

- **SC-5**: The Rust port does not require any public API beyond the allocator and paired release behavior evidenced by the source module.
  **Traceability**: `gnu/malloca.c`, `mmalloca`, `freea`.

## Non-Goals

The Rust rewrite is not required to provide:

- Additional public memory-management APIs.
- Guarantees for freeing memory not returned by this module.
- Persistence, serialization, or cross-thread usage contracts.
- Performance targets beyond preserving functional behavior.
- Caller-visible exposure of internal allocation strategy.