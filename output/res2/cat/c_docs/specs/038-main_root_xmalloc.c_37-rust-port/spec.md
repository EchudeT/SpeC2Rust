# spec.md

## Overview

- **Project**: `cat`
- **Module**: `main_root_xmalloc.c_37`
- **Category**: `main_cluster`
- **Source basis**: `xmalloc.c`
- **Rust branch target**: `038-main_root_xmalloc.c_37-rust-port`
- **Generation date**: `2026-06-07`

## Feature Specification

This module provides allocation helpers that wrap dynamic memory allocation, reallocation, array growth, and zero-initialized allocation behind a single failure model.

The Rust rewrite must implement the same functional boundary:

- provide allocation entry points for single-size allocations, index-sized allocations, and character-buffer allocations;
- provide reallocation entry points for direct byte sizes and element-count × element-size requests;
- provide growth helpers that increase allocation sizes for buffers whose required capacity changes over time;
- provide zero-initialized allocation helpers;
- reject unsuccessful allocation by using the module’s fatal-allocation behavior rather than returning a successful-looking null allocation result for nonzero requests.

The module is not a container library and does not define higher-level ownership models. Its role is limited to memory sizing and allocation behavior that callers in the main program can rely on.

### Supported functional areas

1. **Non-null checked allocation**
   - Requests allocation and ensures the result is acceptable for callers.
   - Includes wrappers for `size_t`-sized and `idx_t`-sized requests.

2. **Checked reallocation**
   - Reallocates existing storage to a requested size.
   - Supports both direct byte-count reallocation and array-style reallocation based on element count and element size.

3. **Growth-oriented reallocation**
   - Expands allocations when the caller tracks capacity and needs a larger buffer.
   - Supports automatic growth from an existing or empty allocation state.
   - Supports growth with caller-provided lower bounds and upper bounds.

4. **Zero-initialized allocation**
   - Allocates memory and initializes the returned region to zero.

## User Scenarios & Testing

### Scenario 1: Allocate a fixed-size object
A caller needs a block of memory of a known size and expects either usable storage or module-level fatal failure behavior.

**Expected support**
- A fixed-size allocation request returns usable memory for successful nonzero allocations.
- The Rust port preserves the fail-fast behavior on allocation failure.

**Test focus**
- Successful allocation for a small positive size.
- Failure path is not silently converted into a successful result.

### Scenario 2: Allocate a character buffer
A caller needs a character buffer of `n` bytes for text or raw byte handling.

**Expected support**
- The module returns a character-addressable allocation sized for the requested byte count.

**Test focus**
- Buffer allocation succeeds for ordinary sizes.
- Returned storage can be written up to the requested extent.

### Scenario 3: Resize an existing allocation
A caller has an existing allocation and must grow or shrink it to a new byte size.

**Expected support**
- Reallocation preserves the wrapper’s checked behavior.
- The operation works whether the incoming pointer is null-equivalent or already allocated.

**Test focus**
- Reallocate from null to a newly allocated block.
- Reallocate an existing block to a larger size.
- Reallocate an existing block to a smaller size.

### Scenario 4: Resize an array by element count
A caller tracks an array as `count × element_size` and needs checked resizing without unchecked size multiplication at the call site.

**Expected support**
- The module accepts element count and element size separately.
- The Rust version preserves the same checked array-allocation behavior.

**Test focus**
- Allocate and reallocate arrays using count and element size inputs.
- Validate that ordinary count/size combinations produce usable storage.

### Scenario 5: Grow a buffer incrementally
A caller appends data repeatedly and tracks current capacity in an external variable.

**Expected support**
- The growth helper increases capacity and updates the tracked size/count value.
- Growth works from an initially empty state.

**Test focus**
- Start with no allocation and zero capacity, then grow.
- Repeated growth calls monotonically increase capacity until limits are reached.
- Updated capacity value matches the returned allocation extent contract expected by callers.

### Scenario 6: Grow with minimum increment and maximum bound
A caller needs capacity growth that is at least a minimum increment but must not exceed a caller-specified maximum element count.

**Expected support**
- The module grows the allocation while respecting the caller-provided lower-bound increment and maximum bound parameters.

**Test focus**
- Growth satisfies minimum increment when expansion is possible.
- Growth does not report a capacity beyond the specified maximum bound.

### Scenario 7: Allocate zero-filled storage
A caller needs newly allocated memory initialized to zero.

**Expected support**
- The returned memory region is zero-initialized for the requested size.

**Test focus**
- Allocate zero-initialized storage and verify all bytes are zero.
- Cover both `size_t`-based and `idx_t`-based entry points.

## Requirements

### Functional Requirements

