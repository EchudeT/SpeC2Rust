# Tasks: module_src_save_stack_14

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parser.c` migration in `src/parser.rs` on branch `077-module_src_save_stack_14-rust-port`.
- [T002] [Story] Wire the new parser module into the existing Rust crate module tree from the nearest inferable crate entry point so `src/parser.rs` is compiled.
- [T003] [P] [Story] Establish placeholder section comments and item layout in `src/parser.rs` for the 11 migrated data structures and the 2 function implementations to keep later file-local migration work aligned.

## Phase 2: Foundational

- [T004] [Story] Define the first grouped set of foundational Rust data structures in `src/parser.rs`, migrating the C module state and stack-related types required before porting any save-stack behavior. Depends on: T001, T002, T003.
- [T005] [P] [Story] Define the second grouped set of foundational Rust data structures in `src/parser.rs`, migrating supporting record, entry, and parser-context types used by the save-stack logic. Depends on: T001, T002, T003.
- [T006] [P] [Story] Define the remaining foundational Rust data structures in `src/parser.rs`, completing all 11 module-local type migrations and their field mappings needed by the target functions. Depends on: T001, T002, T003.
- [T007] [Story] Reconcile the migrated data structures in `src/parser.rs` by connecting references, ownership, and mutability patterns so the full save-stack type graph is usable by function ports. Depends on: T004, T005, T006.

## Phase 3: Functions

- [T008] [Story] Implement the first save-stack-related function from `src/parser.c` in `src/parser.rs`, using the migrated parser and stack data structures without expanding beyond the module’s evidenced behavior. Depends on: T007.
- [T009] [Story] Implement the second save-stack-related function from `src/parser.c` in `src/parser.rs`, completing the function migration for `module_src_save_stack_14` and aligning it with the first ported function’s shared state handling. Depends on: T007, T008.

## Final Phase: Polish

- [T010] [Story] Refine `src/parser.rs` for Rust compile cleanliness by removing temporary placeholders, tightening signatures and visibility to the migrated module scope, and ensuring the completed data-structure and function ports remain consistent with the original `src/parser.c` save-stack module boundaries. Depends on: T009.