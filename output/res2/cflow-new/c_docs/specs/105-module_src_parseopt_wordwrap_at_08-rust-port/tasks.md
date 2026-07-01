# Tasks: module_src_parseopt_wordwrap_at_08

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the port of `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, and expose it from the existing `src/parseopt/mod.rs` module tree.
- [T002] [P] [Story] Review `src/parseopt/wordwrap.c` and map the 18 C data structures and the 2 functions to Rust ownership, lifetime, and visibility decisions, recording the implementation targets directly in `src/parseopt/wordwrap.rs`.
- [T003] [Story] Ensure the branch-local Rust project wiring compiles with an empty `src/parseopt/wordwrap.rs` module stub before data structure migration begins. Depends on: T001

## Phase 2: Foundational

- [T004] [Story] Port the core word-wrapping state and option-holding data structures from `src/parseopt/wordwrap.c` into Rust definitions in `src/parseopt/wordwrap.rs`, preserving field groupings and relationships needed by both target functions. Depends on: T003
- [T005] [P] [Story] Port the supporting helper data structures, enums, and constant-backed record layouts from `src/parseopt/wordwrap.c` into Rust definitions in `src/parseopt/wordwrap.rs`, covering the remaining structures required by the module. Depends on: T003
- [T006] [Story] Reconcile the foundational Rust data model in `src/parseopt/wordwrap.rs` so all 18 migrated structures interoperate without placeholder types, and align constructor/default patterns with how the C module initializes wrapping state. Depends on: T004, T005

## Phase 3: Functions

- [T007] [Story] Implement the function group responsible for word-wrap state preparation and input configuration in `src/parseopt/wordwrap.rs`, using the migrated data structures and matching the control flow of the corresponding C logic. Depends on: T006
- [T008] [Story] Implement the function group responsible for line-wrapping/output progression in `src/parseopt/wordwrap.rs`, completing the second and final function port from `src/parseopt/wordwrap.c` against the shared state model. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Refine `src/parseopt/wordwrap.rs` by removing temporary migration scaffolding, tightening signatures and visibility to module-local needs, and aligning naming/documentation comments with the completed Rust port. Depends on: T008
- [T010] [Story] Perform a final compile-focused pass on `src/parseopt/wordwrap.rs` and `src/parseopt/mod.rs` to resolve integration issues introduced by the migrated word-wrap module and confirm the module is ready for downstream use. Depends on: T009