# Tasks: main_root_xalloc-die.c_26

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/xalloc_die.rs` to host the port of `xalloc-die.c`.
- [T002] [Story] Expose the new module from `src/main.rs` or `src/lib.rs`, whichever is the existing crate entry point for this branch, by adding the `xalloc_die` module declaration. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust item(s) in `src/xalloc_die.rs` needed to represent the module-level behavior of `xalloc-die.c`, keeping the scope limited to what is required by the single exported function. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Implement the `xalloc_die` function in `src/xalloc_die.rs`, porting the behavior from `xalloc-die.c` and using the foundational item(s) established for this module. Depends on: T003.
- [T005] [P] [Story] Wire call sites in `src/main.rs` or `src/lib.rs` that reference the allocation-failure handler to use the Rust `xalloc_die` module entry point, if such wiring is required by the current branch structure. Depends on: T002, T004.

## Final Phase: Polish

- [T006] [Story] Review `src/xalloc_die.rs` and the crate entry file for import cleanup, visibility tightening, and removal of any migration scaffolding that is no longer needed after the `xalloc_die` port. Depends on: T004, T005.