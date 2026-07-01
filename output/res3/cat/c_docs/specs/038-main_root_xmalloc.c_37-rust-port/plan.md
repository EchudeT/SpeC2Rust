# Implementation Plan: main_root_xmalloc.c_37

## Summary

This module ports the allocation helper logic from `xmalloc.c` into Rust, preserving the existing allocation-oriented API surface and migration order without adding new capabilities. The Rust implementation should focus on reproducing the C module’s semantics around allocation sizing, reallocation growth, zero-initialization, and overflow-aware size computations.

The technical approach is to implement a single Rust module that provides direct equivalents for the C functions, using standard-library allocation primitives over raw ownership-neutral memory containers where possible. Since the source module is centered on heap allocation helpers rather than domain objects, the Rust port should avoid introducing extra abstraction layers and instead map each C function to a narrowly scoped Rust function with explicit size checks and consistent failure behavior.

Special care is required for:
- integer overflow during size multiplication and growth calculations,
- differences between C’s null-return / fatal-allocation conventions and Rust’s panic / fallible allocation model,
- preserving the intended behavior of “x*alloc” functions that conceptually never return allocation failure to callers.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep allocation and reallocation paths close to standard-library cost.
  - Avoid unnecessary intermediate buffers or wrapper types.
  - Preserve amortized growth behavior for resize helpers.
  - Ensure overflow checks are constant-time and do not add meaningful overhead beyond required safety validation.

## Module Mapping

### Source to Target

- `xmalloc.c` → `src/main_root_xmalloc_c_37.rs`

If the project already uses a central module tree, expose it with the minimal corresponding entry in:
- `src/lib.rs` or existing parent module file → `pub mod main_root_xmalloc_c_37;`

### Function Mapping

Each C function should map to a direct Rust function in the same Rust module, keeping names as close as practical to the source for migration traceability.

- `_GL_ATTRIBUTE_PURE`
  - No direct Rust implementation.
  - Treat as a removed C-only annotation; preserve intent by keeping helper functions side-effect free where applicable.

- `xmalloc`
  - Rust function allocating an uninitialized or byte-oriented buffer of requested size.
  - Implementation should define a clear internal representation for returned memory, based on how the surrounding port consumes it.

- `ximalloc`
  - Rust wrapper variant for size values corresponding to C integer-indexed allocation conventions.
  - Implement as a thin checked conversion / delegation layer.

- `xcharalloc`
  - Rust byte buffer allocation helper.
  - Prefer direct `u8`-based allocation semantics.

- `xrealloc`
  - Rust reallocation helper preserving existing contents up to the new size.
  - Implement through owned buffer growth/shrink behavior with explicit size validation.

- `xirealloc`
  - Rust checked-conversion wrapper around `xrealloc`.

- `xreallocarray`
  - Rust helper performing `count * element_size` checked multiplication before reallocation.

- `xireallocarray`
  - Rust checked-conversion wrapper around array reallocation.

- `xnmalloc`
  - Rust helper allocating `count * element_size` bytes with checked multiplication.

- `xinmalloc`
  - Rust integer-sized wrapper around `xnmalloc`.

- `x2realloc`
  - Rust growth helper that reallocates according to existing doubling-style semantics.

- `x2nrealloc`
  - Rust helper for count-based growth with checked element-size multiplication and updated capacity/count behavior.

- `xpalloc`
  - Rust capacity-growth helper matching the source module’s sizing policy, upper/lower bounds, and overflow checks as closely as possible.

- `xzalloc`
  - Rust zero-initialized allocation helper.

- `xizalloc`
  - Rust checked-conversion wrapper around zero-initialized allocation.

## Data Model

This module has no declared C structs in the provided analysis and should not introduce new public data structures unless required to express exact function behavior in Rust.

### Data-structure Mapping

- **C raw allocated memory (`void *`, `char *`)**
  - **Rust mapping**: internal byte-oriented owned allocation, preferably `Vec<u8>` or `Box<[u8]>`, depending on whether resizing behavior is required by the specific function.
  - Use `Vec<u8>` for helpers involving reallocation/growth.
  - Use `Box<[u8]>` only if a fixed-size result is required by the consuming code.

