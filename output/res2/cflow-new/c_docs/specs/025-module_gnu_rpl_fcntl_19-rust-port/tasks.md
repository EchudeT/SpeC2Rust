# Tasks: module_gnu_rpl_fcntl_19

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/fcntl.c` port in `src/module_gnu_rpl_fcntl_19.rs`, and expose it from the existing crate entry point if needed for compilation on branch `025-module_gnu_rpl_fcntl_19-rust-port`.
- [T002] [Story] Review `gnu/fcntl.c` and map its 1 data structure and 2 functions into Rust items to be implemented in `src/module_gnu_rpl_fcntl_19.rs`, documenting the intended item names and grouping within the module file. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the module’s foundational data structure port from `gnu/fcntl.c` in `src/module_gnu_rpl_fcntl_19.rs`, preserving the C layout semantics needed by the upcoming function ports. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the first fcntl-related function port from `gnu/fcntl.c` in `src/module_gnu_rpl_fcntl_19.rs`, using the Phase 2 data structure where required and preserving the original module behavior. Depends on: T003
- [T005] [Story] Implement the second fcntl-related function port from `gnu/fcntl.c` in `src/module_gnu_rpl_fcntl_19.rs`, completing the function migration for this module in the same file. Depends on: T003
- [T006] [P] [Story] Reconcile shared constants, helper logic, and call patterns used by both function ports inside `src/module_gnu_rpl_fcntl_19.rs` so the two migrated functions form a consistent module-level implementation without duplicating internal logic. Depends on: T004, T005

## Final Phase: Polish

- [T007] [Story] Perform a module-level polish pass on `src/module_gnu_rpl_fcntl_19.rs` to remove migration leftovers, tighten Rust idioms without changing behavior, and verify the file remains aligned with the original `gnu/fcntl.c` scope. Depends on: T006