# Tasks: module_gnu_fcntl.c_27

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/fcntl.rs` and register it from the existing Rust module tree so `gnu/fcntl.c` has a direct port target in Rust.
- [T002] [P] [Story] Review `gnu/fcntl.c` and map the single exported function and single supporting data structure into `src/gnu/fcntl.rs`, documenting the intended Rust item names as implementation placeholders before coding.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single data structure in `src/gnu/fcntl.rs`, translating the C layout and fields needed by the `gnu/fcntl.c` port. Depends on: T001, T002.

## Phase 3: Functions

- [T004] [Story] Implement the single function from `gnu/fcntl.c` in `src/gnu/fcntl.rs`, wiring it to the translated data structure and preserving the C module’s behavior within the Rust port. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/gnu/fcntl.rs` by resolving compile issues, tightening item visibility to the minimum required by the module tree, and cleaning up the port so the migrated data structure and function integrate cleanly on branch `033-module_gnu_fcntl.c_27-rust-port`. Depends on: T004.