# Task List: module_gnu_error.c_26

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/error.c` port on branch `032-module_gnu_error.c_26-rust-port`, adding the target source file at `src/gnu/error.rs`.
- [T002] [Story] Wire the new module into the Rust project module tree so `src/gnu/error.rs` is compiled and reachable from the crate entry points that already expose `gnu` functionality. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational module-level Rust items in `src/gnu/error.rs` needed to support the `gnu/error.c` port, including constant/state declarations and internal helper signatures directly evidenced by the C module contents. Depends on: T002.

## Phase 3: Error reporting core functions

- [T004] [Story] Port the core error-reporting function group from `gnu/error.c` into `src/gnu/error.rs`, implementing the shared formatting and message emission behavior for the module’s primary reporting entry points. Depends on: T003.
- [T005] [P] [Story] Port the module functions that manage error-reporting configuration/state from `gnu/error.c` into `src/gnu/error.rs`, keeping behavior aligned with the original C module’s global settings interface. Depends on: T003.
- [T006] [Story] Integrate the remaining `gnu/error.c` function implementations into `src/gnu/error.rs`, connecting configuration/state handling with the core reporting path so all 5 module functions are fully ported once. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/error.rs` to remove porting redundancies, align naming and visibility with the Rust project conventions, and verify the migrated `gnu/error.c` logic remains self-contained within the module. Depends on: T006.