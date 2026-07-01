# Tasks: module_src Rust port

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for `module_src` in `src/module_src.rs`, establish the file as the migration target for logic from `src/shc.c`, and wire it into the crate module tree from the existing Rust crate root.
- [ ] T002 [Story] Define the public/private implementation boundary in `src/module_src.rs` so the ported module can contain the translated data structures and all 18 functions with names and visibility aligned to current crate usage. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Identify and translate the 5 C data structures from `src/shc.c` into Rust equivalents in `src/module_src.rs`, preserving field layout intent, ownership model, and mutability requirements needed by the function port. Depends on: T002
- [ ] T004 [P] [Story] Add foundational Rust type aliases, constants, and helper enums in `src/module_src.rs` that are directly required to express the translated structures and function signatures from `src/shc.c`. Depends on: T003
- [ ] T005 [P] [Story] Implement core constructors/default initialization and internal helper methods for the translated data structures in `src/module_src.rs` where the C module relies on zeroed, reset, or manually initialized state. Depends on: T003

## Phase 3: Module state and lifecycle functions

- [ ] T006 [Story] Port the module initialization, reset, and teardown-style functions from `src/shc.c` into `src/module_src.rs`, using the translated data structures and Rust ownership rules instead of C-style manual state handling. Depends on: T003, T005
- [ ] T007 [Story] Port functions that allocate, prepare, or attach module-local state in `src/shc.c` into `src/module_src.rs`, keeping lifecycle behavior grouped with initialization logic and avoiding duplicated setup paths. Depends on: T006

## Phase 4: Input parsing and argument handling functions

- [ ] T008 [Story] Port the command/input parsing functions from `src/shc.c` into `src/module_src.rs`, translating pointer-based parsing into safe Rust string/byte handling while preserving module behavior. Depends on: T003, T004
- [ ] T009 [P] [Story] Port helper functions that validate, normalize, or transform parsed input for the module from `src/shc.c` into `src/module_src.rs`, keeping them grouped with argument handling logic. Depends on: T008

## Phase 5: Core processing and generation functions

- [ ] T010 [Story] Port the primary processing functions that implement the main `module_src` behavior from `src/shc.c` into `src/module_src.rs`, wiring them to the translated data structures and previously ported parsing/lifecycle helpers. Depends on: T006, T008, T009
- [ ] T011 [P] [Story] Port subordinate computation/helper functions used only by the core processing path from `src/shc.c` into `src/module_src.rs`, preserving control flow and data updates without re-splitting already scheduled logic. Depends on: T010
- [ ] T012 [Story] Port any output/emission/build-result functions from `src/shc.c` into `src/module_src.rs`, keeping result construction close to the core processing path that consumes it. Depends on: T010, T011

## Phase 6: Orchestration and public entry functions

- [ ] T013 [Story] Port the top-level orchestration functions from `src/shc.c` into `src/module_src.rs`, connecting initialization, parsing, processing, and output paths into the module’s main callable flow. Depends on: T007, T012
- [ ] T014 [Story] Port remaining public wrapper functions from `src/shc.c` into `src/module_src.rs` that expose the module behavior to the rest of the crate, ensuring each of the 18 functions is migrated exactly once. Depends on: T013

## Final Phase: Polish

- [ ] T015 [Story] Refine `src/module_src.rs` to remove leftover C-centric patterns made unnecessary by Rust, simplify ownership/borrowing around the translated structures, and eliminate duplicated helper logic introduced during the port. Depends on: T014
- [ ] T016 [Story] Review function and structure visibility, naming consistency, and module organization in `src/module_src.rs` so the migrated code is idiomatic Rust while remaining faithful to the original `src/shc.c` module boundaries. Depends on: T015