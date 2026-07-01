# Tasks: module_gnu_getprogname.c_30

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/getprogname.c` port in `src/gnu/getprogname.rs`.
- [T002] [Story] Wire the new module into the Rust project by exporting `src/gnu/getprogname.rs` from the existing `src/gnu/mod.rs` module tree.
- [T003] [P] [Story] Add a placeholder public API surface in `src/gnu/getprogname.rs` for the module data structures and function port so later implementation can proceed without changing module wiring. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Identify and define the two module-local data structures represented in `gnu/getprogname.c` as Rust types in `src/gnu/getprogname.rs`, preserving their module-scoped role and layout intent as closely as the C source requires. Depends on: T003.
- [T005] [Story] Add any associated constants, static storage, or simple helper state required to support the two data structures in `src/gnu/getprogname.rs`. Depends on: T004.

## Phase 3: Functions

- [T006] [Story] Implement the `getprogname` function port from `gnu/getprogname.c` in `src/gnu/getprogname.rs`, using the previously defined data structures and module state. Depends on: T004, T005.
- [T007] [P] [Story] Refine visibility and signatures for the exported `getprogname` API in `src/gnu/getprogname.rs` and any corresponding re-export in `src/gnu/mod.rs` so the ported function matches the module’s intended Rust-facing interface. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Review `src/gnu/getprogname.rs` for C-to-Rust parity issues, remove placeholder code, and simplify any redundant state or helper definitions introduced during porting. Depends on: T006, T007.
- [T009] [Story] Run a final pass on `src/gnu/getprogname.rs` and `src/gnu/mod.rs` to improve naming, comments, and module organization without changing behavior. Depends on: T008.