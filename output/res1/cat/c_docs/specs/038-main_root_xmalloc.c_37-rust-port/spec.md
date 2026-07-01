# spec.md

## Title
Rust Port Functional Specification: `main_root_xmalloc.c_37`

## Metadata
- Project: `cat`
- Module: `main_root_xmalloc.c_37`
- Category: `main_cluster`
- Source file: `xmalloc.c`
- Rust branch: `038-main_root_xmalloc.c_37-rust-port`
- Generation date: `2026-06-06`

## Overview
This module provides allocation helpers that wrap dynamic memory allocation and reallocation operations with fail-fast behavior and size-growth support. The Rust rewrite must preserve the module’s externally observable behavior as an allocation utility used by other parts of the program to obtain non-null memory, resize existing allocations, allocate zero-initialized memory, and grow buffers or element counts safely.

The module’s scope is limited to allocation-related behavior evidenced by `xmalloc.c`. It does not define higher-level containers or ownership models.

## Feature Specification

### Summary
The Rust version must implement an allocation utility module that supports:

- allocating memory blocks for byte counts and index-sized counts,
- reallocating existing blocks,
- allocating arrays based on element count and element size,
- growing allocations according to helper growth rules,
- allocating zero-initialized memory,
- ensuring successful allocation returns a non-null pointer/value,
- treating allocation failure as a non-recoverable module outcome rather than returning a null success value.

### Supported Functional Surface
The module functionality is evidenced by these source functions in `xmalloc.c`:

- non-null validation for allocation results,
- plain allocation helpers,
- reallocation helpers,
- array allocation and array reallocation helpers,
- geometric/incremental growth helpers,
- zero-initialized allocation helpers.

### Behavioral Notes
- Successful allocation and reallocation operations must yield a usable non-null result.
- The module must support both `size_t`-based and `idx_t`-based size/count inputs where present in the C module.
- Growth helpers must update the caller-visible size/count state associated with the allocation request.
- Zero-initialized allocation helpers must return memory whose allocated bytes are initialized to zero.
- Array-oriented helpers must operate in terms of element count multiplied by element size, not as opaque byte-only wrappers.

## User Scenarios & Testing

### Scenario 1: Allocate a fixed-size block for immediate use
A caller needs a block of memory of a known size and expects either a usable allocation or immediate termination/failure handling consistent with the original module.

**Rust module must support:**
- requesting a byte-sized allocation,
- receiving a non-null usable result on success,
- not exposing a null successful result.

**Test focus:**
- allocate a small non-zero size,
- verify the result is usable,
- verify no successful call returns null/absent storage.

### Scenario 2: Allocate memory using index-sized length values
A caller tracks lengths with the project’s index type and needs equivalent allocation support without changing the logical count domain.

**Rust module must support:**
- index-sized allocation requests for plain blocks,
- index-sized zeroed allocation requests,
- index-sized array allocation and reallocation requests where applicable.

**Test focus:**
- pass representative `idx_t`-range values that are valid for the runtime,
- confirm behavior matches the corresponding size-based helpers.

### Scenario 3: Reallocate an existing block to a new size
A caller has an existing allocation and needs it resized while preserving the allocation contract.

**Rust module must support:**
- resizing an existing allocation,
- handling reallocation requests through both size-based and index-based entry points,
- returning a non-null usable result on success.

**Test focus:**
- grow an allocation,
- shrink an allocation,
- verify the returned result is usable after each resize.

### Scenario 4: Allocate or resize an array by element count and element size
A caller stores structured elements and wants allocation in terms of `count × element_size`.

**Rust module must support:**
- fresh array allocation from element count and element size,
- resizing an existing array allocation from element count and element size,
- equivalent support for index-sized count/size variants where present.

**Test focus:**
- allocate arrays with small counts and known element sizes,
- reallocate to larger and smaller counts,
- verify the resulting allocation size semantics correspond to count multiplied by size.

### Scenario 5: Grow a buffer when capacity is insufficient
A caller maintains a capacity variable and needs helper logic that increases allocation size and updates that variable.

**Rust module must support:**
- growth from an existing size/count state,
- update of the caller-managed size/count output parameter equivalent,
- support for both byte-oriented growth and element-oriented growth helpers.

**Test focus:**
- start from a small or zero capacity,
- invoke growth helper repeatedly,
- verify capacity/count increases monotonically as required by each helper’s contract,
- verify returned allocation remains usable.

### Scenario 6: Grow an allocation with minimum increment and upper bound
A caller needs capacity expansion that respects a minimum increase request and a maximum logical element limit.

**Rust module must support:**
- growth based on current count,
- enforcing the caller-specified minimum increment intent,
- honoring the caller-specified maximum logical bound,
- updating the count output to the chosen new capacity.

**Test focus:**
- grow from zero and non-zero starting counts,
- use a minimum increment greater than one,
- use a finite maximum bound,
- verify the new count respects requested growth and does not exceed the provided maximum.

### Scenario 7: Obtain zero-initialized memory
A caller needs newly allocated storage with all bytes initialized to zero.

**Rust module must support:**
- zero-initialized allocation for byte-sized requests,
- zero-initialized allocation for index-sized requests.

**Test focus:**
- allocate zeroed memory,
- verify all bytes in the returned region are zero before caller writes.

