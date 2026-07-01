# Task List: main_root_xmalloc.c_38

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/xmalloc.rs` and register it from the crate root so the port of `xmalloc.c` has a dedicated implementation location on branch `039-main_root_xmalloc.c_38-rust-port`.
- [T002] [P] [Story] Define the public API surface in `src/xmalloc.rs` for the 6 functions identified from `xmalloc.c`, preserving the C module grouping and establishing placeholders for later implementation. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Add module-level type aliases, constants, and internal helper definitions in `src/xmalloc.rs` that are required to support the `xmalloc.c` function implementations, keeping the port scoped to foundations directly evidenced by the source file. Depends on: T002.

## Phase 3: Allocation Wrappers

- [T004] [Story] Implement the basic allocation wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, covering the functions that perform direct memory allocation and enforce the module’s failure behavior. Depends on: T003.
- [T005] [P] [Story] Implement the resize and zero-initializing allocation wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, grouped with related allocation behavior while avoiding duplication with the basic allocation wrappers. Depends on: T003.

## Phase 4: Error Handling and Remaining Module Functions

- [T006] [Story] Implement the allocation failure handling function from `xmalloc.c` in `src/xmalloc.rs`, including the module-specific termination or reporting behavior required by the allocation wrappers. Depends on: T004, T005.
- [T007] [Story] Implement the remaining support function from `xmalloc.c` in `src/xmalloc.rs` that completes the 6-function module port and integrates it with the allocation/error-handling flow. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/xmalloc.rs` for idiomatic Rust naming, visibility, and intra-module documentation comments while preserving the C-derived API requirements and keeping the implementation tightly scoped to `xmalloc.c`. Depends on: T007.
- [T009] [P] [Story] Review `src/xmalloc.rs` for redundant helper logic and simplify internal control flow so the migrated module remains minimal and consistent with the original `xmalloc.c` responsibilities. Depends on: T008.