# Implementation Plan

## Summary
Port `gnu/realloc.c` into a focused Rust module that preserves the existing module boundary and behavior of `rpl_realloc` without adding new capabilities. The Rust implementation should center on safe ownership where possible, while explicitly handling the C semantics that matter for reallocation: resizing heap-backed memory, preserving existing contents up to the copied range, and producing clear failure behavior for allocation errors and edge cases such as zero-sized requests.

Because the source module contains a single allocation-oriented function, the Rust migration should remain narrow: introduce one Rust module corresponding to `gnu/realloc.c`, implement the replacement reallocation routine there, and keep any low-level memory operations encapsulated in that file. If the surrounding port still depends on C-style pointer-based interfaces, the Rust code should isolate any necessary `unsafe` blocks to the smallest possible scope and document the ownership and validity assumptions around the pointer arguments.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are recommended based on the current module scope
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the practical cost profile of the C implementation for reallocation paths
  - Avoid unnecessary intermediate allocations or copies beyond what resize/move semantics require
  - Keep `unsafe` memory manipulation minimal and localized so correctness is not traded away for speculative optimization

## Module Mapping

| C Source File | Rust Target | Notes |
|---|---|---|
| `gnu/realloc.c` | `src/gnu/realloc.rs` | Direct migration target for `rpl_realloc` |
| `rpl_realloc` | `gnu::realloc::rpl_realloc` | Preserve the function role and call pattern as closely as the Rust port architecture allows |

## Data Model

This module does not define custom C structs in the provided input, so no struct-to-struct migration is required.

### Memory and Type Mapping

| C Type / Concept | Rust Mapping | Notes |
|---|---|---|
| `void *` | `*mut u8` or `*mut core::ffi::c_void` | Use raw pointers only if the surrounding port requires C-like allocation interfaces |
| allocation size parameters (`size_t`) | `usize` | Natural Rust mapping |
| null pointer | `core::ptr::null_mut()` | Preserve explicit null handling where required |
| allocation failure signaling | `Result<_, _>` internally, or null/error-compatible external behavior if required by surrounding code | Prefer internal Rust error handling, but do not invent a broader API than needed |

### Ownership Model
- If `rpl_realloc` must remain pointer-based to match the migrated codebase, treat the incoming pointer as externally owned heap memory with strict preconditions about allocator compatibility.
- If the surrounding Rust port has already lifted callers to owned buffers, prefer expressing reallocation in terms of `Vec<u8>` or boxed slices internally, but only when this does not expand the module contract.
- Any `unsafe` conversion between raw pointers and Rust allocation types must be confined to the implementation boundary and justified by allocator/layout invariants.

## Implementation Phases

### Phase 1: Establish the Rust Module Skeleton
- Create `src/gnu/realloc.rs` as the direct destination for `gnu/realloc.c`.
- Add the module declaration in the existing Rust module tree without introducing extra abstraction layers.
- Define the Rust signature for `rpl_realloc` based on how adjacent migrated modules consume it:
  - use a raw-pointer signature if compatibility with C-style callers is still required;
  - otherwise use the narrowest existing Rust-owned representation already present in the port.
- Record the expected semantics for:
  - null input handling
  - zero-size reallocation behavior
  - allocation failure behavior
  - content preservation guarantees

### Phase 2: Implement Reallocation Logic
- Port the body of `rpl_realloc` into Rust, keeping the logic in a single module-local implementation.
- Use standard library allocation facilities where they can express the required semantics directly.
- If raw memory reallocation is necessary, isolate `unsafe` operations for:
  - pointer validity checks
  - layout computation
  - allocation/reallocation/deallocation calls
  - bounded copy of prior contents
- Ensure the implementation does not leak memory or double-free on failure paths.
- Keep behavior aligned with the original module instead of redesigning the API surface.

### Phase 3: Validate Edge Cases and Failure Semantics
- Add unit tests covering the observable behavior of `rpl_realloc`, including:
  - allocation from a null input
  - growth of an existing allocation
  - shrink of an existing allocation
  - zero-size requests
  - failure-oriented behavior where testable without special infrastructure
- Verify that preserved bytes remain intact after resizing where applicable.
- Confirm that any null/error return path matches the expected contract used by the rest of the port.

### Phase 4: Integrate and Tighten Safety Boundaries
- Wire the module into callers migrated on branch `049-module_gnu_realloc.c_43-rust-port` with the smallest necessary signature adaptations.
- Review all `unsafe` blocks for minimal scope and explicit invariants.
- Remove any temporary compatibility code introduced during migration if it is no longer needed after integration.
- Run `cargo test` and fix any behavioral mismatches revealed by module-level or integration-level tests.