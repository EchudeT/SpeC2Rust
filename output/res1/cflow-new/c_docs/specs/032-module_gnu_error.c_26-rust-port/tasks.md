# Task List: module_gnu_error.c_26

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/error.c` port in `src/gnu/error.rs`, and register it from the existing parent module so the `module_gnu_error.c_26` implementation has a dedicated target location.
- [T002] [P] [Story] Add the branch-local module wiring needed to expose `src/gnu/error.rs` through the Rust crate’s module tree, keeping the registration limited to the path implied by `gnu/error.c`.

## Phase 2: Foundational

- [T003] [Story] Establish the foundational Rust definitions in `src/gnu/error.rs` for the `gnu/error.c` port, including any module-local constants, type aliases, imports, and internal helper state directly required by the five translated functions. Depends on: T001.

## Phase 3: Error reporting core functions

- [T004] [Story] Implement the primary formatted error-reporting function group from `gnu/error.c` in `src/gnu/error.rs`, covering the core path that emits messages without process termination. Depends on: T003.
- [T005] [Story] Implement the terminating error-reporting function group from `gnu/error.c` in `src/gnu/error.rs`, covering the path that reports an error and then exits according to the original module behavior. Depends on: T004.
- [T006] [P] [Story] Implement the shared internal formatting and emission helpers used by the public error-reporting entry points in `src/gnu/error.rs`, keeping helper scope module-local and aligned with the original C file structure. Depends on: T003.

## Phase 4: Error stream and state functions

- [T007] [Story] Implement the remaining state-oriented functions from `gnu/error.c` in `src/gnu/error.rs`, including any function-level handling for program name, status, or output stream behavior evidenced by the source module. Depends on: T003.
- [T008] [Story] Reconcile the public function implementations in `src/gnu/error.rs` so the core reporting functions use the shared helpers and state behavior consistently across all five translated functions. Depends on: T004, T005, T006, T007.

## Final Phase: Polish

- [T009] [Story] Review and refine `src/gnu/error.rs` for idiomatic Rust cleanup, removing translation-only artifacts, tightening visibility, and preserving the original `gnu/error.c` behavior without expanding module scope. Depends on: T008.