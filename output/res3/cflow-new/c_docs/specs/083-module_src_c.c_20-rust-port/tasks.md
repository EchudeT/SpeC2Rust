# Tasks: Rust port for `module_src_c.c_20`

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/c.c` migration in `src/c.rs`, and declare the module from the crate root or existing parent module file so the ported implementation has a concrete destination.
- [T002] [P] [Story] Review `src/c.c` and map its 13 data structures and 15 functions into a Rust-side implementation outline within `src/c.rs`, keeping naming and grouping aligned with the source module to support a one-file migration.
- [T003] [Story] Define the initial public/private surface for the migrated module in `src/c.rs`, including placeholders for all data structures and function groups identified from `src/c.c`. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Implement the core Rust representations for all 13 data structures from `src/c.c` in `src/c.rs`, preserving source relationships, ownership assumptions, and field layout semantics required by the module’s functions. Depends on: T003.
- [T005] [P] [Story] Add shared type aliases, enums, constants, and internal helper representations in `src/c.rs` that are directly required to support the migrated data structures and subsequent function implementations. Depends on: T004.
- [T006] [Story] Refine constructor/default/setup patterns for the migrated data structures in `src/c.rs` wherever the C module relies on zero-init, sentinel values, or explicit initialization behavior. Depends on: T004, T005.

## Phase 3: Data lifecycle and initialization functions

- [T007] [Story] Port the functions from `src/c.c` responsible for module-level initialization, object initialization, or state setup into `src/c.rs`, wiring them to the Rust data structures defined in Phase 2. Depends on: T006.
- [T008] [Story] Port the functions from `src/c.c` responsible for object teardown, reset, cleanup, or state release into `src/c.rs`, adapting manual C lifecycle behavior into idiomatic Rust ownership where possible without changing module behavior. Depends on: T007.

## Phase 4: State mutation and transformation functions

- [T009] [Story] Port the functions from `src/c.c` that mutate, update, insert, remove, or otherwise transform the module’s primary state in `src/c.rs`, keeping their logic grouped together around shared data-structure access patterns. Depends on: T008.
- [T010] [P] [Story] Port the functions from `src/c.c` that perform local helper calculations or internal state adjustments used by the mutation/transformation paths into `src/c.rs`, provided they are not already covered by lifecycle tasks. Depends on: T008.

## Phase 5: Queries, lookups, and output-oriented functions

- [T011] [Story] Port the functions from `src/c.c` that read, query, search, or expose derived information from module state into `src/c.rs`, preserving source return semantics and error/signaling conventions as represented in the C module. Depends on: T009, T010.
- [T012] [P] [Story] Port any remaining formatting, emission, or externally visible wrapper functions from `src/c.c` into `src/c.rs`, completing the one-to-one function migration without duplicating logic already implemented in earlier phases. Depends on: T011.

## Final Phase: Polish

- [T013] [Story] Perform a module-wide pass over `src/c.rs` to remove temporary placeholders, reconcile signatures and visibility, and ensure all 15 functions and 13 data structures from `src/c.c` are fully represented exactly once. Depends on: T012.
- [T014] [Story] Refine idiomatic Rust details in `src/c.rs` such as borrowing, pattern matching, and internal helper usage while preserving the migrated C module behavior and keeping the implementation scope limited to `src/c.c`. Depends on: T013.
- [T015] [Story] Complete final compile-readiness cleanup for the migrated module in `src/c.rs`, including import cleanup and elimination of dead scaffolding introduced during porting. Depends on: T014.