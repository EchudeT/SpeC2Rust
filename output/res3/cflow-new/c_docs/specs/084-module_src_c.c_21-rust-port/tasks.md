# Tasks: module_src_c.c_21 Rust port

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for `src/c.c` in `src/c.rs`, and expose it from the crate module tree on branch `084-module_src_c.c_21-rust-port`.
- [ ] T002 [Story] Review `src/c.c` and map the 15 functions and 13 data structures into a Rust port plan documented as code placeholders and TODO sections in `src/c.rs`. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Define the first group of core data structures from `src/c.c` in `src/c.rs`, including their Rust field mappings and ownership/borrowing choices. Depends on: T002
- [ ] T004 [P] [Story] Define the second group of supporting data structures from `src/c.c` in `src/c.rs`, including enums, nested structs, and alias replacements needed by the port. Depends on: T002
- [ ] T005 [Story] Integrate all 13 migrated data structures in `src/c.rs`, resolving cross-references, default construction strategy, and shared type usage needed by the function port. Depends on: T003, T004

## Phase 3: Functions — initialization and lifecycle

- [ ] T006 [Story] Implement the initialization and construction-related functions from `src/c.c` in `src/c.rs`, using the Phase 2 data structures as their Rust backing types. Depends on: T005
- [ ] T007 [P] [Story] Implement the cleanup, reset, or teardown-related functions from `src/c.c` in `src/c.rs`, preserving the original module lifecycle behavior in Rust. Depends on: T005
- [ ] T008 [Story] Reconcile shared state transitions between initialization and teardown function groups in `src/c.rs` so the lifecycle API is internally consistent. Depends on: T006, T007

## Phase 4: Functions — state manipulation and helpers

- [ ] T009 [Story] Implement the internal helper and conversion-related functions from `src/c.c` in `src/c.rs` that support the main module operations. Depends on: T008
- [ ] T010 [P] [Story] Implement the state update and mutation-related functions from `src/c.c` in `src/c.rs`, keeping behavior aligned with the original `src/c.c` logic. Depends on: T008
- [ ] T011 [Story] Resolve shared logic between helper functions and state mutation functions in `src/c.rs`, removing duplication introduced during the direct port. Depends on: T009, T010

## Phase 5: Functions — main operations and control flow

- [ ] T012 [Story] Implement the primary operational functions from `src/c.c` in `src/c.rs` that drive the module’s externally visible behavior. Depends on: T011
- [ ] T013 [P] [Story] Implement the remaining control-flow and decision-making functions from `src/c.c` in `src/c.rs`, completing the port of all 15 functions. Depends on: T011
- [ ] T014 [Story] Align all function signatures and call paths in `src/c.rs` so the full migrated function set composes correctly without placeholder gaps. Depends on: T012, T013

## Final Phase: Polish

- [ ] T015 [Story] Refine the `src/c.rs` implementation by removing porting placeholders, tightening Rust idioms, and simplifying obvious direct-C patterns while preserving behavior. Depends on: T014
- [ ] T016 [Story] Perform a final module pass on `src/c.rs` to verify naming consistency, dead-code cleanup, and completeness of the `src/c.c` migration for this module. Depends on: T015