# Tasks: module_src_parseopt_optsort_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parseopt/help.c` port on branch `104-module_src_parseopt_optsort_07-rust-port`, adding the target source file `src/parseopt/help.rs` and wiring it into the existing Rust module tree.
- [T002] [P] [Story] Review `src/parseopt/help.c` and enumerate the Rust-visible items to port in `src/parseopt/help.rs`, separating the 46 data structures from the 2 functions so later implementation stays aligned with the source module.
- [T003] [Story] Define the initial public/private item layout in `src/parseopt/help.rs` for option-sorting related types and functions, preserving names and grouping needed for the C-to-Rust migration. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Port the foundational constants, enums, aliases, and plain record-like data structures from `src/parseopt/help.c` into Rust definitions in `src/parseopt/help.rs`, covering the module’s low-dependency type layer first. Depends on: T003
- [T005] [P] [Story] Port pointer-linked, nested, or collection-carrying data structures from `src/parseopt/help.c` into Rust ownership-aware definitions in `src/parseopt/help.rs`, using the foundational type layer established for the module. Depends on: T004
- [T006] [Story] Reconcile all 46 migrated data structures in `src/parseopt/help.rs`, filling in cross-references, visibility, default construction choices, and placeholder field typing needed before function porting begins. Depends on: T004, T005

## Phase 3: Functions

- [T007] [Story] Implement the first option/help parsing support function from `src/parseopt/help.c` in `src/parseopt/help.rs`, using the completed Rust data structures and preserving the original module-local behavior. Depends on: T006
- [T008] [Story] Implement the second option/help sorting or formatting support function from `src/parseopt/help.c` in `src/parseopt/help.rs`, completing the functional port of this module and reusing shared type definitions without duplicating logic. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Refine `src/parseopt/help.rs` by removing temporary placeholders introduced during the port, tightening signatures and visibility, and ensuring the translated data structures and 2 functions form a coherent Rust module. Depends on: T008
- [T010] [Story] Perform a final module pass on `src/parseopt/help.rs` to resolve Rust compiler issues specific to the `src/parseopt/help.c` migration, including idiomatic pattern cleanup that does not change established behavior. Depends on: T009