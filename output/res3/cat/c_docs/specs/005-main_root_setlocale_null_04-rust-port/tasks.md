# Task List: main_root_setlocale_null_04

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/setlocale_null.rs` to host the port of logic from `setlocale_null.c` and `setlocale_null-unlocked.c`.
- [T002] [Story] Wire the new module into the crate from `src/main.rs` or `src/lib.rs`, whichever is the existing crate root for the Rust port branch, so the `setlocale_null` module is compiled.
- [T003] [P] [Story] Add function stubs in `src/setlocale_null.rs` for the 7 module functions identified from `setlocale_null.c` and `setlocale_null-unlocked.c`, preserving the source-level grouping by file for later implementation.

## Phase 2: Foundational

- [T004] [Story] Review `setlocale_null.c` and `setlocale_null-unlocked.c` and define any shared internal helper aliases, constants, or minimal private support items directly in `src/setlocale_null.rs` that are required before implementing the functions. Depends on: T001, T003

## Phase 3: Functions

- [T005] [Story] Implement the function group in `src/setlocale_null.rs` corresponding to `setlocale_null.c`, covering the locale-query behavior migrated from that source file. Depends on: T004
- [T006] [P] [Story] Implement the function group in `src/setlocale_null.rs` corresponding to `setlocale_null-unlocked.c`, covering the unlocked locale-query behavior migrated from that source file. Depends on: T004
- [T007] [Story] Reconcile shared logic between the `setlocale_null.c` and `setlocale_null-unlocked.c` ports in `src/setlocale_null.rs` so each of the 7 functions has one complete Rust implementation with no duplicated unfinished paths. Depends on: T005, T006

## Final Phase: Polish

- [T008] [Story] Refine `src/setlocale_null.rs` for idiomatic Rust naming, visibility, and small internal simplifications while preserving the original module behavior. Depends on: T007
- [T009] [Story] Perform a final compile-pass update in `src/setlocale_null.rs` and the crate root file (`src/main.rs` or `src/lib.rs`) to remove dead stubs and ensure the migrated module builds cleanly on branch `005-main_root_setlocale_null_04-rust-port`. Depends on: T008