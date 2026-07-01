# Tasks: main_root_xmalloc.c_38

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/xmalloc.rs` and register it from the crate root so the `xmalloc.c` migration has a dedicated target location.
- [T002] [Story] Review the C functions in `xmalloc.c` and map each one to a Rust function stub in `src/xmalloc.rs`, preserving the module-local migration scope for this branch. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust interfaces in `src/xmalloc.rs` needed by the migrated allocation helpers, including shared error-handling and return-type conventions used across all migrated functions. Depends on: T002

## Phase 3: Allocation wrapper functions

- [T004] [P] [Story] Implement the direct allocation wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, covering the core single-allocation helpers as one grouped migration step. Depends on: T003
- [T005] [P] [Story] Implement the reallocation-related helper functions from `xmalloc.c` in `src/xmalloc.rs`, grouping the resize-oriented behavior together. Depends on: T003
- [T006] [Story] Implement the duplication/convenience allocation helper functions from `xmalloc.c` in `src/xmalloc.rs`, completing the remaining function migrations in this module. Depends on: T004, T005

## Final Phase: Polish

- [T007] [Story] Refine `src/xmalloc.rs` to remove migration scaffolding, align naming and visibility with crate conventions, and verify the full `xmalloc.c` function set is completely represented once in Rust. Depends on: T006