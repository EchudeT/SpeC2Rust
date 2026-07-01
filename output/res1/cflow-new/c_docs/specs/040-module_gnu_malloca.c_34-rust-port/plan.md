# Implementation Plan: module_gnu_malloca.c_34

## Summary

This module ports the allocation helpers from `gnu/malloca.c` into Rust, covering the behaviors currently exposed through `mmalloca` and `freea`. The Rust implementation should preserve the existing allocation/freeing contract while making ownership boundaries explicit and minimizing unsafe code.

The implementation approach is to translate the C allocation strategy into a small Rust module that:
- keeps the logic localized to the migrated file scope,
- uses standard-library allocation primitives where possible,
- isolates any required raw-pointer handling behind narrow internal helpers,
- preserves the paired usage model of allocation through `mmalloca` and release through `freea`.

Because the original C module manages memory manually and may distinguish release behavior based on how memory was obtained, the Rust port should focus on faithfully reproducing that control flow rather than redesigning the API around broader abstractions.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on current module evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the C module’s operational complexity for allocation and release paths
  - Avoid unnecessary copying or intermediate buffers
  - Keep per-call overhead limited to the minimum metadata handling needed to preserve `freea` behavior
  - Maintain predictable behavior for small and large allocation requests within the constraints of Rust’s allocator APIs

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/malloca.c` | `src/module_gnu_malloca.rs` | Single-module migration of the existing allocation helpers |
| `mmalloca` | `module_gnu_malloca::mmalloca` | Port as the primary allocation entry point; retain low-level semantics |
| `freea` | `module_gnu_malloca::freea` | Port as the corresponding release entry point for memory returned by `mmalloca` |

If the project already has a module tree, expose this file through the existing `mod` declarations only as needed to match current call sites. Do not introduce additional submodules beyond the migrated file.

## Data Model

This C module analysis does not list any named structs, so the Rust port should avoid inventing public data types unless they are strictly required to preserve internal allocation metadata.

### Data-structure mapping

| C Construct | Rust Mapping | Notes |
|---|---|---|
| Raw allocated memory region | `*mut u8` or `NonNull<u8>` internally | Use raw pointers internally to match the C contract |
| Allocation size / offset metadata | Internal header layout or equivalent private bookkeeping | Keep private to the module; only include fields required for correct `freea` behavior |
| No named public struct in source analysis | No public struct required | Prefer function-level migration over introducing new public abstractions |

### Memory Management Notes

- If the C implementation stores a marker or header adjacent to returned memory, mirror that layout privately in Rust using a clearly bounded unsafe block.
- If stack-vs-heap distinction exists in the original logic, encode only the minimum metadata needed to let `freea` decide whether deallocation is required.
- Keep ownership external API compatible with the source behavior: memory returned by `mmalloca` is not automatically reclaimed by Rust ownership and must be handled through the module’s paired release path.
- Validate zero-size and overflow-sensitive size calculations explicitly before allocator calls.

## Implementation Phases

### Phase 1: Module Skeleton and API Port

- Create `src/module_gnu_malloca.rs`.
- Add Rust equivalents for:
  - `mmalloca`
  - `freea`
- Wire the module into the crate using standard Rust module declarations.
- Preserve the existing function-oriented interface rather than introducing broader allocator types.
- Define internal constants and private helper functions only where they directly support the migrated functions.

### Phase 2: Allocation Semantics and Unsafe Boundary

- Port the allocation path in `mmalloca` using standard-library allocation facilities.
- Recreate any required in-band metadata layout needed by `freea`.
- Constrain unsafe operations to:
  - raw allocation/deallocation,
  - pointer arithmetic,
  - header read/write adjacent to returned memory.
- Add explicit checks for:
  - zero-length edge cases,
  - size addition/multiplication overflow,
  - null-equivalent allocation failure behavior as required by current call semantics.
- Ensure `freea` only deallocates memory that was allocated through the corresponding heap path and ignores/no-ops where the original C logic does so.

### Phase 3: Behavioral Validation Tests

- Add unit tests covering the migrated file’s expected usage patterns:
  - successful allocation and paired release,
  - repeated allocate/free cycles,
  - edge-size requests,
  - behavior of `freea` against pointers representing each supported allocation path.
- Verify that pointer alignment and header handling are correct for the chosen internal layout.
- Confirm no double-free is introduced by the translated control flow assumptions.

### Phase 4: Integration Cleanup

- Update existing call sites, if needed, to import the new Rust module path without changing module scope beyond this migration.
- Remove any temporary migration scaffolding used during implementation.
- Run `cargo test` and fix any API mismatches caused by the port.
- Keep final code limited to the functionality present in `gnu/malloca.c`, without adding extra allocation utilities or wrappers.