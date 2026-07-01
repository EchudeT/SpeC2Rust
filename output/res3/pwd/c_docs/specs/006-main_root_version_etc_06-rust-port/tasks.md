# Tasks: main_root_version_etc_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `version-etc.c` port on branch `006-main_root_version_etc_06-rust-port`, adding the target source file at `src/version_etc.rs`.
- [T002] [P] [Story] Register the new module file `src/version_etc.rs` from the crate root so the ported implementation is compiled and available to the main cluster. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust-side constants, type aliases, and internal helper signatures needed to support the four functions ported from `version-etc.c` in `src/version_etc.rs`. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Implement the core version/banner formatting logic from `version-etc.c` in `src/version_etc.rs`, covering the function group that emits program version text and package attribution text. Depends on: T003.
- [T005] [P] [Story] Implement the companion function group in `src/version_etc.rs` that handles variant entry points over the same version-reporting behavior, keeping shared formatting paths factored through the foundational helpers. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/version_etc.rs` to remove duplicated formatting paths, align naming and visibility with the crate’s main-cluster conventions, and verify that all four ported functions are wired through the final Rust implementation without redundant code. Depends on: T004, T005.