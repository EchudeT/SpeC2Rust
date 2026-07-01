# Implementation Plan: module_gnu_calloc.c_22

## Summary

This module ports `gnu/calloc.c` into Rust, preserving the behavior of `rpl_calloc` with a narrow scope focused on allocation-size validation, zero-initialized memory creation, and consistent error signaling.

The Rust implementation should translate the C allocation wrapper into a small, self-contained module that:
- checks multiplication overflow for `count * element_size`,
- returns zero-initialized storage on success,
- reports allocation failure or invalid size through idiomatic Rust error handling internally,
- exposes only the minimum API needed to mirror the original module role.

The technical approach should prefer safe Rust where possible:
- use checked arithmetic (`checked_mul`) for size computation,
- use zero-initialized buffers via `vec![0; total]` or equivalent standard-library allocation paths,
- avoid unnecessary unsafe code unless a raw-pointer-compatible boundary is required by the surrounding port.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear zero-initialization cost equivalent to C `calloc`,
  - avoid extra reallocations or intermediate buffers,
  - keep overflow checking constant-time,
  - maintain predictable behavior for zero-sized and large allocation requests.

## Module Mapping

| C Source File | Rust Target File | Notes |
|---|---|---|
| `gnu/calloc.c` | `src/module_gnu_calloc.rs` | Direct port of allocation wrapper logic from the single C source file. |

| C Function | Rust Function | Notes |
|---|---|---|
| `rpl_calloc` | `rpl_calloc` | Preserve name if project conventions allow; otherwise keep a thin compatibility function with the same semantics. |

## Data Model

This module does not define persistent C structs or custom data containers.

| C Type / Pattern | Rust Mapping | Notes |
|---|---|---|
| `void *` return from allocator | `Vec<u8>` or `Box<[u8]>` internally; raw pointer only if required by surrounding API | Prefer owned safe containers inside Rust. |
| `size_t` | `usize` | Natural Rust mapping for allocation sizes. |
| null pointer failure | `Result<_, AllocError>` or `Option<_>` internally | Use explicit error signaling in Rust and convert only at the external boundary if needed. |

### Error Model

| C Behavior | Rust Mapping | Notes |
|---|---|---|
| overflow in `n * s` | dedicated error variant such as `AllocError::SizeOverflow` | Replaces implicit C failure path with explicit checked arithmetic. |
| allocation failure | dedicated error variant such as `AllocError::AllocationFailed` | If allocation is expressed through standard containers, map fallible allocation behavior consistently if exposed. |

## Implementation Phases

### Phase 1: Module Skeleton and API Mapping
- Create `src/module_gnu_calloc.rs`.
- Add the Rust equivalent of `rpl_calloc` with the same input shape based on `usize` size parameters.
- Decide the exact return type based on surrounding project conventions, preferring a safe owned buffer type unless an existing raw-memory interface already exists in the port.
- Keep the module limited to the single migrated function and any minimal private error type needed for implementation.

### Phase 2: Core Allocation Semantics Port
- Port the size computation logic using `checked_mul` to prevent overflow.
- Implement zero-initialized allocation using the Rust standard library.
- Ensure zero-sized requests follow a deliberate, documented behavior aligned with the broader port’s allocation conventions.
- Translate C failure paths into explicit Rust error returns, and only convert to pointer-style outcomes if required at the module boundary.

### Phase 3: Memory and Error Boundary Review
- Verify ownership and drop behavior so allocated memory is released automatically without manual free logic.
- If the surrounding code still expects pointer-like results, isolate any necessary unsafe conversion to a narrow boundary and document invariants.
- Confirm there is no hidden dependence on C global state, errno-style mutation, or allocator-specific side effects beyond failure signaling.

### Phase 4: Tests and Integration Validation
- Add unit tests for:
  - normal allocation with non-zero sizes,
  - zero-initialized contents,
  - zero-sized inputs,
  - multiplication overflow detection,
  - large allocation failure handling where practical without environment-sensitive assumptions.
- Run `cargo test`.
- Validate module wiring so callers use the Rust ported function in place of the original C implementation without adding unrelated abstractions.