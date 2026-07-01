# Tasks: module_gnu_reallocarray.c_44

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the ported implementation at `src/gnu/reallocarray.rs`, mirroring `gnu/reallocarray.c`.
- [T002] [Story] Expose the new module from the Rust crate by adding the corresponding `mod` declaration in the nearest Rust module entry file required to reach `src/gnu/reallocarray.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Establish the foundational API surface in `src/gnu/reallocarray.rs` for the reallocarray port, including the Rust function signature and any internal helper layout needed for checked size computation prior to allocation work.

## Phase 3: Functions

- [T004] [Story] Implement the checked reallocation logic from `gnu/reallocarray.c` in `src/gnu/reallocarray.rs`, including multiplication overflow detection for element count and element size before delegating to the underlying Rust allocation or reallocation path. Depends on: T003.
- [T005] [P] [Story] Integrate the exported reallocarray function into the surrounding Rust module namespace so callers can use the ported API through the crate’s existing `gnu` module path. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Review `src/gnu/reallocarray.rs` and the related module export file for signature consistency, minimal unsafe usage, and alignment with the original C module behavior without expanding scope. Depends on: T004, T005.