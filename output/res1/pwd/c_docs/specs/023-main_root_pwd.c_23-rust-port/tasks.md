# Tasks: Rust port of `main_root_pwd.c_23`

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `pwd.c` port in `src/main_root_pwd.rs`, and expose it from `src/main.rs` or `src/lib.rs` according to the existing crate entry layout for branch `023-main_root_pwd.c_23-rust-port`.
- [T002] [P] [Story] Add the top-level item skeletons in `src/main_root_pwd.rs` for the module’s 18 data structures and 2 functions so later implementation can proceed without changing file layout. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust representations for the `pwd.c` module data structures in `src/main_root_pwd.rs`, preserving the C module’s main-program-oriented state and argument/environment handling shapes required by the port. Depends on: T002.
- [T004] [P] [Story] Define supporting enums, type aliases, constants, and helper structs in `src/main_root_pwd.rs` that are directly required by the `pwd.c` data model, keeping them colocated with the primary data structures. Depends on: T003.
- [T005] [Story] Wire ownership/borrowing and constructor/default patterns for the implemented `pwd.c` data structures in `src/main_root_pwd.rs` so the function port can consume them without placeholder state. Depends on: T003, T004.

## Phase 3: Functions

- [T006] [Story] Port the primary path/reporting function logic from `pwd.c` into `src/main_root_pwd.rs`, using the completed module data structures and preserving the original main-cluster control flow. Depends on: T005.
- [T007] [Story] Port the remaining module-level function from `pwd.c` into `src/main_root_pwd.rs`, grouping it with the primary routine only where it directly supports the same command execution flow. Depends on: T005.
- [T008] [Story] Integrate the two ported functions within the module entry/control path in `src/main_root_pwd.rs`, resolving shared state flow and removing any remaining implementation stubs from setup. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/main_root_pwd.rs` for idiomatic Rust within the proven `pwd.c` behavior scope, simplifying signatures, removing redundant placeholders, and ensuring the final file cleanly represents the migrated module. Depends on: T008.