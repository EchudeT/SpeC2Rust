# Tasks: Port `src/wordsplit/wordsplit.c` to Rust

## Phase 1: Setup

- [ ] [T001] [Story] Create the module file scaffold for the Rust port by adding `src/wordsplit/wordsplit.rs` and wiring it into the crate module tree used by branch `119-module_src_wordsplit_wordsplit.c_08-rust-port`.
- [ ] [T002] [Story] Define the Rust-facing port scope for `src/wordsplit/wordsplit.rs`, listing the C module’s 11 target functions and the in-module types/constants that must move together so later tasks stay constrained to this file migration. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [Story] Port the core wordsplit state structure(s) from `src/wordsplit/wordsplit.c` into Rust data types in `src/wordsplit/wordsplit.rs`, preserving field intent and ownership boundaries needed by the function port. Depends on: T002
- [ ] [T004] [P] [Story] Port supporting enums, flag-like constants, and status/result representations referenced by the module’s functions into `src/wordsplit/wordsplit.rs`. Depends on: T002
- [ ] [T005] [P] [Story] Port the module’s internal helper record types, view structs, and parsed-item/container data structures from `src/wordsplit/wordsplit.c` into Rust definitions in `src/wordsplit/wordsplit.rs`. Depends on: T003
- [ ] [T006] [Story] Normalize the translated data structures in `src/wordsplit/wordsplit.rs` so shared buffers, token lists, and configuration/state fields align with Rust ownership and borrowing rules required by subsequent function groups. Depends on: T003, T004, T005

## Phase 3: Initialization and teardown functions

- [ ] [T007] [Story] Implement the wordsplit object initialization/allocation function group from `src/wordsplit/wordsplit.c` in `src/wordsplit/wordsplit.rs`, using the foundational Rust state and flag types. Depends on: T006
- [ ] [T008] [Story] Implement the reset/cleanup/free function group from `src/wordsplit/wordsplit.c` in `src/wordsplit/wordsplit.rs`, covering release of translated buffers, token arrays, and transient parse state. Depends on: T006
- [ ] [T009] [Story] Reconcile initialization and cleanup paths in `src/wordsplit/wordsplit.rs` so the ported lifecycle functions share consistent state transitions and do not duplicate field handling. Depends on: T007, T008

## Phase 4: Core split and parse functions

- [ ] [T010] [Story] Implement the primary word-splitting entry function group from `src/wordsplit/wordsplit.c` in `src/wordsplit/wordsplit.rs`, mapping the main input-processing flow onto the Rust ported state model. Depends on: T009
- [ ] [T011] [Story] Implement the token parsing/scanning helper function group that supports the main split flow in `src/wordsplit/wordsplit.rs`, including character traversal and token boundary handling required by the C module logic. Depends on: T006
- [ ] [T012] [Story] Integrate the core split entry points with the token parsing helpers in `src/wordsplit/wordsplit.rs`, ensuring each of the ported functions is connected once within the translated control flow. Depends on: T010, T011

## Phase 5: Expansion, options, and output assembly functions

- [ ] [T013] [P] [Story] Implement the option/configuration handling function group from `src/wordsplit/wordsplit.c` in `src/wordsplit/wordsplit.rs`, covering ported setters/readers and in-module flag interpretation used during splitting. Depends on: T006
- [ ] [T014] [P] [Story] Implement the expansion/substitution-related function group from `src/wordsplit/wordsplit.c` in `src/wordsplit/wordsplit.rs`, limited to behaviors evidenced within this module file and its translated state. Depends on: T011
- [ ] [T015] [Story] Implement the output assembly/finalization function group from `src/wordsplit/wordsplit.c` in `src/wordsplit/wordsplit.rs`, producing the final split-word collections from the parsed intermediate state. Depends on: T012, T013, T014

## Final Phase: Polish

- [ ] [T016] [Story] Refine `src/wordsplit/wordsplit.rs` to remove C-specific duplication that became unnecessary during translation, while preserving the original module behavior and keeping all 11 ported functions within this file’s scope. Depends on: T015
- [ ] [T017] [Story] Perform a final pass on `src/wordsplit/wordsplit.rs` for idiomatic Rust cleanup of signatures, enums, and internal ownership usage without expanding beyond the migrated `src/wordsplit/wordsplit.c` module. Depends on: T016