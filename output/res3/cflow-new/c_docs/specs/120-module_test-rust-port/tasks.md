# Tasks: module_test Rust port

## Phase 1: Setup

- [ ] [T001] [Story] Initialize the Rust module scaffold for `module_test` on branch `120-module_test-rust-port`, creating `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` to mirror `test/simple.c`, `test/recursion.c`, and `test/multi.c`.
- [ ] [T002] [Story] Wire the new module files into the Rust crate module tree from `src/test/mod.rs` so `simple`, `recursion`, and `multi` are compiled and available to the project. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [P] [Story] Review `test/simple.c`, `test/recursion.c`, and `test/multi.c` and define any shared function signatures, common aliases, or internal helper declarations directly in `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` before porting behavior. Depends on: T002

## Phase 3: Simple function port

- [ ] [T004] [Story] Port the functions from `test/simple.c` into `src/test/simple.rs`, preserving the C module’s behavior and keeping the implementation scoped to this file. Depends on: T003

## Phase 4: Recursion function port

- [ ] [T005] [Story] Port the functions from `test/recursion.c` into `src/test/recursion.rs`, preserving recursive behavior from the C implementation and keeping the implementation scoped to this file. Depends on: T003

## Phase 5: Multi-file function port

- [ ] [T006] [Story] Port the functions from `test/multi.c` into `src/test/multi.rs`, preserving the C module’s behavior and keeping the implementation scoped to this file. Depends on: T003
- [ ] [T007] [P] [Story] Resolve any direct call relationships among `src/test/simple.rs`, `src/test/recursion.rs`, and `src/test/multi.rs` by updating module visibility and imports only where required by the original C module behavior. Depends on: T004, T005, T006

## Final Phase: Polish

- [ ] [T008] [Story] Run a final compile-and-cleanup pass across `src/test/simple.rs`, `src/test/recursion.rs`, `src/test/multi.rs`, and `src/test/mod.rs`, removing porting leftovers and aligning signatures and visibility with the completed Rust implementation. Depends on: T007