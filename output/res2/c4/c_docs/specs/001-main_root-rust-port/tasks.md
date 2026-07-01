# Tasks: main_root Rust port

## Phase 1: Setup

- [T001] [Story] Initialize the Rust binary entry module for the `main_root` port on branch `001-main_root-rust-port`, creating and wiring `src/main.rs` as the target for functionality migrated from `c4.c` and `hello.c`.
- [T002] [P] [Story] Create Rust module stubs inferred from the source split, adding `src/c4.rs` and `src/hello.rs`, and declare them from `src/main.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Establish the shared foundational module layout for the migrated functions by defining the public/private item boundaries in `src/c4.rs` and `src/hello.rs` so later function ports can be added without changing file ownership. Depends on: T002.

## Phase 3: Core main program functions

- [T004] [Story] Port the main program entry and top-level control-flow functions from `c4.c` into `src/main.rs` and `src/c4.rs`, preserving the original execution order and module ownership. Depends on: T003.
- [T005] [P] [Story] Port the output-oriented helper functions from `hello.c` into `src/hello.rs`, keeping the behavior grouped with the source file they came from. Depends on: T003.
- [T006] [Story] Port the remaining internal helper functions from `c4.c` into `src/c4.rs`, grouping closely related logic together and avoiding duplication of responsibilities already migrated in `src/main.rs`. Depends on: T004.
- [T007] [Story] Integrate the cross-module function calls between `src/main.rs`, `src/c4.rs`, and `src/hello.rs` so the complete set of 10 migrated functions compiles and follows the original `c4.c`/`hello.c` call paths. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Refine the Rust port for `src/main.rs`, `src/c4.rs`, and `src/hello.rs` by removing migration-only scaffolding, tightening visibility, and resolving compile-time warnings without changing behavior. Depends on: T007.