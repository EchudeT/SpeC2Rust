# Tasks: module_src_parseopt_03

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust port workspace for `module_src_parseopt_03` on branch `066-module_src_parseopt_03-rust-port`, mapping C source scope from `src/main.c` into a Rust entry implementation in `src/main.rs`.
- [ ] T002 [Story] Establish the module-local file organization in `src/main.rs` for parse-option migration, including clearly separated sections for translated data structures, option-state handling, argument parsing helpers, and top-level parse flow.
- [ ] T003 [P] [Story] Inventory the 15 C functions and 63 data structures referenced by the parse-option area in `src/main.c`, and annotate their intended Rust destination inside `src/main.rs` to prevent duplicated migration work. Depends on: T001, T002

## Phase 2: Foundational

- [ ] T004 [Story] Port the foundational parse-option constants, enums, and type aliases from `src/main.c` into Rust definitions in `src/main.rs`, preserving naming relationships needed by all later function translations. Depends on: T003
- [ ] T005 [Story] Port the core parse-option state structs and owned field representations from `src/main.c` into Rust structs in `src/main.rs`, covering the central parser context required by multiple functions. Depends on: T004
- [ ] T006 [P] [Story] Port supporting descriptor and metadata structs used to describe options, flags, value carriers, and parse results from `src/main.c` into `src/main.rs`. Depends on: T004
- [ ] T007 [P] [Story] Port remaining small helper data structures used by parse-option control flow, including grouped record types and temporary argument-tracking containers from `src/main.c` into `src/main.rs`. Depends on: T004
- [ ] T008 [Story] Reconcile the translated 63 data structures in `src/main.rs` so shared references, ownership, and mutability patterns support the later function groups without duplicate struct definitions. Depends on: T005, T006, T007

## Phase 3: Core option-state and initialization functions

- [ ] T009 [Story] Implement the initialization-related parse-option functions from `src/main.c` in `src/main.rs`, covering creation, reset, and default-state preparation for the parser context. Depends on: T008
- [ ] T010 [Story] Implement helper functions that attach or register option descriptors into the parser state in `src/main.rs`, translating the corresponding option-table setup logic from `src/main.c`. Depends on: T008, T009
- [ ] T011 [P] [Story] Implement small accessor and state-update helpers tied to parser configuration in `src/main.rs`, translating the matching C utility functions without expanding beyond parse-option scope. Depends on: T008

## Phase 4: Argument scanning and option matching functions

- [ ] T012 [Story] Implement the functions that scan command-line tokens and classify them as options, option arguments, or positional inputs in `src/main.rs`, based on the corresponding logic from `src/main.c`. Depends on: T009, T010, T011
- [ ] T013 [Story] Implement the functions that match scanned tokens against registered option descriptors, including short/long option resolution logic translated from `src/main.c` into `src/main.rs`. Depends on: T010, T012
- [ ] T014 [P] [Story] Implement helper functions that extract, normalize, or advance option argument values during scanning in `src/main.rs`, corresponding to the local token-handling routines in `src/main.c`. Depends on: T012

## Phase 5: Parse execution and result handling functions

- [ ] T015 [Story] Implement the main parse execution functions in `src/main.rs` that drive full option parsing over the input argument list, integrating initialization, scanning, and matching behavior from `src/main.c`. Depends on: T013, T014
- [ ] T016 [Story] Implement the functions that apply matched option effects to parser state and parsed result storage in `src/main.rs`, translating the corresponding action routines from `src/main.c`. Depends on: T013, T015
- [ ] T017 [Story] Implement the remaining completion and return-path functions in `src/main.rs` that finalize parse status, remaining arguments, and parse outcome reporting from the original `src/main.c` logic. Depends on: T015, T016

## Final Phase: Polish

- [ ] T018 [Story] Refine the full `src/main.rs` parse-option port for idiomatic Rust structure, removing redundant transitional definitions introduced during migration while preserving the behavior of all 15 translated functions. Depends on: T017
- [ ] T019 [Story] Perform a final module-scope review in `src/main.rs` to verify each parse-option function and all 63 referenced data structures from `src/main.c` were migrated exactly once and remain confined to the `module_src_parseopt_03` scope. Depends on: T018