# Tasks: main_root_stat_03

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for the `main_root_stat_03` port on branch `004-main_root_stat_03-rust-port`, creating the target source files `src/cat.rs` and `src/fcntl.rs` to mirror `cat.c` and `fcntl.c`.
- [T002] [P] [Story] Wire the new module files into the Rust crate entry structure so code from `src/cat.rs` and `src/fcntl.rs` is reachable from the existing project layout. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the 3 module-local data structures required by `cat.c` and `fcntl.c`, translating them into Rust types in the closest owning target file (`src/cat.rs` or `src/fcntl.rs`) based on usage. Depends on: T001.
- [T004] [Story] Add shared constants, type aliases, and field-level visibility needed to support the translated data structures in `src/cat.rs` and `src/fcntl.rs`, keeping interfaces minimal and aligned with the C module boundaries. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Port the function implemented in `cat.c` into `src/cat.rs`, using the translated Rust data structures and preserving the original main-cluster behavior scope of this module. Depends on: T004.
- [T006] [Story] Port the function implemented in `fcntl.c` into `src/fcntl.rs`, using the translated Rust data structures and preserving the original module-level behavior from the source file. Depends on: T004.
- [T007] [P] [Story] Resolve call-site integration and shared type usage between `src/cat.rs` and `src/fcntl.rs` so both translated functions compile together without duplicating logic. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Refine the Rust port in `src/cat.rs` and `src/fcntl.rs` by removing C-specific remnants, tightening signatures and ownership choices, and ensuring the implementation remains idiomatic without changing module behavior. Depends on: T007.
- [T009] [Story] Perform final compile-pass cleanup for `src/cat.rs` and `src/fcntl.rs`, resolving warnings that are directly introduced by this module migration and keeping file scope limited to `main_root_stat_03`. Depends on: T008.