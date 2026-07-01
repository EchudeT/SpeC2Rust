# Tasks: main_root_quoting_options_01

## Phase 1: Setup

- [T001] [Story] Create the Rust module skeleton for the `quotearg.c` port in `src/quotearg.rs`, and expose it from the crate root so subsequent quoting-option work has a dedicated target file.
- [T002] [P] [Story] Add the initial module wiring needed for `src/quotearg.rs` in the existing Rust project branch, keeping public visibility scoped to the needs of the `main_cluster` port. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the core quoting option data structures and related enums inferred from `quotearg.c` in `src/quotearg.rs`, establishing the Rust representations needed before any function porting begins. Depends on: T001
- [T004] [P] [Story] Add supporting constants, flags, and default/static configuration values used by the quoting option structures in `src/quotearg.rs`, matching the C module’s foundational state. Depends on: T003
- [T005] [Story] Implement constructors, default initializers, and basic state-management helpers for the quoting option structures in `src/quotearg.rs`, so later function groups can share one consistent initialization path. Depends on: T003, T004

## Phase 3: Quoting Option State Functions

- [T006] [Story] Implement the functions that create, clone, or return active quoting option state in `src/quotearg.rs`, grouping the C module’s option-access behavior into one Rust port step. Depends on: T005
- [T007] [Story] Implement the functions that mutate quoting option state, including style selection and flag updates, in `src/quotearg.rs`. Depends on: T006
- [T008] [Story] Implement the functions that configure character-specific or slot-specific quoting option behavior in `src/quotearg.rs`, completing the main option customization surface from `quotearg.c`. Depends on: T007

## Phase 4: Quoting and Argument Formatting Functions

- [T009] [Story] Implement the core argument-quoting functions that consume quoting options and produce quoted output in `src/quotearg.rs`, grouping the primary formatting behavior from `quotearg.c`. Depends on: T005
- [T010] [P] [Story] Implement the convenience wrapper functions around the core quoting path, including default-option and variant entry points, in `src/quotearg.rs`. Depends on: T009
- [T011] [Story] Implement the allocation/buffer-management related quoting entry points from `quotearg.c` in `src/quotearg.rs`, preserving the module’s grouped formatting APIs without duplicating earlier function work. Depends on: T009

## Phase 5: Root/Option Integration

- [T012] [Story] Integrate the root-facing quoting option usage needed by `main_root_quoting_options_01` through the exported APIs in `src/quotearg.rs`, ensuring the module surface matches this port slice’s expected main-cluster behavior. Depends on: T008, T010, T011
- [T013] [P] [Story] Refine public/private boundaries in `src/quotearg.rs` so only the option and quoting APIs needed by the root module remain exposed. Depends on: T012

## Final Phase: Polish

- [T014] [Story] Review `src/quotearg.rs` for idiomatic Rust cleanups, remove redundant C-style state handling introduced during porting, and simplify internal ownership where this does not change module behavior. Depends on: T012
- [T015] [Story] Perform final pass documentation comments and in-file organization for the ported quoting option module in `src/quotearg.rs`, keeping the implementation readable and aligned with the completed function groups. Depends on: T013, T014