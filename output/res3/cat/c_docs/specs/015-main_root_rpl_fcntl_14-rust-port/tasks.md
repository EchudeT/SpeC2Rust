# Tasks: main_root_rpl_fcntl_14

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/fcntl.rs` for the `fcntl.c` migration target and declare its integration from the crate root so the `main_root_rpl_fcntl_14` port has a dedicated implementation location.
- [T002] [Story] Define the public migration surface in `src/fcntl.rs` for the data structure and the 2 functions identified from `fcntl.c`, using Rust signatures and visibility consistent with the module’s role in the main cluster.

## Phase 2: Foundational

- [T003] [Story] Implement the single data structure identified from `fcntl.c` in `src/fcntl.rs`, preserving the C module’s field layout and intent as closely as Rust allows for later function use.
- [T004] [P] [Story] Add supporting constants, type aliases, or internal helper definitions in `src/fcntl.rs` that are directly required by the migrated data structure and function signatures. Depends on: T002, T003

## Phase 3: Functions

- [T005] [Story] Implement the first `fcntl.c` function in `src/fcntl.rs`, wiring it to the migrated data structure and any directly related helper definitions. Depends on: T003, T004
- [T006] [Story] Implement the second `fcntl.c` function in `src/fcntl.rs`, completing the functional migration of `main_root_rpl_fcntl_14` within the Rust module. Depends on: T003, T004
- [T007] [P] [Story] Reconcile shared logic between the two migrated functions in `src/fcntl.rs` so duplicated C-port behavior is kept in local internal helpers only where directly needed. Depends on: T005, T006

## Final Phase: Polish

- [T008] [Story] Review `src/fcntl.rs` for idiomatic Rust adjustments that do not change the migrated `fcntl.c` behavior, including visibility cleanup, error-path consistency, and removal of unnecessary temporary porting scaffolding. Depends on: T007