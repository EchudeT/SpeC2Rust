# Tasks: main_root_clear_ungetc_08

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for this migration in `src/fflush.rs`, aligned to the source module `fflush.c`.
- [T002] [Story] Wire the new module into the crate from `src/lib.rs` or `src/main.rs` by declaring and exposing `src/fflush.rs` as needed for the `cat` Rust port branch `009-main_root_clear_ungetc_08-rust-port`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `fflush.c` and define the minimal Rust-side foundational items required by its two functions directly in `src/fflush.rs`, keeping scope limited to only constants, helper types, or internal helper signatures evidenced by the C file. Depends on: T001.

## Phase 3: Functions

- [T004] [Story] Implement the first migrated function from `fflush.c` in `src/fflush.rs`, translating its current behavior and keeping any required local helpers private to this module. Depends on: T003.
- [T005] [P] [Story] Implement the second migrated function from `fflush.c` in `src/fflush.rs`, preserving the original module-local behavior and using the shared foundational items established for this file. Depends on: T003.
- [T006] [Story] Reconcile shared logic between the two migrated functions in `src/fflush.rs` so both functions use a consistent Rust implementation without duplicating module-local behavior. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Perform a final pass on `src/fflush.rs` to remove migration-only clutter, tighten visibility, and ensure the file remains focused on the `fflush.c` port with no extra scope added. Depends on: T006.