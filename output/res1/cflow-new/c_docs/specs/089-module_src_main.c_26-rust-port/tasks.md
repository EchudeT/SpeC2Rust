# Tasks: module_src_main.c_26 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/main.c` port on branch `089-module_src_main.c_26-rust-port`, adding the target source files `src/main.rs` and `src/main_types.rs`.
- [T002] [P] [Story] Define the module wiring between `src/main.rs` and `src/main_types.rs` so data structures and function implementations for this module can be added incrementally. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port the foundational constants, type aliases, enums, and simple value-holder structs directly inferable from `src/main.c` into `src/main_types.rs`. Depends on: T002
- [T004] [Story] Port the composite structs and nested record layouts used across multiple functions in `src/main.c` into `src/main_types.rs`, preserving field relationships needed by later function work. Depends on: T003
- [T005] [Story] Port the remaining shared data structures, including any static-state carrier structs and grouped container representations from `src/main.c`, into `src/main_types.rs` so all 63 module data structures are available before function migration. Depends on: T004

## Phase 3: Entry and argument flow functions

- [T006] [Story] Implement the program entry-path function group from `src/main.c` in `src/main.rs`, covering `main` and the directly-associated startup/control-flow helpers. Depends on: T005
- [T007] [P] [Story] Implement the argument parsing and command-line option handling function group from `src/main.c` in `src/main.rs`, keeping the translated logic adjacent to the entry-path code that invokes it. Depends on: T005
- [T008] [Story] Connect the entry-path and argument-handling groups in `src/main.rs`, resolving shared state access and call ordering exactly once for this module. Depends on: T006, T007

## Phase 4: Initialization and state preparation functions

- [T009] [P] [Story] Implement the initialization helper function group from `src/main.c` in `src/main.rs`, covering setup routines that prepare module-level state before main processing. Depends on: T005
- [T010] [P] [Story] Implement the configuration/state population helper function group from `src/main.c` in `src/main.rs`, covering routines that fill or normalize the foundational structures defined in `src/main_types.rs`. Depends on: T005
- [T011] [Story] Integrate the initialization and state-preparation groups into the startup sequence in `src/main.rs`. Depends on: T008, T009, T010

## Phase 5: Main processing and termination functions

- [T012] [P] [Story] Implement the primary processing/control helper function group from `src/main.c` in `src/main.rs`, covering the remaining non-entry business logic functions. Depends on: T011
- [T013] [P] [Story] Implement the shutdown, cleanup, and final status/reporting function group from `src/main.c` in `src/main.rs`, covering the remaining termination-oriented functions. Depends on: T011
- [T014] [Story] Wire the primary processing and termination groups into the full module execution path in `src/main.rs`, completing the migration of all 12 functions from `src/main.c`. Depends on: T012, T013

## Final Phase: Polish

- [T015] [Story] Refine the Rust port in `src/main.rs` and `src/main_types.rs` by removing migration scaffolding, tightening visibility and ownership choices, and aligning naming and structure usage with the translated `src/main.c` behavior. Depends on: T014