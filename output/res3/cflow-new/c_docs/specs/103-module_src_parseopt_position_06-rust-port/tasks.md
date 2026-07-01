# Tasks: module_src_parseopt_position_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `wordwrap` port in `src/parseopt/wordwrap.rs`, and expose it from the existing parseopt module tree on branch `103-module_src_parseopt_position_06-rust-port`.
- [T002] [P] [Story] Review `src/parseopt/wordwrap.c` and map its 6 functions and 18 data structures into Rust implementation targets in `src/parseopt/wordwrap.rs`, documenting the one-to-one migration scope directly in code comments or TODO markers. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust data structure set required by `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, defining the module-local structs, enums, type aliases, and state containers needed to represent the C module’s 18 data structures before any function porting begins. Depends on: T002
- [T004] [P] [Story] Translate constant-style definitions, field defaults, and internal initialization helpers inferable from `src/parseopt/wordwrap.c` into idiomatic Rust forms within `src/parseopt/wordwrap.rs`, keeping names and layout aligned with the original module logic. Depends on: T003

## Phase 3: Core state and position handling functions

- [T005] [Story] Port the function group from `src/parseopt/wordwrap.c` responsible for core word-wrap state setup, position tracking, and internal cursor/offset updates into `src/parseopt/wordwrap.rs`, using the foundational data structures already defined. Depends on: T004
- [T006] [P] [Story] Port the function group from `src/parseopt/wordwrap.c` responsible for evaluating wrap positions and maintaining positional decisions during parsing into `src/parseopt/wordwrap.rs`, preserving module-local behavior and data flow. Depends on: T004

## Phase 4: Text wrapping and output behavior functions

- [T007] [Story] Port the function group from `src/parseopt/wordwrap.c` responsible for line breaking, word wrapping transitions, and output text emission behavior into `src/parseopt/wordwrap.rs`. Depends on: T005, T006
- [T008] [Story] Integrate the remaining function logic from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, ensuring all 6 module functions are migrated exactly once and wired to the translated state model without expanding module scope. Depends on: T007

## Final Phase: Polish

- [T009] [Story] Refine `src/parseopt/wordwrap.rs` for Rust idioms by removing temporary migration markers, tightening visibility to module-local where possible, and simplifying ownership/borrowing patterns without changing behavior. Depends on: T008
- [T010] [Story] Perform a final pass on `src/parseopt/wordwrap.rs` to confirm naming consistency with the source module, eliminate duplicated helper logic introduced during porting, and ensure the migrated implementation remains confined to the original `wordwrap.c` responsibilities. Depends on: T009