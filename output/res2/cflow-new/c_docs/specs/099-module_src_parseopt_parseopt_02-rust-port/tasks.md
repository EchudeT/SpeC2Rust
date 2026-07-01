# Tasks: module_src_parseopt_parseopt_02

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the parseopt port by adding `src/parseopt/parseopt.rs` and wiring it into the crate module tree so the migrated implementation from `src/parseopt/parseopt.c` has a dedicated target file.
- [T002] [P] [Story] Define the initial Rust-side module surface in `src/parseopt/parseopt.rs`, including placeholder public/private item organization for parse-option data structures and function groups identified from `src/parseopt/parseopt.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the core parse-option record types and enums from `src/parseopt/parseopt.c` into Rust data definitions in `src/parseopt/parseopt.rs`, covering the primary option descriptors, parser state, parse results, and flag/category constants needed by all function groups. Depends on: T002.
- [T004] [P] [Story] Port supporting helper structs, small records, and related type aliases/constants from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, completing the remaining foundational data structure layer required by the module. Depends on: T003.
- [T005] [Story] Establish Rust ownership, lifetime, and mutability conventions for the migrated parseopt data structures in `src/parseopt/parseopt.rs`, including constructor/default patterns only where directly needed to support function implementation. Depends on: T003, T004.

## Phase 3: Option Table and Parser State Functions

- [T006] [Story] Implement the function group in `src/parseopt/parseopt.rs` responsible for creating, initializing, or resetting parse-option tables and parser state from the migrated C logic. Depends on: T005.
- [T007] [Story] Implement the function group in `src/parseopt/parseopt.rs` that updates parser state while traversing arguments, including index/cursor movement and active-option tracking behavior from `src/parseopt/parseopt.c`. Depends on: T006.
- [T008] [P] [Story] Implement the helper function group in `src/parseopt/parseopt.rs` that resolves option metadata lookups within the configured option table, matching the C module’s internal search and selection behavior. Depends on: T005.

## Phase 4: Argument Classification and Value Parsing Functions

- [T009] [Story] Implement the function group in `src/parseopt/parseopt.rs` that classifies incoming arguments as short options, long options, option terminators, or positional arguments according to `src/parseopt/parseopt.c`. Depends on: T007, T008.
- [T010] [Story] Implement the function group in `src/parseopt/parseopt.rs` that parses option-attached values and separated argument values, preserving the original module’s rules for presence, absence, and consumption of values. Depends on: T009.
- [T011] [P] [Story] Implement internal conversion/normalization helpers in `src/parseopt/parseopt.rs` used by value parsing, such as canonicalizing raw argument fragments into the module’s internal representation where evidenced by the C code. Depends on: T005.

## Phase 5: High-Level Parse Flow Functions

- [T012] [Story] Implement the main parse execution function group in `src/parseopt/parseopt.rs` that coordinates argument iteration, option lookup, classification, and value handling to produce module parse results equivalent to `src/parseopt/parseopt.c`. Depends on: T010, T011.
- [T013] [Story] Implement the function group in `src/parseopt/parseopt.rs` that handles non-option/positional argument collection and final parser completion state, following the C module’s end-of-parse behavior. Depends on: T012.
- [T014] [Story] Implement the function group in `src/parseopt/parseopt.rs` that exposes the migrated public parseopt entry points and return-value behavior expected from this module’s interface. Depends on: T012, T013.

## Final Phase: Polish

- [T015] [Story] Refine `src/parseopt/parseopt.rs` by removing migration scaffolding, consolidating duplicated helper logic introduced during the port, and aligning naming/documentation comments with the finalized Rust implementation while preserving the original C module behavior. Depends on: T014.