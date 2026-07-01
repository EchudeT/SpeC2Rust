# spec.md

## Title

Functional Specification: `main_root_xmalloc.c_28`

## Metadata

- Project: `pwd`
- Module: `main_root_xmalloc.c_28`
- Category: `main_cluster`
- Source file: `xmalloc.c`
- Rust port branch: `028-main_root_xmalloc.c_28-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides allocation helpers for dynamic memory management with failure-handling semantics stronger than raw allocation calls. Its role is to supply callers with memory allocation, reallocation, zero-initialized allocation, array-size-aware allocation, and growth-oriented resizing helpers.

The Rust rewrite must preserve the observable behavior of these helpers as a module-level facility used by other parts of the program to obtain memory or resize existing storage. The module is not specified as a general container library; it is an allocation utility layer.

## Feature Specification

### Summary

The Rust version must implement a set of allocation utilities that:

- allocate memory for a requested size,
- reallocate previously allocated memory,
- allocate memory for element counts and element sizes,
- detect size multiplication overflow where array-style sizing is involved,
- provide zero-initialized allocation,
- support growth-oriented resizing for buffers and arrays,
- reject unsuccessful allocations through the module’s failure path rather than returning a successful-looking pointer/value.

### Functional Scope

The module’s functional scope is limited to allocation support exposed through the following behaviors evidenced by `xmalloc.c`:

1. **Non-null result enforcement for successful allocations**
   - Allocation helpers validate the result of allocation attempts and do not treat a null result as success for nonzero allocation requests.
   - This behavior is represented by the internal result-checking helper and all exported allocation wrappers.

2. **Basic allocation by byte size**
   - The module allocates a block for a requested byte count.
   - It supports both `size_t`-based and `idx_t`-based size entry points.

3. **Character-buffer allocation**
   - The module provides a byte-oriented allocation helper returning character storage.

4. **Reallocation of existing storage**
   - The module resizes an existing allocation to a requested byte size.

5. **Array allocation and reallocation with element count × element size semantics**
   - The module allocates or reallocates storage sized as `count * element_size`.
   - The Rust version must preserve overflow-aware behavior for these array-style operations.

6. **Automatic growth helpers**
   - The module provides helpers that enlarge capacity when the current size/count is insufficient or zero.
   - These helpers update caller-provided size/count state to reflect the new allocation extent.

7. **Parameterized expansion with lower-bound and upper-bound constraints**
   - The module provides a growth helper that expands allocation according to a minimum requested increment, an optional maximum element limit, and an element size.
   - The Rust version must preserve the functional contract that resulting capacity respects caller-supplied bounds or follows the module’s failure path when a valid enlargement cannot be produced.

8. **Zero-initialized allocation**
   - The module allocates storage initialized to zero bytes.

### Out of Scope

The Rust rewrite must not introduce new module responsibilities not evidenced here, including:

- custom allocator configuration,
- thread-safety guarantees,
- persistence or serialization,
- public container abstractions,
- recovery-oriented error APIs beyond the module’s established allocation-failure behavior.

## User Scenarios & Testing

### Scenario 1: Allocate a new fixed-size block

A caller needs a memory block of a known byte size and expects either usable storage or the module’s failure behavior.

**Expected support:**
- allocate by byte count,
- return usable storage for successful requests,
- never present allocation failure as a successful allocation result for nonzero requests.

**Test focus:**
- successful allocation of small nonzero sizes,
- handling of zero-size requests according to the module’s observable semantics,
- failure-path behavior under forced allocation failure.

### Scenario 2: Allocate storage for a character buffer

A caller needs storage for `n` characters to build or hold textual data.

**Expected support:**
- allocate a character-addressable buffer sized by a byte count.

**Test focus:**
- returned storage can hold the requested number of bytes,
- same failure semantics as general allocation.

### Scenario 3: Resize an existing allocation

A caller already owns allocated storage and needs to enlarge or shrink it to a new byte size.

**Expected support:**
- reallocate existing storage,
- support null input where the underlying module semantics treat this as allocation,
- preserve failure behavior on unsuccessful resize.

**Test focus:**
- growth from a smaller to a larger size,
- shrink to a smaller size,
- reallocation from null input,
- failure-path behavior under forced reallocation failure.

### Scenario 4: Allocate or resize an array with overflow-sensitive sizing

A caller stores elements of fixed size and needs space for `n` elements.

**Expected support:**
- allocate or reallocate using `n * element_size`,
- detect invalid sizing when multiplication cannot be represented,
- route invalid or impossible requests through the module’s failure behavior.

**Test focus:**
- valid small `n, s` combinations,
- edge cases near representable limits,
- overflow cases for multiplication,
- both fresh allocation and reallocation forms.

### Scenario 5: Grow a buffer progressively during input accumulation

A caller starts with no or limited capacity and appends data over time, requiring geometric-style growth rather than exact resizing on every append.

**Expected support:**
- growth helper expands allocation and updates the tracked size,
- initial zero-capacity case is handled,
- repeated calls can continue enlarging storage.

**Test focus:**
- growth from zero capacity,
- multiple successive expansions,
- updated capacity value after each call,
- storage remains usable after expansion.

### Scenario 6: Grow an element array with explicit limits

A caller manages an array of elements and needs to increase capacity while respecting:
- a minimum increment,
- a maximum allowed element count,
- a fixed element size.

**Expected support:**
- growth helper chooses a new capacity meeting at least the minimum increase when possible,
- updated count is written back,
- maximum-count constraint is honored,
- impossible expansions follow module failure behavior.

**Test focus:**
- normal growth within bounds,
- growth exactly to the maximum,
- requests that would exceed the maximum,
- zero and nonzero current capacities,
- different element sizes.

### Scenario 7: Obtain zero-filled storage

A caller needs newly allocated memory with all bytes initialized to zero.

**Expected support:**
- zero-initialized allocation for requested size,
- support for both standard and index-sized entry points.

**Test focus:**
- newly allocated bytes are zeroed,
- behavior for small and moderate sizes,
- failure-path behavior under forced allocation failure.

## Requirements

### Functional Requirements

#### FR-1: Checked allocation by byte size
The module shall provide allocation operations that accept a requested byte count and return usable storage for successful requests. For unsuccessful allocation attempts, the module shall follow its allocation-failure behavior rather than reporting success with usable storage absent.

**Traceability:** `xmalloc`, `ximalloc`, internal `check_nonnull` in `xmalloc.c`

#### FR-2: Character-buffer allocation
The module shall provide an allocation operation specialized for character storage by requested byte count.

**Traceability:** `xcharalloc` in `xmalloc.c`

#### FR-3: Checked reallocation by byte size
The module shall provide reallocation operations that resize existing storage to a requested byte count, with the same failure-handling model as checked allocation.

**Traceability:** `xrealloc`, `xirealloc` in `xmalloc.c`

#### FR-4: Array-sized allocation with overflow-aware sizing
The module shall provide allocation operations for `element_count * element_size` requests and shall preserve the source module’s guarded behavior for invalid or unrepresentable total sizes.

**Traceability:** `xnmalloc`, `xinmalloc` in `xmalloc.c`

#### FR-5: Array-sized reallocation with overflow-aware sizing
The module shall provide reallocation operations for `element_count * element_size` requests and shall preserve the source module’s guarded behavior for invalid or unrepresentable total sizes.

**Traceability:** `xreallocarray`, `xireallocarray` in `xmalloc.c`

#### FR-6: Growth reallocation by tracked byte size
The module shall provide a helper that grows an allocation and updates a caller-supplied tracked byte size to the newly chosen allocation size.

**Traceability:** `x2realloc` in `xmalloc.c`

#### FR-7: Growth reallocation by tracked element count
The module shall provide a helper that grows an allocation for elements of fixed size and updates a caller-supplied element count to the newly chosen capacity.

**Traceability:** `x2nrealloc` in `xmalloc.c`

#### FR-8: Parameterized capacity expansion
The module shall provide an expansion helper that, given current allocation state, minimum required increment, maximum permitted element count, and element size, returns enlarged storage and updates the tracked element count. The resulting behavior shall respect caller-supplied bounds or follow the module’s failure behavior when enlargement cannot be validly satisfied.

**Traceability:** `xpalloc` in `xmalloc.c`

#### FR-9: Zero-initialized allocation
The module shall provide allocation operations that return newly allocated storage initialized to zero bytes.

**Traceability:** `xzalloc`, `xizalloc` in `xmalloc.c`

#### FR-10: Support for both standard-size and index-size entry points where present
For operations that exist in both `size_t`-based and `idx_t`-based forms in the source module, the Rust rewrite shall preserve both functional entry paths or equivalent internal handling sufficient to preserve caller-visible behavior in the ported program.

**Traceability:** `ximalloc`, `xirealloc`, `xireallocarray`, `xinmalloc`, `xizalloc` in `xmalloc.c`

### Key Entities

#### 1. Allocated memory block
A dynamically allocated region returned by allocation helpers and accepted by reallocation helpers.

**Relationships:**
- produced by basic, array-sized, growth, and zero-initialized allocation operations,
- consumed by reallocation and growth operations.

#### 2. Byte size
A requested allocation extent expressed as a byte count.

**Relationships:**
- input to basic allocation, reallocation, character allocation, and zero-initialized allocation,
- updated by the byte-growth helper.

#### 3. Element count
A requested or tracked number of elements in array-oriented operations.

**Relationships:**
- combined with element size to define total allocation extent,
- updated by element-growth helpers,
- constrained by maximum-count parameters in parameterized expansion.

#### 4. Element size
The size in bytes of one logical array element.

**Relationships:**
- multiplied by element count in array allocation/reallocation,
- used by growth helpers to determine byte extent for expanded arrays.

#### 5. Maximum element bound
An upper limit on allowed element count for parameterized growth.

**Relationships:**
- constrains the result of parameterized expansion,
- participates in deciding whether growth can succeed.

#### 6. Allocation state pointer/count pair
Caller-maintained storage state represented by:
- a memory pointer, and
- a mutable tracked size or element count.

**Relationships:**
- growth helpers read current state,
- growth helpers write back the new size/count after successful expansion.

## Success Criteria

1. **Allocation correctness**
   - For supported successful requests, each allocation function returns usable storage of at least the requested logical extent.
   - **Traceability:** `xmalloc`, `ximalloc`, `xcharalloc`, `xnmalloc`, `xinmalloc`, `xzalloc`, `xizalloc`

2. **Reallocation correctness**
   - For supported successful requests, each reallocation function returns usable storage for the requested logical extent and accepts prior allocation state as input.
   - **Traceability:** `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`

3. **Zero-initialization correctness**
   - Storage returned by zero-initialized allocation operations is verified as all-zero across the requested byte extent.
   - **Traceability:** `xzalloc`, `xizalloc`

4. **Overflow-guarded array sizing**
   - Test cases that would overflow or otherwise invalidate `count * size` sizing do not produce a false successful allocation result.
   - **Traceability:** `xnmalloc`, `xinmalloc`, `xreallocarray`, `xireallocarray`

5. **Growth state update**
   - On successful growth operations, the caller-provided mutable size/count value is updated to the new capacity chosen by the helper.
   - **Traceability:** `x2realloc`, `x2nrealloc`, `xpalloc`

6. **Bound-respecting expansion**
   - Parameterized expansion never reports success with a resulting element count above the supplied maximum bound.
   - **Traceability:** `xpalloc`

7. **Initial-growth support**
   - Growth helpers operate correctly when starting from null storage and/or zero tracked capacity, as allowed by the source behavior.

8. **Failure-path preservation**
   - Under injected allocation failure or invalid sizing conditions, the Rust module exhibits the same module-level failure semantics expected by callers of this allocation utility layer, rather than silently succeeding.
   - **Traceability:** internal `check_nonnull`; all checked allocation wrappers in `xmalloc.c`

9. **Port integration suitability**
   - The Rust rewrite supplies all behaviors required for callers in the ported program that depend on this allocation utility module, without requiring new public capabilities beyond those specified here.
   - **Traceability:** all functions in `xmalloc.c`