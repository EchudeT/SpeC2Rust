# Tasks: module_gnu_xmalloc.c_57

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/xmalloc.c` port on branch `063-module_gnu_xmalloc.c_57-rust-port`, adding the target source file at `src/gnu/xmalloc.rs`.
- [T002] [P] [Story] Register the new module file `src/gnu/xmalloc.rs` in the existing Rust module tree so it can host the ported `gnu/xmalloc.c` functionality.
- [T003] [Story] Review `gnu/xmalloc.c` and map its 5 exported/internal functions to Rust function stubs in `src/gnu/xmalloc.rs`, preserving original grouping and intended visibility for later implementation. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Define the foundational Rust items in `src/gnu/xmalloc.rs` needed by the ported allocation helpers, including shared imports, type aliases, constants, and internal helper declarations directly evidenced by `gnu/xmalloc.c`. Depends on: T003

## Phase 3: Allocation wrapper functions

- [T005] [Story] Implement the core memory-allocation wrapper functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, covering the functions responsible for plain allocation and zero-initialized allocation behavior. Depends on: T004
- [T006] [P] [Story] Implement the reallocation-oriented wrapper functions from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, covering resize and array-sized resize behavior. Depends on: T004
- [T007] [Story] Implement the remaining failure-handling or size-check helper function from `gnu/xmalloc.c` in `src/gnu/xmalloc.rs`, and wire it into the allocation and reallocation wrappers in `src/gnu/xmalloc.rs`. Depends on: T005, T006

## Final Phase: Polish

- [T008] [Story] Refine `src/gnu/xmalloc.rs` to align naming, error paths, and module-level organization with the original `gnu/xmalloc.c` semantics, and remove any temporary porting scaffolding no longer needed. Depends on: T007