# spec.md

## Title

Rust Functional Specification for `module_gnu_xmalloc.c_56`

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_xmalloc.c_56`
- Category: `module_cluster`
- Source file: `gnu/xmalloc.c`
- Target branch: `062-module_gnu_xmalloc.c_56-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides fallible-hostile memory allocation helpers for callers that need allocation, reallocation, zero-initialized allocation, array-size-aware allocation, and capacity growth support without propagating allocation failure through ordinary return values.

The Rust rewrite must preserve the observable functional role of this module:

- provide allocation helpers for single-size and element-count-based requests,
- support both `size_t`-like and `idx_t`-like size inputs where present in the source API,
- support reallocation and reallocation-with-growth workflows,
- support zero-initialized allocation variants,
- ensure that successful calls return usable storage of the requested size or updated capacity,
- ensure that requests that cannot be satisfied do not silently return a null success value.

This specification covers only behavior evidenced by `gnu/xmalloc.c`.

## Scope

### In Scope

The Rust version must implement the functionality corresponding to the following source-level operations:

- allocate memory for a byte size,
- allocate memory for an `idx_t` size,
- allocate character storage by byte count,
- reallocate an existing allocation to a requested size,
- allocate or reallocate array storage from element count and element size,
- grow allocations using helper functions that update tracked capacity,
- allocate zero-filled storage.

### Out of Scope

The Rust version is not required by this specification to provide:

- additional public APIs beyond the evidenced module surface,
- thread-safety guarantees,
- serialization support,
- persistence or recovery behavior,
- benchmarking-oriented behaviors,
- custom allocator configuration interfaces,
- FFI promises beyond what is required to replace the module internally.

## Source Functional Surface

The source module exposes allocation helpers corresponding to these functions:

- `xmalloc`
- `ximalloc`
- `xcharalloc`
- `xrealloc`
- `xirealloc`
- `xreallocarray`
- `xireallocarray`
- `xnmalloc`
- `xinmalloc`
- `x2realloc`
- `x2nrealloc`
- `xpalloc`
- `xzalloc`
- `xizalloc`
- `xcalloc`

These functions define the required functional boundaries for the Rust rewrite.

## Feature Specification

### Feature 1: Non-optional allocation by explicit size

The module must support allocation from an explicit size value and return storage usable by the caller when the request succeeds.

This feature includes:

- allocation from a `size_t`-like byte count,
- allocation from an `idx_t`-like byte count,
- allocation of character storage from a byte count.

The Rust rewrite must preserve the distinction between ordinary byte-size allocation and `idx_t`-based allocation where the source module provides separate entry points.

### Feature 2: Non-optional reallocation by explicit size

The module must support resizing an existing allocation or allocating fresh storage when used in the source module’s reallocation style.

This feature includes:

- reallocation from a `size_t`-like requested size,
- reallocation from an `idx_t`-like requested size.

The Rust rewrite must preserve the behavior that callers use these helpers to obtain storage of the requested size without separately handling allocation failure as a normal return case.

### Feature 3: Array-aware allocation and reallocation

The module must support allocation and reallocation based on element count and element size rather than only total byte size.

This feature includes:

- allocation from `(count, element_size)`,
- reallocation from `(count, element_size)`,
- both `size_t`-based and `idx_t`-based count/size variants where present.

The Rust rewrite must preserve the functional intent that callers can request array storage using logical element counts, not only precomputed byte totals.

### Feature 4: Growth-oriented reallocation with updated capacity tracking

The module must support helper operations used by callers that grow buffers progressively and track capacity through caller-owned size variables.

This feature includes:

- growth from a current byte-size value through `x2realloc`,
- growth from a current element-count value and element size through `x2nrealloc`,
- generalized capacity expansion through `xpalloc` with caller-supplied current count, minimum increment, optional maximum bound, and element size.

The Rust rewrite must preserve the observable behavior that these helpers both return storage and update the caller-visible tracked capacity/count variable to reflect the new allocation extent.

### Feature 5: Zero-initialized allocation

The module must support obtaining newly allocated storage whose contents are initialized to zero.

This feature includes:

- zero-initialized allocation by explicit byte size,
- zero-initialized allocation by `idx_t` size,
- zero-initialized array allocation from `(count, element_size)`.

The Rust rewrite must preserve the zero-initialization guarantee for successful calls to these variants.

### Feature 6: Failure handling as a non-success return path

Across the module surface, allocation helpers are intended to shield callers from ordinary null-on-failure handling.

The Rust rewrite must therefore preserve the functional contract that:

- successful calls yield valid storage for the requested operation, and
- allocation or size computations that cannot be satisfied are not reported as an ordinary successful null result.

This requirement is evidenced by the `x*` allocation helper role of all listed functions in `gnu/xmalloc.c`.

## User Scenarios & Testing

### Scenario 1: Caller needs a new heap buffer of a known byte size

A caller requests a fixed amount of storage and immediately uses the returned region.

