# Tasks: module_gnu_rlimit_08

## Phase 1: Setup

- [T001] [Story] Create the module Rust source file at `src/gnu/getdtablesize.rs` and register it from the existing GNU module tree on branch `014-module_gnu_rlimit_08-rust-port`.
- [T002] [Story] Establish the module skeleton in `src/gnu/getdtablesize.rs` for the `gnu/getdtablesize.c` migration, including placeholders for the module data structure and the two exported/internal function implementations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the single module-local data structure inferred from `gnu/getdtablesize.c` in `src/gnu/getdtablesize.rs`, keeping its layout and responsibility aligned with the source module. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the primary `getdtablesize`-related function logic in `src/gnu/getdtablesize.rs`, using the Phase 2 data structure where required and preserving the behavior of `gnu/getdtablesize.c`. Depends on: T003.
- [T005] [P] [Story] Implement the remaining helper/supporting function from `gnu/getdtablesize.c` in `src/gnu/getdtablesize.rs`, grouped with the main file migration and reusing the established module structure. Depends on: T003.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/getdtablesize.rs` by removing migration placeholders, tightening signatures and visibility, and ensuring the ported functions and data structure are consistently organized for the completed module. Depends on: T004, T005.