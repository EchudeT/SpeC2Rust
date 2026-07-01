# Tasks: main_root Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust entry module scaffold for the `main_root` port by adding `src/main_root.rs` and wiring it into the crate from the existing Rust project branch `001-main_root-rust-port`.
- [T002] [P] [Story] Establish the file-level migration boundary for `sds.c` in `src/main_root.rs`, adding placeholder sections for data structures and function groups to keep the port localized to the source module. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Identify and define the 5 data structures inferred from `sds.c` as Rust `struct`/`enum` types in `src/main_root.rs`, preserving the original module-local ownership and layout intent needed by the function port. Depends on: T002
- [T004] [Story] Implement associated constructors/default initialization and internal helper representations required by those 5 data structures in `src/main_root.rs`, limited to what is needed to support the ported functions from `sds.c`. Depends on: T003

## Phase 3: Initialization and lifecycle functions

- [T005] [Story] Port the startup, module initialization, and top-level lifecycle functions from `sds.c` into `src/main_root.rs`, using the Phase 2 data structures as the Rust state carrier. Depends on: T004
- [T006] [Story] Port the teardown, cleanup, and shutdown-oriented functions from `sds.c` into `src/main_root.rs`, keeping resource release logic aligned with the Rust ownership model without expanding module scope. Depends on: T005

## Phase 4: Core state and data manipulation functions

- [T007] [P] [Story] Port the core state mutation functions from `sds.c` into `src/main_root.rs`, grouping functions that create, update, reset, or otherwise manipulate the main module state. Depends on: T004
- [T008] [P] [Story] Port the core data access and query functions from `sds.c` into `src/main_root.rs`, grouping functions that read or derive information from the main module state without duplicating lifecycle work. Depends on: T004
- [T009] [Story] Reconcile shared helper usage between mutation and query groups in `src/main_root.rs`, ensuring each function from `sds.c` is ported exactly once and grouped consistently. Depends on: T007, T008

## Phase 5: Input, command, and control-flow functions

- [T010] [P] [Story] Port input-processing and argument-handling functions from `sds.c` into `src/main_root.rs`, grouping top-level parsing or dispatch entrypoints that feed the module state. Depends on: T009
- [T011] [P] [Story] Port command/control-flow functions from `sds.c` into `src/main_root.rs`, grouping operational branches, command execution paths, or mode-selection logic present in the main module. Depends on: T009
- [T012] [Story] Integrate the input and command groups in `src/main_root.rs` so the migrated control path matches the original `sds.c` flow through the main module. Depends on: T010, T011

## Phase 6: Output, reporting, and remaining utility functions

- [T013] [P] [Story] Port output, display, or reporting functions from `sds.c` into `src/main_root.rs`, grouping user-visible formatting and emission logic within the module boundary. Depends on: T012
- [T014] [P] [Story] Port remaining module-local utility/helper functions from `sds.c` into `src/main_root.rs`, limited to helpers directly supporting the 45 migrated functions and not already covered in earlier phases. Depends on: T012
- [T015] [Story] Complete wiring of all remaining function call sites in `src/main_root.rs` so every function migrated from `sds.c` is connected to its Rust counterpart without duplication across phases. Depends on: T013, T014

## Final Phase: Polish

- [T016] [Story] Refine `src/main_root.rs` for idiomatic Rust naming, ownership cleanup, and removal of temporary migration placeholders while preserving the behavior of the ported `sds.c` module. Depends on: T015
- [T017] [Story] Perform a final pass on `src/main_root.rs` to reduce avoidable cloning/copying and simplify internal control flow introduced during migration, keeping changes constrained to the `main_root` module. Depends on: T016