- **C size values (`size_t`, integer counts)**
  - **Rust mapping**: `usize`
  - For “i”-prefixed wrappers, perform explicit checked conversion from source integer type to `usize` if needed by surrounding code.

- **C in/out count parameters used by growth helpers**
  - **Rust mapping**: `&mut usize` where the original logic updates element counts/capacity alongside allocation.

### Error and Failure Model

Because the C allocation helpers typically centralize failure handling rather than propagating ordinary allocation errors, the Rust port should use one consistent internal strategy:
- perform explicit checked arithmetic before allocation,
- abort via panic with a narrow, allocation-specific message when invariants fail or allocation cannot be satisfied, if the larger port expects non-recovering semantics,
- avoid introducing `Result`-based public APIs unless the surrounding migrated call sites already require them.

This keeps behavior aligned with the source module’s role as a fatal-on-failure allocator layer.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and base allocation helpers

### Goals
- Establish the target Rust file and module export.
- Implement the simplest allocation entry points first.
- Set the module-wide approach for size checking and failure behavior.

### Tasks
- Create `src/main_root_xmalloc_c_37.rs`.
- Add minimal module exposure from the existing crate root or parent module file.
- Implement internal checked helpers for:
  - converting incoming size-like values to `usize`,
  - validating multiplication for byte counts,
  - validating growth computations before allocation.
- Port the simplest allocation functions:
  - `xmalloc`
  - `ximalloc`
  - `xcharalloc`
  - `xzalloc`
  - `xizalloc`
- Remove `_GL_ATTRIBUTE_PURE` as a C-only annotation and preserve only its practical meaning where useful.

### Notes
- Keep helper visibility restricted unless a function must remain externally callable.
- Do not add allocator traits, custom error enums, or generic allocation frameworks.

## Phase 2: Port reallocation and array-sized allocation helpers

### Goals
- Migrate functions whose primary complexity is checked size arithmetic and buffer resizing.
- Keep the implementation close to the source function boundaries.

### Tasks
- Implement:
  - `xrealloc`
  - `xirealloc`
  - `xnmalloc`
  - `xinmalloc`
  - `xreallocarray`
  - `xireallocarray`
- Ensure all `count * size` operations use checked multiplication before allocation or resize.
- Preserve zero-size edge behavior according to the source semantics used by the wider project.
- Add focused unit tests for:
  - simple allocation sizes,
  - reallocation preserving prior contents,
  - array-size overflow detection,
  - zero-size inputs where applicable.

### Notes
- Prefer reusing a small set of internal checked-size helpers rather than duplicating arithmetic logic.
- Keep public signatures as close as possible to what downstream migrated code needs.

## Phase 3: Port growth-policy helpers

### Goals
- Implement the functions that encode the module’s capacity expansion policy.
- Match source growth rules without embellishment.

### Tasks
- Implement:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- Translate the original growth calculations carefully, including:
  - minimum growth behavior,
  - doubling or near-doubling rules,
  - maximum-size clamping,
  - overflow detection before byte-size computation.
- Use mutable count/capacity parameters where the C API updates caller-visible size state.
- Add tests covering:
  - first-growth from empty state,
  - repeated growth,
  - boundary conditions near `usize::MAX`,
  - count updates after successful growth.

### Notes
- Do not substitute Rust container auto-growth heuristics for the source logic where these helpers explicitly define capacity policy.
- Keep the implementation arithmetic transparent and directly traceable to the C code.

## Phase 4: Validation and call-site alignment

### Goals
- Confirm the module is usable by the rest of the Rust port without broad redesign.
- Finalize behavior consistency and test coverage.

### Tasks
- Review migrated function signatures against expected call patterns in the branch.
- Adjust only the minimum necessary type details to fit existing consumers.
- Add unit tests for cross-function consistency, such as:
  - `xnmalloc` and `xreallocarray` using identical overflow rules,
  - `xzalloc` producing zeroed memory,
  - growth helpers returning sizes compatible with subsequent realloc helpers.
- Run `cargo test` and resolve any behavior mismatches discovered during integration.

### Completion Criteria
- All functions listed from `xmalloc.c` have Rust counterparts or documented no-op annotation handling.
- Allocation, reallocation, zero-fill, and growth logic are covered by tests.
- The module remains a direct migration unit with no extra supporting subsystems added.