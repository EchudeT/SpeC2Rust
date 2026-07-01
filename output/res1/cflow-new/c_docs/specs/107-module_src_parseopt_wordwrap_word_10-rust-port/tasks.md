# Tasks: module_src_parseopt_wordwrap_word_10

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parseopt/wordwrap.c` migration in `src/parseopt/wordwrap.rs`, and expose it from the existing `src/parseopt/mod.rs` if needed for branch `107-module_src_parseopt_wordwrap_word_10-rust-port`.
- [T002] [Story] Define the migration surface for this module in `src/parseopt/wordwrap.rs` by adding the Rust-visible module section, placeholders for the 18 data structures, and placeholders for the 2 function ports so later implementation stays confined to the source module. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the foundational word-wrapping state/data definitions from `src/parseopt/wordwrap.c` into Rust data structures in `src/parseopt/wordwrap.rs`, preserving field relationships and ownership layout needed by the module logic. Depends on: T002.
- [T004] [P] [Story] Port the supporting option/format descriptor data structures referenced by the word-wrapping logic from `src/parseopt/wordwrap.c` into Rust definitions in `src/parseopt/wordwrap.rs`, keeping names and grouping aligned with the C module. Depends on: T002.
- [T005] [P] [Story] Port the remaining helper/container data structures from `src/parseopt/wordwrap.c` into Rust definitions in `src/parseopt/wordwrap.rs`, including any enums, flags, and aggregate records required before function implementation. Depends on: T002.
- [T006] [Story] Reconcile the full set of 18 migrated data structures in `src/parseopt/wordwrap.rs`, replacing placeholders with complete definitions and aligning shared types so both module functions can be implemented without adding new structural work later. Depends on: T003, T004, T005.

## Phase 3: Functions

- [T007] [Story] Implement the core word-selection and wrapping function from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, using the migrated data structures and preserving the original module-local behavior. Depends on: T006.
- [T008] [Story] Implement the companion formatting/output function from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, wiring it to the core wrapping logic without duplicating responsibilities across phases. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/parseopt/wordwrap.rs` by removing migration placeholders, tightening signatures and visibility to the module’s actual usage, and ensuring the Rust port remains scoped to the original `src/parseopt/wordwrap.c` responsibilities. Depends on: T008.