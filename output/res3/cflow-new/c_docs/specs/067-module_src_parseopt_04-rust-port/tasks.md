# Tasks: module_src_parseopt_04

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/main.c` port on branch `067-module_src_parseopt_04-rust-port`, adding the target Rust source files `src/main.rs` and `src/parseopt.rs`.
- [T002] [P] [Story] Wire `src/main.rs` to declare and import the `parseopt` module from `src/parseopt.rs` so later migrated data structures and functions can be placed without changing the file layout. Depends on: T001.
- [T003] [Story] Establish the porting layout inside `src/parseopt.rs`, reserving sections for module-level state, translated data structures, and grouped function implementations derived from `src/main.c`. Depends on: T001.

## Phase 2: Foundational

- [T004] [Story] Translate the module-level constants, enums, aliases, and shared record definitions required by the parse-option logic from `src/main.c` into Rust definitions in `src/parseopt.rs`. Depends on: T003.
- [T005] [Story] Implement the core owned and borrowed Rust structs representing the module’s option-parsing state and related parsed-value containers in `src/parseopt.rs`, preserving the relationships required by all migrated functions. Depends on: T004.
- [T006] [P] [Story] Implement auxiliary data structures used by the module’s parsing and dispatch paths, including any grouped fields, flags, and descriptor records inferable from `src/main.c`, in `src/parseopt.rs`. Depends on: T004.
- [T007] [Story] Reconcile the translated foundational types into a coherent Rust API inside `src/parseopt.rs`, resolving field ownership, mutability, and shared access patterns needed before function migration. Depends on: T005, T006.

## Phase 3: Functions

- [T008] [Story] Implement the parse-option entry and high-level control-flow function group in `src/parseopt.rs`, migrating the top-level routine(s) from `src/main.c` that coordinate option scanning using the foundational parse state. Depends on: T007.
- [T009] [P] [Story] Implement the option-token inspection and classification function group in `src/parseopt.rs`, migrating related helper routine(s) from `src/main.c` that identify, normalize, or categorize incoming arguments. Depends on: T007.
- [T010] [P] [Story] Implement the option-value extraction and state-update function group in `src/parseopt.rs`, migrating related helper routine(s) from `src/main.c` that assign parsed values into the translated module data structures. Depends on: T007.
- [T011] [Story] Integrate the migrated function groups in `src/parseopt.rs`, ensuring the 5 translated functions call through the Rust data structures consistently and without duplicating parsing responsibilities. Depends on: T008, T009, T010.
- [T012] [Story] Update `src/main.rs` to invoke the migrated parse-option entry points from `src/parseopt.rs` in place of the original `src/main.c` module behavior. Depends on: T011.

## Final Phase: Polish

- [T013] [Story] Refine the Rust port in `src/parseopt.rs` and `src/main.rs` by removing temporary placeholders, tightening signatures and visibility, and simplifying control flow while preserving the behavior of the migrated `src/main.c` parse-option module. Depends on: T012.
- [T014] [P] [Story] Perform a final pass on `src/parseopt.rs` to reduce redundant allocations and unnecessary cloning introduced during migration, keeping optimizations limited to the implemented module scope. Depends on: T013.
- [T015] [Story] Review the final file-local organization of `src/parseopt.rs` so the translated data structures appear before their dependent functions and grouped function implementations remain aligned with the original `src/main.c` module responsibilities. Depends on: T013.