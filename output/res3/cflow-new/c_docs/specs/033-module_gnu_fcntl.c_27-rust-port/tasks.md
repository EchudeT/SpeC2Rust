# Tasks: module_gnu_fcntl.c_27

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/fcntl.rs` and register it from the existing Rust module tree so the port of `gnu/fcntl.c` has a dedicated target location.
- [T002] [P] [Story] Review `gnu/fcntl.c` and map its single exported function and single data structure into Rust items to be implemented in `src/gnu/fcntl.rs`, keeping naming and scope aligned with the source module.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single data structure in `src/gnu/fcntl.rs`, including its fields, visibility, and Rust-native representation required by the `gnu/fcntl.c` port. Depends on: T001, T002.

## Phase 3: Functions

- [T004] [Story] Implement the module’s single function in `src/gnu/fcntl.rs`, using the data structure from this module where required and preserving the behavior of `gnu/fcntl.c`. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/gnu/fcntl.rs` for module-level consistency with the Rust port branch, removing migration scaffolding, tightening signatures and visibility, and ensuring the final file remains focused on the `gnu/fcntl.c` translation. Depends on: T004.