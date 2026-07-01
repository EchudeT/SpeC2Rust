# Tasks: module_src_delete_level_12

## Phase 1: Setup

- [T001] [Story] Create the Rust module target for `src/symbol.c` in `src/symbol.rs`, and register it from the crate root used by branch `075-module_src_delete_level_12-rust-port`.
- [T002] [P] [Story] Establish the initial item layout in `src/symbol.rs` for the 27 translated data structures and 2 function placeholders, keeping names and visibility aligned with the C module migration target.

## Phase 2: Foundational

- [T003] [Story] Translate and define the core data structures from `src/symbol.c` into Rust types in `src/symbol.rs`, covering direct struct/enum/alias representations required before any function implementation. Depends on: T001, T002.
- [T004] [P] [Story] Add Rust field-level type mapping and internal ownership/reference decisions for the translated data structures in `src/symbol.rs`, preserving the C module’s data layout intent as needed for the later function port. Depends on: T003.
- [T005] [Story] Complete the full set of 27 data-structure declarations in `src/symbol.rs`, including any supporting constants or local helper type aliases directly evidenced by `src/symbol.c`. Depends on: T004.

## Phase 3: Functions

- [T006] [Story] Implement the first function from `src/symbol.c` in `src/symbol.rs`, wiring it to the translated data structures and preserving the original module-local behavior. Depends on: T005.
- [T007] [Story] Implement the second function from `src/symbol.c` in `src/symbol.rs`, completing the functional port against the finalized data structures. Depends on: T005.
- [T008] [Story] Reconcile shared logic, call relationships, and visibility between the two migrated functions inside `src/symbol.rs` so the module matches the original `src/symbol.c` organization without duplicating behavior. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [P] [Story] Refine `src/symbol.rs` for Rust idioms that do not change behavior, including cleanup of temporary placeholders, tighter type usage, and clearer module-local organization after the port. Depends on: T008.
- [T010] [Story] Perform a final migration review of `src/symbol.rs` to ensure all 27 data structures and 2 functions from `src/symbol.c` are represented exactly once and that task-scope changes remain limited to this module. Depends on: T009.