## Requirements

### Functional Requirements

#### FR-1: Non-null successful allocation result
The module shall ensure that any successful allocation helper covered by this module yields a non-null usable result.
- Traceability: `check_nonnull`, `xmalloc`, `ximalloc`, `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2realloc`, `x2nrealloc`, `xpalloc`, `xzalloc`, `xizalloc`

#### FR-2: Byte-count allocation
The module shall allocate memory for a caller-specified byte count through a size-based entry point.
- Traceability: `xmalloc`

#### FR-3: Index-count allocation
The module shall allocate memory for a caller-specified count expressed in the module’s index-sized domain.
- Traceability: `ximalloc`

#### FR-4: Character-buffer allocation
The module shall provide allocation for a character buffer using a byte-count input specialized for character storage.
- Traceability: `xcharalloc`

#### FR-5: Byte-count reallocation
The module shall resize an existing allocation using a caller-specified byte count.
- Traceability: `xrealloc`

#### FR-6: Index-count reallocation
The module shall resize an existing allocation using an index-sized count input.
- Traceability: `xirealloc`

#### FR-7: Array allocation by count and element size
The module shall allocate storage for arrays based on element count multiplied by element size, for both size-based and index-based input variants provided by the source module.
- Traceability: `xnmalloc`, `xinmalloc`

#### FR-8: Array reallocation by count and element size
The module shall resize existing array storage based on element count multiplied by element size, for both size-based and index-based input variants provided by the source module.
- Traceability: `xreallocarray`, `xireallocarray`

#### FR-9: Automatic growth of byte-sized allocation state
The module shall support growing an allocation while updating a caller-supplied size state that represents the current allocation extent.
- Traceability: `x2realloc`

#### FR-10: Automatic growth of element-count allocation state
The module shall support growing an allocation while updating a caller-supplied element-count state and using a caller-supplied element size.
- Traceability: `x2nrealloc`

#### FR-11: Parameterized capacity growth with lower and upper bounds
The module shall support allocation growth based on:
- current logical count,
- required minimum increment,
- maximum logical count bound,
- element size,
and shall update the caller-supplied count to the new chosen capacity.
- Traceability: `xpalloc`

#### FR-12: Zero-initialized allocation
The module shall allocate new storage initialized to zero for both size-based and index-based input variants provided by the source module.
- Traceability: `xzalloc`, `xizalloc`

#### FR-13: Existing-allocation and fresh-allocation growth usage
Where growth helpers accept an existing allocation input, the module shall support both:
- growing from an existing allocation, and
- allocating from an initially empty/no-allocation state.
- Traceability: `x2realloc`, `x2nrealloc`, `xpalloc`

### Key Entities

#### Allocation Request
A request consisting of one or more size-related inputs used to determine the amount of memory to obtain or resize.
- Forms evidenced in this module:
  - byte count,
  - index-sized count,
  - element count plus element size,
  - current count plus growth constraints.
- Traceability: all allocation and growth helpers in `xmalloc.c`

#### Allocation Result
The returned memory object/reference produced by allocation or reallocation helpers.
- Relationship:
  - produced from an allocation request,
  - must be non-null on success,
  - may represent fresh storage or resized existing storage.
- Traceability: all public helper functions and `check_nonnull`

#### Caller-Managed Size/Count State
A caller-owned mutable size/count value updated by growth helpers to reflect the new allocation extent or capacity.
- Forms evidenced in this module:
  - `size_t *ps`,
  - `size_t *pn`,
  - `idx_t *pn`.
- Relationship:
  - consumed as current state,
  - updated to new capacity by growth helpers.
- Traceability: `x2realloc`, `x2nrealloc`, `xpalloc`

#### Existing Allocation Input
An optional pre-existing allocation passed for resizing or growth.
- Relationship:
  - may be absent for fresh allocation,
  - may be present for reallocation/growth.
- Traceability: `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `x2realloc`, `x2nrealloc`, `xpalloc`

## Success Criteria

1. All functional behaviors described in FR-1 through FR-13 are implemented in the Rust port with traceable coverage to the corresponding source functions in `xmalloc.c`.

2. For every successful allocation and reallocation path represented by the source module, the Rust implementation returns a usable non-null result and does not represent success with a null allocation.
- Traceability: `check_nonnull` and all allocation-returning functions

3. Zero-initialized allocation paths produce storage whose full allocated byte range is zeroed before caller use.
- Traceability: `xzalloc`, `xizalloc`

4. Array allocation and reallocation paths size storage according to element count and element size semantics for all source-supported variants.
- Traceability: `xnmalloc`, `xinmalloc`, `xreallocarray`, `xireallocarray`

5. Growth helpers update the caller-visible size/count state on success and support both fresh-allocation and existing-allocation usage modes defined by the source module.
- Traceability: `x2realloc`, `x2nrealloc`, `xpalloc`

6. Parameterized growth behavior using minimum increment and maximum count bound is preserved for the Rust port within the functional contract of the source module.
- Traceability: `xpalloc`

7. The Rust rewrite remains within the allocation-helper scope of `xmalloc.c` and does not require additional public capabilities beyond those evidenced by the source module.
- Traceability: entire module scope in `xmalloc.c`