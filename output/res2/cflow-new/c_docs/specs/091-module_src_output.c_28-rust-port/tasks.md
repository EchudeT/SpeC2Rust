# Tasks: module_src_output.c_28 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/output.c` port on branch `091-module_src_output.c_28-rust-port`, adding the target source file at `src/output.rs` and wiring the module into the existing crate module tree.
- [T002] [P] [Story] Review `src/output.c` and map the 10 C data structures and 2 functions to Rust items to be implemented in `src/output.rs`, documenting direct name and responsibility correspondence as implementation comments or TODO markers in that file.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the first group of foundational Rust data structures in `src/output.rs`, covering the basic standalone structs or enums from `src/output.c` that do not depend on other module-local data structures.
  - Depends on: T002
- [T004] [P] [Story] Implement the second group of foundational Rust data structures in `src/output.rs`, covering dependent or composite structs/enums from `src/output.c` that build on the items introduced in T003.
  - Depends on: T002, T003
- [T005] [Story] Add shared Rust type definitions, field ownership/lifetime decisions, and constructor/default helpers in `src/output.rs` needed to make all 10 migrated data structures compile together cleanly.
  - Depends on: T003, T004

## Phase 3: Output function implementation

- [T006] [Story] Implement the first function from `src/output.c` in `src/output.rs`, grouping any closely related internal logic directly with the function while preserving the original module-local behavior.
  - Depends on: T005
- [T007] [Story] Implement the second function from `src/output.c` in `src/output.rs`, reusing the migrated data structures and any shared helpers established earlier without duplicating logic.
  - Depends on: T005, T006

## Final Phase: Polish

- [T008] [Story] Refine `src/output.rs` for idiomatic Rust by removing temporary migration markers, tightening visibility, and simplifying ownership/borrowing where possible without changing the migrated behavior.
  - Depends on: T006, T007
- [T009] [P] [Story] Perform a final compile-focused pass on `src/output.rs` to resolve warnings introduced by the port, consolidate small private helpers, and ensure the module remains isolated to the scope of `src/output.c`.
  - Depends on: T008