# Tasks: module_gnu_memchr.c_35

## Phase 1: Setup

- [T001] [Story] Create the Rust module target for `gnu/memchr.c` by adding a dedicated source file at `src/gnu/memchr.rs` on branch `041-module_gnu_memchr.c_35-rust-port`.
- [T002] [Story] Register the new module in the Rust module tree so `src/gnu/memchr.rs` is compiled from its parent module declarations.
- [T003] [P] [Story] Add a placeholder public API entry in `src/gnu/memchr.rs` for the ported `memchr` functionality, preserving a direct mapping to the C module scope. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Define the foundational Rust function signature and internal byte-scan interface in `src/gnu/memchr.rs` needed to represent the single function from `gnu/memchr.c`. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the Rust port of the `memchr` function in `src/gnu/memchr.rs`, keeping behavior aligned with `gnu/memchr.c` and limiting the work to this module’s single function migration. Depends on: T004.
- [T006] [P] [Story] Perform an intra-file integration pass in `src/gnu/memchr.rs` to ensure the exported function name, visibility, and call shape are consistent with the surrounding Rust module tree after the `memchr` implementation is added. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/memchr.rs` by removing placeholder code, tightening the implementation structure, and simplifying any migration-only scaffolding left from the port. Depends on: T006.
- [T008] [Story] Review the completed `src/gnu/memchr.rs` module for idiomatic Rust cleanup and small local optimizations that do not expand scope beyond the original `gnu/memchr.c` behavior. Depends on: T007.