Functions involved: explicit-size allocation helpers.

Expected support in Rust:

- allocate storage for the requested size,
- return a usable allocation handle/pointer equivalent for downstream module use,
- avoid exposing a normal successful null result.

Test coverage:

- request a small positive size and verify allocation succeeds,
- request size through both ordinary-size and `idx_t`-style entry points where applicable,
- verify the character allocation variant returns storage usable for `n` bytes.

### Scenario 2: Caller resizes an existing allocation

A caller already owns allocated storage and needs to grow or shrink it to a new requested size.

Functions involved: explicit-size reallocation helpers.

Expected support in Rust:

- accept an existing allocation input,
- return storage representing the resized allocation,
- support fresh allocation behavior when used with a null/empty prior allocation input if that is how the source reallocation helpers are used.

Test coverage:

- reallocate a prior allocation to a larger size,
- reallocate to a smaller size,
- exercise both ordinary-size and `idx_t`-style variants.

### Scenario 3: Caller allocates an array by element count and element size

A caller knows logical element count and element width and wants the module to obtain the required storage.

Functions involved: array-aware allocation helpers.

Expected support in Rust:

- accept count and element size,
- provide storage sized for the array request,
- treat the operation as an array allocation workflow rather than requiring the caller to precompute total bytes.

Test coverage:

- allocate storage for a small array,
- verify both `size_t`-based and `idx_t`-based array allocation variants behave consistently for equivalent values.

### Scenario 4: Caller resizes an array by element count and element size

A caller holds prior array storage and changes the intended element count.

Functions involved: array-aware reallocation helpers.

Expected support in Rust:

- accept prior allocation plus `(count, element_size)`,
- return resized storage for the requested logical array size.

Test coverage:

- grow an existing array allocation,
- shrink an existing array allocation,
- verify consistency across `size_t` and `idx_t` variants where both exist.

### Scenario 5: Caller grows a buffer incrementally while tracking capacity

A caller appends data over time and relies on helper functions to choose a larger allocation and update stored capacity metadata.

Functions involved: `x2realloc`, `x2nrealloc`, `xpalloc`.

Expected support in Rust:

- accept caller-owned current capacity/count state,
- enlarge capacity when more space is required,
- write back the new capacity/count to the caller-visible state,
- return storage corresponding to the updated extent.

Test coverage:

- start from an initial zero or empty capacity state and grow once,
- perform repeated growth and verify the tracked size/count increases,
- verify `xpalloc` honors minimum growth intent and updates the count variable,
- verify growth helpers operate in terms of bytes or elements as defined by each function family.

### Scenario 6: Caller needs zero-filled storage

A caller requires newly allocated memory initialized to zero before first use.

Functions involved: `xzalloc`, `xizalloc`, `xcalloc`.

Expected support in Rust:

- allocate storage of the requested extent,
- ensure newly returned bytes are zeroed.

Test coverage:

- allocate zero-initialized byte storage and verify contents are zero,
- allocate zero-initialized array storage and verify all bytes are zero,
- exercise both explicit-size and array-count forms.

## Requirements

### Functional Requirements

#### FR-1: Explicit-size allocation

The Rust module shall provide allocation operations corresponding to `xmalloc`, `ximalloc`, and `xcharalloc` from `gnu/xmalloc.c` that allocate storage for the requested byte count and return usable storage on success.

#### FR-2: Explicit-size reallocation

The Rust module shall provide reallocation operations corresponding to `xrealloc` and `xirealloc` from `gnu/xmalloc.c` that resize an existing allocation to the requested size and return usable storage on success.

#### FR-3: Array allocation by count and element size

The Rust module shall provide array allocation operations corresponding to `xnmalloc` and `xinmalloc` from `gnu/xmalloc.c` that allocate storage for `count × element_size` requests.

#### FR-4: Array reallocation by count and element size

The Rust module shall provide array reallocation operations corresponding to `xreallocarray` and `xireallocarray` from `gnu/xmalloc.c` that resize storage for `count × element_size` requests.

#### FR-5: Byte-capacity growth helper

The Rust module shall provide a growth helper corresponding to `x2realloc` from `gnu/xmalloc.c` that accepts caller-managed current size state, returns enlarged storage, and updates that size state to the new allocation size.

#### FR-6: Element-capacity growth helper

The Rust module shall provide a growth helper corresponding to `x2nrealloc` from `gnu/xmalloc.c` that accepts caller-managed current element count state plus element size, returns enlarged storage, and updates the element count state to the new count.

#### FR-7: Generalized capacity expansion

The Rust module shall provide a generalized allocation growth operation corresponding to `xpalloc` from `gnu/xmalloc.c` that:
- accepts caller-managed current element count state,
- accepts a minimum increment,
- accepts a maximum bound parameter,
- accepts an element size,
- returns enlarged storage, and
- updates the caller-managed count state to the resulting allocation count.

#### FR-8: Zero-initialized byte allocation

