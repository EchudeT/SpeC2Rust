# spec.md

## Title

Functional Specification: `main_root_xmalloc.c_28` Rust Port

## Document Control

- **Project**: `pwd`
- **Module**: `main_root_xmalloc.c_28`
- **Category**: `main_cluster`
- **Source file(s)**: `xmalloc.c`
- **Target branch**: `028-main_root_xmalloc.c_28-rust-port`
- **Generation date**: 2026-06-09

## Overview

This module provides allocation helpers that wrap memory allocation and reallocation operations with consistent failure handling, size-growth support, overflow-aware array sizing, and zero-initialized allocation variants.

The Rust rewrite must preserve the module’s observable allocation behavior exposed by the functions in `xmalloc.c`. In particular, the Rust version must support:

- allocation of a requested number of bytes,
- reallocation of an existing allocation to a requested size,
- allocation and reallocation for element-count × element-size requests,
- growth-oriented reallocation where capacity is increased and returned through caller-provided size/count variables,
- bounded growth for expandable buffers or arrays,
- zero-initialized allocation variants,
- rejection of invalid or unrepresentable size requests through the same fail-fast allocation contract used by this module.

This module is a support layer for callers that want allocation requests either to succeed and return usable storage, or to fail through the module’s fatal-allocation behavior rather than by returning a normal error result.

## Feature Specification

### Feature 1: Non-null allocation contract

The module provides allocation helpers that do not expose successful zero/non-zero ambiguity to callers. When an allocation request is considered successful, the returned pointer is non-null and usable according to the requested size semantics. If the request cannot be satisfied, the module uses its fail-fast allocation behavior instead of returning a normal failure value.

This applies to direct allocation, reallocation, array allocation, and zero-initialized allocation entry points.

**Traceability**: `check_nonnull`, `xmalloc`, `ximalloc`, `xcharalloc`, `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `xzalloc`, `xizalloc` in `xmalloc.c`.

### Feature 2: Size-typed allocation entry points

The module supports allocation APIs that accept both `size_t`-based sizes and `idx_t`-based sizes. The Rust version must preserve the same functional distinction where callers can request memory using either size domain and receive equivalent allocation behavior under the module’s fail-fast contract.

**Traceability**: `xmalloc`, `ximalloc`, `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `xzalloc`, `xizalloc`.

### Feature 3: Array-size allocation and reallocation

The module supports requests expressed as element count multiplied by element size. The Rust version must preserve behavior for both initial allocation and resizing of arrays/buffers where the total byte size is derived from `(count, element_size)` rather than passed directly as a precomputed byte total.

The module must treat total-size computation as part of the API contract, including handling requests that cannot be represented safely under the underlying size type.

**Traceability**: `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2nrealloc`, `xpalloc`.

### Feature 4: Growth-oriented reallocation

The module provides helpers that expand an allocation while also updating caller-owned size/count state. The Rust version must preserve two growth styles:

- growth based on byte-size state,
- growth based on element-count state and element size.

These functions are intended for callers that maintain expandable storage and need the helper to choose a larger capacity and publish that new capacity back to the caller.

**Traceability**: `x2realloc`, `x2nrealloc`.

### Feature 5: Bounded capacity expansion

The module supports expandable allocation with explicit minimum growth and explicit maximum bound. The Rust version must preserve support for:

- an existing allocation and current element count,
- a required minimum increment,
- a maximum permitted element count,
- an element size,
- returning both the resized storage and the updated count.

The module must ensure that resulting capacity changes satisfy the caller’s minimum-growth request while respecting the provided maximum bound, or else trigger the module’s fail-fast behavior if the request cannot be satisfied within representable or permitted limits.

**Traceability**: `xpalloc`.

### Feature 6: Zero-initialized allocation

The module provides allocation entry points that return newly allocated memory initialized to zero bytes for the requested total size, for both `size_t` and `idx_t` size domains.

**Traceability**: `xzalloc`, `xizalloc`.

## User Scenarios & Testing

### Scenario 1: Allocate a fixed-size working buffer

A caller needs a buffer of a known byte length and does not want to manually check for allocation failure at each call site. The caller requests storage through the module and receives usable memory or the module terminates through its allocation-failure path.

**Supported by**: `xmalloc`, `ximalloc`, `xcharalloc`.

**Testing focus**:
- requesting a positive byte size returns usable storage,
- `xcharalloc` returns storage suitable for byte/character data,
- requests that cannot be fulfilled do not produce a normal null return.

### Scenario 2: Resize an existing buffer to an exact byte size

