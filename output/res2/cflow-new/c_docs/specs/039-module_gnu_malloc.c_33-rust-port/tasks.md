# Tasks: module_gnu_malloc.c_33

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/malloc.c` in `src/gnu/malloc.rs`, and declare it from the existing parent module file that exposes `gnu`.
- [T002] [P] [Story] Review `gnu/malloc.c` and map the contained C function to its Rust target signature and placement in `src/gnu/malloc.rs`; document any required `libc`-level imports directly in that file.
- [T003] [Story] Verify the branch `039-module_gnu_malloc.c_33-rust-port` builds with the new `src/gnu/malloc.rs` module wired in, without adding functionality yet. Depends on: T001.

## Phase 2: Foundational

- [T004] [Story] Confirm that `gnu/malloc.c` introduces no module-specific data structures or persistent state, and keep `src/gnu/malloc.rs` limited to the minimal imports, aliases, and constants required by the function port. Depends on: T002.

## Phase 3: Functions

- [T005] [Story] Port the single function implemented in `gnu/malloc.c` into `src/gnu/malloc.rs`, preserving the original allocation-related behavior, argument handling, return semantics, and any error signaling present in the C source. Depends on: T004.
- [T006] [Story] Integrate the ported function with the surrounding Rust module surface in `src/gnu/malloc.rs`, ensuring its visibility and naming align with the migrated `gnu` module layout and that no duplicate implementation remains elsewhere. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/malloc.rs` to remove unused imports or scaffolding left from migration while preserving the behavior of the ported function. Depends on: T006.
- [T008] [Story] Perform a final module-level review of `src/gnu/malloc.rs` and its parent module declaration to confirm the migration stays scoped to `gnu/malloc.c` and compiles cleanly on branch `039-module_gnu_malloc.c_33-rust-port`. Depends on: T007.