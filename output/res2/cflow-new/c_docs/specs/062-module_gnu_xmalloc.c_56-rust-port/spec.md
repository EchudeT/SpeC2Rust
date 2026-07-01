# spec.md

## Title
Rust Functional Specification for `module_gnu_xmalloc.c_56`

## Document Metadata
- Project: `cflow-new`
- Module: `module_gnu_xmalloc.c_56`
- Category: `module_cluster`
- Source file: `gnu/xmalloc.c`
- Rust branch: `062-module_gnu_xmalloc.c_56-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides checked dynamic memory allocation helpers. Its purpose is to centralize allocation, reallocation, zero-initialized allocation, array-sized allocation, and capacity-growth allocation behind routines that do not silently return allocation failure to the caller.

The Rust rewrite must preserve the functional behavior of this allocation helper module as a boundary used by other code that needs:
- allocation of raw byte regions,
- resizing of existing allocations,
- allocation sized by element count and element size,
- automatic growth of capacities for expandable buffers,
- zero-filled allocation,
- checked handling of size computations involving multiplication and growth.

The module does not define domain-specific data models. It operates on generic memory regions and size/count values.

## Scope
In scope:
- Checked allocation of a requested byte size.
- Checked reallocation of an existing allocation to a requested byte size.
- Checked allocation and reallocation for array-style `count × element_size` requests.
- Checked zero-initialized allocation.
- Capacity-growth support for appendable storage.
- Support for both `size_t`-based and `idx_t`-based size/count entry points where present in the source module.

Out of scope:
- New public APIs beyond the source module’s function set.
- Domain-specific container abstractions.
- Thread-safety guarantees not evidenced by the module.
- Persistence, serialization, recovery, or benchmarking behavior.

## Source Functions in Scope
The specification is traced to the following exported behaviors in `gnu/xmalloc.c`:
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

## Feature Specification

### 1. Checked single-size allocation
The module must provide allocation helpers that accept a requested allocation size and return storage for that size when the request is valid and satisfiable.

Behavioral boundary:
- The request is expressed as a number of bytes.
- The helper does not expose allocation failure as a normal successful return of a null pointer/value.
- The Rust version must preserve the module’s checked-allocation contract for both plain-size and `idx_t`-based entry points.

Traces:
- `xmalloc`
- `ximalloc`
- `xcharalloc`

### 2. Checked resizing of existing allocations
The module must provide helpers that resize an existing allocation to a caller-specified number of bytes.

Behavioral boundary:
- The operation accepts an existing allocation handle/reference and a target size.
- The operation supports the module’s existing null-input allocation semantics where applicable through the underlying realloc-style behavior.
- The helper preserves checked-allocation behavior and does not silently report allocation failure as a normal successful null return.

Traces:
- `xrealloc`
- `xirealloc`

### 3. Checked array-size allocation and reallocation
The module must support allocation requests specified as element count and element size, including reallocation of existing storage to hold `n` elements of size `s`.

Behavioral boundary:
- The module performs checked size computation for `count × element_size`.
- Requests that overflow the representable allocation size are not allowed to proceed as successful allocations.
- The Rust version must preserve this checked behavior for both `size_t` and `idx_t` variants.

Traces:
- `xreallocarray`
- `xireallocarray`
- `xnmalloc`
- `xinmalloc`
- `xcalloc`

### 4. Checked capacity growth for expandable buffers
The module must support growth-oriented reallocation helpers intended for callers that maintain a current capacity and need to increase it as more elements are appended.

Behavioral boundary:
- The helper computes a larger capacity than the current one when growth is needed.
- The helper updates the caller-visible capacity/count state to reflect the new allocation extent.
- The helper is suitable for repeated growth of linear storage without requiring the caller to choose each next size manually.
- Growth must remain checked against invalid or overflowed size computations.

Traces:
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

### 5. Checked zero-initialized allocation
The module must provide allocation helpers that return newly allocated storage initialized to zero.

Behavioral boundary:
- Returned storage is zero-filled over the requested allocation extent.
- Checked allocation semantics remain in effect.
- Both plain-size and `idx_t`-based forms must be preserved where present.

Traces:
- `xzalloc`
- `xizalloc`
- `xcalloc`

## User Scenarios & Testing

### Scenario 1: Allocate a fixed-size buffer
A caller needs a buffer of a known byte size and expects either usable storage or immediate failure handling by the module contract, rather than manual null checking.

The Rust version must support:
- requesting a fixed number of bytes,
- receiving usable storage for that extent on success,
- preserving checked-failure semantics.

Relevant traces:
- `xmalloc`
- `ximalloc`
- `xcharalloc`

Suggested tests:
- Request a small nonzero size and verify storage is returned.
- Request size through both plain-size and `idx_t` entry points and verify equivalent behavior.
- For the character-specific allocator, verify the returned region can be used as a byte/character buffer of the requested size.

### Scenario 2: Resize an existing allocation
A caller has previously allocated storage and needs to increase or decrease its size without manually allocating a new region.

The Rust version must support:
- resizing an existing allocation to a requested byte size,
- preserving realloc-style allocation semantics for null input where applicable,
- preserving checked-failure semantics.

Relevant traces:
- `xrealloc`
- `xirealloc`

Suggested tests:
- Allocate a buffer, resize it larger, and verify the resulting allocation extent matches the requested target.
- Allocate a buffer, resize it smaller, and verify the operation completes.
- Invoke the realloc helper with a null allocation input and verify it behaves as an allocation request under the module’s realloc-style contract.

### Scenario 3: Allocate storage for an array of elements
A caller knows an element count and element width and wants storage sized for the full array without manually multiplying sizes.

The Rust version must support:
- allocating `n × s` bytes using checked multiplication,
- reallocating an existing region to `n × s`,
- rejecting overflowed size computations through the module’s checked-failure path.

Relevant traces:
- `xnmalloc`
- `xinmalloc`
- `xreallocarray`
- `xireallocarray`
- `xcalloc`

Suggested tests:
- Allocate a small array and verify the total allocation extent matches `n × s`.
- Reallocate an existing array allocation to a larger element count.
- Exercise a deliberately overflowed `count × size` request and verify it does not produce a normal successful allocation result.

### Scenario 4: Grow a dynamic buffer as elements are appended
A caller maintains a buffer plus a current capacity value and needs the module to choose a larger capacity when existing space is insufficient.

The Rust version must support:
- growing from an initial empty or null allocation state,
- updating the caller-visible capacity/count output,
- increasing capacity monotonically when more space is needed,
- enforcing checked growth computations.

Relevant traces:
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

Suggested tests:
- Start with no allocation and zero capacity, request growth, and verify capacity becomes positive and storage is returned.
- Repeatedly request growth and verify the reported capacity increases.
- Use `xpalloc` with a minimum increment requirement and verify the resulting capacity satisfies that minimum growth when growth is needed.
- Use `xpalloc` with an upper bound and verify growth does not report a capacity above the supplied maximum.

### Scenario 5: Obtain zero-filled storage
A caller needs newly allocated memory whose contents are initialized to zero.

The Rust version must support:
- allocating a zeroed byte region of a requested size,
- allocating a zeroed array region for `n × s`,
- preserving checked-allocation behavior.

Relevant traces:
- `xzalloc`
- `xizalloc`
- `xcalloc`

Suggested tests:
- Allocate a small zeroed region and verify all bytes are zero.
- Repeat through both plain-size and `idx_t` zeroing entry points where present.
- Allocate a zeroed array and verify the full `n × s` region is zero-initialized.

## Requirements

### Functional Requirements

#### FR-1: Checked fixed-size allocation
The module shall provide fixed-size allocation operations for byte counts expressed through both `size_t`-style and `idx_t`-style inputs where the source module does so.

The operations shall:
- accept a requested byte size,
- return usable storage for that size on success,
- preserve the module’s checked-allocation contract rather than treating allocation failure as a normal null success result.

Traces:
- `xmalloc`
- `ximalloc`
- `xcharalloc`

#### FR-2: Checked reallocation by byte size
The module shall provide reallocation operations that resize an existing allocation to a specified byte size.

The operations shall:
- accept an existing allocation or null-equivalent input,
- resize or allocate according to realloc-style semantics,
- preserve the module’s checked-allocation contract.

Traces:
- `xrealloc`
- `xirealloc`

#### FR-3: Checked array-size computation
The module shall provide allocation and reallocation operations based on element count and element size.

The operations shall:
- compute total requested size from `count × element_size`,
- detect invalid size computations that exceed representable allocation size,
- prevent such invalid computations from being treated as successful allocations.

Traces:
- `xreallocarray`
- `xireallocarray`
- `xnmalloc`
- `xinmalloc`
- `xcalloc`

#### FR-4: Zero-initialized allocation
The module shall provide allocation operations that return newly allocated storage initialized to zero.

The operations shall:
- zero-fill the full requested extent,
- preserve checked-allocation behavior,
- support the source module’s plain-size, `idx_t`, and array forms where present.

Traces:
- `xzalloc`
- `xizalloc`
- `xcalloc`

#### FR-5: Automatic growth reallocation
The module shall provide growth-oriented reallocation for callers that track capacity externally.

The operations shall:
- accept caller-supplied current capacity/count state,
- compute and allocate a larger storage extent when growth is required,
- update the caller-visible capacity/count output to the new extent,
- preserve checked handling of growth and multiplication arithmetic.

Traces:
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

#### FR-6: Minimum-growth and maximum-bound support for generalized growth
The generalized growth operation shall support caller constraints on growth size.

The operation shall:
- accept a minimum increment requirement,
- accept a maximum element bound,
- return an updated capacity/count that respects the maximum bound and satisfies growth requirements when growth succeeds.

Traces:
- `xpalloc`

### Key Entities

#### 1. Allocation extent
A requested size in bytes representing the total amount of storage to allocate or reallocate.

Relationships:
- Used directly by fixed-size allocation and reallocation operations.
- May be derived from element count and element size in array-oriented operations.

Traces:
- `xmalloc`
- `ximalloc`
- `xcharalloc`
- `xrealloc`
- `xirealloc`
- `xzalloc`
- `xizalloc`

#### 2. Element count and element size
A pair of values representing array length and per-element width.

Relationships:
- Combined to derive an allocation extent.
- Used by array allocation, array reallocation, zeroed array allocation, and generalized growth operations.

Traces:
- `xreallocarray`
- `xireallocarray`
- `xnmalloc`
- `xinmalloc`
- `xcalloc`
- `x2nrealloc`
- `xpalloc`

#### 3. Existing allocation reference
A handle/reference to previously allocated storage, or a null-equivalent value for allocation-through-reallocation semantics.

Relationships:
- Input to reallocation and growth functions.
- May be replaced by a new allocation result after resize/growth.

Traces:
- `xrealloc`
- `xirealloc`
- `xreallocarray`
- `xireallocarray`
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

#### 4. Caller-visible capacity/count state
A mutable size/count value maintained by the caller and updated by growth operations to reflect current allocation capacity.

Relationships:
- Input/output state for automatic growth helpers.
- Determines the next growth result and is updated after successful growth.

Traces:
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

#### 5. Bounds and growth constraints
Caller-supplied limits controlling generalized growth behavior.

Relationships:
- `n_incr_min` expresses minimum growth requirement.
- `n_max` expresses maximum permitted element count/capacity.
- Applied only by the generalized growth helper.

Traces:
- `xpalloc`

## Success Criteria

### SC-1: Fixed-size allocation behavior is preserved
For each fixed-size allocation entry point, the Rust module can successfully allocate small valid sizes and returns usable storage corresponding to the requested extent.

Traceability:
- `xmalloc`
- `ximalloc`
- `xcharalloc`

### SC-2: Reallocation behavior is preserved
For each byte-size reallocation entry point, the Rust module can resize an allocation to both larger and smaller extents, and null-input reallocation follows realloc-style allocation behavior where applicable.

Traceability:
- `xrealloc`
- `xirealloc`

### SC-3: Array-size overflow is checked
For array-oriented allocation and reallocation entry points, requests whose `count × element_size` computation overflows or exceeds the supported allocation range are not reported as normal successful allocations.

Traceability:
- `xreallocarray`
- `xireallocarray`
- `xnmalloc`
- `xinmalloc`
- `xcalloc`

### SC-4: Zero-initialization is observable
For zeroing allocation entry points, tests over the entire requested region confirm that newly allocated bytes are zero-initialized.

Traceability:
- `xzalloc`
- `xizalloc`
- `xcalloc`

### SC-5: Growth helpers update caller-visible capacity
For growth-oriented entry points, when growth is requested from an initial or previously allocated state, the Rust module returns storage and updates the caller-visible capacity/count output to a larger valid value.

Traceability:
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

### SC-6: Generalized growth respects growth constraints
For the generalized growth helper, tests confirm that:
- resulting capacity growth satisfies the minimum increment requirement when growth occurs, and
- reported capacity does not exceed the supplied maximum bound.

Traceability:
- `xpalloc`

### SC-7: No unsupported behavioral expansion is introduced
The Rust rewrite exposes only the functionality evidenced by the source module’s allocation, reallocation, zeroing, array-sizing, and growth helpers, without requiring new public capabilities to preserve module behavior.

Traceability:
- `gnu/xmalloc.c`