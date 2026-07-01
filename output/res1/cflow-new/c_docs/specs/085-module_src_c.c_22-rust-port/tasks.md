# Tasks: module_src_c.c_22 Rust port

## Phase 1: Setup

- [T001] [Story] Create the module target file at `src/c.rs` on branch `085-module_src_c.c_22-rust-port`, establishing the Rust destination for the port of `src/c.c`.
- [T002] [Story] Wire `src/c.rs` into the crate module tree from the existing Rust project entry/module declaration point so the migrated module is compiled. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the 13 module-local data structures from `src/c.c` in `src/c.rs`, translating C structs/unions/enums/constants into Rust-native type definitions before function migration. Depends on: T001.
- [T004] [P] [Story] Add foundational type aliases, helper constants, and internal field/default initializers in `src/c.rs` that are directly required by the translated data structures. Depends on: T003.
- [T005] [Story] Reconcile ownership/borrowing strategy for the translated module data in `src/c.rs` so the upcoming function ports can use the defined structures without placeholder APIs. Depends on: T003, T004.

## Phase 3: Core state and lifecycle functions

- [T006] [Story] Port the function group in `src/c.c` responsible for module state creation, initialization, or reset into `src/c.rs`, using the foundational data structures already defined. Depends on: T005.
- [T007] [Story] Port the function group in `src/c.c` responsible for teardown, cleanup, or state finalization into `src/c.rs`, keeping behavior aligned with the original C module lifecycle. Depends on: T006.

## Phase 4: Data mutation and update functions

- [T008] [P] [Story] Port the function group in `src/c.c` that mutates or updates the module’s primary data structures into `src/c.rs`. Depends on: T005.
- [T009] [P] [Story] Port the function group in `src/c.c` that inserts, removes, or otherwise manages elements/records within module-owned collections in `src/c.rs`. Depends on: T005.
- [T010] [Story] Consolidate shared internal helpers used by the migrated mutation/management functions inside `src/c.rs` to avoid duplicating logic from `src/c.c`. Depends on: T008, T009.

## Phase 5: Query, formatting, and remaining function ports

- [T011] [P] [Story] Port the function group in `src/c.c` that reads, queries, or computes results from the module state into `src/c.rs`. Depends on: T005.
- [T012] [P] [Story] Port the function group in `src/c.c` that performs output-oriented, formatting-oriented, or conversion-oriented behavior into `src/c.rs`, if such functions exist among the remaining unmigrated functions. Depends on: T005.
- [T013] [Story] Port any remaining functions from `src/c.c` not yet covered by earlier groups into `src/c.rs`, assigning each function to exactly one final implementation task so all 9 functions are migrated once. Depends on: T007, T010, T011, T012.

## Final Phase: Polish

- [T014] [Story] Refine the completed `src/c.rs` port by removing C-specific implementation artifacts, tightening Rust visibility and signatures, and ensuring the final module layout remains consistent with the original `src/c.c` scope. Depends on: T013.
- [T015] [Story] Perform a final compile-oriented review of `src/c.rs` and its module wiring to resolve any remaining integration issues introduced during the port. Depends on: T002, T014.