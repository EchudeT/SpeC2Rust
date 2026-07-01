# Tasks: module_gnu_GL_ATTRIBUTE_12

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for this port branch by creating `src/gnu/error.rs`, `src/gnu/hash.rs`, and updating the module exports in `src/gnu/mod.rs` so the migrated files from `gnu/error.c` and `gnu/hash.c` have direct Rust targets.
- [T002] [P] [Story] Define the initial shared module boundaries between `src/gnu/error.rs` and `src/gnu/hash.rs`, including placeholders for the 49 migrated data structures and the 3 functions, so later implementation stays aligned with the original file split. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port and define the data structures owned by `gnu/error.c` into `src/gnu/error.rs`, preserving C-level field layout semantics where needed for correct function migration. Depends on: T002
- [T004] [P] [Story] Port and define the data structures owned by `gnu/hash.c` into `src/gnu/hash.rs`, preserving the original structure relationships needed by the hash-related function implementations. Depends on: T002
- [T005] [Story] Implement shared type aliases, constants, and internal helper representations required by the migrated data structures across `src/gnu/error.rs` and `src/gnu/hash.rs`, keeping them colocated with the owning file and avoiding cross-file duplication. Depends on: T003, T004

## Phase 3: Error-file Functions

- [T006] [Story] Implement the function group from `gnu/error.c` in `src/gnu/error.rs`, wiring it to the migrated `error.c` data structures and local constants without extending behavior beyond the original module scope. Depends on: T003, T005

## Phase 4: Hash-file Functions

- [T007] [Story] Implement the function group from `gnu/hash.c` in `src/gnu/hash.rs`, using the migrated hash data structures and any file-local helpers required by the original C logic. Depends on: T004, T005
- [T008] [P] [Story] Implement the remaining hash-related function from `gnu/hash.c` in `src/gnu/hash.rs`, grouped with the other hash functionality but kept as a distinct task only if it is independently migratable within the same file. Depends on: T004, T005

## Final Phase: Polish

- [T009] [Story] Refine the migrated implementations in `src/gnu/error.rs` and `src/gnu/hash.rs` by removing temporary placeholders, tightening visibility to module-local where possible, and resolving compile-time integration issues introduced during the file migration. Depends on: T006, T007, T008
- [T010] [Story] Review `src/gnu/mod.rs`, `src/gnu/error.rs`, and `src/gnu/hash.rs` for final module consistency, ensuring the data structures and all 3 migrated functions are exposed only as required by the Rust port. Depends on: T009