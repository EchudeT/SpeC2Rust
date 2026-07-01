# Tasks: module_gnu_reallocarray.c_44

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the ported implementation in `src/gnu/reallocarray.rs`, mapped from `gnu/reallocarray.c`.
- [T002] [Story] Expose the new module from the nearest Rust module tree by updating `src/gnu/mod.rs` to declare `reallocarray`.
- [T003] [Story] Wire the module into the crate module tree by updating the appropriate parent module file, if required by the existing `src/gnu/mod.rs` placement. Depends on: T002

## Phase 2: Foundational

- [T004] [Story] Define the foundational internal helper logic in `src/gnu/reallocarray.rs` needed to support checked element-count and element-size multiplication before allocation resizing.
- [T005] [Story] Define the Rust-side result/error shape in `src/gnu/reallocarray.rs` required to represent overflow-detected reallocation outcomes, if the surrounding crate does not already provide an equivalent type. Depends on: T004

## Phase 3: Functions

- [T006] [Story] Implement the `reallocarray` port in `src/gnu/reallocarray.rs`, preserving the C module behavior of performing overflow-checked size computation before delegating to reallocation logic. Depends on: T004, T005
- [T007] [P] [Story] Update internal call sites within `src/gnu/reallocarray.rs` to use the shared checked-size helper so the function implementation stays aligned with the original module’s single responsibility. Depends on: T006

## Final Phase: Polish

- [T008] [Story] Review `src/gnu/reallocarray.rs` and related module declarations for idiomatic Rust naming, minimal visibility, and removal of any migration scaffolding no longer needed after function implementation. Depends on: T006, T007
- [T009] [Story] Run crate-level formatting adjustments on the touched Rust files `src/gnu/reallocarray.rs` and `src/gnu/mod.rs`, plus any directly updated parent module file from T003. Depends on: T008