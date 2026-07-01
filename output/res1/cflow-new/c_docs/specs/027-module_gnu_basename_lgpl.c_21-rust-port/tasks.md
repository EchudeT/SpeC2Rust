# Tasks: module_gnu_basename-lgpl.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the port of `gnu/basename-lgpl.c` in `src/gnu/basename_lgpl.rs`, and declare it from the nearest existing Rust module entry point so the module is compiled on branch `027-module_gnu_basename_lgpl.c_21-rust-port`.
- [T002] [P] [Story] Add placeholder public function signatures in `src/gnu/basename_lgpl.rs` for the 2 functions ported from `gnu/basename-lgpl.c`, preserving C-level behavior expectations and documenting source mapping comments for each function. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Establish shared internal helper logic in `src/gnu/basename_lgpl.rs` for basename-style path scanning used by this module’s functions, keeping the implementation private and limited to logic directly required by `gnu/basename-lgpl.c`. Depends on: T002.

## Phase 3: Basename Function Implementation

- [T004] [Story] Implement the primary basename extraction function from `gnu/basename-lgpl.c` in `src/gnu/basename_lgpl.rs`, porting the C path-trimming and final-component selection behavior exactly. Depends on: T003.
- [T005] [P] [Story] Implement the companion basename-related function from `gnu/basename-lgpl.c` in `src/gnu/basename_lgpl.rs`, reusing the shared internal scanning logic where applicable without duplicating path parsing behavior. Depends on: T003.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/basename_lgpl.rs` to remove redundant logic between the 2 function ports, tighten visibility of private helpers, and verify the final module remains narrowly scoped to `gnu/basename-lgpl.c` behavior. Depends on: T004, T005.