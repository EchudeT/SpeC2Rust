# spec.md

## Title

Rust Functional Specification for `module_gnu_xmalloc.c_56`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_xmalloc.c_56`
- Category: `module_cluster`
- Source file: `gnu/xmalloc.c`
- Rust branch: `062-module_gnu_xmalloc.c_56-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides allocation helpers that wrap heap allocation, reallocation, zero-initialized allocation, array-sized allocation, and capacity growth for dynamically sized buffers. The defining behavior of the module is that allocation requests either succeed and return usable storage, or they do not return normally.

The Rust rewrite must preserve the module’s externally observable allocation behavior for the supported operations in `gnu/xmalloc.c`, including:

- allocating a fixed number of bytes,
- allocating using `idx_t`-sized counts,
- reallocating existing storage,
- allocating or reallocating arrays from element count and element size,
- growing storage according to module-defined expansion rules,
- producing zero-initialized storage where applicable.

## Scope

### In Scope

The Rust version must implement the behavior represented by the following source-level functions in `gnu/xmalloc.c`:

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

### Out of Scope

The Rust specification does not require capabilities not evidenced by this module, including:

- new allocation APIs beyond the listed behaviors,
- thread-safety guarantees,
- persistence or serialization,
- recovery-oriented allocation fallbacks,
- foreign-function interface design,
- performance targets beyond functional correctness.

## Feature Specification

### Feature 1: Non-returning-on-failure allocation wrappers

The module provides wrappers for heap allocation and reallocation whose functional contract is stronger than plain allocator return conventions: successful requests return a pointer-like handle to storage of the requested size, and failed requests are not reported by returning a normal success value such as a null pointer.

This applies to:

- single-size allocation (`xmalloc`, `ximalloc`, `xcharalloc`),
- reallocation (`xrealloc`, `xirealloc`),
- zero-initialized allocation (`xzalloc`, `xizalloc`),
- count-and-element-size allocation (`xcalloc`, `xnmalloc`, `xinmalloc`),
- count-and-element-size reallocation (`xreallocarray`, `xireallocarray`).

The Rust rewrite must preserve this behavioral boundary: callers using the module do not perform ordinary null-result handling after these functions.

### Feature 2: Array-size aware allocation and reallocation

The module supports requests expressed as a pair of values: element count and element size. This includes fresh allocation and reallocation. The functional purpose is to size storage in units of elements rather than only bytes.

The Rust version must support:

- allocation from `(n, s)`,
- reallocation from `(p, n, s)`,
- the same behavior for both `size_t`-based and `idx_t`-based variants.

### Feature 3: Capacity growth for resizable storage

The module provides functions that compute a larger allocation size or element count and then reallocate accordingly. These are intended for code that incrementally grows arrays or buffers.

This behavior appears in:

- `x2realloc`,
- `x2nrealloc`,
- `xpalloc`.

The Rust rewrite must preserve the observable growth-oriented contract:

- when a caller provides the current logical size or count by reference,
- the function returns reallocated storage sized for growth,
- the referenced size/count is updated to the new capacity decision,
- the resulting capacity is at least sufficient for the required growth constraint encoded by the function’s inputs.

### Feature 4: Zero-initialized storage

The module provides allocation helpers that return zero-filled memory for the requested size or count/size combination.

This applies to:

- `xzalloc`,
- `xizalloc`,
- `xcalloc`.

The Rust version must preserve zero-initialization as an observable postcondition for newly allocated storage from these operations.

## User Scenarios & Testing

### Scenario 1: Allocate a fixed-size buffer

A caller needs a heap buffer of `N` bytes and expects either a usable allocation or abnormal termination/non-returning failure behavior. The caller does not branch on a null result.

Supported by:

- `xmalloc`
- `ximalloc`
- `xcharalloc`

#### Test expectations

- Requesting a positive size returns storage usable for at least that many bytes.
- `xcharalloc(n)` returns storage usable as a character buffer of length `n`.
- The Rust port does not expose ordinary null-success semantics for successful returns.

### Scenario 2: Resize an existing allocation

A caller already owns allocated storage and needs it resized while preserving normal realloc semantics on success.

Supported by:

- `xrealloc`
- `xirealloc`

#### Test expectations

- Reallocating to a requested size returns storage usable for the new size.
- Reallocation from an existing allocation succeeds or does not return normally on failure.
- Reallocation behavior is available for both `size_t`-style and `idx_t`-style size inputs.

### Scenario 3: Allocate an array by element count

A caller wants space for `n` elements of size `s` instead of manually computing total bytes.

Supported by:

- `xnmalloc`
- `xinmalloc`
- `xcalloc`

#### Test expectations

- Allocation returns storage large enough for `n * s` bytes when the request is valid.
- `xcalloc` returns zero-initialized storage for the requested array extent.
- Count/size-based variants operate for both native size counts and `idx_t` counts where defined.

### Scenario 4: Reallocate an array by element count

A caller grows or shrinks an existing array and specifies the new array extent as element count and element size.

Supported by:

- `xreallocarray`
- `xireallocarray`

#### Test expectations

- Reallocation returns storage sized for the requested array extent.
- The Rust version preserves the module contract that failed allocation does not yield a normal null result.
- The element-count form is supported for both `size_t` and `idx_t` variants.

### Scenario 5: Grow a buffer with internally chosen next capacity

A caller tracks current capacity externally and asks the module to choose a larger capacity and resize storage.

Supported by:

- `x2realloc`
- `x2nrealloc`

#### Test expectations

- The function updates the caller-provided capacity variable.
- Returned storage matches the updated capacity and element size contract.
- Growth from an initially empty or zero-capacity state yields a usable initial allocation.
- Repeated growth calls increase capacity enough to continue appending data without requiring the caller to manually compute the next size.

