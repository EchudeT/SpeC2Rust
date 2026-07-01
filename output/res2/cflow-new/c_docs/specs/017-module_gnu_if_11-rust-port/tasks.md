# Tasks: module_gnu_if_11

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for the `gnu/vasnprintf.c` port in `src/gnu/vasnprintf.rs`, and expose it from the existing `src/gnu/mod.rs` module tree on branch `017-module_gnu_if_11-rust-port`.
- [ ] T002 [Story] Review `gnu/vasnprintf.c` and map the 2 target functions and 1 data structure into Rust items to be implemented in `src/gnu/vasnprintf.rs`; document the item list as implementation comments at the top of the file. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Implement the module’s foundational data structure from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, preserving the original module-local responsibilities and field semantics needed by the function port. Depends on: T002

## Phase 3: Formatting Core Functions

- [ ] T004 [Story] Port the lower-level helper function from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, wiring it to the foundational data structure and preserving the original module-local formatting/allocation behavior. Depends on: T003
- [ ] T005 [Story] Port the top-level `vasnprintf` functionality from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, using the helper implemented for the module’s formatting flow and keeping behavior aligned with the C source. Depends on: T004

## Final Phase: Polish

- [ ] T006 [Story] Refine `src/gnu/vasnprintf.rs` to remove obvious porting scaffolding comments, tighten internal item visibility, and align naming/documentation with surrounding Rust module conventions without changing module behavior. Depends on: T005