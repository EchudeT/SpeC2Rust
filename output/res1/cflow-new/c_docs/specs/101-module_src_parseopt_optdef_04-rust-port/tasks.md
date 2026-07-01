# Tasks: module_src_parseopt_optdef_04

## Phase 1: Setup

- [T001] [Story] Create the Rust module file scaffolding for the parse option port by adding `src/parseopt/help.rs` and `src/parseopt/parseopt.rs`, and register them in the existing Rust module tree for branch `101-module_src_parseopt_optdef_04-rust-port`.
- [T002] [P] [Story] Define the initial migration surface in `src/parseopt/help.rs` and `src/parseopt/parseopt.rs` with placeholder Rust items that mirror the C module boundaries from `src/parseopt/help.c` and `src/parseopt/parseopt.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the option-definition data structures required by this module from `src/parseopt/parseopt.c` into Rust types in `src/parseopt/parseopt.rs`, preserving field relationships needed by all option-definition functions. Depends on: T002.
- [T004] [Story] Port the help/usage-facing data structures required by this module from `src/parseopt/help.c` into Rust types in `src/parseopt/help.rs`, preserving field relationships needed by help formatting and option display logic. Depends on: T002.
- [T005] [P] [Story] Add shared enums, constants, and type aliases used across option-definition and help behavior in `src/parseopt/parseopt.rs` and `src/parseopt/help.rs`, keeping names and roles aligned with the original C module semantics. Depends on: T003, T004.
- [T006] [Story] Wire the foundational Rust data structures across `src/parseopt/parseopt.rs` and `src/parseopt/help.rs` so function implementations can reference a stable internal API for option definitions and help generation. Depends on: T003, T004, T005.

## Phase 3: Option-definition core functions

- [T007] [Story] Implement the core option-definition construction and initialization functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, grouped around creating and preparing option definition records. Depends on: T006.
- [T008] [Story] Implement the option-definition mutation and attribute-assignment functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, grouped around updating names, flags, argument metadata, and descriptive fields on existing definitions. Depends on: T007.
- [T009] [Story] Implement the option-definition lookup, traversal, and retrieval functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, grouped around accessing stored definitions needed by parsing and help output. Depends on: T008.

## Phase 4: Help and presentation functions

- [T010] [P] [Story] Implement the help-text formatting functions from `src/parseopt/help.c` in `src/parseopt/help.rs`, grouped around rendering usage lines and descriptive text from the Rust option-definition structures. Depends on: T006.
- [T011] [Story] Implement the option display and listing functions from `src/parseopt/help.c` in `src/parseopt/help.rs`, grouped around presenting individual and aggregate option definitions consistently with the original module behavior. Depends on: T009, T010.
- [T012] [Story] Connect help rendering in `src/parseopt/help.rs` to the option-definition APIs in `src/parseopt/parseopt.rs`, replacing any remaining placeholders with end-to-end module interactions equivalent to the C implementation. Depends on: T009, T010, T011.

## Final Phase: Polish

- [T013] [Story] Refine the Rust port in `src/parseopt/parseopt.rs` and `src/parseopt/help.rs` by removing migration placeholders, tightening internal signatures, and aligning naming and control flow with idiomatic Rust while preserving C module behavior. Depends on: T012.
- [T014] [Story] Perform final module-level review of `src/parseopt/parseopt.rs` and `src/parseopt/help.rs` to ensure the 10 migrated functions and supporting data structures are fully covered without duplicated work across phases. Depends on: T013.