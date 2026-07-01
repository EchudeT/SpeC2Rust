# Tasks: main_root Rust port

**Input**: C module analysis for `yank.c`
**Branch**: `001-main_root-rust-port`

## Phase 1: Setup

- [ ] T001 [Story] Initialize the Rust entry module for the `main_root` port in `src/main.rs`, creating the migration target for logic currently held in `yank.c`.
- [ ] T002 [Story] Establish the internal module layout in `src/main.rs` for the `main_root` port so later data structures and function groups can be added without changing target file placement.

## Phase 2: Foundational

- [ ] T003 [Story] Define the Rust representations for the module’s 15 data structures in `src/main.rs`, preserving the C module’s main-program ownership and layout intent where applicable. Depends on: T001, T002
- [ ] T004 [P] [Story] Add shared type aliases, constants, and simple helper enums required by the migrated `main_root` data structures in `src/main.rs`. Depends on: T003
- [ ] T005 [Story] Implement constructors/default initialization paths for the foundational `main_root` data structures in `src/main.rs` where needed to support direct function migration from `yank.c`. Depends on: T003, T004

## Phase 3: Startup and argument flow

- [ ] T006 [Story] Port the program startup and top-level control-flow functions from `yank.c` into `src/main.rs`, including the Rust `main` entry and immediate initialization path. Depends on: T003, T005
- [ ] T007 [P] [Story] Port command-line argument intake and top-level option dispatch functions from `yank.c` into `src/main.rs`. Depends on: T006
- [ ] T008 [Story] Wire startup, argument handling, and foundational state together in `src/main.rs` so the migrated main-path functions operate on the Rust data structures. Depends on: T006, T007

## Phase 4: Core operational functions

- [ ] T009 [Story] Port the core operational function group from `yank.c` into `src/main.rs`, covering the main module behavior after startup using the established Rust data structures. Depends on: T008
- [ ] T010 [P] [Story] Port closely related state-update and utility functions used by the main operational path from `yank.c` into `src/main.rs`. Depends on: T003, T005
- [ ] T011 [Story] Integrate the migrated operational and utility functions in `src/main.rs`, removing placeholder paths and ensuring each migrated function is connected exactly once. Depends on: T009, T010

## Phase 5: Remaining function group

- [ ] T012 [Story] Port the remaining supporting functions from `yank.c` into `src/main.rs`, grouping any residual initialization, cleanup, or local helpers by direct call relationship. Depends on: T011
- [ ] T013 [Story] Replace outstanding stubs in `src/main.rs` with the final migrated implementations for all 13 functions from `yank.c`. Depends on: T012

## Final Phase: Polish

- [ ] T014 [Story] Refine `src/main.rs` for idiomatic Rust ownership, error propagation, and removal of C-style migration scaffolding while preserving the behavior of the `main_root` port. Depends on: T013
- [ ] T015 [Story] Perform a final pass on `src/main.rs` to simplify duplicated migration logic, tighten module-local visibility, and ensure the file is ready for continued integration on branch `001-main_root-rust-port`. Depends on: T014