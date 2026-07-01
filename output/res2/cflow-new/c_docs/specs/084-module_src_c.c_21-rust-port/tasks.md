# Tasks: module_src_c.c_21 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/c.c` in `src/c.rs`, and expose it from the crate root/module tree used by branch `084-module_src_c.c_21-rust-port`.
- [T002] [P] [Story] Review `src/c.c` and map the 13 data structures and 15 functions into a Rust implementation plan documented inline in `src/c.rs` as migration TODO sections, preserving source grouping and names where practical. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the primary Rust type definitions in `src/c.rs` for all data structures directly represented in `src/c.c`, translating C structs/unions/enums/constants into idiomatic Rust forms needed by the module. Depends on: T002
- [T004] [Story] Implement shared field initialization, default construction helpers, and internal aliases/constants in `src/c.rs` required by the migrated data structures from `src/c.c`. Depends on: T003
- [T005] [Story] Implement foundational internal helper methods on the migrated data structures in `src/c.rs` when they are required to support multiple function ports from `src/c.c`. Depends on: T004

## Phase 3: Core state and lifecycle functions

- [T006] [Story] Port the data-structure creation, initialization, and reset-related functions from `src/c.c` into `src/c.rs`, using the foundational Rust types already defined for module state setup. Depends on: T005
- [T007] [P] [Story] Port the teardown, cleanup, and release-related functions from `src/c.c` into `src/c.rs`, matching the original module ownership and lifecycle behavior in Rust. Depends on: T005
- [T008] [Story] Reconcile shared state transitions between the initialization/reset group and the cleanup/release group in `src/c.rs` so lifecycle functions operate consistently without duplicating logic. Depends on: T006, T007

## Phase 4: Core data processing functions

- [T009] [Story] Port the main data manipulation and update functions from `src/c.c` into `src/c.rs`, covering the central state-transforming behavior of the module. Depends on: T008
- [T010] [P] [Story] Port supporting lookup, access, and query functions from `src/c.c` into `src/c.rs` that read or derive information from the migrated module state. Depends on: T008
- [T011] [Story] Integrate the update/manipulation and lookup/query function groups in `src/c.rs`, ensuring shared helpers and common branches from `src/c.c` are represented once. Depends on: T009, T010

## Phase 5: Remaining control and utility functions

- [T012] [Story] Port the remaining control-flow, coordination, and utility functions from `src/c.c` into `src/c.rs` that do not belong to lifecycle or core data-processing groups. Depends on: T011
- [T013] [Story] Resolve any module-local constant handling, return-value mapping, and internal error/flag conventions in `src/c.rs` required to complete parity for all 15 migrated functions. Depends on: T012

## Final Phase: Polish

- [T014] [Story] Refine `src/c.rs` for idiomatic Rust within the scope of the port by removing redundant temporary translation artifacts, consolidating duplicated internal logic, and clarifying function/type visibility.
- [T015] [Story] Perform a final pass on `src/c.rs` to confirm all 13 data structures and 15 functions from `src/c.c` are migrated exactly once and that dependency ordering has been satisfied across the module. Depends on: T013, T014