# Task List: main_root_hard-locale.c_28

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/hard_locale.rs` for the port of `hard-locale.c`.
- [T002] [Story] Register the new module in `src/main.rs` or the existing main-cluster module wiring so `src/hard_locale.rs` is compiled on branch `029-main_root_hard_locale.c_28-rust-port`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `hard-locale.c` and define the minimal Rust-visible function signature and any module-local constants needed in `src/hard_locale.rs`, keeping the implementation scoped to the single exported behavior from this C file. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Implement the port of the sole function from `hard-locale.c` in `src/hard_locale.rs`, preserving the original locale-related main-cluster behavior and return semantics. Depends on: T003.
- [T005] [P] [Story] Update call sites in `src/main.rs` or the existing main-cluster integration point to use the Rust implementation from `src/hard_locale.rs` in place of the C-module behavior. Depends on: T002, T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/hard_locale.rs` and its integration points for idiomatic Rust naming, visibility, and minimal comments while keeping behavior aligned with `hard-locale.c`. Depends on: T004, T005.