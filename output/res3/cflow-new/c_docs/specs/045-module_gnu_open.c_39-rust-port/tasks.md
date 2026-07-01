# Tasks: module_gnu_open.c_39 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/open.c` migration on branch `045-module_gnu_open.c_39-rust-port`, adding the target source file at `src/gnu/open.rs`.
- [T002] [Story] Wire the new module into the crate module tree so `src/gnu/open.rs` is compiled and reachable from the existing Rust project entry points. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Identify and define the 2 data structures required by `gnu/open.c` in `src/gnu/open.rs`, preserving only the fields and visibility needed by the migrated module logic. Depends on: T002
- [T004] [P] [Story] Add foundational constructors, defaults, or internal helper methods for the data structures in `src/gnu/open.rs` when directly required to support the migrated function implementation. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Port the single function from `gnu/open.c` into idiomatic Rust in `src/gnu/open.rs`, mapping its control flow and interactions onto the Rust data structures defined for this module. Depends on: T003, T004
- [T006] [P] [Story] Integrate any function-local constants, internal helper logic, or module-private adaptations needed by the migrated `gnu/open.c` function into `src/gnu/open.rs` without expanding beyond the original module scope. Depends on: T005

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/open.rs` for Rust idioms and compile cleanliness, removing migration-only rough edges while preserving the original `gnu/open.c` behavior. Depends on: T005, T006
- [T008] [Story] Review the module boundary and exported items for `src/gnu/open.rs` to ensure only the interfaces required by the migrated `gnu/open.c` module remain exposed. Depends on: T007