#### FR-1: Checked allocation wrappers
The module shall provide wrappers for allocating memory by a requested size and shall enforce the module’s checked-allocation behavior for those requests.

**Traceability**
- `xmalloc`
- `ximalloc`
- `xcharalloc`
- `check_nonnull`

#### FR-2: Checked reallocation wrappers
The module shall provide wrappers for reallocating an existing allocation to a requested size and shall enforce the same checked-allocation behavior during resize operations.

**Traceability**
- `xrealloc`
- `xirealloc`
- `check_nonnull`

#### FR-3: Array-oriented allocation and reallocation
The module shall support allocation and reallocation based on element count and element size so callers can request storage for arrays without manually collapsing the request to a single byte size at the call site.

**Traceability**
- `xreallocarray`
- `xireallocarray`
- `xnmalloc`
- `xinmalloc`

#### FR-4: Capacity-growth reallocation
The module shall support growth-oriented reallocation for callers that maintain capacity externally and need the module to enlarge storage and update the tracked capacity.

**Traceability**
- `x2realloc`
- `x2nrealloc`

#### FR-5: Parameterized bounded growth
The module shall support growth with caller-supplied current count, minimum increment, maximum bound, and element size, and shall return updated storage together with an updated count/capacity value.

**Traceability**
- `xpalloc`

#### FR-6: Zero-initialized allocation
The module shall support allocation helpers that return newly allocated zero-filled storage.

**Traceability**
- `xzalloc`
- `xizalloc`

#### FR-7: Dual size-domain support
Where the C module accepts both `size_t`-based and `idx_t`-based request forms, the Rust rewrite shall preserve both functional input domains at the module boundary used by the ported callers.

**Traceability**
- `ximalloc`
- `xirealloc`
- `xireallocarray`
- `xinmalloc`
- `xizalloc`
- `xpalloc`

### Key Entities

#### Allocation request
A request for storage expressed either as:
- a direct size in bytes, or
- an element count together with an element size.

This is the primary input entity for all module operations.

**Related functions**
- direct-size operations: `xmalloc`, `ximalloc`, `xrealloc`, `xirealloc`, `xzalloc`, `xizalloc`
- count × size operations: `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2nrealloc`, `xpalloc`

#### Existing allocation pointer
A possibly empty or previously allocated memory reference supplied for resize or growth operations.

**Related functions**
- `xrealloc`
- `xirealloc`
- `xreallocarray`
- `xireallocarray`
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

#### Tracked capacity/count
A caller-owned numeric state updated by growth helpers to reflect the new allocation extent after successful growth.

**Related functions**
- `x2realloc`
- `x2nrealloc`
- `xpalloc`

#### Bounds and growth constraints
Caller-provided minimum increment, maximum count bound, and element size values that shape growth behavior for bounded expansion.

**Related functions**
- `x2nrealloc`
- `xpalloc`

## Success Criteria

### SC-1: Allocation behavior parity
For all supported allocation entry points traced in this specification, successful ordinary-size requests shall return usable storage representing the requested extent.

**Measured by**
- Unit tests covering each functional area with representative positive-size inputs.

### SC-2: Fatal-on-allocation-failure parity
For allocation and reallocation failures covered by this module’s checked behavior, the Rust port shall not report success with a usable result object or null-equivalent success value for a nonzero request.

**Measured by**
- Negative-path tests or integration assertions aligned with the port’s chosen fatal-allocation mechanism.

### SC-3: Reallocation behavior parity
Resize operations shall support both empty-input and existing-allocation scenarios and shall return storage matching the requested resized extent when successful.

**Measured by**
- Tests for null/empty-to-allocated transition.
- Tests for grow and shrink reallocation cases.

### SC-4: Array request support parity
Array-style entry points shall accept count and element-size inputs and successfully serve valid ordinary requests without requiring callers to precompute a single byte size outside the module.

**Measured by**
- Tests for allocation and reallocation through count × size interfaces.

### SC-5: Growth helper state update parity
Growth-oriented helpers shall update the caller-tracked capacity/count output when growth succeeds.

**Measured by**
- Tests confirming output count/capacity changes after `x2realloc`/`x2nrealloc`/`xpalloc`-equivalent operations.

### SC-6: Bounded growth parity
The bounded growth helper shall honor provided growth constraints such that successful results do not exceed the supplied maximum bound and provide at least the required minimum increment when growth is possible within bounds.

**Measured by**
- Constraint-focused tests for minimum increment and maximum bound handling.

### SC-7: Zero-initialization parity
Zero-initialized allocation helpers shall return memory whose full requested extent is initialized to zero.

**Measured by**
- Bytewise verification tests over newly allocated regions from zero-allocation entry points.