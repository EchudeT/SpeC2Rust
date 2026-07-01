# Tasks: main_root_setlocale_null_04

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/main_root_setlocale_null_04.rs` and register it from the crate entry point used by the `cat` Rust port branch `005-main_root_setlocale_null_04-rust-port` so the ported `setlocale_null.c` and `setlocale_null-unlocked.c` logic has a dedicated compilation unit.
- [T002] [P] [Story] Add initial module skeletons in `src/main_root_setlocale_null_04.rs` for the function groups inferred from `setlocale_null.c` and `setlocale_null-unlocked.c`, keeping locked and unlocked locale-query paths organized for later direct migration. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust representations in `src/main_root_setlocale_null_04.rs` needed by this module’s seven functions, limited to shared aliases, constants, and helper types directly required to express the setlocale-null query behavior from `setlocale_null.c` and `setlocale_null-unlocked.c`. Depends on: T002.
- [T004] [P] [Story] Add shared internal helper scaffolding in `src/main_root_setlocale_null_04.rs` for common locale-category handling and null-result propagation so the function ports can reuse one implementation path without duplicating migration logic. Depends on: T003.

## Phase 3: Locked setlocale-null functions

- [T005] [Story] Port the locked setlocale-null function group from `setlocale_null.c` into `src/main_root_setlocale_null_04.rs`, implementing the public-facing locale query functions that use the standard locked path and wiring them to the shared helpers. Depends on: T004.
- [T006] [Story] Port any remaining internal support functions from `setlocale_null.c` into `src/main_root_setlocale_null_04.rs`, completing the locked-path migration for all functions assigned to that source file without duplicating behavior already covered by shared helpers. Depends on: T005.

## Phase 4: Unlocked setlocale-null functions

- [T007] [Story] Port the unlocked setlocale-null function group from `setlocale_null-unlocked.c` into `src/main_root_setlocale_null_04.rs`, implementing the unlocked locale query entry points and mapping them onto the shared category/null-handling foundation. Depends on: T004.
- [T008] [P] [Story] Integrate the unlocked-path support functions from `setlocale_null-unlocked.c` into `src/main_root_setlocale_null_04.rs`, finishing migration of all remaining functions from that file while keeping unlocked-specific behavior isolated from the locked path where required. Depends on: T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/main_root_setlocale_null_04.rs` to remove migration duplication between locked and unlocked implementations, align naming and visibility with the Rust port conventions, and ensure all seven ported functions are cleanly organized by source-file-derived responsibility. Depends on: T006, T008.