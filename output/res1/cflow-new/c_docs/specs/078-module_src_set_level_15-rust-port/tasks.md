# Task List: module_src_set_level_15

## Phase 1: Setup

- [T001] [Story] Create the Rust module file layout for the ported cluster by adding `src/main.rs` and `src/output.rs`, mirroring the analyzed C module file boundaries from `src/main.c` and `src/output.c`.
- [T002] [P] [Story] Wire the new module files into the Rust project branch `078-module_src_set_level_15-rust-port` so that shared items can be referenced between `src/main.rs` and `src/output.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and define the Rust representations required for the 73 module data structures in `src/main.rs`, keeping each type limited to structures directly evidenced by `src/main.c`. Depends on: T002.
- [T004] [P] [Story] Define the Rust representations required for data structures owned by output-related logic in `src/output.rs`, limited to structures directly evidenced by `src/output.c` and shared structures already established in `src/main.rs`. Depends on: T003.
- [T005] [Story] Consolidate shared field types, enums, aliases, and struct relationships across `src/main.rs` and `src/output.rs` so the module’s foundational data model is complete before function migration. Depends on: T003, T004.

## Phase 3: Core module functions

- [T006] [Story] Port the function logic from `src/main.c` into `src/main.rs`, implementing the main module-side behavior against the Rust data structures established in Phase 2. Depends on: T005.
- [T007] [Story] Port the function logic from `src/output.c` into `src/output.rs`, implementing the output-side behavior using the shared and local Rust data structures established in Phase 2. Depends on: T005.
- [T008] [Story] Resolve call flow and data exchange between `src/main.rs` and `src/output.rs` so the two migrated functions interoperate consistently within the Rust module cluster. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine the migrated Rust code in `src/main.rs` and `src/output.rs` by removing C-centric migration artifacts, tightening ownership/borrowing choices, and simplifying data access without changing module behavior. Depends on: T008.
- [T010] [P] [Story] Perform a final pass on `src/main.rs` and `src/output.rs` for naming consistency, visibility cleanup, and module-local organization aligned with the migrated file boundaries. Depends on: T009.