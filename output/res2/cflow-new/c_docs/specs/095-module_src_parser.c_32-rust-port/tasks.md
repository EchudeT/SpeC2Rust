# Tasks: module_src_parser.c_32 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parser.c` port on branch `095-module_src_parser.c_32-rust-port`, adding the target source file `src/parser.rs` and wiring it into the crate module tree.
- [T002] [P] [Story] Establish the migration surface in `src/parser.rs` with placeholders for the 11 data structures and 2 functions identified from `src/parser.c`, preserving C-to-Rust naming traceability in comments for later implementation.
- [T003] [Story] Review `src/parser.rs` imports and module visibility so the new parser module compiles cleanly within the existing Rust project structure before functional implementation. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Port the foundational data structures from `src/parser.c` into Rust definitions in `src/parser.rs`, implementing the first cohesive subset of parser state and plain data carriers required by both functions.
- [T005] [P] [Story] Port the remaining parser-specific data structures from `src/parser.c` into `src/parser.rs`, completing all 11 structure definitions with Rust field mappings aligned to their C layout and usage intent.
- [T006] [Story] Reconcile shared ownership, mutability, and lifetime expectations across all migrated parser data structures in `src/parser.rs`, adjusting type signatures so the full set of 11 definitions can be used by the function layer without placeholder types. Depends on: T004, T005

## Phase 3: Functions

- [T007] [Story] Implement the first parser function from `src/parser.c` in `src/parser.rs`, using the completed data structures and preserving the original control flow and parser state transitions. Depends on: T006
- [T008] [Story] Implement the second parser function from `src/parser.c` in `src/parser.rs`, integrating with the same migrated parser state and any helper logic already introduced for the first function. Depends on: T006
- [T009] [Story] Align the two migrated functions in `src/parser.rs` by resolving shared local logic, consolidating direct structure interactions, and ensuring both functions use consistent Rust-side parser state conventions. Depends on: T007, T008

## Final Phase: Polish

- [T010] [Story] Refine `src/parser.rs` for idiomatic Rust within the migrated scope by tightening visibility, removing setup placeholders and migration comments no longer needed for implementation, and eliminating compile warnings introduced during the port. Depends on: T009
- [T011] [Story] Perform a final pass on `src/parser.rs` to verify the `src/parser.c` migration is complete for this module, confirming all 11 data structures and 2 functions are represented exactly once in the Rust port task scope. Depends on: T010