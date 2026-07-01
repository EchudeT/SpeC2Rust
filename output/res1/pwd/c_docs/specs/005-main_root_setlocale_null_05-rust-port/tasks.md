# Tasks: main_root_setlocale_null_05

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port workspace on branch `005-main_root_setlocale_null_05-rust-port` and create the target module files `src/bin/setlocale_null.rs` and `src/bin/setlocale_null_unlocked.rs` to mirror `setlocale_null.c` and `setlocale_null-unlocked.c`.
- [T002] [P] [Story] Wire both binary entry files `src/bin/setlocale_null.rs` and `src/bin/setlocale_null_unlocked.rs` with minimal `main` scaffolding and module-level placeholders for the functions ported from the corresponding C sources.

## Phase 2: Foundational

- [T003] [Story] Review `setlocale_null.c` and `setlocale_null-unlocked.c` for shared constants, helper aliases, and locale-handling call patterns, then define the shared foundational items once in the directly owning Rust file(s) `src/bin/setlocale_null.rs` and `src/bin/setlocale_null_unlocked.rs` as appropriate. Depends on: T001, T002.

## Phase 3: Locale query and validation functions

- [T004] [Story] Port the locale-query helper functions from `setlocale_null.c` into `src/bin/setlocale_null.rs`, preserving the original null-argument behavior and return-path semantics. Depends on: T003.
- [T005] [P] [Story] Port the corresponding locale-query helper functions from `setlocale_null-unlocked.c` into `src/bin/setlocale_null_unlocked.rs`, preserving the unlocked variant behavior and null-argument handling from the source file. Depends on: T003.

## Phase 4: Program entry flow

- [T006] [Story] Implement the `main` program flow in `src/bin/setlocale_null.rs` so it invokes the ported helpers in the same order and with the same observable control flow as `setlocale_null.c`. Depends on: T004.
- [T007] [P] [Story] Implement the `main` program flow in `src/bin/setlocale_null_unlocked.rs` so it invokes the ported helpers in the same order and with the same observable control flow as `setlocale_null-unlocked.c`. Depends on: T005.

## Final Phase: Polish

- [T008] [Story] Refine `src/bin/setlocale_null.rs` and `src/bin/setlocale_null_unlocked.rs` to remove duplicated placeholder code, align shared locale-handling logic with the final ported behavior, and ensure the two binaries remain faithful file-by-file migrations of their C counterparts. Depends on: T006, T007.