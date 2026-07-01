# Task List: module_gnu_dup2.c_25

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/dup2.c` port on branch `031-module_gnu_dup2.c_25-rust-port`, adding the target source file at `src/gnu/dup2.rs` and wiring the module export from its parent `src/gnu/mod.rs` if not already present.
- [T002] [P] [Story] Review the C source `gnu/dup2.c` and map its 4 functions and 1 data structure into Rust implementation items to be placed in `src/gnu/dup2.rs`, documenting the exact migration scope in code comments or TODO markers without expanding beyond the source file. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure from `gnu/dup2.c` in `src/gnu/dup2.rs`, preserving the original field layout and intent as closely as Rust allows for the ported logic. Depends on: T002

## Phase 3: Core dup2 port

- [T004] [Story] Implement the primary `dup2`-related function logic from `gnu/dup2.c` in `src/gnu/dup2.rs`, porting the central file-descriptor duplication behavior and keeping the implementation aligned with the original module flow. Depends on: T003
- [T005] [Story] Implement the closely related helper function(s) that support the core duplication path in `src/gnu/dup2.rs`, grouping only the helper logic directly used by the main `dup2` behavior from `gnu/dup2.c`. Depends on: T004

## Phase 4: Descriptor state handling

- [T006] [P] [Story] Implement the remaining function(s) in `src/gnu/dup2.rs` that manage descriptor-state checks or transitions required by the `gnu/dup2.c` port, completing the 4-function migration without introducing extra module responsibilities. Depends on: T003
- [T007] [Story] Integrate the Phase 4 function(s) with the core duplication flow in `src/gnu/dup2.rs`, ensuring the full control path from the original `gnu/dup2.c` is represented once and only once in the Rust port. Depends on: T005, T006

## Final Phase: Polish

- [T008] [Story] Refine `src/gnu/dup2.rs` for idiomatic Rust structure while preserving C-module behavior, removing temporary migration markers, tightening internal visibility, and verifying that all 4 functions and the 1 data structure from `gnu/dup2.c` are fully accounted for in the final port. Depends on: T007