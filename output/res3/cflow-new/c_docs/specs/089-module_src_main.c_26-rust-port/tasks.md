# Tasks: module_src_main.c_26 Rust port

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for the `src/main.c` port on branch `089-module_src_main.c_26-rust-port`, adding the target source file `src/main.rs` and wiring the module entry layout needed for the port.
- [ ] T002 [P] [Story] Review `src/main.c` and map the 12 functions and 63 data structures into a Rust implementation plan inside `src/main.rs`, keeping names and grouping aligned with the source module for direct migration.
- [ ] T003 [Story] Define the migration boundaries in `src/main.rs` so that all ported items from `src/main.c` have reserved placement for data structures first and function implementations after them. Depends on: T001, T002

## Phase 2: Foundational

- [ ] T004 [Story] Port the core standalone data structures from `src/main.c` into Rust definitions in `src/main.rs`, covering scalar records, enums, aliases, and constants that do not depend on other module-local structures. Depends on: T003
- [ ] T005 [P] [Story] Port the dependent and nested data structures from `src/main.c` into `src/main.rs`, including structures that reference other module-local types and preserving source field relationships. Depends on: T004
- [ ] T006 [Story] Reconcile all 63 migrated data structures in `src/main.rs`, resolving ordering, visibility, and type linkage so the function port can compile against a complete foundational type set. Depends on: T004, T005

## Phase 3: Initialization and entry-flow functions

- [ ] T007 [Story] Port the startup and entry-flow function group from `src/main.c` into `src/main.rs`, implementing the functions responsible for module initialization and top-level execution sequencing against the migrated data structures. Depends on: T006
- [ ] T008 [P] [Story] Port the argument, option, or invocation-handling function group from `src/main.c` into `src/main.rs`, keeping control-flow behavior aligned with the C source where these functions feed the main execution path. Depends on: T006
- [ ] T009 [Story] Integrate the initialization and entry-flow function groups in `src/main.rs`, resolving shared call paths and state handoff between startup logic and invocation handling. Depends on: T007, T008

## Phase 4: State-processing and helper functions

- [ ] T010 [P] [Story] Port the internal state-processing function group from `src/main.c` into `src/main.rs`, implementing functions that transform, update, or inspect the module-local runtime structures. Depends on: T006
- [ ] T011 [P] [Story] Port the local helper and utility function group from `src/main.c` into `src/main.rs`, covering reusable support routines that are only evidenced within this module. Depends on: T006
- [ ] T012 [Story] Connect the state-processing and helper functions in `src/main.rs` to the previously ported entry-flow logic, ensuring each of the migrated functions has a single Rust implementation and correct call-site usage. Depends on: T009, T010, T011

## Final Phase: Polish

- [ ] T013 [Story] Perform a final pass on `src/main.rs` to remove migration gaps, align Rust control flow and type usage with `src/main.c`, and simplify any direct C-to-Rust translations that can be safely refined without changing behavior. Depends on: T012
- [ ] T014 [Story] Clean up `src/main.rs` for Rust idioms within the existing module scope, resolving obvious duplication, tightening signatures, and addressing compile-time issues introduced during the port. Depends on: T013