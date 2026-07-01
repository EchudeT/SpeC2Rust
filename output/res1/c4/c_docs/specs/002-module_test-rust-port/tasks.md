# Tasks: module_test Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust target files for the `module_test` port by adding module source files corresponding to `test/c4.c` and `test/hello.c`.
  - Files: `src/test/c4.rs`, `src/test/hello.rs`
  - Dependencies: none

- [T002] [Story] Wire the new Rust module files into the crate module tree so `module_test` builds on branch `002-module_test-rust-port`.
  - Files: `src/test/mod.rs` or `src/lib.rs`
  - Dependencies: T001

## Phase 2: Foundational

- [T003] [Story] Review `test/c4.c` and `test/hello.c` and define the minimal shared Rust module-level items needed before function migration, keeping parity with the C module and avoiding new unsupported abstractions.
  - Files: `src/test/c4.rs`, `src/test/hello.rs`
  - Dependencies: T002

## Phase 3: Functions

- [T004] [P] [Story] Port the functions implemented in `test/hello.c` into `src/test/hello.rs`, preserving the original module behavior and function boundaries.
  - Files: `src/test/hello.rs`
  - Dependencies: T003

- [T005] [P] [Story] Port the functions implemented in `test/c4.c` into `src/test/c4.rs`, preserving the original module behavior and function boundaries.
  - Files: `src/test/c4.rs`

- [T006] [Story] Resolve any direct call relationships between the migrated `c4` and `hello` functions and finalize imports/visibility so the full `module_test` module compiles coherently.
  - Files: `src/test/c4.rs`, `src/test/hello.rs`, `src/test/mod.rs` or `src/lib.rs`
  - Dependencies: T004, T005

## Final Phase: Polish

- [T007] [Story] Perform a final cleanup pass on the migrated `module_test` Rust files to remove redundant translation artifacts and align naming, signatures, and file organization with the existing crate conventions without changing behavior.
  - Files: `src/test/c4.rs`, `src/test/hello.rs`, `src/test/mod.rs` or `src/lib.rs`
  - Dependencies: T006

- [T008] [Story] Verify the `module_test` Rust port builds cleanly on branch `002-module_test-rust-port` and fix any remaining module-local compile issues in the migrated files.
  - Dependencies: T007