# Task List: module_gnu_basename-lgpl.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/basename-lgpl.c` in `src/gnu/basename_lgpl.rs`, and expose it from `src/gnu/mod.rs` for the `027-module_gnu_basename_lgpl.c_21-rust-port` branch.
- [T002] [Story] Add a module integration point for the ported basename functionality in the nearest crate root module file already used by `src/gnu/mod.rs`, updating only directly related Rust module declarations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `src/gnu/basename_lgpl.rs` and define any module-local foundational items actually needed by the two basename-related function ports; if no Rust data structures or helper types are required, establish the minimal function-level skeletons only. Depends on: T001.

## Phase 3: Functions

- [T004] [P] [Story] Port the first basename-related function from `gnu/basename-lgpl.c` into `src/gnu/basename_lgpl.rs`, preserving the original module-level behavior with Rust string or path byte handling as required by the C source. Depends on: T003.
- [T005] [P] [Story] Port the second basename-related function from `gnu/basename-lgpl.c` into `src/gnu/basename_lgpl.rs`, keeping its logic aligned with the C implementation and the same module scope as the source file. Depends on: T003.
- [T006] [Story] Reconcile shared logic between the two ported basename functions in `src/gnu/basename_lgpl.rs`, ensuring the final Rust implementation matches the original `gnu/basename-lgpl.c` behavior without duplicating function work across the module. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/basename_lgpl.rs` and related module declarations for idiomatic Rust naming, visibility, and inline documentation comments where directly helpful to the ported basename behavior, without changing scope. Depends on: T006.
- [T008] [Story] Perform a final compile-focused cleanup of the touched files `src/gnu/basename_lgpl.rs` and `src/gnu/mod.rs`, removing any leftover placeholder code from migration and confirming dependency ordering is reflected in the final module layout. Depends on: T007.