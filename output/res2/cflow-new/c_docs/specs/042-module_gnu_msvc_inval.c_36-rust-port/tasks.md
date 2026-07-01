# Task List: module_gnu_msvc-inval.c_36

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module file for the `gnu/msvc-inval.c` port in `src/gnu/msvc_inval.rs`.
- [ ] T002 [Story] Register the new module in the Rust project’s module tree so `src/gnu/msvc_inval.rs` is compiled.
  - Depends on: T001

## Phase 2: Foundational

- [ ] T003 [P] [Story] Identify and define the Rust equivalents for the module-local data structures referenced by `gnu/msvc-inval.c` in `src/gnu/msvc_inval.rs`.
  - Notes: cover all 7 evidenced data structures before function porting.
  - Depends on: T001
- [ ] T004 [Story] Add the required constants, type aliases, and supporting field definitions needed by the ported data structures in `src/gnu/msvc_inval.rs`.
  - Depends on: T003

## Phase 3: Functions

- [ ] T005 [Story] Port the function implemented in `gnu/msvc-inval.c` into idiomatic Rust in `src/gnu/msvc_inval.rs`, wiring it to the previously defined Rust data structures and preserving the original module behavior.
  - Depends on: T004

## Final Phase: Polish

- [ ] T006 [Story] Refine `src/gnu/msvc_inval.rs` by removing migration scaffolding, tightening visibility, and aligning naming and organization with the Rust project’s existing conventions.
  - Depends on: T005