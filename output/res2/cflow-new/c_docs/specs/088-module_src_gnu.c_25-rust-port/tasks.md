# Tasks: module_src_gnu.c_25 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/gnu.c` in `src/gnu.rs` and register it from the crate root or parent module so the ported module is compiled on branch `088-module_src_gnu.c_25-rust-port`.
- [T002] [P] [Story] Review `src/gnu.c` and map its single data structure and single function into Rust module items in `src/gnu.rs`, documenting the intended Rust names and ownership model in code comments before implementation.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure from `src/gnu.c` in `src/gnu.rs`, preserving the C module’s field semantics and visibility needed by the module function.
  - Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the single function from `src/gnu.c` in `src/gnu.rs`, translating its logic to operate on the Rust data structure and keeping behavior aligned with the original module contract.
  - Depends on: T003

## Final Phase: Polish

- [T005] [Story] Refine `src/gnu.rs` by removing temporary migration comments, tightening signatures and visibility to the minimum required, and ensuring the module reads idiomatically while preserving the original `src/gnu.c` behavior.
  - Depends on: T004