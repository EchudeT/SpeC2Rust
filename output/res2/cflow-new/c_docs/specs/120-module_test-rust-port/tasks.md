# Tasks: module_test Rust port

**Input**: C module analysis for `module_test`
**Branch**: `120-module_test-rust-port`

## Phase 1: Setup

- [ ] T001 [Story] Initialize the Rust module file layout for the `module_test` port by creating `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` to mirror `test/simple.c`, `test/recursion.c`, and `test/multi.c`.
- [ ] T002 [Story] Wire the new module files into the Rust crate module tree so `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` are compiled from their parent module declarations. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Define the foundational module-level function signatures and shared visibility boundaries in `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` so the 11 migrated functions have stable Rust entry points before implementation. Depends on: T002

## Phase 3: Simple file functions

- [ ] T004 [P] [Story] Port the non-recursive functions from `test/simple.c` into `src/test/simple.rs`, preserving the original function-level behavior and keeping the implementation scoped to this source file. Depends on: T003

## Phase 4: Recursion file functions

- [ ] T005 [P] [Story] Port the recursive functions from `test/recursion.c` into `src/test/recursion.rs`, preserving the original recursion behavior and base/termination conditions in Rust. Depends on: T003

## Phase 5: Multi file functions

- [ ] T006 [P] [Story] Port the grouped functions from `test/multi.c` into `src/test/multi.rs`, preserving the original per-function behavior and any intra-file call relationships. Depends on: T003

## Final Phase: Polish

- [ ] T007 [Story] Review `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` to remove C-specific migration artifacts, align function signatures and visibility with actual module use, and ensure the port remains idiomatic without changing behavior. Depends on: T004, T005, T006
- [ ] T008 [Story] Run a final compile pass on the integrated `module_test` Rust port and resolve any remaining module-path or cross-file integration issues in `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs`. Depends on: T007