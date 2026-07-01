# Tasks: module_gnu_getprogname.c_30

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/getprogname.rs` to host the port of `gnu/getprogname.c`.
- [T002] [Story] Update the Rust module declarations in `src/gnu/mod.rs` to expose `src/gnu/getprogname.rs` on branch `036-module_gnu_getprogname.c_30-rust-port`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the two foundational data structures required by `gnu/getprogname.c` inside `src/gnu/getprogname.rs`, keeping their fields and ownership model aligned with the C module’s usage. Depends on: T001.
- [T004] [P] [Story] Add internal constructors or default initialization helpers for the data structures in `src/gnu/getprogname.rs` where needed to support the function port cleanly. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the module’s single exported/internal function from `gnu/getprogname.c` in `src/gnu/getprogname.rs`, using the Phase 2 data structures and preserving the original module behavior. Depends on: T003, T004.
- [T006] [Story] Wire any function visibility or re-export needed for callers through `src/gnu/mod.rs` so the Rust port matches the module layout implied by `gnu/getprogname.c`. Depends on: T002, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/getprogname.rs` to remove C-specific porting artifacts, simplify ownership/borrowing where possible, and ensure the final implementation stays minimal and idiomatic without changing behavior. Depends on: T005.
- [T008] [Story] Perform a final module-level consistency pass across `src/gnu/getprogname.rs` and `src/gnu/mod.rs` to confirm naming, visibility, and file organization are aligned with the Rust port scope for `module_gnu_getprogname.c_30`. Depends on: T006, T007.