# Tasks: module_gnu_stat_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the ported sources by adding `src/gnu/stat.rs` and `src/gnu/xmalloc.rs`, and register them from the existing `src/gnu/mod.rs` if needed for the `011-module_gnu_stat_05-rust-port` branch.
- [T002] [P] [Story] Add placeholder public items in `src/gnu/stat.rs` and `src/gnu/xmalloc.rs` that mirror the C module split, so later data structures and functions can be implemented in place. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the two module-local data structures required by `gnu/stat.c` inside `src/gnu/stat.rs`, preserving the C module’s field layout and responsibility at the Rust type level. Depends on: T002.
- [T004] [P] [Story] Add constructors, default initialization, or internal helper impl blocks for the data structures in `src/gnu/stat.rs` only where directly needed to support the ported `gnu/stat.c` functions. Depends on: T003.

## Phase 3: Memory-allocation support functions

- [T005] [Story] Port the function logic from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, implementing the allocation helper behavior required by this module without expanding beyond the source file’s evidenced functions. Depends on: T002.
- [T006] [P] [Story] Adjust visibility and call surfaces between `src/gnu/xmalloc.rs` and `src/gnu/stat.rs` so the allocation helper functions are usable by the stat-related port exactly where required. Depends on: T005.

## Phase 4: Stat functionality

- [T007] [Story] Port the function logic from `gnu/stat.c` into `src/gnu/stat.rs`, wiring it to the two Rust data structures and any required helpers from `src/gnu/xmalloc.rs`. Depends on: T004, T006.
- [T008] [P] [Story] Refine the translated control flow and error/value handling in `src/gnu/stat.rs` to express the original `gnu/stat.c` behavior idiomatically in Rust while preserving semantics. Depends on: T007.

## Final Phase: Polish

- [T009] [Story] Remove temporary placeholders and dead code from `src/gnu/stat.rs` and `src/gnu/xmalloc.rs`, ensuring the module compiles cleanly as an integrated Rust port of `gnu/stat.c` and `gnu/xmalloc.c`. Depends on: T008.
- [T010] [Story] Review the final module organization under `src/gnu/stat.rs` and `src/gnu/xmalloc.rs` to confirm source-to-target migration completeness and consistent naming aligned with the original C module scope. Depends on: T009.