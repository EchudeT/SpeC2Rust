# Tasks: module_gnu_rpl_fcntl_19

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/fcntl.c` port on branch `025-module_gnu_rpl_fcntl_19-rust-port`, adding the target source file at `src/gnu/fcntl.rs`.
- [T002] [P] [Story] Wire the new module file into the Rust crate module tree so `src/gnu/fcntl.rs` is compiled and reachable from the existing `src/gnu/mod.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and port the single data structure evidenced in `gnu/fcntl.c` into Rust, defining its Rust equivalent in `src/gnu/fcntl.rs` before function translation begins. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Port the flag-control functionality from the first function in `gnu/fcntl.c` into Rust in `src/gnu/fcntl.rs`, preserving the original module-local behavior and using the Phase 2 data structure where required. Depends on: T003.
- [T005] [Story] Port the remaining `fcntl`-related helper/function from `gnu/fcntl.c` into Rust in `src/gnu/fcntl.rs`, keeping it aligned with the translated control-flow and constants already established for the module. Depends on: T003, T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/fcntl.rs` to remove C-oriented migration artifacts, consolidate duplicated logic introduced during translation, and ensure the two ported functions and single data structure form a coherent Rust module. Depends on: T004, T005.