# Tasks: module_src_parseopt_file_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/main.c` port on branch `070-module_src_parseopt_file_07-rust-port`, adding the target Rust source file `src/main.rs`.
- [T002] [P] [Story] Establish the initial Rust file layout in `src/main.rs` for this module cluster, reserving sections for parse-option related data structures and function ports from `src/main.c`.

## Phase 2: Foundational

- [T003] [Story] Port and define the foundational parse-option data structures inferred from `src/main.c` into Rust in `src/main.rs`, covering the full set of module-local structs, enums, type aliases, and constant-style definitions needed before function migration. Depends on: T001, T002
- [T004] [P] [Story] Refine ownership, borrowing, and lifetime boundaries for the ported parse-option data structures in `src/main.rs` so the function implementations can use them without C-style aliasing assumptions. Depends on: T003
- [T005] [Story] Add Rust-native constructors, default initializers, and helper methods in `src/main.rs` for the ported parse-option data structures where required to support direct replacement of the `src/main.c` usage patterns. Depends on: T003

## Phase 3: Function Porting

- [T006] [Story] Port the first parse-option function from `src/main.c` into `src/main.rs`, translating its control flow to Rust while preserving its interaction with the previously ported data structures. Depends on: T004, T005
- [T007] [Story] Port the second parse-option function from `src/main.c` into `src/main.rs`, completing the module’s function migration and preserving its original integration points within the file-level parsing flow. Depends on: T004, T005
- [T008] [P] [Story] Reconcile shared helper logic, common argument/state handling, and any duplicated local C patterns across the two ported functions in `src/main.rs` without changing module behavior. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Perform a final compile-focused cleanup pass on `src/main.rs`, resolving Rust idioms, visibility, and signature adjustments needed to make the module port internally consistent after data-structure and function migration. Depends on: T008
- [T010] [Story] Review the completed `src/main.rs` port against `src/main.c` to remove leftover C-specific scaffolding comments and ensure the module scope remains limited to the parse-option file migration. Depends on: T009