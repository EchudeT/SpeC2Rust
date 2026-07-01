# Task List: module_src_parseopt_file_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/main.c` port on branch `070-module_src_parseopt_file_07-rust-port`, adding the target Rust source file `src/main.rs`.
- [T002] [P] [Story] Define the initial module layout in `src/main.rs` for this ported area, reserving sections for parse-option data structures and the two function implementations.
- [T003] [Story] Review the C content in `src/main.c` and map the 63 data structures and 2 functions into a Rust port plan documented inline in `src/main.rs` comments as migration anchors. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Port the core parse-option related type definitions from `src/main.c` into Rust data structures in `src/main.rs`, preserving ownership and field relationships needed by the module functions. Depends on: T003
- [T005] [P] [Story] Port supporting enums, flags, and constant-style structural definitions from `src/main.c` into `src/main.rs` so the core parse-option types compile cleanly. Depends on: T003
- [T006] [Story] Complete the remaining supporting structs/unions-equivalent representations required to cover the full set of 63 C data structures in `src/main.rs`. Depends on: T004, T005
- [T007] [Story] Resolve cross-references among the ported data structures in `src/main.rs`, including pointer-like links, optional fields, and borrowed versus owned representations required by the original module layout. Depends on: T006

## Phase 3: Functions

- [T008] [Story] Implement the first parse-option file handling function from `src/main.c` in `src/main.rs`, using the already ported data structures and keeping behavior aligned with the original module logic. Depends on: T007
- [T009] [Story] Implement the second parse-option file handling function from `src/main.c` in `src/main.rs`, completing the module’s function port with the same data-structure contracts. Depends on: T007
- [T010] [Story] Integrate the two ported functions in `src/main.rs` by resolving shared helper flow, argument/state handoff, and direct use of the migrated parse-option structures without reintroducing C-specific patterns. Depends on: T008, T009

## Final Phase: Polish

- [T011] [P] [Story] Refine the Rust definitions and function bodies in `src/main.rs` to remove redundant placeholders left from the migration scaffold and to align naming and visibility with the ported module scope. Depends on: T010
- [T012] [Story] Perform a final compile-focused cleanup in `src/main.rs`, simplifying obvious C-to-Rust translation artifacts while keeping the implementation limited to behavior evidenced by `src/main.c`. Depends on: T011