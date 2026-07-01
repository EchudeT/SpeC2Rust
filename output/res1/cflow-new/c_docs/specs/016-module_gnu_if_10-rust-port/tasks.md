# Tasks: module_gnu_if_10

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/vasnprintf.c` port on branch `016-module_gnu_if_10-rust-port`, adding the target source file at `src/gnu/vasnprintf.rs`.
- [T002] [P] [Story] Wire the new module into the Rust crate module tree so `src/gnu/vasnprintf.rs` is compiled, updating the nearest inferable module declaration file for the `src/gnu` namespace. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the single foundational data structure required by the `gnu/vasnprintf.c` port in `src/gnu/vasnprintf.rs`, preserving the C module’s role and field semantics needed by the ported functions. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Port the core formatted-output support function from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, using the Phase 2 data structure and keeping behavior aligned with the source module’s formatting/allocation responsibilities. Depends on: T003.
- [T005] [Story] Port the remaining `gnu/vasnprintf.c` function into `src/gnu/vasnprintf.rs`, grouping it with the formatted-output implementation and sharing the same internal data structure and helper flow without duplicating logic. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/vasnprintf.rs` to remove C-specific migration leftovers, simplify ownership and buffer handling where possible, and ensure the two ported functions and their shared data structure are internally consistent and idiomatic for the completed Rust module. Depends on: T005.