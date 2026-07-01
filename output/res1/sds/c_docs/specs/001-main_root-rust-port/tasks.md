# Tasks: main_root Rust Port

**Input**: C analysis for module `main_root` (`sds.c`)
**Branch**: `001-main_root-rust-port`

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for the `main_root` port by creating `src/main_root.rs` and wiring it into the crate root from the existing Rust entry file.
- [T002] [P] [Story] Define the migration boundary for `sds.c` inside `src/main_root.rs`, including placeholder sections for the 5 data structures and grouped function implementations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the first core data structure from `sds.c` into Rust in `src/main_root.rs`, preserving field layout and ownership assumptions required by dependent functions. Depends on: T002.
- [T004] [P] [Story] Port the second core data structure from `sds.c` into Rust in `src/main_root.rs`, keeping its fields and invariants aligned with the C module behavior. Depends on: T002.
- [T005] [P] [Story] Port the third core data structure from `sds.c` into Rust in `src/main_root.rs`, including any associated constants or internal helper representations directly evidenced by the source module. Depends on: T002.
- [T006] [P] [Story] Port the fourth core data structure from `sds.c` into Rust in `src/main_root.rs`, matching the original module’s role and data access patterns. Depends on: T002.
- [T007] [P] [Story] Port the fifth core data structure from `sds.c` into Rust in `src/main_root.rs`, completing the foundational type set used by the module functions. Depends on: T002.
- [T008] [Story] Reconcile the 5 migrated data structures in `src/main_root.rs` so shared aliases, enum-style distinctions, and internal construction rules are consistent before function porting begins. Depends on: T003, T004, T005, T006, T007.

## Phase 3: Functions

- [T009] [Story] Implement the initialization and creation function group from `sds.c` in `src/main_root.rs`, covering functions that allocate, construct, or bootstrap the migrated `main_root` data structures. Depends on: T008.
- [T010] [P] [Story] Implement the lifecycle and destruction function group from `sds.c` in `src/main_root.rs`, covering functions that release, reset, or tear down `main_root` state. Depends on: T008.
- [T011] [P] [Story] Implement the copy and conversion function group from `sds.c` in `src/main_root.rs`, covering functions that duplicate state or translate between internal representations used by the module. Depends on: T008.
- [T012] [Story] Implement the mutation and update function group from `sds.c` in `src/main_root.rs`, covering functions that change contents, sizes, flags, or other stored state after creation. Depends on: T008.
- [T013] [P] [Story] Implement the query and accessor function group from `sds.c` in `src/main_root.rs`, covering functions that inspect `main_root` state and return derived values without changing ownership. Depends on: T008.
- [T014] [P] [Story] Implement the comparison and matching function group from `sds.c` in `src/main_root.rs`, covering functions that compare instances, contents, or attributes defined in the original module. Depends on: T008.
- [T015] [Story] Implement the parsing and formatting function group from `sds.c` in `src/main_root.rs`, covering functions that interpret input into module structures or render module state outward. Depends on: T008.
- [T016] [Story] Implement the remaining internal helper functions from `sds.c` in `src/main_root.rs`, covering support routines not already assigned to another function group so each of the 45 C functions is migrated exactly once. Depends on: T009, T010, T011, T012, T013, T014, T015.

## Final Phase: Polish

- [T017] [Story] Refine `src/main_root.rs` to remove C-port leftovers, consolidate duplicated logic introduced during grouped migration, and align signatures and visibility with the final Rust module surface. Depends on: T016.
- [T018] [Story] Perform a final pass on `src/main_root.rs` for idiomatic Rust cleanups that preserve `sds.c` behavior, including ownership simplification and local implementation tightening without expanding module scope. Depends on: T017.