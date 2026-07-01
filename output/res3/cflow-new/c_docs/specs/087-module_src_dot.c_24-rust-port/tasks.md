# Tasks: module_src_dot.c_24

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for the `src/dot.c` port on branch `087-module_src_dot.c_24-rust-port`, adding the target source file at `src/dot.rs`.
- [ ] T002 [Story] Wire the new `src/dot.rs` module into the existing Rust crate module tree so code from `src/dot.c` can be implemented in the corresponding Rust module.
- [ ] T003 [P] [Story] Review `src/dot.c` and record the Rust-side inventory of the 3 functions and 3 data structures to be ported directly in `src/dot.rs`; use this inventory to keep later implementation scoped to this module.

## Phase 2: Foundational

- [ ] T004 [Story] Define the first ported data structure from `src/dot.c` in `src/dot.rs`, preserving its role and field layout semantics needed by the module’s functions. Depends on: T001, T003
- [ ] T005 [P] [Story] Define the second ported data structure from `src/dot.c` in `src/dot.rs`, preserving its role and field layout semantics needed by the module’s functions. Depends on: T001, T003
- [ ] T006 [P] [Story] Define the third ported data structure from `src/dot.c` in `src/dot.rs`, preserving its role and field layout semantics needed by the module’s functions. Depends on: T001, T003
- [ ] T007 [Story] Reconcile shared type relationships, ownership/borrowing strategy, and constructor/default patterns across the 3 ported data structures in `src/dot.rs` so the later function ports can use them directly without duplicating translation logic. Depends on: T004, T005, T006

## Phase 3: Functions

- [ ] T008 [Story] Port the first `src/dot.c` function into `src/dot.rs`, using the new Rust data structures and preserving the original module behavior. Depends on: T007
- [ ] T009 [Story] Port the second `src/dot.c` function into `src/dot.rs`, grouping it with the related dot-module behavior it supports and using the shared structures defined earlier. Depends on: T007
- [ ] T010 [Story] Port the third `src/dot.c` function into `src/dot.rs`, completing the functional coverage of the `src/dot.c` module within `src/dot.rs`. Depends on: T007
- [ ] T011 [Story] Integrate the 3 ported functions inside `src/dot.rs` by resolving call ordering, shared helper flow, and any direct C-to-Rust control-flow translation issues without introducing functionality outside `src/dot.c`. Depends on: T008, T009, T010

## Final Phase: Polish

- [ ] T012 [Story] Refine `src/dot.rs` for idiomatic Rust within the scope of the `src/dot.c` port, simplifying obvious translation artifacts while preserving behavior. Depends on: T011
- [ ] T013 [Story] Perform a final module pass on `src/dot.rs` to remove dead translation scaffolding, verify the 3 data structures and 3 functions are fully migrated, and ensure the module is ready for the branch `087-module_src_dot.c_24-rust-port`. Depends on: T012