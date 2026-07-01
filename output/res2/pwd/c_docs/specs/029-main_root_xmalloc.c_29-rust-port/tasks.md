# Tasks: main_root_xmalloc.c_29

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `xmalloc.c` port in `src/xmalloc.rs`, and register it from the existing crate entry point used by the `pwd` project on branch `029-main_root_xmalloc.c_29-rust-port`.
- [T002] [P] [Story] Establish the function migration surface in `src/xmalloc.rs` by adding placeholders/signatures for the 6 functions from `xmalloc.c`, keeping names and responsibilities aligned with the C module for later implementation. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the foundational allocation/error-handling patterns needed by this module in `src/xmalloc.rs`, covering the shared internal behavior that the migrated allocation helpers will reuse. Depends on: T001

## Phase 3: Core allocation functions

- [T004] [Story] Implement the basic allocation functions from `xmalloc.c` in `src/xmalloc.rs`, grouping the functions that provide direct memory allocation behavior and wiring them to the shared logic from this module. Depends on: T003
- [T005] [P] [Story] Implement the zero-initialized allocation functions from `xmalloc.c` in `src/xmalloc.rs`, grouping the functions that provide cleared allocation behavior and reusing the same module-level failure path. Depends on: T003

## Phase 4: Resizing and duplication functions

- [T006] [Story] Implement the reallocation function from `xmalloc.c` in `src/xmalloc.rs`, preserving the source module’s resize semantics through the shared allocation/error-handling logic. Depends on: T003
- [T007] [P] [Story] Implement the remaining helper function from `xmalloc.c` in `src/xmalloc.rs` that completes the module’s 6-function migration set, keeping its behavior local to this file and aligned with the original C responsibility. Depends on: T003

## Final Phase: Polish

- [T008] [Story] Review `src/xmalloc.rs` to remove placeholder code, tighten function signatures and visibility, and ensure the 6 migrated functions are consistently organized as the Rust port of `xmalloc.c`. Depends on: T004, T005, T006, T007