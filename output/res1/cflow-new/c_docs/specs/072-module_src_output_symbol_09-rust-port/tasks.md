# Tasks: module_src_output_symbol_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffolding for this port in `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`, and wire the modules into the crate so later symbol-output migration work has target files.
- [T002] [Story] Establish shared type placeholders and imports needed by this module cluster in `src/output.rs`, with module-level references prepared for `src/gnu.rs` and `src/posix.rs`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port and define the data structures owned by the output-symbol flow in `src/output.rs`, covering the module-local structs, enums, and aliases needed before function migration. Depends on: T002
- [T004] [P] [Story] Port GNU-specific supporting data structures required by the symbol-output path in `src/gnu.rs`, keeping them aligned with the interfaces established in `src/output.rs`. Depends on: T003
- [T005] [P] [Story] Port POSIX-specific supporting data structures required by the symbol-output path in `src/posix.rs`, keeping them aligned with the interfaces established in `src/output.rs`. Depends on: T003

## Phase 3: Symbol output functions

- [T006] [Story] Implement the core symbol-output function(s) migrated from `src/output.c` in `src/output.rs`, using the foundational data structures as the shared interface for this module cluster. Depends on: T003
- [T007] [P] [Story] Implement the GNU-side symbol formatting/output function migrated from `src/gnu.c` in `src/gnu.rs`, integrating with the shared output data structures and core flow. Depends on: T004, T006
- [T008] [P] [Story] Implement the POSIX-side symbol formatting/output function migrated from `src/posix.c` in `src/posix.rs`, integrating with the shared output data structures and core flow. Depends on: T005, T006

## Final Phase: Polish

- [T009] [Story] Refine the cross-module interfaces and eliminate migration-time placeholders across `src/output.rs`, `src/gnu.rs`, and `src/posix.rs` so the three symbol-output implementations are consistent and minimal. Depends on: T007, T008
- [T010] [Story] Perform a final cleanup pass on `src/output.rs`, `src/gnu.rs`, and `src/posix.rs` to simplify Rust ownership/borrowing usage and remove dead porting artifacts introduced during migration. Depends on: T009