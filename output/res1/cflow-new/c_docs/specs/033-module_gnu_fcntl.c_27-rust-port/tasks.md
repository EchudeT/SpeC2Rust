# Tasks: module_gnu_fcntl.c_27

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/fcntl.c` port in `src/gnu/fcntl.rs`, and expose it from the nearest existing module entry point for the `gnu` namespace.
- [T002] [P] [Story] Review `gnu/fcntl.c` and map the single exported function and single data structure to Rust items to be implemented in `src/gnu/fcntl.rs`; record any constant or type dependencies that must stay local to this module.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure from `gnu/fcntl.c` in `src/gnu/fcntl.rs`, preserving its field layout and module-local visibility requirements inferred from the C source. Depends on: T001, T002.

## Phase 3: Functions

- [T004] [Story] Implement the single function from `gnu/fcntl.c` in `src/gnu/fcntl.rs`, using the Phase 2 data structure where required and keeping behavior aligned with the original module’s GNU `fcntl` logic. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/gnu/fcntl.rs` by resolving compile-time integration issues, tightening item visibility, and removing any temporary migration scaffolding introduced during the port. Depends on: T004.