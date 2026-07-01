# Tasks: module_gnu_msvc-inval.c_36

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/msvc-inval.c` in `src/gnu/msvc_inval.rs`, and register it from the existing `src/gnu/mod.rs` or `src/lib.rs` so the ported module is compiled.
- [T002] [P] [Story] Add the module-level item layout in `src/gnu/msvc_inval.rs` for the 7 data structures and 1 function identified for this C module, preserving a migration-oriented structure and naming strategy derived from `gnu/msvc-inval.c`.
- [T003] [Story] Verify the branch-local build wiring for the new module path `src/gnu/msvc_inval.rs`, resolving any compile-time visibility/import issues introduced by the new module registration. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Port the first group of foundational data structures from `gnu/msvc-inval.c` into Rust definitions in `src/gnu/msvc_inval.rs`, covering the core base types and aliases needed before function implementation. Depends on: T003
- [T005] [P] [Story] Port the remaining data structures from `gnu/msvc-inval.c` into Rust definitions in `src/gnu/msvc_inval.rs`, including any C-compatible enums, structs, unions, constants, or function-pointer representations evidenced by the source module. Depends on: T003
- [T006] [Story] Reconcile the full set of 7 ported data structures in `src/gnu/msvc_inval.rs`, aligning field types, visibility, derives, and internal relationships so the module function can be implemented against stable Rust definitions. Depends on: T004, T005

## Phase 3: Functions

- [T007] [Story] Implement the module’s single function from `gnu/msvc-inval.c` in `src/gnu/msvc_inval.rs`, using the previously ported Rust data structures and preserving the original module behavior as closely as the C source requires. Depends on: T006
- [T008] [Story] Integrate any module-local helper constants, conditional logic, or compatibility handling directly required by the function implementation in `src/gnu/msvc_inval.rs`, without expanding beyond behavior evidenced by `gnu/msvc-inval.c`. Depends on: T007

## Final Phase: Polish

- [T009] [Story] Refine `src/gnu/msvc_inval.rs` by removing migration-only placeholders, tightening signatures and internal visibility, and resolving warnings introduced during the data-structure and function port. Depends on: T008
- [T010] [Story] Perform a final module review of `src/gnu/msvc_inval.rs` and its registration site in `src/gnu/mod.rs` or `src/lib.rs` to confirm the C module migration is complete, deduplicated, and limited to the scope of `gnu/msvc-inval.c`. Depends on: T009