# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the `main_root_clear_ungetc_09` port by adding `src/fflush.rs` and exposing it from `src/lib.rs` or `src/main.rs` according to the existing crate layout on branch `009-main_root_clear_ungetc_09-rust-port`.
- [T002] [P] [Story] Add placeholder item declarations in `src/fflush.rs` for the two functions migrated from `fflush.c`, keeping signatures and visibility aligned with the surrounding Rust port conventions. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `fflush.c` and map any required existing Rust-side stream or input-buffer state dependencies needed by this module, then wire `src/fflush.rs` to use those existing crate types without introducing new data structures. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the root clear operation from `fflush.c` in `src/fflush.rs`, preserving the original module behavior and integrating with the crate’s existing stream state abstractions. Depends on: T003.
- [T005] [Story] Implement the ungetc-related clear operation from `fflush.c` in `src/fflush.rs`, grouping the remaining migrated function logic with the same stream state handling used by this module. Depends on: T003.

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/fflush.rs` to remove migration placeholders, resolve any duplication introduced during the two function ports, and ensure the file follows existing Rust project idioms without changing module scope. Depends on: T004, T005.