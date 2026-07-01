# Tasks: module_gnu_GL_ATTRIBUTE_12

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module layout for the ported code by creating `src/gnu/error.rs` and `src/gnu/hash.rs`, and register them from the existing Rust module tree for branch `018-module_gnu_GL_ATTRIBUTE_12-rust-port`.
- [T002] [P] [Story] Establish shared module-level type and item visibility needed by both `src/gnu/error.rs` and `src/gnu/hash.rs`, keeping exported names aligned with the C module surface. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the foundational data structures defined in `gnu/error.c` into Rust in `src/gnu/error.rs`, including their field layout, ownership model, and internal helper representations required before any function implementation. Depends on: T001.
- [T004] [Story] Port the foundational data structures defined in `gnu/hash.c` into Rust in `src/gnu/hash.rs`, including their field layout, ownership model, and internal helper representations required before any function implementation. Depends on: T001.
- [T005] [P] [Story] Reconcile shared constants, enums, aliases, and cross-module structural assumptions between `src/gnu/error.rs` and `src/gnu/hash.rs` so the ported data structures compile together without function bodies. Depends on: T003, T004.

## Phase 3: Error module functions

- [T006] [Story] Implement the function group from `gnu/error.c` in `src/gnu/error.rs`, wiring the already ported error-related data structures to reproduce the original module behavior. Depends on: T003, T005.

## Phase 4: Hash module functions

- [T007] [Story] Implement the function group from `gnu/hash.c` in `src/gnu/hash.rs`, wiring the already ported hash-related data structures to reproduce the original module behavior. Depends on: T004, T005.

## Phase 5: Cross-module integration

- [T008] [Story] Integrate the completed `src/gnu/error.rs` and `src/gnu/hash.rs` implementations within the Rust module tree, resolving any direct usage relationships and compile-time interface mismatches introduced during the port. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/gnu/error.rs` and `src/gnu/hash.rs` for idiomatic Rust implementation details that preserve C-module behavior, including removal of redundant scaffolding, tightening visibility, and simplifying internal data handling without expanding scope. Depends on: T008.