A caller already owns allocated storage and must resize it to a specific new byte length. The module resizes the storage and returns the updated allocation under the same fail-fast contract.

**Supported by**: `xrealloc`, `xirealloc`.

**Testing focus**:
- reallocation to a larger size returns usable storage,
- reallocation from a non-null existing allocation preserves the exact-size request contract,
- failure conditions use the module’s fatal-allocation behavior instead of ordinary error returns.

### Scenario 3: Allocate or resize an array by element count

A caller manages an array of fixed-size elements and wants the module to compute the total byte requirement from element count and element size. The module performs the size derivation and allocation/reallocation safely.

**Supported by**: `xnmalloc`, `xinmalloc`, `xreallocarray`, `xireallocarray`.

**Testing focus**:
- valid `(count, size)` combinations allocate or resize correctly,
- overflow or unrepresentable total-size combinations are not silently wrapped,
- no normal success path yields an unusable null pointer.

### Scenario 4: Grow a byte buffer dynamically

A caller maintains a byte buffer whose capacity is stored in a mutable size variable. When more space is needed, the caller invokes the growth helper, receives a resized allocation, and observes that the recorded capacity has been increased.

**Supported by**: `x2realloc`.

**Testing focus**:
- when growth is required, returned storage is usable and the size variable is updated,
- repeated growth operations monotonically increase capacity unless bounded by failure,
- capacity bookkeeping after the call matches the returned allocation contract.

### Scenario 5: Grow an element array dynamically

A caller maintains a growable array with a count/capacity variable and an element size. The caller asks the module to enlarge storage, and the module returns the reallocated storage while updating the count variable to the new capacity.

**Supported by**: `x2nrealloc`.

**Testing focus**:
- returned storage matches the updated capacity and element size,
- the count variable is updated on successful growth,
- invalid or overflowing growth requests are rejected through fail-fast behavior.

### Scenario 6: Expand storage with minimum increment and maximum limit

A caller needs capacity growth that is constrained: at least a minimum number of additional elements are required, but capacity must never exceed a caller-specified maximum. The module chooses an enlarged capacity meeting these rules or fails fatally if this cannot be done.

**Supported by**: `xpalloc`.

**Testing focus**:
- successful calls increase capacity by at least the requested minimum increment,
- successful calls do not set capacity beyond the specified maximum,
- requests impossible under the maximum bound trigger failure behavior.

### Scenario 7: Allocate zeroed memory

A caller needs newly allocated memory whose bytes are initialized to zero. The module allocates and zero-initializes the requested size under the same fail-fast contract.

**Supported by**: `xzalloc`, `xizalloc`.

**Testing focus**:
- allocated memory is entirely zeroed,
- both size domains are supported,
- allocation failure does not surface as a normal null success result.

## Requirements

### Functional Requirements

#### FR-1: Direct byte allocation
The Rust module shall provide allocation operations corresponding to direct byte-count requests and shall return usable non-null storage on success under the module’s fail-fast allocation contract.

**Traceability**: `xmalloc`, `ximalloc`, `xcharalloc`.

#### FR-2: Exact-size reallocation
The Rust module shall provide reallocation operations for an existing allocation and a requested new total size, preserving the module’s fail-fast contract.

**Traceability**: `xrealloc`, `xirealloc`.

#### FR-3: Count-by-size allocation
The Rust module shall provide allocation operations where total bytes are derived from element count and element size, rather than requiring callers to precompute the product.

**Traceability**: `xnmalloc`, `xinmalloc`.

#### FR-4: Count-by-size reallocation
The Rust module shall provide reallocation operations where the new total size is derived from element count and element size.

**Traceability**: `xreallocarray`, `xireallocarray`.

#### FR-5: Safe size derivation
The Rust module shall treat multiplication-based size derivation as part of its contract and shall not permit silent wraparound or successful allocation based on an invalid reduced total size when count and element size are not safely representable.

**Traceability**: `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`, `x2nrealloc`, `xpalloc`.

#### FR-6: Growth helper with mutable byte-size state
The Rust module shall provide a growth helper that accepts caller-owned byte-size state, reallocates storage to a larger capacity, and updates that state to the resulting capacity on success.

**Traceability**: `x2realloc`.

#### FR-7: Growth helper with mutable element-count state
The Rust module shall provide a growth helper that accepts caller-owned element-count state plus element size, reallocates storage to a larger capacity, and updates the count state on success.

**Traceability**: `x2nrealloc`.

#### FR-8: Bounded growth helper
The Rust module shall provide a growth operation that expands an allocation using current count, minimum required increment, maximum permitted count, and element size, and shall update the count to the selected new capacity on success.

