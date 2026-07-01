# Tasks: cflow-new / module_src_set_level_15

## Phase 1: Setup

- [T001] [Story] Create the Rust module file scaffolding for the ported cluster by adding or updating `src/main.rs` and `src/output.rs` to host the functionality migrated from `src/main.c` and `src/output.c`.
- [T002] [P] [Story] Define the module wiring between `src/main.rs` and `src/output.rs`, including public/private item visibility needed for the migrated data structures and function calls.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Inventory and implement the Rust representations for the data structures evidenced by this module cluster in `src/main.rs`, preserving the layout and field semantics needed by the migrated functions from `src/main.c`.
  - Depends on: T002
- [T004] [P] [Story] Inventory and implement the Rust representations for the data structures evidenced by this module cluster in `src/output.rs`, preserving the layout and field semantics needed by the migrated functions from `src/output.c`.
- [T005] [Story] Reconcile shared or cross-file data structure definitions between `src/main.rs` and `src/output.rs` so each type is defined once and exposed where needed without duplicating the same migration work.
  - Depends on: T003, T004

## Phase 3: Main-side function migration

- [T006] [Story] Port the function from `src/main.c` into `src/main.rs`, translating its control flow and data-structure access to Rust while keeping behavior aligned with the original module.
  - Depends on: T005

## Phase 4: Output-side function migration

- [T007] [Story] Port the function from `src/output.c` into `src/output.rs`, translating its control flow and data-structure access to Rust while keeping behavior aligned with the original module.
  - Depends on: T005
- [T008] [P] [Story] Connect the migrated output-side function usage with the main-side module flow by updating call sites and imports across `src/main.rs` and `src/output.rs`.
  - Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Refine the migrated Rust code in `src/main.rs` and `src/output.rs` to remove C-oriented artifacts, simplify ownership/borrowing where possible, and ensure idiomatic module-local organization without changing behavior.
  - Depends on: T008
- [T010] [Story] Perform a final compile-pass cleanup across `src/main.rs` and `src/output.rs`, resolving warnings and visibility mismatches introduced during the migration.
  - Depends on: T009