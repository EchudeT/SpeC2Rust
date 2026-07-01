# Tasks: module_gnu_rlimit_08

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/getdtablesize.c` in `src/module_cluster/module_gnu_rlimit_08.rs`, and expose it from the existing module tree on branch `014-module_gnu_rlimit_08-rust-port`.
- [T002] [P] [Story] Add placeholder public items in `src/module_cluster/module_gnu_rlimit_08.rs` for the module’s 1 data structure and 2 functions so later implementation work can proceed with stable names.
- [T003] [Story] Review the C source behavior in `gnu/getdtablesize.c` and map each exported item to its Rust counterpart in `src/module_cluster/module_gnu_rlimit_08.rs` before implementation begins. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Implement the module’s required foundational data structure from `gnu/getdtablesize.c` in `src/module_cluster/module_gnu_rlimit_08.rs`, preserving the C-visible field intent and Rust ownership semantics. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Implement the primary file-descriptor table size retrieval logic translated from `gnu/getdtablesize.c` in `src/module_cluster/module_gnu_rlimit_08.rs`, using the Phase 2 data structure where required. Depends on: T004
- [T006] [Story] Implement the remaining helper or fallback function from `gnu/getdtablesize.c` in `src/module_cluster/module_gnu_rlimit_08.rs`, keeping behavior aligned with the original module and avoiding duplicate logic with the primary retrieval path. Depends on: T004

## Final Phase: Polish

- [T007] [P] [Story] Refine `src/module_cluster/module_gnu_rlimit_08.rs` by removing placeholder code, tightening internal visibility, and consolidating shared logic between the two translated functions without changing module behavior. Depends on: T005, T006
- [T008] [Story] Perform a final pass on `src/module_cluster/module_gnu_rlimit_08.rs` to verify the module compiles cleanly within the existing project module tree and that the migrated file remains scope-aligned with `gnu/getdtablesize.c`. Depends on: T007