### Scenario 6: Grow an allocation with minimum increment and maximum bound

A caller manages a dynamically growing array with explicit minimum-growth and maximum-count constraints.

Supported by:

- `xpalloc`

#### Test expectations

- The function updates the caller-provided element count to the selected new capacity.
- The new capacity is sufficient to satisfy the minimum increment requirement represented by the inputs.
- The new capacity does not exceed the provided maximum-count bound when such a bound applies.
- Returned storage is sized consistently with the updated count and element size.

### Scenario 7: Allocate zero-filled storage

A caller requires newly allocated memory to be initialized to zero bytes before use.

Supported by:

- `xzalloc`
- `xizalloc`
- `xcalloc`

#### Test expectations

- Returned storage contains zero values across the allocated extent.
- Zero-initialized allocation is available for both byte-count and count/size request styles represented by the module.

## Requirements

### Functional Requirements

#### FR-1: Fixed-size allocation
The module shall provide allocation operations that accept a byte size and return usable heap storage for that size on success, corresponding to `xmalloc`, `ximalloc`, and `xcharalloc` in `gnu/xmalloc.c`.

#### FR-2: Reallocation
The module shall provide operations that resize previously allocated storage to a requested byte size, corresponding to `xrealloc` and `xirealloc`.

#### FR-3: Array-based allocation
The module shall provide operations that allocate storage from an element count and element size, corresponding to `xnmalloc`, `xinmalloc`, and `xcalloc`.

#### FR-4: Array-based reallocation
The module shall provide operations that reallocate storage from an element count and element size, corresponding to `xreallocarray` and `xireallocarray`.

#### FR-5: Zero-initialized allocation
The module shall provide allocation operations whose returned newly allocated storage is zero-initialized, corresponding to `xzalloc`, `xizalloc`, and `xcalloc`.

#### FR-6: Growth reallocation with caller-tracked size
The module shall provide growth-oriented reallocation that updates a caller-supplied size or count value to the newly chosen capacity, corresponding to `x2realloc` and `x2nrealloc`.

#### FR-7: Growth allocation with minimum increment and maximum bound
The module shall provide growth-oriented reallocation that accepts current count, minimum increment, maximum count, and element size, and updates the caller-supplied count to the selected capacity, corresponding to `xpalloc`.

#### FR-8: Non-normal failure reporting contract
For all allocation and reallocation operations in this module, allocation failure shall not be represented to callers as ordinary successful return with a null pointer result; the Rust rewrite shall preserve the module’s non-returning-on-failure contract evidenced by the `x*` allocation wrappers in `gnu/xmalloc.c`.

#### FR-9: Support for both native-size and index-size request forms
Where the C module exposes both `size_t`-based and `idx_t`-based variants, the Rust rewrite shall preserve both request forms as distinct supported behaviors, corresponding to `ximalloc`, `xirealloc`, `xireallocarray`, `xinmalloc`, and `xizalloc`.

### Key Entities

#### Entity 1: Allocation extent
A requested allocation extent is the amount of storage to obtain, expressed either as:

- a raw byte count, or
- an element count combined with an element size.

This entity is used by all allocation and reallocation functions.

#### Entity 2: Existing allocation handle
An existing allocation handle represents storage previously returned by this module and later passed back for resizing. This entity is used by `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `x2realloc`, `x2nrealloc`, and `xpalloc`.

#### Entity 3: Caller-maintained capacity/count
A mutable size or count value supplied by the caller represents the current capacity before growth and receives the new capacity after growth. This entity is central to `x2realloc`, `x2nrealloc`, and `xpalloc`.

#### Entity 4: Maximum count bound
A caller-supplied upper bound on element count constrains growth decisions in `xpalloc`.

#### Entity 5: Element size
An element-size value defines the size of each logical item in array-oriented allocation and growth operations. It participates in the relationship between count-based capacity and total allocated bytes in `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2nrealloc`, `xpalloc`, and `xcalloc`.

## Success Criteria

### SC-1: Fixed-size allocation coverage
The Rust module provides callable behavior equivalent in scope to `xmalloc`, `ximalloc`, and `xcharalloc`, and tests demonstrate successful allocation for representative valid sizes.

### SC-2: Reallocation coverage
The Rust module provides behavior equivalent in scope to `xrealloc` and `xirealloc`, and tests demonstrate that existing allocations can be resized and used at the new size.

### SC-3: Array allocation coverage
The Rust module provides behavior equivalent in scope to `xnmalloc`, `xinmalloc`, `xcalloc`, `xreallocarray`, and `xireallocarray`, and tests demonstrate correct allocation or reallocation from `(count, element_size)` inputs.

### SC-4: Zero-initialization correctness
Tests verify that memory returned by the Rust equivalents of `xzalloc`, `xizalloc`, and `xcalloc` is zero-filled across the requested extent immediately after allocation.

### SC-5: Growth API state update correctness
Tests verify that the Rust equivalents of `x2realloc`, `x2nrealloc`, and `xpalloc` update the caller-visible size/count state after growth and return storage consistent with the updated capacity.

### SC-6: Minimum-growth and bound handling
Tests for the Rust equivalent of `xpalloc` verify that resulting capacity satisfies the requested minimum increment and respects the supplied maximum-count constraint for valid bounded inputs.

### SC-7: Non-null normal return contract
Tests and API review verify that successful Rust allocation operations do not model success as a nullable normal return corresponding to C null-pointer success cases, preserving the module’s fail-or-return behavior.

### SC-8: Source traceability
Each implemented Rust behavior can be traced to one or more functions in `gnu/xmalloc.c`, and no additional public functional capability outside the evidenced source scope is required for acceptance.