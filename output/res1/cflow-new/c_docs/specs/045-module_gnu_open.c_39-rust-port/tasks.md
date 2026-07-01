# Task List: module_gnu_open.c_39 Rust Port

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for `gnu/open.c` on branch `045-module_gnu_open.c_39-rust-port`, creating the target Rust source file at `src/gnu/open.rs`.
- [T002] [P] [Story] Register the new module file `src/gnu/open.rs` in the existing Rust module tree so it is reachable from the crate structure. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port and define the first C data structure from `gnu/open.c` as a Rust type in `src/gnu/open.rs`, preserving only the fields and visibility required by this module. Depends on: T001.
- [T004] [Story] Port and define the second C data structure from `gnu/open.c` as a Rust type in `src/gnu/open.rs`, aligned with the module-local usage expected by the function port. Depends on: T001.

## Phase 3: Functions

- [T005] [Story] Implement the module’s single function from `gnu/open.c` in `src/gnu/open.rs`, translating its logic to idiomatic Rust and wiring it to the two ported data structures as needed. Depends on: T003, T004.
- [T006] [P] [Story] Integrate any module-local constants, helper expressions, or inline logic required by the function port directly within `src/gnu/open.rs`, keeping the implementation scoped to behavior evidenced by `gnu/open.c`. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Review `src/gnu/open.rs` for compile cleanliness, remove any migration-only placeholders, and refine naming and visibility to match the finalized Rust module boundary. Depends on: T005, T006.