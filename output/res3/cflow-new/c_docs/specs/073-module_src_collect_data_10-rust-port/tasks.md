# Tasks: module_src_collect_data_10

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the `src/symbol.c` port on branch `073-module_src_collect_data_10-rust-port`, adding the target Rust file `src/symbol.rs` and wiring it into the crate module tree where this C file is being migrated.
- [T002] [Story] Review `src/symbol.c` and enumerate the 27 migrated data structures and 2 functions into a Rust-side implementation checklist inside `src/symbol.rs` as porting placeholders and migration comments. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the core foundational Rust type definitions in `src/symbol.rs` for the direct C-to-Rust equivalents required by `src/symbol.c`, including base structs, enums, aliases, and constant representations that other migrated items depend on. Depends on: T002
- [T004] [P] [Story] Implement the remaining plain data-holder structures from `src/symbol.c` in `src/symbol.rs`, preserving field relationships and ownership layout needed for later function ports. Depends on: T003
- [T005] [P] [Story] Implement pointer-linked, nested, or cross-referential data structures from `src/symbol.c` in `src/symbol.rs`, choosing Rust representations that preserve the original module semantics without expanding scope beyond the source file. Depends on: T003
- [T006] [Story] Consolidate the full set of 27 migrated data structures in `src/symbol.rs`, resolve inter-structure references, and ensure the file exposes the internal items needed by the upcoming function implementations. Depends on: T004, T005

## Phase 3: Functions

- [T007] [Story] Port the first `src/symbol.c` function into `src/symbol.rs`, implementing its symbol/data-collection behavior against the completed Rust data structures. Depends on: T006
- [T008] [Story] Port the second `src/symbol.c` function into `src/symbol.rs`, implementing the remaining symbol/data-collection behavior and reusing the same Rust data model established for this module. Depends on: T006
- [T009] [Story] Integrate and reconcile shared helper logic used by the two migrated functions directly within `src/symbol.rs`, removing placeholder logic and aligning call relationships to the original `src/symbol.c` flow. Depends on: T007, T008

## Final Phase: Polish

- [T010] [Story] Refine `src/symbol.rs` for Rust idioms and module completeness by cleaning temporary migration scaffolding, tightening visibility, and simplifying obvious C-style patterns that are no longer needed after the port. Depends on: T009
- [T011] [Story] Perform a final pass on `src/symbol.rs` to verify the migrated data structures and both functions remain scoped to the original `src/symbol.c` responsibilities and that no unevidenced module behavior was introduced. Depends on: T010