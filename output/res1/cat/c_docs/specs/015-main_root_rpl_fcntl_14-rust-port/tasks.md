# Tasks: main_root_rpl_fcntl_14

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for the `fcntl.c` port on branch `015-main_root_rpl_fcntl_14-rust-port`, creating the target source file at `src/fcntl.rs` and wiring the module into the existing crate entry points as needed for this module.
- [T002] [P] [Story] Define the migration surface for `fcntl.c` in `src/fcntl.rs`, documenting the C-to-Rust items to be ported for this module: 1 data structure and 2 functions. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure from `fcntl.c` in `src/fcntl.rs`, preserving the original field intent and Rust visibility needed by the module’s functions. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the first `fcntl.c` function in `src/fcntl.rs`, using the Phase 2 data structure where required and keeping behavior aligned with the original module role. Depends on: T003.
- [T005] [Story] Implement the second `fcntl.c` function in `src/fcntl.rs`, completing the functional port of this module and integrating with the first function where applicable. Depends on: T003.

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/fcntl.rs` for idiomatic Rust within the completed port scope, removing migration scaffolding, tightening signatures and visibility, and resolving any remaining integration issues from the `fcntl.c` port. Depends on: T004, T005.