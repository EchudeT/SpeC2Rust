# Tasks: module_src_parseopt_optdef_04

## Phase 1: Setup

- [T001] [Story] Create Rust module files for the parseopt port in `src/parseopt/help.rs` and `src/parseopt/parseopt.rs`, and wire them into the existing Rust crate module tree for branch `101-module_src_parseopt_optdef_04-rust-port`.
- [T002] [Story] Establish the module-local Rust API surface in `src/parseopt/help.rs` and `src/parseopt/parseopt.rs`, defining the public/private item boundaries needed to receive migrated data structures and functions from `src/parseopt/help.c` and `src/parseopt/parseopt.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and port the option-definition data structures required by `src/parseopt/help.c` into Rust types in `src/parseopt/help.rs`, preserving field relationships and ownership expectations needed by the module functions. Depends on: T002.
- [T004] [P] [Story] Inventory and port the parse-option state, configuration, and support data structures required by `src/parseopt/parseopt.c` into Rust types in `src/parseopt/parseopt.rs`, preserving the original module-local layout semantics needed by the parser logic. Depends on: T002.
- [T005] [Story] Reconcile shared data structure usage between `src/parseopt/help.rs` and `src/parseopt/parseopt.rs`, adding imports or shared type visibility so both migrated files can use consistent option-definition and parser state representations without duplication. Depends on: T003, T004.

## Phase 3: Help and option-definition functions

- [T006] [Story] Implement the help/usage text construction functions migrated from `src/parseopt/help.c` in `src/parseopt/help.rs`, using the Phase 2 option-definition Rust types as their sole backing structures. Depends on: T005.
- [T007] [Story] Implement the option-definition formatting and display support functions migrated from `src/parseopt/help.c` in `src/parseopt/help.rs`, grouping all remaining help-side functions that operate on the same option metadata and output preparation flow. Depends on: T006.

## Phase 4: Parse-option core functions

- [T008] [Story] Implement the parse-option initialization and parser-state setup functions migrated from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, using the foundational Rust parser and option-definition types. Depends on: T005.
- [T009] [Story] Implement the core option scanning and argument-consumption functions migrated from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, grouping the main parse flow that walks arguments and matches option definitions. Depends on: T008.
- [T010] [Story] Implement the remaining parse-option dispatch and completion functions migrated from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, covering the rest of the parser behavior without re-splitting already scheduled functions. Depends on: T009.

## Final Phase: Polish

- [T011] [P] [Story] Refine the Rust implementation in `src/parseopt/help.rs` and `src/parseopt/parseopt.rs` to remove migration-only scaffolding, tighten signatures and visibility, and align the final module code with idiomatic Rust while preserving the C module behavior. Depends on: T007, T010.