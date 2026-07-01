# spec.md

## Title

Rust Functional Specification for `main_root_xmalloc.c_37`

## Summary

This module provides checked heap-allocation helpers used by the `cat` program’s main cluster. Its purpose is to centralize dynamic memory allocation behavior so callers can request allocation, reallocation, zero-initialized allocation, and capacity growth using size parameters expressed either as `size_t`-like values or `idx_t`-like values.

The Rust rewrite must preserve the module’s observable behavior as an allocation utility layer:

- allocate memory for a requested byte count or element count,
- reallocate existing allocations,
- grow allocations for expanding buffers or arrays,
- zero-initialize allocated memory where required,
- reject allocation results that are null or otherwise invalid for the requested operation,
- handle multiplication-based size requests through checked total-size computation before allocation,
- support both fixed-size requests and growth-oriented requests driven by caller-provided size/capacity variables.

This specification is limited to functionality evidenced by `xmalloc.c` and its exported function set.

## Scope

In scope:

- Checked allocation and reallocation helpers.
- Element-count × element-size allocation helpers.
- Growth helpers that update caller-managed capacity values.
- Zero-initialized allocation helpers.
- Support for both `size_t` and `idx_t` sized request variants where present.

Out of scope:

- Any allocator configurability beyond the behaviors evidenced by the module.
- New public APIs not represented by the analyzed functions.
- Threading, persistence, serialization, FFI, recovery workflows, or performance guarantees not stated by the source module boundary.

## Feature Specification

### Overview

The module defines a family of allocation functions that wrap standard heap operations with stronger behavioral guarantees for callers. The common role of these functions is to provide a successful allocation result for valid requests or otherwise fail instead of silently returning an unusable null pointer for a nonzero request.

The Rust version must implement the following functional areas:

1. **Checked single-size allocation**
   - Allocate a block for a requested number of bytes.
   - Provide variants for `size_t`-based and `idx_t`-based size inputs.
   - Provide a character-buffer allocation helper equivalent in behavior to allocating `n` bytes for characters.

2. **Checked reallocation**
   - Resize an existing allocation to a caller-requested size.
   - Preserve the expected realloc-style behavior of accepting an existing allocation handle and returning a handle for the resized storage.

3. **Checked array allocation and reallocation**
   - Support requests expressed as element count × element size.
   - Ensure total-size calculation is checked before attempting allocation or reallocation.
   - Provide both `size_t` and `idx_t` based variants where the source module provides them.

4. **Automatic growth allocation**
   - Provide helpers that enlarge allocations for callers maintaining resizable buffers or arrays.
   - Update the caller-supplied capacity/size variable to the new chosen capacity.
   - Ensure growth honors the caller’s minimum increment and maximum bound constraints where such parameters exist.

5. **Zero-initialized allocation**
   - Allocate memory initialized to zero for the requested size.
   - Provide both `size_t` and `idx_t` variants.

6. **Nonnull result enforcement**
   - The module contains a nonnull-check helper and all checked allocation entry points depend on the same functional guarantee: allocation results returned to callers must not be null when the operation is considered successful.

### Supported Operations

The Rust rewrite must cover the behaviors represented by these source functions:

- nonnull validation of allocation results,
- byte-count allocation,
- index-sized allocation,
- char-buffer allocation,
- byte-count reallocation,
- index-sized reallocation,
- array reallocation by count and element size,
- array allocation by count and element size,
- doubling / growth-style reallocation driven by caller-owned size state,
- bounded growth allocation driven by minimum increment and maximum size,
- zeroed allocation.

## User Scenarios & Testing

### Scenario 1: Allocate a simple byte buffer

A caller needs a heap buffer of a known byte length for temporary processing.

Expected support:

- The caller requests `n` bytes.
- The module returns a usable allocation result for that size.
- If the allocation cannot be provided, the module does not return a successful null result.

Test focus:

- successful allocation for a positive size,
- handling of zero-size request according to the module’s checked-allocation behavior,
- no successful path returns an unusable null allocation handle.

### Scenario 2: Allocate storage for `n` elements of size `s`

A caller needs memory for an array and wants total-size checking to happen in the allocation helper rather than manually.

Expected support:

- The caller supplies element count and element size.
- The module computes the required total allocation size safely.
- The allocation succeeds with a usable result when the total is valid and available.
- Invalid total-size computations are not treated as successful allocations.

