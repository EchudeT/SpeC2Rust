# Tasks: module_src_c.c_20 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/c.c` migration on branch `083-module_src_c.c_20-rust-port`, adding the target source file `src/c.rs` and wiring it into the existing crate module tree.
- [T002] [P] [Story] Establish the migration surface in `src/c.rs` with Rust-side placeholders for the 13 data structures and 15 functions identified from `src/c.c`, keeping names and grouping aligned to the source module for incremental implementation.

## Phase 2: Foundational

- [T003] [Story] Implement the primary Rust data type definitions in `src/c.rs` for the 13 data structures migrated from `src/c.c`, preserving source relationships, field intent, and ownership layout needed by later function ports. Depends on: T001, T002
- [T004] [P] [Story] Add associated enums, type aliases, and constant definitions in `src/c.rs` that are directly required to make the migrated data structures from `src/c.c` compile cleanly in Rust. Depends on: T003
- [T005] [Story] Refine constructors, default state helpers, and internal representation details in `src/c.rs` only where they are required to support the upcoming function groups from `src/c.c`. Depends on: T003, T004

## Phase 3: Core state and lifecycle functions

- [T006] [Story] Port the initialization, allocation, and teardown-related functions from `src/c.c` into `src/c.rs`, using the foundational data structures as the canonical Rust state model. Depends on: T005
- [T007] [P] [Story] Port the basic state reset and object preparation functions from `src/c.c` into `src/c.rs`, keeping behavior grouped with lifecycle handling and avoiding duplication across later phases. Depends on: T006

## Phase 4: Core processing and mutation functions

- [T008] [Story] Port the main data-processing and state-mutation function group from `src/c.c` into `src/c.rs`, translating the module’s central operational logic onto the Rust structures defined in `src/c.rs`. Depends on: T006, T007
- [T009] [P] [Story] Port closely related helper functions from `src/c.c` into `src/c.rs` that are invoked by the main processing path and operate on the same module state. Depends on: T008
- [T010] [Story] Integrate the remaining intermediate computation and update functions from `src/c.c` into `src/c.rs`, ensuring each of the identified module functions is implemented exactly once within its functional group. Depends on: T008, T009

## Phase 5: Query, output, and module entry functions

- [T011] [Story] Port the read-only access, query, and status-reporting functions from `src/c.c` into `src/c.rs`, keeping them separated from mutation-heavy logic while reusing the completed Rust state definitions. Depends on: T008, T010
- [T012] [P] [Story] Port any formatting, conversion, or result-emission functions present in `src/c.c` into `src/c.rs` where they are part of the module’s externally visible behavior. Depends on: T011
- [T013] [Story] Complete the top-level coordination or entry-point functions from `src/c.c` in `src/c.rs`, connecting lifecycle, processing, and query groups into the final module flow. Depends on: T010, T011, T012

## Final Phase: Polish

- [T014] [Story] Remove placeholder code and reconcile signatures, visibility, and internal helper usage in `src/c.rs` so the Rust port of `src/c.c` is internally consistent and free of migration scaffolding. Depends on: T013
- [T015] [Story] Perform a final pass on `src/c.rs` to simplify obvious C-to-Rust translation artifacts, tighten ownership/borrowing where already evidenced by the ported logic, and align code organization with the completed module structure. Depends on: T014