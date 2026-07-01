# Tasks: module_gnu_itold.c_32

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the C source migration in `src/gnu/itold.rs`, mirroring `gnu/itold.c` within branch `038-module_gnu_itold.c_32-rust-port`.
- [T002] [Story] Expose the migrated module from the Rust module tree by updating the nearest inferable module declaration file to include `src/gnu/itold.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/itold.c` and define the minimal Rust-side type aliases, imports, and internal helper constants needed directly by the migrated implementation in `src/gnu/itold.rs`. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Port the integer-to-long-double conversion functionality from `gnu/itold.c` into its Rust equivalent in `src/gnu/itold.rs`, preserving the original module-local behavior and signature mapping. Depends on: T003.
- [T005] [P] [Story] Align the Rust implementation in `src/gnu/itold.rs` with existing project numeric conventions by resolving any required module-local imports or call-site-visible re-exports that are directly needed for the migrated function to compile cleanly. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/itold.rs` for idiomatic Rust within the limits of the source migration, removing redundant C-style constructs while preserving behavior. Depends on: T004.
- [T007] [Story] Perform a final compile-focused pass on the migrated files touched for this module to verify module declarations and the `src/gnu/itold.rs` implementation remain consistent and buildable together. Depends on: T002, T005, T006.