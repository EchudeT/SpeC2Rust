# Tasks: module_gnu_basename-lgpl.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the port at `src/gnu/basename_lgpl.rs`, establishing the target location for the functionality migrated from `gnu/basename-lgpl.c`.
- [T002] [Story] Expose the new module from the nearest Rust module entry point by adding the appropriate `mod`/`pub mod` declaration for `src/gnu/basename_lgpl.rs` in `src/gnu/mod.rs` or the corresponding parent module file. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational internal helpers and type aliases, if needed, in `src/gnu/basename_lgpl.rs` to support the basename port without introducing unrelated abstractions. Depends on: T001.

## Phase 3: Basename function implementation

- [T004] [Story] Implement the first basename-related function from `gnu/basename-lgpl.c` in `src/gnu/basename_lgpl.rs`, preserving the original module behavior and adapting pointer/string handling to Rust as required. Depends on: T003.
- [T005] [P] [Story] Implement the second basename-related function from `gnu/basename-lgpl.c` in `src/gnu/basename_lgpl.rs`, keeping the implementation aligned with the C source semantics and Rust module boundaries. Depends on: T003.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/basename_lgpl.rs` to remove migration scaffolding, consolidate shared basename logic used by the two functions, and ensure the exported API remains minimal and module-scoped as appropriate. Depends on: T004, T005.