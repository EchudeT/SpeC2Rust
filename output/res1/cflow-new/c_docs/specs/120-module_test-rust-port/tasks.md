# Tasks: module_test Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the `module_test` port on branch `120-module_test-rust-port`, adding `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` to mirror `test/simple.c`, `test/recursion.c`, and `test/multi.c`.
- [T002] [P] [Story] Wire the new module files into the Rust crate module tree so `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` are compiled and accessible. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `test/simple.c`, `test/recursion.c`, and `test/multi.c` and define any shared Rust type aliases, helper enums, constants, or internal helper signatures required by their 11 functions, placing each item only in the directly corresponding target file under `src/test/`. Depends on: T002.

## Phase 3: Simple module functions

- [T004] [Story] Port the functions from `test/simple.c` into `src/test/simple.rs`, preserving the original control-flow behavior and function boundaries from the C module. Depends on: T003.

## Phase 4: Recursion module functions

- [T005] [Story] Port the functions from `test/recursion.c` into `src/test/recursion.rs`, preserving recursive behavior and original function boundaries from the C module. Depends on: T003.

## Phase 5: Multi-file test functions

- [T006] [P] [Story] Port the functions from `test/multi.c` into `src/test/multi.rs`, preserving the original control-flow behavior and any file-local helpers inferred from the source. Depends on: T003.

## Final Phase: Polish

- [T007] [Story] Refine the Rust implementations in `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` to remove C-specific artifacts, resolve compile issues, and align signatures and visibility across the module without changing behavior. Depends on: T004, T005, T006.
- [T008] [Story] Run a final pass on `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` to simplify idiomatic Rust control flow and eliminate duplicated helper logic introduced during migration. Depends on: T007.