# plan.md

## Summary

This module ports the allocation wrapper logic from `gnu/malloc.c` into a focused Rust module that preserves the existing module boundary and behavior scope without adding new facilities. The C side exposes a single function, `rpl_malloc`, whose role is to provide allocation behavior with explicit handling around edge cases that are traditionally relevant in C allocation APIs.

The Rust implementation should mirror that narrow responsibility by introducing a small module that centralizes this allocation compatibility logic and adapts it to Rust’s ownership and safety model. Because Rust does not expose raw heap allocation through a direct equivalent of `malloc` for normal application code, the implementation should choose the smallest Rust representation that preserves the caller-visible contract needed by the rest of the port. If the surrounding port still operates on raw pointers, use standard-library allocation primitives from `std::alloc`; if call sites can be migrated simultaneously, prefer returning owned buffers such as `Vec<u8>` only where that exactly matches the migrated usage. The implementation should not generalize beyond the existing function.

The technical approach is therefore:
- port only `rpl_malloc`;
- keep the module thin and allocation-focused;
- map C allocation/error semantics explicitly into Rust results or controlled failure behavior, according to how the surrounding port layer expects allocation failure to be represented;
- avoid introducing extra helper subsystems beyond what is needed to migrate the existing function and its call sites.

## Technical Context

- **Language/Version**: Rust 1.81 or current stable compatible with the repository baseline
- **Primary Dependencies**: Rust standard library only (`std`, especially `std::alloc` and core pointer/size handling as needed)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time wrapper overhead relative to direct allocation.
  - Avoid extra copies or intermediate buffers.
  - Match C-side allocation size handling closely, especially for zero-sized and boundary allocations.
  - Keep generated code simple enough that the wrapper is inlinable where appropriate.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `gnu/malloc.c` | `src/gnu/malloc.rs` | Direct module migration for allocation wrapper logic. |
| `rpl_malloc` | `pub(crate)` or module-scoped function `rpl_malloc` in `src/gnu/malloc.rs` | Visibility should be no broader than required by migrated callers. |

If the crate already has a `gnu` module tree, this file should be added to that existing structure. If not, create only the minimum standard declarations needed:
- `src/gnu/mod.rs`
- `src/gnu/malloc.rs`

No additional abstraction layers should be introduced unless required by call-site compatibility.

## Data Model

This module has no declared C structs or persistent data structures to port.

### C-to-Rust Type Mapping Used by the Function

| C Type / Concept | Rust Mapping | Notes |
|---|---|---|
| `size_t` | `usize` | Direct size mapping. |
| `void *` return | `*mut u8` or another raw pointer type required by migrated callers | Use raw pointers only if the surrounding port still depends on C-like allocation ownership. |
| allocation failure | `Result<_, AllocError>` or process-fatal behavior, depending on required compatibility | Final choice must align with existing caller expectations in the port. |
| zero-size allocation edge case | explicit branch in Rust | Must preserve the intended behavior from the C wrapper rather than relying on unspecified assumptions. |

Because there are no named structs, the main data-model concern is preserving allocation semantics:
- integer size input remains `usize`;
- returned memory ownership must be clearly documented at the Rust call boundary;
- deallocation strategy must remain consistent with whichever allocator API is used.

## Implementation Phases

### Phase 1: Establish module skeleton and function contract

- Create the Rust file mapping for `gnu/malloc.c` under the standard crate source tree.
- Add the `rpl_malloc` function with the exact migrated responsibility and no additional exported API.
- Determine the narrowest viable Rust signature based on existing or imminent callers:
  - raw-pointer signature if preserving C-like interfaces is necessary;
  - otherwise a strictly equivalent owned allocation type only if all known call sites permit that simplification.
- Document the ownership and failure contract at the function boundary.
- Encode explicit handling for size-related edge cases rather than leaving them implicit.

### Phase 2: Implement allocation logic and error semantics

- Port the body of `rpl_malloc` using Rust standard allocation primitives only.
- Reproduce the C wrapper’s decision points around:
  - zero-byte requests;
  - overflow-sensitive size handling where relevant;
  - allocation failure behavior.
- Keep unsafe code tightly scoped if raw allocation is required.
- Ensure any unsafe block states:
  - which allocator primitive is used;
  - what layout assumptions are made;
  - what ownership the caller receives.
- Avoid adding custom allocators, wrapper types, or recovery layers not present in the source scope.

### Phase 3: Migrate and verify call-site compatibility

- Update all call sites that reference the original `rpl_malloc` behavior to use the Rust module path and signature.
- Confirm that allocation and deallocation expectations remain paired correctly after migration.
- Add focused tests for:
  - non-zero successful allocation path;
  - zero-size request behavior;
  - boundary-size handling that is practical to validate in unit tests.
- Run `cargo test` and resolve any signature or ownership mismatches revealed by integration with surrounding migrated code.

### Phase 4: Final cleanup and safety review

- Minimize function visibility to the smallest required scope.
- Review the implementation for unnecessary unsafe surface area and reduce it where possible without changing semantics.
- Confirm the module contains only the migrated functionality from `gnu/malloc.c`.
- Remove temporary migration comments or compatibility shims that are no longer needed once callers are aligned.