# Implementation Plan

## Summary

Port `gnu/calloc.c` into a focused Rust module that preserves the existing module boundary and behavior of `rpl_calloc` without introducing broader allocator abstractions or unrelated utilities.

The Rust implementation should center on a single allocation helper that:
- computes `nmemb * size` with explicit overflow checking,
- performs zero-initialized allocation using Rust’s allocator interfaces,
- returns an allocation result in a form appropriate to the surrounding Rust project API,
- keeps error handling explicit rather than relying on unchecked arithmetic or implicit null handling.

Because the source module contains one function and no custom data structures, the migration should remain minimal: one Rust source file, one function-level port, and tests covering overflow, zero-sized requests, and successful zeroed allocation.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::alloc`, `std::ptr`, `std::mem`, standard result/error types)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the constant-time overflow check before allocation.
  - Use zeroed allocation directly rather than allocating and manually clearing.
  - Avoid extra heap copies or wrapper layers beyond what is required to represent the allocation safely.
  - Match the original module’s lightweight role as a thin checked allocation primitive.

## Module Mapping

| C File | C Function | Rust Target |
|---|---|---|
| `gnu/calloc.c` | `rpl_calloc` | `src/module_gnu_calloc.rs::rpl_calloc` |

### Proposed Rust File Layout

```text
src/
  module_gnu_calloc.rs
```

If the branch already has a central module registry, add only the minimal `mod` declaration and re-export needed to make this port callable from existing migrated code.

## Data Model

This module defines no C structs or persistent data containers.

### Data Mapping

| C Element | Rust Mapping |
|---|---|
| `size_t` | `usize` |
| allocated memory block | zero-initialized heap allocation represented as raw pointer or narrow Rust-owned allocation type, depending on existing project conventions |
| `NULL` on allocation failure | `None` or `Result<_, AllocError-like project error>` if the surrounding codebase already prefers result-based propagation |

### Memory Management Notes

- The core semantic requirement is checked multiplication before allocation.
- Zero-length allocation behavior must be decided once and documented in the function contract according to project usage:
  - either preserve C-like allocator semantics with a non-dereferenceable but valid representation,
  - or use an empty Rust allocation/container if the caller model is already Rust-native.
- If the function must remain pointer-oriented to match nearby migrated code, use `std::alloc::alloc_zeroed` with a `Layout` built from the checked total size.
- Any raw allocation path must also define the matching deallocation responsibility in code comments and tests, even if deallocation occurs elsewhere.

## Implementation Phases

### Phase 1: Module Scaffold and Signature Port

- Create `src/module_gnu_calloc.rs`.
- Add the Rust equivalent of `rpl_calloc` with a signature chosen to match the current migration style of the project:
  - prefer `usize` parameters for `nmemb` and `size`,
  - use the narrowest return type compatible with existing callers.
- Add the minimal module declaration/export wiring required by the crate.
- Document the allocation ownership expectations at the function boundary.

### Phase 2: Allocation Logic Migration

- Implement checked multiplication for `nmemb * size` using `checked_mul`.
- Map overflow to the chosen failure path immediately, without attempting allocation.
- Build the allocation using Rust standard allocation primitives:
  - construct `Layout` from the computed size and alignment appropriate to the stored representation,
  - allocate zeroed memory directly.
- Handle allocation failure explicitly.
- Keep the implementation local to this module; do not introduce allocator helper subsystems.

### Phase 3: Edge Cases and Safety Tightening

- Verify behavior for:
  - `nmemb == 0`,
  - `size == 0`,
  - multiplication overflow,
  - small successful allocations.
- Ensure any unsafe block is limited to the allocator call and pointer handling.
- Add concise safety comments explaining:
  - why the layout is valid,
  - why overflow is impossible after the checked multiplication,
  - what ownership the caller receives.

### Phase 4: Tests and Final Integration

- Add unit tests in the module or adjacent test module for:
  - overflow rejection,
  - zeroed contents after successful allocation,
  - zero-sized request behavior,
  - nonzero successful allocation path.
- Run `cargo test`.
- Adjust only the immediate call sites or exports needed to compile this port on branch `028-module_gnu_calloc.c_22-rust-port`.