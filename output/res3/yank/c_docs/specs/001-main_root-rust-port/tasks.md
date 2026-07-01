# Tasks: main_root Rust port

**Input**: C module analysis for `yank.c`
**Branch**: `001-main_root-rust-port`

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust main-module scaffold for the `main_root` port in `src/main.rs`, establishing placeholders for data structures and function groups migrated from `yank.c`.
- [ ] T002 [P] [Story] Add a dedicated module file `src/main_root.rs` and wire it from `src/main.rs` so the `yank.c` migration work is isolated to a direct Rust target file.
- [ ] T003 [Story] Define the initial migration layout in `src/main_root.rs` with commented sections for the 15 data structures and 13 functions identified in `yank.c`. Depends on: T002

## Phase 2: Foundational

- [ ] T004 [Story] Port the first group of core data structures from `yank.c` into Rust definitions in `src/main_root.rs`, covering top-level program-state structures needed broadly by main-flow logic. Depends on: T003
- [ ] T005 [P] [Story] Port the second group of supporting data structures from `yank.c` into Rust definitions in `src/main_root.rs`, covering configuration, option, and argument-related records inferred from main-module responsibilities. Depends on: T003
- [ ] T006 [P] [Story] Port the third group of remaining helper data structures from `yank.c` into Rust definitions in `src/main_root.rs`, covering transient and utility records required by the module. Depends on: T003
- [ ] T007 [Story] Reconcile the 15 migrated data structures in `src/main_root.rs` by resolving ownership, borrowing, enums, and default construction patterns required by the later function ports. Depends on: T004, T005, T006

## Phase 3: Functions

- [ ] T008 [Story] Implement the program-entry and top-level control-flow function group from `yank.c` in `src/main.rs` and `src/main_root.rs`, mapping C main-module orchestration into Rust call flow. Depends on: T007
- [ ] T009 [P] [Story] Implement the argument and option-processing function group from `yank.c` in `src/main_root.rs`, using the migrated configuration and argument data structures. Depends on: T007
- [ ] T010 [P] [Story] Implement the initialization and state-setup function group from `yank.c` in `src/main_root.rs`, constructing the module’s runtime state from the foundational structures. Depends on: T007
- [ ] T011 [Story] Implement the main execution and command-dispatch function group from `yank.c` in `src/main_root.rs`, connecting parsed inputs and initialized state to the module’s primary behavior. Depends on: T008, T009, T010
- [ ] T012 [P] [Story] Implement the output, reporting, and termination-related function group from `yank.c` in `src/main_root.rs`, preserving the C module’s externally visible main-program behavior. Depends on: T011
- [ ] T013 [P] [Story] Implement the remaining helper and utility functions from `yank.c` in `src/main_root.rs`, assigning each of the leftover functions exactly once to complete the 13-function migration. Depends on: T011
- [ ] T014 [Story] Integrate all migrated function groups in `src/main.rs` and `src/main_root.rs` so the Rust main module executes through the same high-level path as `yank.c`. Depends on: T012, T013

## Final Phase: Polish

- [ ] T015 [Story] Refine the `main_root` Rust port in `src/main.rs` and `src/main_root.rs` by removing migration placeholders, tightening signatures and visibility, and simplifying control flow without changing behavior. Depends on: T014
- [ ] T016 [Story] Perform a final consistency pass on the `yank.c` to Rust migration in `src/main.rs` and `src/main_root.rs`, ensuring all 15 data structures and 13 functions are represented once and grouped coherently. Depends on: T015