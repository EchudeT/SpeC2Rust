# Task List: `main_root_setlocale_null_05`

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/bin/setlocale_null.rs` as the target migration unit for `setlocale_null.c` and `setlocale_null-unlocked.c`.
- [T002] [Story] Register `src/bin/setlocale_null.rs` in the Rust project on branch `005-main_root_setlocale_null_05-rust-port`, preserving this module as a standalone main-program migration target.

## Phase 2: Foundational

- [T003] [Story] Establish the foundational module structure in `src/bin/setlocale_null.rs` for the 7 migrated functions, including internal helper/function boundaries inferred from `setlocale_null.c` and `setlocale_null-unlocked.c`. Depends on: T001, T002.

## Phase 3: Locale access and unlocked variant functions

- [T004] [P] [Story] Port the locale-query helper logic from `setlocale_null-unlocked.c` into `src/bin/setlocale_null.rs`, keeping behavior aligned with the unlocked variant semantics. Depends on: T003.
- [T005] [P] [Story] Port the remaining non-entry locale access functions from `setlocale_null.c` into `src/bin/setlocale_null.rs`, grouping the shared setlocale-null behavior in one implementation pass. Depends on: T003.
- [T006] [Story] Reconcile the shared logic between the migrated `setlocale_null.c` and `setlocale_null-unlocked.c` function groups inside `src/bin/setlocale_null.rs` so each of the 7 functions has a single clear Rust implementation boundary without duplicating behavior. Depends on: T004, T005.

## Phase 4: Main entry integration

- [T007] [Story] Port the main-program entry logic from `setlocale_null.c` into `src/bin/setlocale_null.rs`, wiring it to the migrated locale-related functions and preserving module-level execution flow. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/bin/setlocale_null.rs` by removing migration-only duplication, tightening function visibility, and ensuring the final file cleanly represents the combined port of `setlocale_null.c` and `setlocale_null-unlocked.c`. Depends on: T007.