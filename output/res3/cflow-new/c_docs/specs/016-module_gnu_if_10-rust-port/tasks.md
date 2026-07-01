# Tasks: module_gnu_if_10

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, and expose it from the existing parent module file needed to compile the new module on branch `016-module_gnu_if_10-rust-port`.
- [ ] T002 [Story] Define the module port boundary in `src/gnu/vasnprintf.rs` by listing the C-origin items to be migrated for this module: the single data structure and the 2 functions from `gnu/vasnprintf.c`.

## Phase 2: Foundational

- [ ] T003 [Story] Implement the module data structure from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, preserving the fields and ownership model required by the formatting logic. Depends on: T001, T002

## Phase 3: Functions

- [ ] T004 [Story] Implement the lower-level helper function from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, using the Phase 2 data structure as its working state. Depends on: T003
- [ ] T005 [Story] Implement the top-level `vasnprintf` formatting function port from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, wiring it to the helper logic and the module data structure. Depends on: T003, T004

## Final Phase: Polish

- [ ] T006 [Story] Refine `src/gnu/vasnprintf.rs` to align the Rust implementation with the original C module behavior, removing migration scaffolding and tightening signatures and internal organization without expanding module scope. Depends on: T005