**Traceability**: `xpalloc`.

#### FR-9: Minimum increment and maximum bound enforcement
For bounded growth, the Rust module shall ensure that a successful result satisfies the caller’s minimum increment requirement without exceeding the specified maximum count.

**Traceability**: `xpalloc`.

#### FR-10: Zero-initialized allocation
The Rust module shall provide allocation operations that return newly allocated memory initialized to zero bytes for the requested total size.

**Traceability**: `xzalloc`, `xizalloc`.

#### FR-11: Fail-fast behavior for invalid or unsatisfied allocation requests
The Rust module shall preserve the module’s fail-fast behavior for requests that cannot produce a valid allocation result, including allocation failure and invalid size computation cases covered by this module’s APIs.

**Traceability**: `check_nonnull` and all allocation/reallocation entry points in `xmalloc.c`.

### Key Entities

#### Entity 1: Allocation request
An allocation request is the input describing the desired storage size. Depending on API, it is expressed as:

- a direct byte size,
- an element count plus element size,
- or a current allocation plus a requested new size.

This entity is the primary input to all module functions.

**Traceability**: all public allocation/reallocation functions in `xmalloc.c`.

#### Entity 2: Existing allocation
Some operations accept an existing allocation reference and resize it. This entity links prior storage ownership with a new requested size or growth operation.

**Traceability**: `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `x2realloc`, `x2nrealloc`, `xpalloc`.

#### Entity 3: Mutable capacity/count state
Growth-oriented APIs use caller-provided mutable size/count variables to carry current capacity in bytes or elements and to receive the new capacity after successful growth.

**Traceability**: `x2realloc` (`size_t *ps`), `x2nrealloc` (`size_t *pn`), `xpalloc` (`idx_t *pn`).

#### Entity 4: Growth constraints
Bounded expansion uses:
- a minimum required increment,
- a maximum allowed count,
- an element size.

These values constrain how capacity may increase.

**Traceability**: `xpalloc`.

#### Entity 5: Size domains
The module operates with two caller-visible size domains:
- `size_t`-based values,
- `idx_t`-based values.

The Rust port must preserve behavior for both categories of entry points.

**Traceability**: paired `x*` and `xi*` functions throughout `xmalloc.c`.

## Success Criteria

### SC-1: Direct allocation behavior
For every direct-allocation entry point in this module, valid requests produce usable non-null storage, and unsuccessful requests do not return ordinary success values.

**Traceability**: `xmalloc`, `ximalloc`, `xcharalloc`, `check_nonnull`.

### SC-2: Reallocation behavior
For every exact-size reallocation entry point, resizing requests succeed with returned usable storage when satisfiable, and otherwise follow the module’s fail-fast behavior.

**Traceability**: `xrealloc`, `xirealloc`.

### SC-3: Array sizing correctness
For count-by-size allocation and reallocation entry points, test cases with valid count/size products succeed, and cases with overflowing or invalid products do not succeed using wrapped byte totals.

**Traceability**: `xreallocarray`, `xireallocarray`, `xnmalloc`, `xinmalloc`.

### SC-4: Growth state update correctness
For growth helpers, successful calls update the caller-provided size/count variable to the new capacity associated with the returned allocation.

**Traceability**: `x2realloc`, `x2nrealloc`, `xpalloc`.

### SC-5: Growth monotonicity under successful expansion
For calls intended to enlarge existing capacity, a successful result increases capacity rather than leaving the tracked capacity unchanged, except where exact behavior is constrained by the API’s initial-state semantics.

**Traceability**: `x2realloc`, `x2nrealloc`, `xpalloc`.

### SC-6: Bounded growth compliance
For bounded expansion, successful results satisfy both:
- capacity increase is at least the requested minimum increment, and
- resulting capacity does not exceed the specified maximum.

**Traceability**: `xpalloc`.

### SC-7: Zero-initialization correctness
Zero-initialized allocation entry points return newly allocated memory whose full requested byte region is zeroed.

**Traceability**: `xzalloc`, `xizalloc`.

### SC-8: Size-domain coverage
The Rust port includes behaviorally corresponding support for both `size_t`-style and `idx_t`-style entry points present in the source module.

**Traceability**: all paired `x*`/`xi*` functions in `xmalloc.c`.

### SC-9: No unsupported capability expansion
The Rust port limits itself to the allocation, reallocation, growth, bounded-growth, and zero-initialization behaviors evidenced by this module and does not require new public capabilities beyond those functions.

**Traceability**: entire `xmalloc.c` module scope.