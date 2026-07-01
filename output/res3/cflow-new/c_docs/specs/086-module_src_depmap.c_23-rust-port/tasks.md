# Tasks: module_src_depmap.c_23

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/depmap.c` port on branch `086-module_src_depmap.c_23-rust-port`, adding the target source file at `src/depmap.rs` and wiring its module declaration into the existing Rust crate entry structure if not already present.
- [T002] [Story] Review `src/depmap.c` and map its 1 data structure and 6 functions into a Rust-side implementation outline inside `src/depmap.rs`, defining the migration boundaries for this module only. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the module’s core data structure from `src/depmap.c` as Rust-native type definitions in `src/depmap.rs`, preserving the C module’s ownership and representation requirements as closely as practical in safe Rust. Depends on: T002
- [T004] [P] [Story] Add associated constructors, default state initialization, and internal helper methods required to support the migrated `depmap` data structure in `src/depmap.rs`, limited to helpers directly needed by the 6 ported functions. Depends on: T003

## Phase 3: Core dependency map lifecycle functions

- [T005] [Story] Port the function group responsible for dependency map creation and teardown from `src/depmap.c` into Rust functions or impl methods in `src/depmap.rs`, using the Phase 2 data structure as the sole backing representation. Depends on: T003
- [T006] [Story] Port the function group responsible for inserting or registering dependency relationships from `src/depmap.c` into `src/depmap.rs`, keeping behavior aligned with the original module’s state transitions. Depends on: T003, T005
- [T007] [Story] Port the function group responsible for dependency lookup, traversal, or query behavior from `src/depmap.c` into `src/depmap.rs`, reusing the established Rust data structure and avoiding duplicate state handling. Depends on: T003, T005, T006

## Phase 4: Remaining module-specific function group

- [T008] [P] [Story] Port the remaining function group from `src/depmap.c` that does not fit the lifecycle, registration, or query groupings into `src/depmap.rs`, preserving original module-local behavior and signatures as adapted for Rust. Depends on: T003
- [T009] [Story] Reconcile all 6 migrated functions in `src/depmap.rs` so shared helper usage, error/state conventions, and internal visibility are consistent across the module. Depends on: T005, T006, T007, T008

## Final Phase: Polish

- [T010] [Story] Refine `src/depmap.rs` by removing migration-only scaffolding, tightening type and borrow usage, and simplifying control flow without changing the behavior of the ported data structure or 6 functions. Depends on: T009
- [T011] [Story] Perform a final module-level review of `src/depmap.rs` to ensure the `src/depmap.c` migration is complete, with no duplicated function ports and no scope expansion beyond the original module responsibilities. Depends on: T010