The Rust module shall provide zero-initialized allocation operations corresponding to `xzalloc` and `xizalloc` from `gnu/xmalloc.c` that return newly allocated storage whose bytes are initialized to zero.

#### FR-9: Zero-initialized array allocation

The Rust module shall provide a zero-initialized array allocation operation corresponding to `xcalloc` from `gnu/xmalloc.c` that returns newly allocated storage for `count × element_size` with zeroed contents.

#### FR-10: Non-success on unsatisfied allocation

For all operations corresponding to the source functions in `gnu/xmalloc.c`, the Rust module shall preserve the module-level contract that failure to satisfy allocation or resizing does not appear to callers as an ordinary successful null allocation result.

#### FR-11: Preservation of caller-visible size/count updates

For all growth helpers with pointer-to-size/count inputs in `gnu/xmalloc.c` (`x2realloc`, `x2nrealloc`, `xpalloc`), the Rust module shall preserve the caller-visible update of the tracked size/count value when the operation succeeds.

### Key Entities

#### Entity 1: Allocation request size

A scalar size value represents a requested number of bytes for direct allocation and reallocation operations.

Relationship to functions:

- used by explicit-size allocators,
- used by explicit-size reallocators,
- used by zero-initialized byte allocators.

#### Entity 2: Indexed size (`idx_t`-style) request

A scalar indexed-size value represents an alternate size domain used by several source entry points.

Relationship to functions:

- used by `ximalloc`,
- used by `xirealloc`,
- used by `xinmalloc`,
- used by `xireallocarray`,
- used by `xizalloc`,
- used by `xpalloc` state and increment parameters.

#### Entity 3: Allocation handle / prior allocation input

A caller-held allocation reference represents either:
- no prior allocation, or
- previously allocated storage subject to resizing.

Relationship to functions:

- input and output for reallocation helpers,
- input and output for growth helpers.

#### Entity 4: Element count and element size pair

A pair of scalar values describes logical array allocation size as `count × element_size`.

Relationship to functions:

- used by array allocation,
- used by array reallocation,
- used by zero-initialized array allocation,
- used by element-based growth helpers.

#### Entity 5: Caller-managed capacity/count state

A caller-owned mutable size/count variable tracks current allocation extent and is updated by growth helpers.

Relationship to functions:

- `x2realloc` updates byte-size state,
- `x2nrealloc` updates element-count state,
- `xpalloc` updates element-count state.

#### Entity 6: Maximum bound and minimum increment controls

Capacity growth may be constrained or directed by:
- a minimum increment value,
- an optional maximum count bound.

Relationship to functions:

- used by `xpalloc` to determine acceptable resulting count.

## Success Criteria

### SC-1: Source surface coverage

The Rust rewrite implements behavior covering all source functional roles evidenced by the functions listed from `gnu/xmalloc.c`.

### SC-2: Fixed-size allocation behavior

Tests demonstrate that explicit-size allocation helpers successfully return usable storage for representative positive-size requests, including ordinary-size and `idx_t`-style variants.

Traceability: `xmalloc`, `ximalloc`, `xcharalloc`.

### SC-3: Reallocation behavior

Tests demonstrate that explicit-size reallocation helpers can resize existing allocations for both larger and smaller requested sizes.

Traceability: `xrealloc`, `xirealloc`.

### SC-4: Array allocation behavior

Tests demonstrate that array allocation helpers accept `(count, element_size)` inputs and return usable storage for the resulting logical array size.

Traceability: `xnmalloc`, `xinmalloc`.

### SC-5: Array reallocation behavior

Tests demonstrate that array reallocation helpers can resize prior array allocations using `(count, element_size)` inputs.

Traceability: `xreallocarray`, `xireallocarray`.

### SC-6: Growth helper state update behavior

Tests demonstrate that growth helpers update the caller-visible tracked size/count variable after successful growth.

Traceability: `x2realloc`, `x2nrealloc`, `xpalloc`.

### SC-7: Incremental growth behavior

Tests demonstrate that repeated use of growth helpers from an initially empty or minimal state results in monotonically increased capacity/count until caller needs are met or a documented bound is reached.

Traceability: `x2realloc`, `x2nrealloc`, `xpalloc`.

### SC-8: Zero-initialization behavior

Tests demonstrate that storage returned by zero-initialized allocation helpers contains zeroed bytes across the requested extent.

Traceability: `xzalloc`, `xizalloc`, `xcalloc`.

### SC-9: No ordinary successful null result on unsatisfied allocation

Tests or reviewable contract checks demonstrate that the Rust rewrite does not represent allocation failure for these helpers as an ordinary successful null allocation result to callers.

Traceability: all functions in `gnu/xmalloc.c`.

## Acceptance Notes

- Conformance is determined by matching the functional behavior described above, not by reproducing C implementation structure.
- Internal Rust design may differ, provided the evidenced functional boundaries and caller-visible behaviors are preserved.
- No additional capabilities should be introduced as part of this module rewrite unless required elsewhere by independently evidenced project interfaces.