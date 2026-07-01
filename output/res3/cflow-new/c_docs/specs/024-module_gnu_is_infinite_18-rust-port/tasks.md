# Tasks: module_gnu_is_infinite_18

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/vasnprintf.c` migration in `src/gnu/vasnprintf.rs`.
- [T002] [Story] Expose the new module from the existing Rust module tree by updating the nearest inferable module declaration file for `src/gnu/vasnprintf.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and port the single data structure used by this module from `gnu/vasnprintf.c` into Rust definitions in `src/gnu/vasnprintf.rs`. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Port the `isfinite`-related helper logic from `gnu/vasnprintf.c` into Rust in `src/gnu/vasnprintf.rs`, preserving module-local behavior and C control flow as needed. Depends on: T003.
- [T005] [Story] Port the `isnan`/floating-point classification companion logic from `gnu/vasnprintf.c` into Rust in `src/gnu/vasnprintf.rs`, keeping it aligned with the migrated `isfinite` handling. Depends on: T003.
- [T006] [P] [Story] Integrate the two migrated floating-point classification functions into the Rust `src/gnu/vasnprintf.rs` implementation paths that correspond to their original use in `gnu/vasnprintf.c`. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Review `src/gnu/vasnprintf.rs` for idiomatic Rust cleanup, remove any migration-only scaffolding left after function porting, and ensure the module remains limited to behavior evidenced by `gnu/vasnprintf.c`. Depends on: T006.