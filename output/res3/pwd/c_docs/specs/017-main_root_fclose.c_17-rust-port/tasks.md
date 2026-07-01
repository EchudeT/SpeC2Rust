# Tasks: main_root_fclose.c_17

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for this migration unit at `src/fclose.rs` and declare it from the crate root so `fclose.c` functionality has a dedicated Rust target on branch `017-main_root_fclose.c_17-rust-port`.
- [T002] [Story] Review the two functions from `fclose.c` and map each one to its Rust destination in `src/fclose.rs`, documenting any required signatures and shared module-level dependencies before implementation. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Establish the foundational module scaffolding in `src/fclose.rs` for the `fclose.c` port, including any shared imports, internal helper layout, and placeholders needed by both migrated functions without introducing new data structures. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the primary file-close behavior from `fclose.c` in `src/fclose.rs`, preserving the original module responsibility and integrating with the shared scaffolding prepared for this migration unit. Depends on: T003
- [T005] [Story] Implement the remaining supporting function from `fclose.c` in `src/fclose.rs`, grouped with the close-path logic it serves and avoiding duplicate handling of the same behavior. Depends on: T003
- [T006] [P] [Story] Reconcile the two migrated functions in `src/fclose.rs` so shared control flow, return handling, and internal visibility match the original `fclose.c` module behavior without re-implementing either function. Depends on: T004, T005

## Final Phase: Polish

- [T007] [Story] Polish `src/fclose.rs` by removing temporary migration placeholders, tightening module-level comments, and simplifying any redundant logic introduced during the `fclose.c` port while keeping behavior unchanged. Depends on: T006