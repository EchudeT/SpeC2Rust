# Tasks: module_src_parseopt_position_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module skeleton for the ported parseopt position logic in `src/parseopt/wordwrap.rs`, and expose it from the existing parseopt module tree used by branch `103-module_src_parseopt_position_06-rust-port`.
- [T002] [P] [Story] Review `src/parseopt/wordwrap.c` and map the 6 functions and 18 data structures to Rust items to be implemented in `src/parseopt/wordwrap.rs`; record naming and grouping decisions as implementation comments in that file.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust data structures required by `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, covering all 18 module-local structures, enums, aliases, and state holders needed before function porting. Depends on: T001, T002.
- [T004] [P] [Story] Add core constructors, default state initialization, and internal helper methods for the new wordwrap-related Rust data structures in `src/parseopt/wordwrap.rs`, limited to behavior directly required by the C module. Depends on: T003.

## Phase 3: Core Position and Wrap State Functions

- [T005] [Story] Port the low-level functions from `src/parseopt/wordwrap.c` that initialize, reset, or update wrap/position tracking state into `src/parseopt/wordwrap.rs`, preserving the original module behavior. Depends on: T003, T004.
- [T006] [P] [Story] Port the helper functions from `src/parseopt/wordwrap.c` that compute or adjust wordwrap positioning decisions based on the foundational state structures in `src/parseopt/wordwrap.rs`. Depends on: T003, T004.
- [T007] [Story] Integrate the state-management and position-calculation function group inside `src/parseopt/wordwrap.rs`, resolving shared data flow and removing remaining C-style assumptions incompatible with Rust ownership and borrowing. Depends on: T005, T006.

## Phase 4: Remaining Module Functions

- [T008] [Story] Port the remaining higher-level functions from `src/parseopt/wordwrap.c` that consume the wrap/position state and produce the module’s final position-related behavior in `src/parseopt/wordwrap.rs`. Depends on: T007.
- [T009] [P] [Story] Align all translated function signatures, internal visibility, and call sites within `src/parseopt/wordwrap.rs` so the complete 6-function module API is internally consistent and ready for use by adjacent parseopt code. Depends on: T008.

## Final Phase: Polish

- [T010] [Story] Refine `src/parseopt/wordwrap.rs` by removing redundant translation scaffolding, simplifying control flow where it preserves C behavior, and ensuring idiomatic Rust organization for the completed module. Depends on: T009.
- [T011] [Story] Perform a final module pass on `src/parseopt/wordwrap.rs` to verify all 6 functions and 18 data structures from `src/parseopt/wordwrap.c` have been migrated exactly once and that task-scoped implementation remains limited to this module. Depends on: T010.