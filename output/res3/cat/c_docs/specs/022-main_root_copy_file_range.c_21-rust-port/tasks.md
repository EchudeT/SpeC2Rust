# Tasks: main_root_copy-file-range.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `copy-file-range.c` in `src/copy_file_range.rs`, and declare it from the crate root in `src/lib.rs` or `src/main.rs` according to the existing project layout.
- [T002] [P] [Story] Add placeholder item declarations in `src/copy_file_range.rs` for the module’s single data structure and single function so later implementation work has a stable target. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure in `src/copy_file_range.rs`, translating the C definition from `copy-file-range.c` into an idiomatic Rust type while preserving fields and module-local responsibilities. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the module’s single function in `src/copy_file_range.rs`, porting the behavior from `copy-file-range.c` and wiring it to the foundational data structure introduced for this module. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/copy_file_range.rs` for Rust idioms and module integration by cleaning up signatures, visibility, imports, and error/return handling needed for the completed port without changing scoped behavior. Depends on: T004.