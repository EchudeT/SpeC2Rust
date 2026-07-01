# Tasks: module_src_collect_data_10

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the `src/symbol.c` port on branch `073-module_src_collect_data_10-rust-port`, adding the target Rust source file `src/symbol.rs` and wiring it into the crate module tree if not already present.
- [T002] [P] [Story] Define the migration scope in `src/symbol.rs` with placeholders for the 27 data structures and 2 function ports from `src/symbol.c`, keeping names and grouping aligned with the source module for incremental implementation. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port and declare the foundational data structures from `src/symbol.c` into `src/symbol.rs`, preserving field layout, ownership model, and type relationships required by the module’s symbol/data collection logic. Depends on: T002
- [T004] [P] [Story] Implement associated Rust enums, type aliases, constants, and simple container/value definitions in `src/symbol.rs` that are required to complete the full set of 27 data structures. Depends on: T002
- [T005] [Story] Complete the remaining composite and cross-referencing data structures in `src/symbol.rs`, resolving links between structs/enums introduced in earlier foundational tasks so the module data model is internally consistent. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Implement the first function from `src/symbol.c` in `src/symbol.rs`, translating its core symbol/data collection behavior to operate on the ported Rust data structures without expanding module scope. Depends on: T005
- [T007] [Story] Implement the second function from `src/symbol.c` in `src/symbol.rs`, grouping it with the first as the module’s functional port and preserving the original interaction with the module data structures. Depends on: T006

## Final Phase: Polish

- [T008] [Story] Refine `src/symbol.rs` to remove migration placeholders, resolve any remaining type/borrow issues, and align naming, visibility, and module organization with the completed `src/symbol.c` port. Depends on: T007
- [T009] [P] [Story] Perform a final pass on `src/symbol.rs` for idiomatic Rust cleanup limited to this module’s migrated code, including simplifying local implementations where it does not change the C module behavior. Depends on: T008