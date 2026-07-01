# Tasks: main_root_setlocale_null_04

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/setlocale_null.rs` and declare the module from `src/main.rs` for the `setlocale_null.c` / `setlocale_null-unlocked.c` migration target.
- [T002] [Story] Establish the Rust entry integration in `src/main.rs` so the migrated `main`-cluster logic can call into `src/setlocale_null.rs` without changing scope beyond this module. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Add foundational Rust definitions in `src/setlocale_null.rs` for the module’s shared locale-handling flow, limited to the minimal internal aliases/helpers needed to support the migrated functions from `setlocale_null.c` and `setlocale_null-unlocked.c`. Depends on: T001.

## Phase 3: Functions

### Phase 3A: Locale query and wrapper functions

- [T004] [P] [Story] Implement the direct locale-query function(s) migrated from `setlocale_null.c` in `src/setlocale_null.rs`, preserving the null-locale lookup behavior required by this module. Depends on: T003.
- [T005] [P] [Story] Implement the unlocked/wrapper variant function(s) migrated from `setlocale_null-unlocked.c` in `src/setlocale_null.rs`, keeping behavior aligned with the paired locale-query logic and avoiding duplicate implementation paths. Depends on: T003.
- [T006] [Story] Consolidate shared logic between the `setlocale_null.c` and `setlocale_null-unlocked.c` function groups inside `src/setlocale_null.rs` so each migrated C function has a single Rust implementation site. Depends on: T004, T005.

### Phase 3B: Main-cluster integration functions

- [T007] [Story] Implement the remaining main-cluster support function group in `src/setlocale_null.rs` for this module’s non-entry helper functions, covering the rest of the migrated function set from the two C files. Depends on: T006.
- [T008] [Story] Implement the module’s `main`-path function migration in `src/main.rs`, wiring it to the Rust functions in `src/setlocale_null.rs` and preserving the original main-cluster behavior for this module. Depends on: T002, T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/setlocale_null.rs` and `src/main.rs` to remove redundant branching introduced during migration, keep locale-handling code idiomatic Rust, and ensure the final file layout cleanly reflects the split source origins without expanding module scope. Depends on: T008.