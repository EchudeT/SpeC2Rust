# Tasks: main_root_copy-file-range.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `copy-file-range.c` in `src/copy_file_range.rs`, and register it from the crate root so the module is compiled on branch `022-main_root_copy_file_range.c_21-rust-port`.
  - Depends on: none

## Phase 2: Foundational

- [T002] [Story] Port the single module-level data structure from `copy-file-range.c` into Rust in `src/copy_file_range.rs`, preserving the fields and role needed by the module’s function implementation.
  - Depends on: T001

## Phase 3: Functions

- [T003] [Story] Implement the module’s single `copy_file_range`-related function in `src/copy_file_range.rs`, using the ported data structure and preserving the C module behavior at the Rust module level.
  - Depends on: T002

## Final Phase: Polish

- [T004] [Story] Refine `src/copy_file_range.rs` by resolving Rust compile issues, tightening signatures and internal ownership/borrowing choices, and removing any migration-only scaffolding left from the port.
  - Depends on: T003