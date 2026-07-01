# Task List: module_src_c.c_21 Rust Port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/c.c` in `src/c.rs`, expose it from the crate root or parent module, and align the file layout with branch `084-module_src_c.c_21-rust-port`.
- [T002] [P] [Story] Define the module-level migration boundary in `src/c.rs`: add placeholders for the 13 data structures and 15 functions identified for this port so later tasks can be filled without changing module shape. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the core data structure set in `src/c.rs` by translating the C module's primary structs, enums, and type aliases that are directly required by multiple functions. Depends on: T002.
- [T004] [P] [Story] Implement the remaining supporting data structures in `src/c.rs`, including helper structs and constants/local type representations used by narrower function groups. Depends on: T003.
- [T005] [Story] Refine ownership, borrowing, and visibility for all 13 migrated data structures in `src/c.rs` so the function phases can use stable Rust-native interfaces without revisiting structure definitions. Depends on: T003, T004.

## Phase 3: Initialization and State Functions

- [T006] [Story] Implement the function group in `src/c.rs` responsible for module setup, object initialization, and default state construction using the foundational data structures. Depends on: T005.
- [T007] [P] [Story] Implement the function group in `src/c.rs` responsible for reset, cleanup-equivalent, or state transition helpers that operate on already-initialized module data. Depends on: T005.
- [T008] [Story] Reconcile shared state handling across the initialization/state function group in `src/c.rs`, removing placeholder behavior and matching the original C control flow for these functions. Depends on: T006, T007.

## Phase 4: Core Processing Functions

- [T009] [Story] Implement the main processing function group in `src/c.rs` for the module's central logic path, covering the highest-dependency functions that consume the migrated structures. Depends on: T008.
- [T010] [P] [Story] Implement adjacent helper functions in `src/c.rs` that support the core processing path with intermediate calculations, lookups, or state updates. Depends on: T008.
- [T011] [Story] Integrate the core processing and helper function groups in `src/c.rs` so the full central execution path is represented once, without duplicate scheduling of any migrated function. Depends on: T009, T010.

## Phase 5: Output and Utility Functions

- [T012] [Story] Implement the remaining externally visible utility or formatting/output-related functions in `src/c.rs` that complete the module's public behavior after core processing is in place. Depends on: T011.
- [T013] [P] [Story] Implement the remaining internal utility/helper functions in `src/c.rs` that are independent of output-facing behavior but required to complete the set of 15 migrated functions. Depends on: T011.
- [T014] [Story] Finalize call relationships and remove any remaining stubs in `src/c.rs` so all function migrations from `src/c.c` are fully wired to the Rust data structures. Depends on: T012, T013.

## Final Phase: Polish

- [T015] [Story] Polish `src/c.rs` by simplifying translated control flow, eliminating unnecessary temporary state introduced during migration, and tightening idiomatic Rust usage while preserving the behavior of the original `src/c.c`. Depends on: T014.
- [T016] [Story] Perform a final module review in `src/c.rs` for naming consistency, visibility cleanup, and dead-code/stub removal related to the `module_src_c.c_21` migration. Depends on: T015.