# Tasks: module_gnu_progname.c_42

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/progname.rs` to host the port of `gnu/progname.c`.
- [T002] [Story] Wire `src/gnu/progname.rs` into the crate module tree by updating the nearest Rust module declaration file required to expose `gnu::progname`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/progname.c` and define the minimal Rust item layout in `src/gnu/progname.rs` needed for the module-level state and function signatures used by the port. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the progname-related function from `gnu/progname.c` in `src/gnu/progname.rs`, preserving the C module behavior and mapping any module-local state access into Rust. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/gnu/progname.rs` to remove porting scaffolding, tighten visibility to the minimal required surface, and align naming and documentation comments with crate conventions. Depends on: T004.