# Implementation Plan

## Summary

Port `gnu/realloc.c` into a Rust module that preserves the original module boundary and behavior of `rpl_realloc` without introducing broader allocation utilities or extra abstraction layers. The Rust implementation should focus on reproducing the realloc-specific semantics, especially around size handling, null-pointer behavior, allocation failure signaling, and safe ownership boundaries where possible.

The preferred technical approach is to implement a narrowly scoped Rust module that exposes a function corresponding to `rpl_realloc`, using Rust’s allocation primitives from the standard library. Because the source module operates at raw-memory level, the Rust port will likely need a small amount of `unsafe` code to represent C-compatible reallocation behavior faithfully. That unsafe surface should be kept local to the migrated function and documented in code with explicit invariants.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain allocation behavior close to the original C implementation.
  - Avoid unnecessary copying beyond what reallocation requires.
  - Keep overhead limited to minimal checks needed for Rust-side safety and semantic parity.
  - Preserve constant-time decision paths around null and zero-size cases where applicable.

## Module Mapping

| C Source File | Rust Target File | Notes |
| --- | --- | --- |
| `gnu/realloc.c` | `src/gnu/realloc.rs` | Direct port of module logic with the same narrow responsibility. |
| `gnu/realloc.c` | `src/gnu/mod.rs` | Module declaration only, if the project already groups GNU replacements under a `gnu` namespace. |

### Function Mapping

| C Function | Rust Function | Notes |
| --- | --- | --- |
| `rpl_realloc` | `rpl_realloc` | Keep the name to preserve traceability during migration; implement with raw-pointer-compatible semantics if required by surrounding code. |

## Data Model

This module contains no standalone C structs or persistent data structures.

### Data-Structure Mapping

| C Type | Rust Mapping | Notes |
| --- | --- | --- |
| `void *` | `*mut u8` or `*mut core::ffi::c_void` | Choose based on surrounding crate conventions; prefer `*mut c_void` at external/raw interface boundaries. |
| `size_t` | `usize` | Direct Rust mapping for allocation sizes. |

## Implementation Phases

### Phase 1: Module Scaffold and Interface Definition

- Create `src/gnu/realloc.rs`.
- Add the module declaration in `src/gnu/mod.rs` only if this namespace does not already exist.
- Define the Rust signature for `rpl_realloc` based on how the surrounding port represents C allocation functions.
- Decide the raw pointer interface type:
  - use `*mut core::ffi::c_void` if matching C-style APIs across the port,
  - or `*mut u8` internally with conversion at the boundary.
- Document expected invariants for callers, especially:
  - pointer provenance,
  - whether the pointer must come from the same allocator,
  - behavior for null input,
  - behavior for zero-size requests.

### Phase 2: Core Reallocation Port

- Port the logic of `rpl_realloc` directly from `gnu/realloc.c` into Rust.
- Implement the exact decision flow from the C source rather than redesigning behavior.
- Use standard-library allocation facilities appropriate for realloc-like behavior.
- Keep `unsafe` code limited to:
  - raw pointer handling,
  - allocation and reallocation operations,
  - copying/moving memory if required by the chosen implementation path.
- Ensure the Rust version handles:
  - null pointer input as allocation,
  - resizing existing allocations,
  - zero-size behavior according to the original module semantics,
  - failure reporting in the same observable way expected by the project.
- Avoid introducing helper subsystems unless required to complete the direct port.

### Phase 3: Error Handling and Memory Semantics Validation

- Verify how the original function signals allocation failure and mirror that behavior in Rust.
- Ensure no Rust panic-based allocation behavior leaks into this API if the original C contract expects null returns or another explicit failure form.
- Confirm that memory is neither double-freed nor reallocated through mismatched allocator paths.
- Review integer conversions and size calculations to ensure:
  - no silent truncation,
  - no overflow in layout or resize computations,
  - zero-size handling remains intentional and explicit.

### Phase 4: Tests and Migration Verification

- Add unit tests in the module or crate test layout covering:
  - allocation from null pointer,
  - growth of an existing allocation,
  - shrink of an existing allocation,
  - zero-size request behavior,
  - failure-path expectations where they can be tested deterministically.
- If surrounding migrated allocation modules already exist, add focused compatibility tests to confirm consistent behavior across the GNU replacement set.
- Run `cargo test` and fix any signature or semantic mismatches discovered during integration.