Test focus:

- correct allocation for small valid products,
- rejection/failure behavior when count × size exceeds representable range,
- parity of behavior between `size_t` and `idx_t` variants where both exist.

### Scenario 3: Reallocate an existing buffer to a larger size

A caller has a previously allocated region and must expand it as more data is accumulated.

Expected support:

- The caller passes the existing allocation and a new requested size.
- The module returns a usable allocation result for the resized storage.
- The helper preserves realloc-style use with both null and non-null input allocation handles when applicable to the original function semantics.

Test focus:

- growth from an existing allocation,
- reallocation from a null input acting as allocation where applicable,
- no successful path returns null for a nonzero requested result.

### Scenario 4: Reallocate an array by element count and size

A caller tracks arrays in terms of element count rather than total bytes.

Expected support:

- The caller supplies existing allocation, new element count, and element size.
- The helper checks total size before reallocation.
- The helper returns a usable resized allocation result or fails instead of succeeding with an invalid result.

Test focus:

- valid count/size resize,
- overflow in total-size calculation,
- behavior consistency between direct byte-count reallocation and array-oriented reallocation.

### Scenario 5: Grow an expandable buffer with automatic capacity management

A caller maintains a capacity variable and needs a helper to choose a larger capacity when existing storage is full.

Expected support:

- The caller passes current allocation and a mutable size/capacity variable.
- The helper chooses a larger allocation size and updates that variable.
- The returned allocation is usable for the new capacity.
- Growth does not violate provided maximum bounds in APIs that accept one.
- Minimum growth constraints are honored in APIs that accept one.

Test focus:

- initial growth from empty or null storage,
- subsequent growth from a nonzero capacity,
- updated capacity value is larger when growth is requested,
- bounded growth respects `n_max`,
- minimum increment requirement is reflected in the updated capacity.

### Scenario 6: Allocate zero-initialized storage

A caller needs memory whose bytes start at zero.

Expected support:

- The caller requests a byte count or `idx_t` count.
- The returned storage is zero-initialized.

Test focus:

- allocated region contents are zeroed,
- usable result for valid sizes,
- parity between size-based and idx-based zero-allocation helpers.

## Requirements

### Functional Requirements

#### FR-1 Checked allocation for direct sizes
Traceability: `xmalloc`, `ximalloc`, `xcharalloc`, `check_nonnull` in `xmalloc.c`

The Rust module shall provide checked allocation operations for direct size requests, including byte-count allocation and the character-buffer allocation form. For successful operations, the returned allocation result shall be non-null/usable for the requested storage purpose.

#### FR-2 Checked reallocation for direct sizes
Traceability: `xrealloc`, `xirealloc`, `check_nonnull` in `xmalloc.c`

The Rust module shall provide checked reallocation operations that resize an existing allocation according to a direct size request. For successful operations, the returned allocation result shall be non-null/usable.

#### FR-3 Checked allocation for counted arrays
Traceability: `xnmalloc`, `xinmalloc` in `xmalloc.c`

The Rust module shall provide allocation operations that accept element count and element size, and shall base allocation on the checked total size implied by their product.

#### FR-4 Checked reallocation for counted arrays
Traceability: `xreallocarray`, `xireallocarray` in `xmalloc.c`

The Rust module shall provide reallocation operations that accept element count and element size, and shall base reallocation on the checked total size implied by their product.

#### FR-5 Growth reallocation with caller-managed size state
Traceability: `x2realloc`, `x2nrealloc` in `xmalloc.c`

The Rust module shall provide growth-oriented reallocation helpers that use caller-supplied mutable size/capacity state. These operations shall return resized storage and update the caller’s tracked size/capacity to the chosen new value.

#### FR-6 Bounded growth allocation
Traceability: `xpalloc` in `xmalloc.c`

The Rust module shall provide a growth allocation helper that:
- accepts caller-managed current element count/capacity,
- accepts a minimum increment,
- accepts a maximum bound,
- accepts an element size,
- returns storage sized for the updated capacity,
- updates the caller-managed count/capacity to the chosen new value.

#### FR-7 Zero-initialized allocation
Traceability: `xzalloc`, `xizalloc` in `xmalloc.c`

The Rust module shall provide checked allocation operations that return newly allocated storage initialized to zero.

