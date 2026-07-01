# Task List: module_src_posix.c_33

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/posix.c` port in `src/posix.rs`, and wire the module into the crate from the existing Rust module tree on branch `096-module_src_posix.c_33-rust-port`.
- [T002] [P] [Story] Review `src/posix.c` and map the 1 data structure and 2 functions into Rust items to be implemented in `src/posix.rs`; record the target Rust item names and grouping directly in module comments or developer notes in `src/posix.rs`.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust representation of the single data structure identified from `src/posix.c` in `src/posix.rs`, preserving the C module’s field layout and usage semantics as required by the two module functions. Depends on: T001, T002

## Phase 3: Functions

- [T004] [Story] Implement the first function from `src/posix.c` in `src/posix.rs`, using the Phase 2 data structure where applicable and preserving the original module-local POSIX behavior. Depends on: T003
- [T005] [Story] Implement the second function from `src/posix.c` in `src/posix.rs`, grouping it with the same POSIX module port in `src/posix.rs` and matching the original C behavior. Depends on: T003

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/posix.rs` for idiomatic Rust within the bounds of the C port: remove duplication introduced during migration, tighten visibility of the data structure and functions to match module usage, and verify naming and inline documentation are consistent with the `src/posix.c` source intent. Depends on: T004, T005