# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/xmalloc.c` port in `src/gnu/xmalloc.rs`, and wire it into the crate module tree from the existing `src/gnu/mod.rs` or nearest inferable parent module file for this path.
- [T002] [P] [Story] Establish the initial Rust API surface in `src/gnu/xmalloc.rs` for all functions migrated from `gnu/xmalloc.c`, preserving the C module grouping and naming intent where appropriate for the Rust port.
- [T003] [Story] Review dependencies from `gnu/xmalloc.c` and add only directly required Rust imports, crate-module references, and allocation-related bindings inside `src/gnu/xmalloc.rs`.
  **Depends on:** T001, T002

## Phase 2: Foundational

- [T004] [Story] Define shared foundational items needed by the module in `src/gnu/xmalloc.rs`, including common type aliases, helper signatures, constants, and internal utility boundaries required before implementing the module’s allocation functions.
  **Depends on:** T003
- [T005] [P] [Story] Implement any module-local allocation error handling helper logic in `src/gnu/xmalloc.rs` that is reused across multiple migrated functions, keeping it scoped to behavior evidenced by `gnu/xmalloc.c`.
  **Depends on:** T004

## Phase 3: Core allocation wrappers

- [T006] [Story] Implement the primary allocation wrapper functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, grouping the base memory acquisition behaviors first so later resizing and duplication functions can reuse them.
  **Depends on:** T005
- [T007] [P] [Story] Implement zero-initializing allocation wrapper functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, grouped with the core allocation APIs they are functionally paired with.
  **Depends on:** T006
- [T008] [P] [Story] Implement overflow-checked size computation helpers used by allocation entry points in `src/gnu/xmalloc.rs`, only where directly required by the migrated functions from `gnu/xmalloc.c`.
  **Depends on:** T005

## Phase 4: Resize and reallocation functions

- [T009] [Story] Implement reallocation and resize-oriented functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, using the shared failure-handling and checked-size helpers established earlier.
  **Depends on:** T006, T008
- [T010] [P] [Story] Implement any array-growth or count-by-element reallocation variants from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, grouped together to avoid duplicating size-checking logic.
  **Depends on:** T008, T009

## Phase 5: Duplication helpers

- [T011] [Story] Implement memory-duplication helper functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, reusing the module’s allocation wrappers rather than duplicating raw allocation behavior.
  **Depends on:** T006
- [T012] [P] [Story] Implement string-duplication helper functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, grouped separately from generic memory duplication while sharing the same module-local error path.
  **Depends on:** T006, T011

## Phase 6: Remaining function migration and module completeness

- [T013] [Story] Implement any remaining standalone utility functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs` that do not fit the earlier allocation, reallocation, or duplication groups, ensuring each of the module’s 15 functions is migrated exactly once.
  **Depends on:** T009, T010, T011, T012
- [T014] [Story] Reconcile Rust function signatures and visibility in `src/gnu/xmalloc.rs` with the intended crate usage of the original `gnu/xmalloc.c` module, removing placeholder items introduced during setup.
  **Depends on:** T013

## Final Phase: Polish

- [T015] [Story] Refine `src/gnu/xmalloc.rs` for idiomatic Rust implementation details that preserve the C module behavior, including deduplicating helper paths, simplifying internal control flow, and tightening imports without expanding module scope.
  **Depends on:** T014
- [T016] [Story] Perform a final module pass on `src/gnu/xmalloc.rs` and related module declaration files to confirm the port is self-consistent, compiles within the crate structure, and contains no duplicated or unmigrated function work items.
  **Depends on:** T015