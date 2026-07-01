# Tasks: module_src_parseopt_parseopt_03

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the parseopt port in `src/parseopt/mod.rs` and `src/parseopt/parseopt.rs`, and expose the new module from the existing crate entry point as needed for branch `100-module_src_parseopt_parseopt_03-rust-port`.
- [T002] [P] [Story] Establish the initial item map in `src/parseopt/parseopt.rs` for the 53 C-backed data structures and 10 functions from `src/parseopt/parseopt.c`, using Rust placeholders and comments that preserve source-level grouping for the later migration work.

## Phase 2: Foundational

- [T003] [Story] Port the core parse-option state and configuration data structures from `src/parseopt/parseopt.c` into Rust definitions in `src/parseopt/parseopt.rs`, covering the structs and enums that represent parser context, option descriptors, argument metadata, and parse mode flags. Depends on: T001, T002.
- [T004] [P] [Story] Port the supporting constant, alias, and helper data structures from `src/parseopt/parseopt.c` into Rust definitions in `src/parseopt/parseopt.rs`, covering the remaining static layout items required by the parser but not directly implementing behavior. Depends on: T001, T002.
- [T005] [Story] Reconcile the full set of 53 migrated data structures in `src/parseopt/parseopt.rs` so the foundational types compile together with consistent ownership, borrowing, and visibility boundaries needed by the function port. Depends on: T003, T004.

## Phase 3: Option model and parser state functions

- [T006] [Story] Implement the function group in `src/parseopt/parseopt.rs` that constructs, initializes, or normalizes parse-option state from the migrated foundational data structures, keeping behavior aligned with the corresponding setup logic in `src/parseopt/parseopt.c`. Depends on: T005.
- [T007] [Story] Implement the function group in `src/parseopt/parseopt.rs` that inspects option definitions and parser state to classify supported options, argument expectations, and parser modes during option handling. Depends on: T005, T006.

## Phase 4: Argument consumption and parse execution functions

- [T008] [Story] Implement the function group in `src/parseopt/parseopt.rs` that consumes raw argument input, advances parser state, and applies matched option definitions during command-line parsing. Depends on: T007.
- [T009] [Story] Implement the function group in `src/parseopt/parseopt.rs` that handles parse completion paths, including final state updates and result shaping needed after argument scanning finishes. Depends on: T008.

## Phase 5: Output and help-related functions

- [T010] [P] [Story] Implement the function group in `src/parseopt/parseopt.rs` responsible for formatting or assembling parse-option usage/help-related data if such routines are present among the remaining unmigrated functions in `src/parseopt/parseopt.c`. Depends on: T005.
- [T011] [Story] Integrate the remaining standalone function logic from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, assigning each of the 10 source functions to exactly one Rust implementation path and resolving call flow between state setup, argument parsing, and any help/usage support. Depends on: T009, T010.

## Final Phase: Polish

- [T012] [Story] Refine `src/parseopt/parseopt.rs` and `src/parseopt/mod.rs` by removing migration placeholders, tightening signatures and visibility, and simplifying internal control flow without changing the ported behavior. Depends on: T011.