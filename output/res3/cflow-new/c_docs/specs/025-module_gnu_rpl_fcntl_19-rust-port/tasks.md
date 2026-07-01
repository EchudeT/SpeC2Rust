# Tasks: module_gnu_rpl_fcntl_19

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/fcntl.c` port in `src/gnu/fcntl.rs`, and register it from the existing Rust module tree used by the `cflow-new` crate for branch `025-module_gnu_rpl_fcntl_19-rust-port`.
- [T002] [P] [Story] Establish the initial Rust file structure in `src/gnu/fcntl.rs` for this module port, including placeholders for the module data structure and the two function implementations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the single data structure inferred from `gnu/fcntl.c` in `src/gnu/fcntl.rs`, preserving the C module’s role and field layout semantics as closely as Rust allows for the port. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the first function from `gnu/fcntl.c` in `src/gnu/fcntl.rs`, using the Phase 2 data-structure definition where required and keeping behavior aligned with the source module. Depends on: T003.
- [T005] [P] [Story] Implement the second function from `gnu/fcntl.c` in `src/gnu/fcntl.rs`, using the Phase 2 data-structure definition where required and keeping behavior aligned with the source module. Depends on: T003.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/fcntl.rs` to remove placeholder code, tighten signatures and visibility to match actual module usage, and ensure the completed Rust port remains cohesive with the original `gnu/fcntl.c` module scope. Depends on: T004, T005.