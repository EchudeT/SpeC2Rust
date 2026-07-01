# Tasks: main_root_copy-file-range.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/copy_file_range.rs` and register it from the crate root so the ported `copy-file-range.c` logic has a dedicated target location on branch `022-main_root_copy_file_range.c_21-rust-port`.
- [T002] [P] [Story] Add the initial public/internal item skeletons in `src/copy_file_range.rs` for the module’s single data structure and single function, keeping names and visibility aligned with the C module scope. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure in `src/copy_file_range.rs`, translating the C definition into an idiomatic Rust struct/enum layout while preserving the fields required by the module function. Depends on: T002.

## Phase 3: Function Implementation

- [T004] [Story] Implement the module’s single function in `src/copy_file_range.rs`, porting the behavior from `copy-file-range.c` and wiring it to the foundational data structure created for this module. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/copy_file_range.rs` for Rust correctness and maintainability by resolving compile-time issues, tightening signatures/ownership, and removing any migration scaffolding left from the initial port. Depends on: T004.