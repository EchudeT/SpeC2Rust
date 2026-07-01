# Tasks: module_gnu_xmalloc.c_57

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port workspace for this module on branch `063-module_gnu_xmalloc.c_57-rust-port`, and create the target source file `src/gnu/xmalloc.rs` corresponding to `gnu/xmalloc.c`.
- [T002] [P] [Story] Wire `src/gnu/xmalloc.rs` into the crate module tree so the Rust project exposes the migrated `gnu::xmalloc` module.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust allocation helpers and module-level imports in `src/gnu/xmalloc.rs` needed to support the `gnu/xmalloc.c` function ports, keeping interfaces aligned with the C module responsibilities. Depends on: T001, T002.

## Phase 3: Core allocation functions

- [T004] [Story] Port the primary allocation routines from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, grouping the base memory-allocation functions that provide the module’s core behavior. Depends on: T003.
- [T005] [P] [Story] Port the resize/reallocation-related functions from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, keeping them consistent with the foundational allocation helpers already established. Depends on: T003.
- [T006] [Story] Integrate the remaining helper function from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs` so the full set of 5 module functions is available without duplicating responsibilities across phases. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/xmalloc.rs` for idiomatic Rust structure, remove migration leftovers, and verify the module compiles cleanly with the rest of the Rust project. Depends on: T006.