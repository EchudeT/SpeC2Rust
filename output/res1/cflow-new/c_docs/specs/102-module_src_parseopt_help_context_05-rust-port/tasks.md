# Task List: module_src_parseopt_help_context_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parseopt/help.c` port on branch `102-module_src_parseopt_help_context_05-rust-port`, adding the target source file at `src/parseopt/help.rs`.
- [T002] [Story] Wire the new `src/parseopt/help.rs` module into the existing Rust crate module tree so its items are reachable from the parseopt area. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the Rust equivalents for the help-context data structures represented in `src/parseopt/help.c`, placing the core type declarations in `src/parseopt/help.rs`. Depends on: T002.
- [T004] [P] [Story] Implement the simple supporting structs, enums, and constant representations required by the help-context port in `src/parseopt/help.rs`. Depends on: T003.
- [T005] [P] [Story] Implement the composite and nested data structures needed to model the remaining help-context state from `src/parseopt/help.c` in `src/parseopt/help.rs`. Depends on: T003.
- [T006] [Story] Reconcile the full set of Rust data structures in `src/parseopt/help.rs` so the complete help-context type surface is available before function porting. Depends on: T004, T005.

## Phase 3: Functions

- [T007] [Story] Port the first help-context function from `src/parseopt/help.c` into `src/parseopt/help.rs`, using the completed Rust data structures and preserving its original role within parseopt help handling. Depends on: T006.
- [T008] [Story] Port the second help-context function from `src/parseopt/help.c` into `src/parseopt/help.rs`, integrating it with the first ported function where required by the original C module flow. Depends on: T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/parseopt/help.rs` to remove C-specific implementation leftovers, align naming and ownership with Rust conventions, and ensure the module is internally consistent after the function port. Depends on: T008.
- [T010] [Story] Review the migrated `src/parseopt/help.rs` for dead code, unnecessary mutability, and straightforward simplifications that do not change the ported behavior. Depends on: T009.