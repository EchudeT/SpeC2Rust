# Tasks: module_src_save_stack_14

## Phase 1: Setup

- [T001] [Story] Create the Rust module skeleton for the `src/parser.c` migration in `src/parser.rs`, and register the module from the existing crate root so `module_src_save_stack_14` code has a dedicated implementation location on branch `077-module_src_save_stack_14-rust-port`.
- [T002] [P] [Story] Add placeholder item sections in `src/parser.rs` for the 11 migrated data structures and the 2 migrated functions, keeping names and grouping aligned to the `src/parser.c` save-stack area to support incremental porting. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Identify and define the first group of save-stack-related core data structures from `src/parser.c` in `src/parser.rs`, covering the structures directly required to represent stack state and stack entry ownership. Depends on: T002
- [T004] [P] [Story] Define the second group of supporting save-stack-related data structures from `src/parser.c` in `src/parser.rs`, covering helper records and ancillary fields that do not introduce new function behavior. Depends on: T002
- [T005] [P] [Story] Define the remaining migrated data structures from `src/parser.c` in `src/parser.rs`, completing all 11 structure translations and reconciling field types and visibility with the earlier foundational definitions. Depends on: T003, T004
- [T006] [Story] Refine the combined data structure set in `src/parser.rs` so shared references, lifetimes or owned storage choices are internally consistent for the save-stack implementation and ready for function porting. Depends on: T005

## Phase 3: Functions

- [T007] [Story] Implement the first save-stack function from `src/parser.c` in `src/parser.rs`, using the completed Rust data structures and preserving the original stack-state behavior. Depends on: T006
- [T008] [Story] Implement the second save-stack function from `src/parser.c` in `src/parser.rs`, completing the functional port for `module_src_save_stack_14` and integrating it with the first migrated function where required. Depends on: T007

## Final Phase: Polish

- [T009] [Story] Perform a focused cleanup pass in `src/parser.rs` to remove placeholder code, tighten item visibility, and simplify obvious Rust idioms without changing the migrated save-stack behavior. Depends on: T008
- [T010] [Story] Review `src/parser.rs` for migration completeness against the `src/parser.c` save-stack segment, ensuring all 11 data structures and 2 functions are present exactly once and grouped coherently. Depends on: T009