# Tasks: module_gnu_free.c_28

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/free.rs` for the port of `gnu/free.c`.
- [T002] [Story] Register the new module from the crate module tree so `src/gnu/free.rs` is compiled on branch `034-module_gnu_free.c_28-rust-port`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/free.c` and define any module-local aliases, constants, or helper signatures required for the Rust port directly in `src/gnu/free.rs`. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Port the single function from `gnu/free.c` into `src/gnu/free.rs`, preserving the original module behavior and adapting memory-release semantics to safe/idiomatic Rust where possible. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/gnu/free.rs` by removing migration scaffolding, tightening imports and visibility, and ensuring the implementation is consistent with surrounding Rust module conventions. Depends on: T004.