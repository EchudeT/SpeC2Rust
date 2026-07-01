# Implementation Plan

## Summary

Port `gnu/reallocarray.c` into a focused Rust module that preserves the original allocation semantics relevant to `reallocarray`: resize an allocation based on `nmemb * size` while preventing integer-overflow during size computation and handling allocation failure explicitly.

The Rust implementation should stay minimal and module-scoped. The main technical approach is:

- represent the C routine as a small Rust API in a single corresponding module;
- compute total allocation size with checked multiplication before any allocation attempt;
- use Rust allocation primitives appropriate for raw-memory resizing rather than introducing higher-level containers where they would alter semantics;
- model null/allocation-failure behavior explicitly via return types, keeping ownership and deallocation rules clear.

This port should migrate only the behavior of the existing C file and function, without adding auxiliary facilities beyond what is required for correctness and tests.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::alloc`, `core::ptr`, `core::mem`, `core::num`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve constant-time overflow checking for `nmemb * size`;
  - keep allocation/reallocation overhead equivalent to direct allocator use;
  - avoid extra copying beyond what reallocation inherently requires;
  - do not introduce wrapper layers that materially change memory behavior.

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/reallocarray.c` | `src/module_gnu_reallocarray.rs` | Direct port of the single function in a single Rust module. |
| `reallocarray` | `module_gnu_reallocarray::reallocarray` | Preserve the narrow scope of the original API; implement checked size multiplication and raw reallocation logic. |

If the project already uses a central `mod.rs` or `lib.rs`, this module should be added there with only the required export wiring.

## Data Model

This module has no named C structs or custom data structures to migrate.

### Function-level C to Rust representation

| C Element | Rust Representation | Notes |
|---|---|---|
| `void *` input/output pointer | `*mut u8` (or `*mut c_void` only if required by surrounding project API) | Prefer `*mut u8` internally for allocator operations. |
| `size_t nmemb` | `usize` | Direct width-aligned mapping. |
| `size_t size` | `usize` | Direct width-aligned mapping. |
| computed `nmemb * size` | `usize` via `checked_mul` | Prevent overflow before allocation. |

### Error/Result Model

Because C `reallocarray` reports failure through a null pointer and error state, the Rust port should choose one of the following based on the surrounding crate conventions:

1. **Low-level compatibility shape**: return `*mut u8`, using null on failure.
2. **Rust-internal safer shape**: return `Result<*mut u8, AllocErrorKind>` with a thin compatibility adapter if needed internally.

For a restrained migration, prefer the smallest surface that fits the existing project structure. If no established crate-wide error type exists, a local minimal error enum is acceptable only if needed to avoid ambiguous failure causes between overflow and allocator failure.

## Implementation Phases

### Phase 1: Module Skeleton and API Mapping

- Create `src/module_gnu_reallocarray.rs`.
- Add the module declaration to the crate root if required.
- Define the Rust signature for `reallocarray` based on the expected project-facing API.
- Decide and document the pointer type used internally (`*mut u8` preferred).
- Keep the module limited to the single migrated function and any immediately necessary private helpers.

### Phase 2: Core Reallocation Logic

- Implement checked multiplication with `usize::checked_mul` for `nmemb * size`.
- Map overflow to the chosen failure path immediately, without allocation attempt.
- Implement raw allocation behavior using the standard allocator interfaces.
- Handle the key cases explicitly:
  - null input pointer behaving as fresh allocation;
  - resize of existing allocation;
  - zero-sized total request according to the intended compatibility behavior adopted for the crate;
  - allocator failure propagation.
- Ensure ownership rules are explicit and no double-free or invalid-layout path is introduced.

### Phase 3: Safety Review and Edge-case Validation

- Review all `unsafe` blocks and limit them to:
  - allocator calls;
  - raw pointer handling;
  - layout construction and resize operations.
- Verify layout creation uses valid alignment/size assumptions.
- Confirm behavior for:
  - multiplication overflow;
  - very large but non-overflowing sizes that allocator rejects;
  - null pointer input;
  - existing pointer reallocation path;
  - zero element or zero size inputs.
- Keep comments focused on invariants required for memory safety.

### Phase 4: Tests and Integration Check

- Add unit tests covering:
  - successful small allocation/reallocation;
  - overflow rejection;
  - zero-size edge behavior;
  - null pointer allocation path.
- Run `cargo test`.
- Confirm the module is reachable through the intended crate path and that no extra public API was introduced beyond the migrated function.