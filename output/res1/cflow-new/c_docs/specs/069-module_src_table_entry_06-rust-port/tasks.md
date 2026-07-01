# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/symbol.c` port in `src/symbol.rs`, and wire the module into the crate from the existing Rust module tree on branch `069-module_src_table_entry_06-rust-port`.
- [T002] [P] [Story] Define the migration surface for `module_src_table_entry_06` in `src/symbol.rs` by listing the C-owned data structures and the 8 target functions as Rust items/placeholders, keeping names and grouping aligned to the source module. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the foundational data structures from `src/symbol.c` into Rust in `src/symbol.rs`, including the core table-entry representation and directly related struct/enum/type definitions required before any function implementation. Depends on: T002.
- [T004] [P] [Story] Add Rust field typing, constructors/defaults, and ownership/borrowing decisions for the remaining module-local data structures inferred from `src/symbol.c` in `src/symbol.rs`, ensuring the full set of 27 data structures is represented before function logic is added. Depends on: T003.
- [T005] [Story] Reconcile shared relationships among the migrated data structures in `src/symbol.rs` so function groups can operate on stable Rust representations without revisiting structure layout later. Depends on: T003, T004.

## Phase 3: Table Entry Creation and Initialization Functions

- [T006] [Story] Implement the function group in `src/symbol.rs` responsible for creating, initializing, or allocating source table entry state from `src/symbol.c`, using the Phase 2 Rust data structures directly. Depends on: T005.
- [T007] [P] [Story] Implement any closely related helper functions in `src/symbol.rs` that prepare default values or attach initial metadata to new table entries, grouped with creation/initialization behavior from `src/symbol.c`. Depends on: T006.

## Phase 4: Table Entry Lookup and Update Functions

- [T008] [Story] Implement the function group in `src/symbol.rs` responsible for locating or selecting source table entries, preserving the original `src/symbol.c` logic flow while adapting to Rust ownership and reference rules. Depends on: T005.
- [T009] [P] [Story] Implement the function group in `src/symbol.rs` that updates, mutates, or links source table entry contents after lookup, grouped with the corresponding `src/symbol.c` behavior and avoiding overlap with creation logic. Depends on: T008.

## Phase 5: Table Entry Cleanup and Accessor Functions

- [T010] [Story] Implement the function group in `src/symbol.rs` responsible for cleanup, reset, or release-oriented table entry behavior present in `src/symbol.c`, expressed with Rust drop-safe patterns where applicable. Depends on: T005.
- [T011] [P] [Story] Implement the remaining accessor or conversion-style functions in `src/symbol.rs` needed to complete the 8-function port from `src/symbol.c`, assigning each function to exactly one group and avoiding duplicate scheduling. Depends on: T006, T008, T009, T010.

## Final Phase: Polish

- [T012] [Story] Refine `src/symbol.rs` for idiomatic Rust naming consistency, remove now-unneeded migration placeholders, and simplify internal APIs without changing the ported behavior. Depends on: T011.
- [T013] [Story] Perform a final module pass in `src/symbol.rs` to resolve compile-time issues from the `src/symbol.c` migration, tighten visibility to module-appropriate scope, and ensure the table-entry implementation is coherent as a single Rust module. Depends on: T012.