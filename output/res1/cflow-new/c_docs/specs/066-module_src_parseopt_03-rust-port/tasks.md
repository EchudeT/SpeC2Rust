# Tasks: module_src_parseopt_03

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port scaffold for the `src/main.c` parse-option module on branch `066-module_src_parseopt_03-rust-port`, creating `src/main.rs` as the migration target entry file for this module slice.
- [T002] [Story] Establish the module layout in `src/main.rs` for the parse-option port, reserving sections for migrated constants, data structures, and function groups from `src/main.c`.
- [T003] [P] [Story] Define the Rust-side migration inventory in `src/main.rs`, listing the 15 target functions and the parse-option-related structure set to keep implementation scope aligned with this module.

## Phase 2: Foundational

- [T004] [Story] Port the foundational parse-option constants, enums, and type aliases from `src/main.c` into Rust definitions in `src/main.rs` that are required before any function migration. Depends on: T001, T002.
- [T005] [Story] Implement the core parse-option state and configuration data structures in `src/main.rs`, covering the primary records directly used to hold option parsing context. Depends on: T004.
- [T006] [Story] Implement auxiliary parse-option descriptor and value-holder data structures in `src/main.rs`, including supporting records referenced by the primary parsing state. Depends on: T005.
- [T007] [P] [Story] Add shared constructors, default initialization, and lightweight helper methods on the ported parse-option data structures in `src/main.rs` where directly needed to support function migration. Depends on: T005, T006.

## Phase 3: Functions - option state initialization and low-level parsing helpers

- [T008] [Story] Implement the function group in `src/main.rs` responsible for initializing parse-option state from the migrated data structures and preparing the parser for argument scanning. Depends on: T005, T007.
- [T009] [Story] Implement the low-level helper function group in `src/main.rs` that reads individual option tokens and updates parse-option state during scanning. Depends on: T008.
- [T010] [P] [Story] Implement the helper function group in `src/main.rs` for interpreting option argument forms and normalizing parsed values into the migrated value-holder structures. Depends on: T006, T009.

## Phase 4: Functions - option matching, dispatch, and parse flow control

- [T011] [Story] Implement the function group in `src/main.rs` that matches input tokens against migrated option descriptors and selects the applicable parse path. Depends on: T006, T009.
- [T012] [Story] Implement the function group in `src/main.rs` that dispatches recognized options into the appropriate state/configuration updates using the migrated descriptor structures. Depends on: T010, T011.
- [T013] [Story] Implement the top-level parse flow function group in `src/main.rs` that coordinates initialization, token scanning, matching, and dispatch for the module’s command-line option handling. Depends on: T008, T012.

## Phase 5: Functions - validation and module integration

- [T014] [Story] Implement the function group in `src/main.rs` that validates parsed option combinations and final parser state before control returns to the broader `main` flow. Depends on: T013.
- [T015] [Story] Integrate the migrated parse-option function group into the existing Rust `src/main.rs` execution path corresponding to the original `src/main.c` usage site for this module slice. Depends on: T013, T014.

## Final Phase: Polish

- [T016] [Story] Refine the migrated parse-option code in `src/main.rs` to remove C-specific porting artifacts, simplify ownership/borrowing boundaries, and keep behavior aligned with the original `src/main.c` module logic. Depends on: T015.
- [T017] [P] [Story] Review `src/main.rs` for duplication across the migrated parse-option structures and function groups, consolidating only directly overlapping logic introduced during the port. Depends on: T016.