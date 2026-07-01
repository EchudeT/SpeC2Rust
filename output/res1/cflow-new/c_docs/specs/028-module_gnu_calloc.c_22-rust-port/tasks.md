# Tasks: module_gnu_calloc.c_22

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/calloc.c` in `src/gnu/calloc.rs`, and register it from the existing GNU module entry point such as `src/gnu/mod.rs` so the ported implementation has a direct target location.
- [T002] [P] [Story] Review the C source `gnu/calloc.c` and map its required Rust-side imports and dependencies within `src/gnu/calloc.rs`, keeping scope limited to items directly needed by the module function implementation.
- [T003] [Story] Verify the branch `028-module_gnu_calloc.c_22-rust-port` builds with the new module wiring in place before function migration begins. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Establish any module-local type aliases, constants, or helper signatures directly evidenced by `gnu/calloc.c` inside `src/gnu/calloc.rs`, avoiding introduction of new data structures not required by the source analysis. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Port the allocation function implemented by `gnu/calloc.c` into `src/gnu/calloc.rs`, preserving the original module behavior and keeping the implementation grouped as the sole function migration for this module. Depends on: T004.
- [T006] [P] [Story] Update any direct call sites or module exports made necessary by the migrated allocation function so the Rust module exposes the ported entry point through the same inferred GNU module surface, using only directly related files such as `src/gnu/mod.rs`. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/calloc.rs` for idiomatic Rust clarity and remove any migration-only scaffolding that is no longer needed after the function port is wired and compiling. Depends on: T005, T006.
- [T008] [Story] Run final formatting and compile verification for the touched Rust files `src/gnu/calloc.rs` and `src/gnu/mod.rs` to confirm the module port integrates cleanly. Depends on: T007.