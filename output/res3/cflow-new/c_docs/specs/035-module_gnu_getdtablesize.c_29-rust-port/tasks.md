# Tasks: module_gnu_getdtablesize.c_29

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/getdtablesize.rs` and register it from the existing GNU module tree so the ported implementation has a direct destination matching `gnu/getdtablesize.c`.
- [T002] [P] [Story] Add the branch-local module wiring needed for `src/gnu/getdtablesize.rs` to compile within the current crate layout, keeping the registration limited to the `gnu` module path.

## Phase 2: Foundational

- [T003] [Story] Define the data structure inferred from `gnu/getdtablesize.c` in `src/gnu/getdtablesize.rs`, preserving only the fields and visibility required to support the module’s single function implementation. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Implement the core Rust equivalent of the single function from `gnu/getdtablesize.c` in `src/gnu/getdtablesize.rs`, using the Phase 2 data structure where required and keeping behavior aligned with the source module’s GNU-specific purpose. Depends on: T003.
- [T005] [P] [Story] Integrate any module-local imports, constants, and internal helper wiring directly required by the function implementation in `src/gnu/getdtablesize.rs`, without expanding beyond dependencies evidenced by `gnu/getdtablesize.c`. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Review `src/gnu/getdtablesize.rs` for idiomatic Rust cleanup, remove any migration scaffolding left from the port, and ensure the final module remains narrowly scoped to the original `gnu/getdtablesize.c` behavior. Depends on: T005.