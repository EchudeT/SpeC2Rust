# Tasks: module_gnu_gl_msvc_inval_per_thread_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the port of `gnu/msvc-inval.c` in `src/gnu/msvc_inval.rs`, and declare/export it from the nearest existing `src/gnu/mod.rs` or crate root module file so the new module is compiled on branch `013-module_gnu_gl_msvc_inval_per_thread_07-rust-port`.
- [T002] [P] [Story] Establish the module skeleton in `src/gnu/msvc_inval.rs` with placeholders for the 7 translated data structures and 4 translated functions, keeping names and grouping aligned to the source module for later implementation.

## Phase 2: Foundational

- [T003] [Story] Translate and define the core 7 data structures from `gnu/msvc-inval.c` into Rust in `src/gnu/msvc_inval.rs`, preserving their original roles, field relationships, and module-local visibility requirements. Depends on: T001, T002.
- [T004] [Story] Add foundational constants, type aliases, and internal helper state directly required by those translated data structures in `src/gnu/msvc_inval.rs`, only where evidenced by `gnu/msvc-inval.c`. Depends on: T003.

## Phase 3: Per-thread invalid-parameter state access functions

- [T005] [Story] Implement the function group in `src/gnu/msvc_inval.rs` that initializes, retrieves, or exposes the per-thread invalid-parameter state represented by the translated data structures from `gnu/msvc-inval.c`. Depends on: T003, T004.
- [T006] [P] [Story] Implement the companion function group in `src/gnu/msvc_inval.rs` that updates, installs, or clears the module’s invalid-parameter handler/state for the current thread, matching the behavior expressed in `gnu/msvc-inval.c`. Depends on: T003, T004.

## Phase 4: Module behavior completion

- [T007] [Story] Implement the remaining exported or module-visible functions from `gnu/msvc-inval.c` in `src/gnu/msvc_inval.rs`, completing the full set of 4 translated functions without duplicating responsibilities already covered in T005-T006. Depends on: T005, T006.
- [T008] [Story] Reconcile interactions among the translated functions and data structures in `src/gnu/msvc_inval.rs`, ensuring call flow, ownership/borrowing choices, and any lazy or static module state match the original module logic. Depends on: T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/gnu/msvc_inval.rs` by removing placeholders, tightening visibility, and simplifying Rust control flow while preserving the C module’s behavior and keeping the implementation scoped strictly to `gnu/msvc-inval.c`. Depends on: T008.
- [T010] [Story] Perform a final compile-oriented review of the module integration points in `src/gnu/msvc_inval.rs` and the corresponding module declaration file updated in T001, confirming the Rust port is complete and consistently wired into the project. Depends on: T009.