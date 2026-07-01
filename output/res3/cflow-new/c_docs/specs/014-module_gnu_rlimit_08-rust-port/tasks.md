# Tasks: module_gnu_rlimit_08

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/getdtablesize.c` migration in `src/module_gnu_rlimit_08.rs`, and expose it from the crate root or containing module tree as required by the branch structure.
- [T002] [P] [Story] Add the file-level module documentation in `src/module_gnu_rlimit_08.rs` describing the scope of the port from `gnu/getdtablesize.c` and limiting implementation to the inferred module surface.

## Phase 2: Foundational

- [T003] [Story] Define the single foundational data structure inferred from `gnu/getdtablesize.c` in `src/module_gnu_rlimit_08.rs`, matching the C module’s rlimit-related shape closely enough to support the function ports. Depends on: T001.
- [T004] [Story] Add constructor or conversion helpers on the foundational data structure in `src/module_gnu_rlimit_08.rs` only if required to support the direct function implementations from `gnu/getdtablesize.c`. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the rlimit retrieval helper function from `gnu/getdtablesize.c` in `src/module_gnu_rlimit_08.rs`, using the foundational data structure defined for this module. Depends on: T003, T004.
- [T006] [Story] Implement the exported `getdtablesize` behavior from `gnu/getdtablesize.c` in `src/module_gnu_rlimit_08.rs`, grouping the final descriptor-limit computation with the helper logic from the same source file. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/module_gnu_rlimit_08.rs` to remove redundant intermediate logic introduced during porting and ensure the final Rust code remains narrowly aligned with the semantics of `gnu/getdtablesize.c`. Depends on: T006.
- [T008] [Story] Review the public/private visibility of items in `src/module_gnu_rlimit_08.rs` so that only the ported module surface required by `gnu/getdtablesize.c` remains exposed. Depends on: T007.