#### FR-8 Support both size domains evidenced by the source module
Traceability: `ximalloc`, `xirealloc`, `xireallocarray`, `xinmalloc`, `xizalloc`, `xpalloc` in `xmalloc.c`

Where the source module exposes both `size_t`-based and `idx_t`-based entry points, the Rust rewrite shall preserve support for both corresponding request forms within the rewritten module interface.

#### FR-9 Checked size multiplication before counted allocation/reallocation
Traceability: `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2nrealloc`, `xpalloc` in `xmalloc.c`

For all operations whose requested storage size depends on multiplying a count by an element size, the Rust module shall validate that the implied total size is representable and suitable for allocation before attempting the allocation or reallocation.

#### FR-10 No successful null result from checked allocation helpers
Traceability: `check_nonnull` and all exported allocation/reallocation helpers in `xmalloc.c`

The Rust module shall preserve the checked-allocation contract that successful completion of these helpers does not yield a null allocation result for callers expecting allocated storage.

### Key Entities

#### Allocation handle
Traceability: all exported functions in `xmalloc.c`

The central entity is an opaque heap-allocation result representing owned dynamically allocated storage returned to a caller or derived from reallocation of prior storage. The module operates on this entity but does not define higher-level payload structure.

#### Requested size
Traceability: `xmalloc`, `xrealloc`, `xzalloc`, `x2realloc` in `xmalloc.c`

A direct scalar size identifying a byte count for allocation or reallocation.

#### Indexed size
Traceability: `ximalloc`, `xirealloc`, `xireallocarray`, `xinmalloc`, `xizalloc`, `xpalloc` in `xmalloc.c`

A scalar size expressed in the module’s `idx_t` domain, used for APIs that mirror the direct-size operations but accept this alternate size type.

#### Element count and element size pair
Traceability: `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2nrealloc`, `xpalloc` in `xmalloc.c`

A pair of values defining array storage requirements. Their relationship determines the total storage size for allocation or reallocation and must be checked before use.

#### Mutable capacity/count state
Traceability: `x2realloc`, `x2nrealloc`, `xpalloc` in `xmalloc.c`

A caller-owned mutable numeric variable that records current capacity or element count for resizable storage. Growth helpers read this state, choose a larger value, update it, and return storage sized to match the updated state.

#### Growth constraints
Traceability: `xpalloc` in `xmalloc.c`

Inputs comprising minimum required growth and maximum permitted size. They constrain how much storage may be added in a bounded growth operation.

## Success Criteria

### SC-1 Direct allocation behavior is preserved
Traceability: `xmalloc`, `ximalloc`, `xcharalloc`

For valid allocation requests covered by the source module, the Rust rewrite returns usable allocated storage for each direct-allocation entry point and does not treat a null result as a successful outcome.

### SC-2 Reallocation behavior is preserved
Traceability: `xrealloc`, `xirealloc`

For valid resize requests, the Rust rewrite returns usable resized storage for the direct reallocation entry points and preserves realloc-style use from caller perspective.

### SC-3 Counted allocation and reallocation check total size
Traceability: `xnmalloc`, `xinmalloc`, `xreallocarray`, `xireallocarray`

Tests for count-and-size entry points demonstrate that valid products allocate correctly and overflow or otherwise invalid total-size products are not accepted as successful allocations.

### SC-4 Growth helpers update caller-managed state
Traceability: `x2realloc`, `x2nrealloc`, `xpalloc`

Tests show that each growth helper updates the caller-managed size/capacity variable to reflect the newly allocated capacity when growth succeeds.

### SC-5 Bounded growth honors constraints
Traceability: `xpalloc`

Tests show that bounded growth requests do not produce a resulting capacity smaller than the required minimum increment target and do not exceed the supplied maximum bound.

### SC-6 Zero-initialized allocation is observable
Traceability: `xzalloc`, `xizalloc`

Tests verify that storage returned by the zero-allocation helpers is initialized to zero bytes across the requested region.

### SC-7 Interface coverage matches the analyzed module boundary
Traceability: all listed functions in `xmalloc.c`

The Rust rewrite exposes functionality corresponding to every analyzed exported operation in this module, without omitting any evidenced functional area.

### SC-8 Failure paths do not appear as successful allocations
Traceability: `check_nonnull` and all allocation/reallocation helpers in `xmalloc.c`

Validation tests confirm that the Rust rewrite does not return a successful null allocation result from checked allocation APIs.