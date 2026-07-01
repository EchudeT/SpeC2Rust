# spec.md

## Title

Functional Specification — `main_root_xmalloc.c_28` Rust Port

## Metadata

- **Project**: `pwd`
- **Module**: `main_root_xmalloc.c_28`
- **Category**: `main_cluster`
- **Source file**: `xmalloc.c`
- **Rust branch**: `028-main_root_xmalloc.c_28-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides allocation helpers that wrap dynamic memory allocation and reallocation operations with consistent failure handling, size-growth support, overflow-checked array sizing, and zero-initialized allocation variants.

The Rust rewrite must preserve the module’s functional role as a centralized allocator utility layer used by higher-level code to:

- allocate a requested number of bytes,
- allocate typed or counted storage using element-count × element-size semantics,
- reallocate existing storage,
- grow allocations according to helper growth rules,
- reject null allocation results,
- reject invalid or overflowing size computations,
- provide zero-filled allocation where requested.

This specification covers only the behaviors evidenced by `xmalloc.c`.

## Scope

### In Scope

- Non-null-returning allocation helpers corresponding to the functions defined in `xmalloc.c`.
- Reallocation helpers for direct byte sizes and count-based sizes.
- Array allocation helpers that validate multiplication-based sizing.
- Growth helpers that enlarge existing allocations and update caller-provided size/count state.
- Zero-initialized allocation helpers.

### Out of Scope

- Any allocator customization interface.
- Any public API beyond the functions evidenced in this module.
- Threading guarantees.
- Memory ownership tracking beyond standard allocation/reallocation semantics.
- Recovery mechanisms after allocation failure, beyond the module’s failure behavior.

## Feature Specification

### Summary of Provided Functionality

The module defines a family of allocation functions with one common contract: successful calls return a usable non-null pointer to storage meeting the requested sizing semantics, and unsuccessful calls do not silently return a null result for the caller to handle as an ordinary success path.

The Rust version must implement the following functional families:

1. **Basic allocation**
   - Allocate a block by byte count.
   - Support both `size_t`-based and `idx_t`-based size inputs.
   - Support a character-buffer allocation convenience form.

2. **Basic reallocation**
   - Reallocate an existing block to a requested byte count.

3. **Array-sized allocation and reallocation**
   - Allocate or reallocate storage based on `count × element_size`.
   - Detect invalid total sizes arising from arithmetic overflow or out-of-range products.
   - Support both `size_t`-based and `idx_t`-based count and size forms.

4. **Growth-oriented reallocation**
   - Increase allocation size when the exact next size is not fixed by the caller.
   - Update the caller’s tracked allocation size or element count after successful growth.
   - Support growth from both an already-allocated buffer and a null/uninitialized allocation state.

5. **Parameterized capacity growth**
   - Grow a caller-managed allocation according to:
     - current logical capacity,
     - required minimum increment,
     - maximum allowed logical count,
     - element size.
   - Return reallocated storage and update the tracked logical capacity.

6. **Zero-initialized allocation**
   - Allocate storage initialized to zero bytes.
   - Support both `size_t`-based and `idx_t`-based size forms.

### Behavioral Boundaries

The Rust rewrite must preserve these boundaries:

- The module is an allocation utility module, not a container module.
- It operates on raw storage sizing requests rather than storing higher-level metadata structures.
- Count-based helpers must be semantically distinct from plain byte-size helpers because they validate size multiplication.
- Growth helpers must update caller-owned size/count state on success.
- Null results from underlying allocation primitives are not exposed as ordinary successful outcomes.

## User Scenarios & Testing

### Scenario 1: Allocate a fixed-size working buffer

A caller needs a buffer of a known byte length for transient processing. The caller requests storage through the basic allocation helper and expects a usable non-null result.

**Rust version must support:**
- allocating the exact requested byte count,
- returning usable storage on success,
- enforcing module failure behavior rather than returning null as a success result.

**Test focus:**
- request a small positive size and verify allocation succeeds,
- verify returned storage is non-null/usable,
- verify deallocation by the surrounding system remains possible through normal ownership handling in Rust.

### Scenario 2: Allocate character storage for text processing

A caller needs a character buffer and uses the character-specific allocation helper for convenience.

**Rust version must support:**
- allocation semantics equivalent to a byte-count allocation for character storage,
- no behavioral distinction other than return type intent.

**Test focus:**
- allocate a character-sized buffer,
- verify length matches requested capacity intent.

### Scenario 3: Resize an existing buffer

A caller has existing storage and needs to enlarge or shrink it with reallocation.

**Rust version must support:**
- reallocation of existing storage to a requested size,
- acceptance of an existing pointer/allocation state,
- failure behavior consistent with the module when reallocation cannot be satisfied.

**Test focus:**
- allocate then reallocate to a larger size,
- allocate then reallocate to a smaller size,
- verify the returned allocation is usable after each successful call.

### Scenario 4: Allocate an array using element count and element size

A caller needs storage for `n` elements of size `s` and uses an array helper rather than manually multiplying.

**Rust version must support:**
- total-size computation from count and element size,
- rejection of overflowed or invalid products,
- successful allocation when the product is valid.

**Test focus:**
- allocate with small valid `n` and `s`,
- test boundary values near multiplication overflow,
- verify overflow does not produce wrapped allocation sizes.

### Scenario 5: Reallocate an array with checked sizing

A caller has an existing array allocation and must resize it to a new element count while preserving checked multiplication semantics.

**Rust version must support:**
- checked `count × size` computation during reallocation,
- successful resize for valid products,
- failure behavior for invalid products.

**Test focus:**
- reallocate a valid array to a larger valid count,
- attempt reallocation with an overflowing product and verify rejection.

### Scenario 6: Grow a buffer when only current capacity is known

A caller tracks an allocated byte count and needs a helper to choose a larger size when the current buffer fills.

**Rust version must support:**
- taking the current tracked size by mutable reference,
- computing a larger allocation amount,
- reallocating storage,
- updating the tracked size to the new capacity on success.

**Test focus:**
- initialize capacity state and call growth helper repeatedly,
- verify tracked size strictly increases when growth is needed,
- verify returned storage remains usable.

### Scenario 7: Grow an element buffer with tracked element count

A caller manages an array of elements, tracks capacity as element count, and needs growth based on element size.

**Rust version must support:**
- taking current element count by mutable reference,
- increasing capacity by growth logic rather than only an exact target,
- using element size to compute underlying byte allocation safely,
- updating tracked count on success.

**Test focus:**
- start from zero or null allocation and grow,
- grow multiple times and confirm count increases,
- verify count updates match allocated-capacity semantics, not merely requested minimum increment.

### Scenario 8: Grow capacity with explicit minimum increment and maximum bound

A caller needs finer control over capacity growth, including:
- current capacity,
- minimum required increase,
- maximum permitted logical capacity,
- element size.

The caller uses the parameterized growth helper to obtain a larger allocation without exceeding the maximum logical bound.

**Rust version must support:**
- growth that satisfies at least the required increment when possible,
- respect for the supplied maximum logical count,
- safe byte-size computation from logical element count and element size,
- update of the caller’s capacity variable on success.

**Test focus:**
- grow from zero capacity with a positive minimum increment,
- grow repeatedly while respecting a finite maximum,
- test near-maximum values and verify no capacity above the supplied bound is reported.

### Scenario 9: Allocate zero-filled storage

A caller needs storage initialized to zero, for example before populating a structure or buffer.

**Rust version must support:**
- allocation of the requested size,
- zero initialization across the full allocated region.

**Test focus:**
- allocate zeroed storage and verify all bytes are zero,
- repeat for both direct-size and `idx_t`-style entry points.

## Requirements

### Functional Requirements

#### FR-1: Non-null allocation result enforcement
The module shall enforce that successful allocation-family operations do not expose a null pointer/result as an ordinary successful outcome.

**Traceability**: `check_nonnull`, `xmalloc`, `ximalloc`, `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2realloc`, `x2nrealloc`, `xpalloc`, `xzalloc`, `xizalloc`

#### FR-2: Direct byte-count allocation
The module shall allocate storage for a caller-specified byte count through direct-size entry points for both `size_t`-based and `idx_t`-based inputs.

**Traceability**: `xmalloc`, `ximalloc`

#### FR-3: Character-buffer allocation convenience
The module shall provide a convenience allocation form for character storage using a caller-specified byte/count input.

**Traceability**: `xcharalloc`

#### FR-4: Direct byte-count reallocation
The module shall reallocate existing storage to a caller-specified byte count through direct-size entry points for both `size_t`-based and `idx_t`-based inputs.

**Traceability**: `xrealloc`, `xirealloc`

#### FR-5: Checked array allocation
The module shall allocate storage sized as `count × element_size` and shall reject invalid total sizes caused by arithmetic overflow or out-of-range multiplication.

**Traceability**: `xnmalloc`, `xinmalloc`

#### FR-6: Checked array reallocation
The module shall reallocate storage sized as `count × element_size` and shall reject invalid total sizes caused by arithmetic overflow or out-of-range multiplication.

**Traceability**: `xreallocarray`, `xireallocarray`

#### FR-7: Byte-capacity growth helper
The module shall provide a growth helper that:
- accepts an existing allocation state and a mutable tracked byte-size value,
- chooses a larger allocation size,
- reallocates storage accordingly,
- updates the tracked byte-size value on success.

**Traceability**: `x2realloc`, `x2nrealloc`

#### FR-8: Element-capacity growth helper
The module shall provide a growth helper that:
- accepts an existing allocation state,
- accepts a mutable tracked logical element count,
- uses element size to determine byte allocation,
- increases capacity and updates the tracked element count on success.

**Traceability**: `x2nrealloc`

#### FR-9: Parameterized capacity growth with bounds
The module shall provide a growth function that enlarges a caller-managed allocation based on:
- current logical capacity,
- required minimum increment,
- maximum logical capacity,
- element size,

and shall update the tracked logical capacity on success while respecting the supplied maximum bound.

**Traceability**: `xpalloc`

#### FR-10: Zero-initialized allocation
The module shall allocate storage initialized to zero bytes through direct-size entry points for both `size_t`-based and `idx_t`-based inputs.

**Traceability**: `xzalloc`, `xizalloc`

#### FR-11: Null initial allocation support for growth paths
The growth-oriented helpers shall support operation from an initial state representing no existing allocation.

**Traceability**: `x2realloc`, `x2nrealloc`, `xpalloc`

#### FR-12: Caller state mutation on successful growth
Where a helper accepts a mutable size/count parameter representing current capacity, the module shall update that caller-visible value to the new capacity after successful growth.

**Traceability**: `x2realloc`, `x2nrealloc`, `xpalloc`

### Key Entities

This module does not define custom record-like data structures. Its key entities are functional and scalar:

- **Allocation block**
  - A region of dynamically allocated storage returned to the caller.
  - Used by all allocation and reallocation functions.

- **Byte size**
  - A scalar representing a total number of bytes to allocate or reallocate.
  - Used by direct allocation, reallocation, zero-allocation, and byte-growth helpers.

- **Element count**
  - A scalar representing logical capacity in units of elements rather than bytes.
  - Used by checked array helpers and element-growth helpers.

- **Element size**
  - A scalar representing the byte size of one logical element.
  - Combined with element count for checked total-size computation.

- **Tracked capacity reference**
  - A caller-owned mutable size/count variable passed to growth helpers.
  - Updated by the module to reflect the new successful capacity.

- **Maximum logical bound**
  - A caller-supplied upper limit on logical element count for bounded growth.
  - Used by `xpalloc`.

### Entity Relationships

- An **allocation block** is created or resized according to either a **byte size** or a computed total from **element count × element size**.
- A **tracked capacity reference** describes the current capacity associated with an **allocation block** and is updated when growth helpers enlarge the block.
- A **maximum logical bound** constrains how far a growth helper may increase an **element count**-based capacity.
- **Zero-initialized allocation** is a specialized form of allocation block creation with the additional requirement that returned bytes begin as zero.

## Success Criteria

### SC-1: Direct allocation parity
For valid byte-size inputs accepted by the C module’s direct allocation functions, the Rust version returns usable allocated storage for the corresponding Rust-facing operation.

**Traceability**: `xmalloc`, `ximalloc`

### SC-2: Character allocation parity
The Rust version provides character-buffer allocation behavior equivalent to direct byte allocation for the requested size.

**Traceability**: `xcharalloc`

### SC-3: Direct reallocation parity
For valid reallocation requests accepted by the C module, the Rust version successfully resizes existing allocations and returns usable storage.

**Traceability**: `xrealloc`, `xirealloc`

### SC-4: Overflow-safe array sizing
For array allocation and reallocation requests where `count × element_size` would overflow or exceed representable allocation size, the Rust version rejects the request rather than allocating a wrapped or truncated size.

**Traceability**: `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`

### SC-5: Growth state update correctness
After each successful call to a growth helper, the caller-visible tracked size/count is updated to the new capacity and is greater than or equal to the capacity required by that helper’s contract.

**Traceability**: `x2realloc`, `x2nrealloc`, `xpalloc`

### SC-6: Bounded growth correctness
For `xpalloc`, the Rust version never reports a new logical capacity greater than the supplied maximum logical bound.

**Traceability**: `xpalloc`

### SC-7: Null-initial growth support
Growth helpers in the Rust version can create a new allocation when invoked with an initial state equivalent to no prior allocation.

**Traceability**: `x2realloc`, `x2nrealloc`, `xpalloc`

### SC-8: Zero initialization correctness
For zero-allocation entry points, all bytes in the returned storage are zero-initialized before caller use.

**Traceability**: `xzalloc`, `xizalloc`

### SC-9: No silent null-success path
Across all successful allocation-family operations covered by this module, the Rust version does not represent success with a null allocation result.

**Traceability**: `check_nonnull` and all exported allocation helpers

## Acceptance Notes

- The Rust rewrite may adapt signatures to Rust ownership and safety idioms, but it must preserve the functional behaviors specified above.
- Validation must focus on observable allocation semantics, checked sizing behavior, growth behavior, zero-initialization behavior, and caller-visible capacity updates.
- No requirement in this specification implies new public APIs or capabilities beyond those evidenced in `xmalloc.c`.