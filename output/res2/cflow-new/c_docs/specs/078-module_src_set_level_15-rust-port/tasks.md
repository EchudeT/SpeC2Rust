# Tasks: cflow-new / module_src_set_level_15

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port module scaffold for `module_src_set_level_15` in `src/main.rs` and `src/output.rs`, mapping the C source coverage from `src/main.c` and `src/output.c` onto the Rust target files for this branch.
- [T002] [P] [Story] Define the module-level item layout in `src/main.rs` and `src/output.rs`, reserving sections for shared data structures, function declarations, and call flow integration points needed by the two migrated C functions. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Inventory and implement the foundational Rust representations for the module’s required C-backed data structures in `src/main.rs`, covering the data definitions directly needed by the migrated logic from `src/main.c`. Depends on: T002
- [T004] [P] [Story] Inventory and implement the foundational Rust representations for the module’s required C-backed data structures in `src/output.rs`, covering the data definitions directly needed by the migrated logic from `src/output.c`. Depends on: T002
- [T005] [Story] Reconcile shared structure usage across `src/main.rs` and `src/output.rs`, moving or aligning duplicated definitions so both migrated functions depend on a consistent Rust data model without redefining the same structures twice. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Port the function logic from `src/main.c` into `src/main.rs`, wiring it to the Phase 2 Rust data structures and preserving the original module-local behavior and state updates. Depends on: T005
- [T007] [Story] Port the function logic from `src/output.c` into `src/output.rs`, wiring it to the Phase 2 Rust data structures and preserving the original module-local behavior and output-related state handling. Depends on: T005
- [T008] [Story] Integrate the two migrated functions across `src/main.rs` and `src/output.rs`, resolving call relationships, shared state access, and data handoff between the source-derived Rust files. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Refine the Rust port in `src/main.rs` and `src/output.rs` by removing migration scaffolding, tightening type usage, and simplifying control flow where possible without changing the behavior established by the migrated C module. Depends on: T008
- [T010] [P] [Story] Perform a final pass on `src/main.rs` and `src/output.rs` to clean up naming consistency, inline documentation comments, and file-local organization so the ported module is maintainable and aligned with the project branch layout. Depends on: T009