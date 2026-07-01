# Tasks: module_src_gnu.c_25

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/gnu.c` migration in `src/gnu.rs`, and register it from the crate root on branch `088-module_src_gnu.c_25-rust-port`.
- [T002] [Story] Review `src/gnu.c` and define the Rust-side module boundary in `src/gnu.rs`, listing the single data structure and single function to be ported without expanding scope beyond the C source. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure from `src/gnu.c` in `src/gnu.rs`, preserving only the fields and invariants evidenced by the C module. Depends on: T002
- [T004] [P] [Story] Add basic Rust constructors or helper impl blocks in `src/gnu.rs` only where required to support the migrated function’s direct use of the module data structure. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Port the single function from `src/gnu.c` into `src/gnu.rs`, mapping its logic to idiomatic Rust while preserving the C module behavior and using the migrated data structure. Depends on: T003, T004
- [T006] [Story] Integrate the migrated function’s visibility and call surface in `src/gnu.rs` so it matches the module’s required internal/external usage evidenced by `src/gnu.c`. Depends on: T005

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu.rs` for Rust idioms by removing migration-only scaffolding, tightening types, and simplifying control flow without changing the behavior established by the C module. Depends on: T006
- [T008] [Story] Perform a final file-level review of `src/gnu.rs` to confirm the module migration is complete, deduplicated, and limited to the single data structure and single function from `src/gnu.c`. Depends on: T007