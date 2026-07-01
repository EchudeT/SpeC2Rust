# Task List: module_gnu_hash.c_31

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/hash.c` on branch `037-module_gnu_hash.c_31-rust-port`, adding the target source file `src/gnu/hash.rs` and wiring it into the existing crate module tree.
- [T002] [P] [Story] Establish the module-local item layout in `src/gnu/hash.rs`, reserving sections for data structures, constants, helper routines, and exported function implementations for this port.
- [T003] [Story] Review `gnu/hash.c` and map all 9 functions and referenced 49 data structures into the Rust target file `src/gnu/hash.rs`, documenting the one-to-one migration inventory as implementation comments or TODO markers to prevent omissions. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Port the foundational type definitions from `gnu/hash.c` into Rust in `src/gnu/hash.rs`, defining the direct struct, enum, alias, and constant representations required by the module before any function bodies are implemented. Depends on: T003
- [T005] [P] [Story] Implement nested and helper data structures in `src/gnu/hash.rs` that are referenced by the primary GNU hash state and record layouts, preserving field relationships and ownership/borrowing strategy needed by later functions. Depends on: T004
- [T006] [P] [Story] Implement the remaining module-specific container and record structures in `src/gnu/hash.rs` so that all 49 referenced data structures from `gnu/hash.c` have Rust representations available to function groups. Depends on: T004
- [T007] [Story] Reconcile all inter-structure references in `src/gnu/hash.rs`, finalizing derives, visibility, placeholder defaults, and internal helper constructors only where needed to support direct migration of the 9 functions. Depends on: T005, T006

## Phase 3: Hash Core Functions

- [T008] [Story] Implement the core GNU hash computation routine group in `src/gnu/hash.rs`, porting the function or functions responsible for hash value generation and any tightly coupled local helpers from `gnu/hash.c`. Depends on: T007
- [T009] [Story] Implement the bucket and chain navigation routine group in `src/gnu/hash.rs`, porting the functions that traverse GNU hash table bucket/chain structures and consume the foundational hash outputs. Depends on: T008
- [T010] [Story] Integrate shared internal helper logic in `src/gnu/hash.rs` used exclusively by the hash computation and bucket/chain traversal functions, consolidating duplicated C-side local behavior during direct migration without expanding scope. Depends on: T008, T009

## Phase 4: Table Construction and Update Functions

- [T011] [Story] Implement the GNU hash table initialization and construction routine group in `src/gnu/hash.rs`, porting the function or functions that create or populate module hash state from `gnu/hash.c`. Depends on: T007
- [T012] [Story] Implement the symbol insertion or update routine group in `src/gnu/hash.rs`, porting the functions that attach entries to GNU hash buckets/chains and update associated module records. Depends on: T011
- [T013] [Story] Align construction-time helper logic in `src/gnu/hash.rs` so the table-building and update functions share the same Rust data handling rules as the traversal path. Depends on: T009, T012

## Phase 5: Lookup, Validation, and Cleanup Functions

- [T014] [Story] Implement the lookup-facing routine group in `src/gnu/hash.rs`, porting the functions that resolve entries through the GNU hash structures using the previously migrated hash and traversal logic. Depends on: T009, T013
- [T015] [P] [Story] Implement any remaining validation or state-checking function group from `gnu/hash.c` in `src/gnu/hash.rs`, limited to behavior explicitly present in the source module and required by lookup or construction flows. Depends on: T013
- [T016] [P] [Story] Implement any remaining teardown, reset, or final state-management function group from `gnu/hash.c` in `src/gnu/hash.rs`, limited to direct migration of explicit module functions. Depends on: T011, T012
- [T017] [Story] Complete the migration of the final unported functions from `gnu/hash.c` into `src/gnu/hash.rs`, ensuring each of the 9 source functions is implemented exactly once and connected to the appropriate shared structures. Depends on: T014, T015, T016

## Final Phase: Polish

- [T018] [Story] Refine `src/gnu/hash.rs` for direct C-to-Rust behavioral fidelity, tightening pattern matches, ownership boundaries, and internal naming consistency without changing module scope. Depends on: T017
- [T019] [Story] Remove obsolete migration placeholders and resolve any remaining TODO inventory markers in `src/gnu/hash.rs`, confirming that all mapped data structures and functions from `gnu/hash.c` are fully ported. Depends on: T018
- [T020] [Story] Perform a final module pass on `src/gnu/hash.rs` to simplify local implementation details and eliminate redundant helper code introduced during migration while preserving the source module behavior. Depends on: T019