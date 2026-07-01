# Tasks: main_root Rust port

**Input**: C module analysis for project `which`, module `main_root`
**Branch**: `001-main_root-rust-port`

## Phase 1: Setup

- [ ] T001 [Story] Initialize the Rust module layout for the `main_root` port by creating `src/main.rs`, `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` to mirror `which.c`, `bash.c`, `getopt.c`, and `getopt1.c`.
- [ ] T002 [Story] Wire the crate entrypoint in `src/main.rs` to declare and import the `which`, `bash`, and `getopt` modules so later function migration can be attached in place. Depends on: T001
- [ ] T003 [P] [Story] Add initial shared type aliases, constants, and placeholder public interfaces inferred from the C module into `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` so data structures and functions can be migrated without reshaping files later. Depends on: T002

## Phase 2: Foundational

- [ ] T004 [Story] Port the core `which.c` data structures and related state holders into Rust in `src/which.rs`, keeping their ownership and mutability model suitable for the main program flow. Depends on: T003
- [ ] T005 [P] [Story] Port the `bash.c` data structures used by shell/path interpretation into Rust in `src/bash.rs`, limited to structures directly required by functions from `bash.c`. Depends on: T003
- [ ] T006 [P] [Story] Port the option-parser data structures and parser state from `getopt.c` and `getopt1.c` into Rust in `src/getopt.rs`, consolidating shared parser representations needed by both source files. Depends on: T003
- [ ] T007 [Story] Reconcile cross-module structure usage by exposing only the required shared types between `src/which.rs`, `src/bash.rs`, and `src/getopt.rs`, keeping call signatures aligned with the original C module boundaries. Depends on: T004, T005, T006

## Phase 3: Option parsing functions

- [ ] T008 [Story] Implement the base short-option parsing functions migrated from `getopt.c` into `src/getopt.rs`, using the foundational parser state already ported. Depends on: T006, T007
- [ ] T009 [Story] Implement the long-option and extended parsing functions migrated from `getopt1.c` into `src/getopt.rs`, reusing the shared parser structures from `getopt.c` without duplicating parser logic. Depends on: T008
- [ ] T010 [Story] Connect the public option-parsing entrypoints in `src/getopt.rs` so `src/which.rs` can consume a single coherent parser surface matching the original main-module behavior. Depends on: T009

## Phase 4: Shell/path support functions

- [ ] T011 [Story] Implement the lower-level shell and quoting helper functions migrated from `bash.c` in `src/bash.rs`, limited to helpers needed by the command-resolution flow. Depends on: T005, T007
- [ ] T012 [Story] Implement the higher-level shell/path interpretation functions migrated from `bash.c` in `src/bash.rs`, building on the helper layer without duplicating path-processing behavior elsewhere. Depends on: T011

## Phase 5: Main program and command resolution functions

- [ ] T013 [Story] Implement the internal command/path lookup helper functions migrated from `which.c` in `src/which.rs`, grouping the non-entrypoint logic that performs command resolution and output preparation. Depends on: T004, T007, T010, T012
- [ ] T014 [Story] Implement the argument-processing and top-level program control functions migrated from `which.c` in `src/which.rs`, wiring them to the Rust option parser and shell/path support modules. Depends on: T013
- [ ] T015 [Story] Implement the executable entrypoint in `src/main.rs` to invoke the migrated main control path from `src/which.rs`. Depends on: T014

## Final Phase: Polish

- [ ] T016 [Story] Refine cross-module signatures, remove migration placeholders, and simplify ownership/borrowing in `src/main.rs`, `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` so the Rust port remains faithful to the original C module layout without redundant compatibility code. Depends on: T015
- [ ] T017 [Story] Perform final cleanup of duplicated constants, internal visibility, and module organization across `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` to leave a cohesive `main_root` Rust implementation. Depends on: T016