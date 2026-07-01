# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for this migration unit in `src/hard_locale.rs` and register it from the crate entry used by the `pwd` project so the ported code can be compiled from the `019-main_root_hard_locale.c_19-rust-port` branch.
- [T002] [P] [Story] Add a migration placeholder in `src/hard_locale.rs` for the functionality from `hard-locale.c`, keeping the module boundary aligned with the original C file.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust API surface in `src/hard_locale.rs` needed by the port of `hard-locale.c`, including function signature(s), required imports, and internal constants/helpers only if directly needed by the module function implementation. Depends on: T001, T002.

## Phase 3: Functions

- [T004] [Story] Implement the locale-hardness evaluation function from `hard-locale.c` in `src/hard_locale.rs`, preserving the original module behavior and keeping the logic scoped to this single-file migration unit. Depends on: T003.
- [T005] [P] [Story] Integrate call-site visibility for the implemented `hard-locale.c` function through the crate entry or module exposure path already used by the `pwd` Rust project, using only the necessary edits tied to `src/hard_locale.rs`. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/hard_locale.rs` to remove migration placeholders, tighten imports and visibility, and ensure the final Rust module remains minimal and consistent with the single-function scope of `hard-locale.c`. Depends on: T004, T005.