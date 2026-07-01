# Tasks: module_src Rust port

**Input**: C module analysis for `src/shc.c`
**Branch**: `001-module_src-rust-port`

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module file `src/module_src.rs` and declare it from the crate entry so the ported `module_src` implementation has a dedicated target matching `src/shc.c`.
- [ ] T002 [Story] Add the initial module skeleton in `src/module_src.rs` with placeholders for the 5 module data structures and 18 functions to establish the port layout before implementation.

## Phase 2: Foundational

- [ ] T003 [Story] Define the 5 data structures from `src/shc.c` in `src/module_src.rs`, preserving their module-local roles, field relationships, and ownership model needed by later function ports. Depends on: T001, T002
- [ ] T004 [P] [Story] Add shared Rust type aliases, enums, constants, and helper representations in `src/module_src.rs` that are directly required to express the ported structures and function signatures from `src/shc.c`. Depends on: T003
- [ ] T005 [Story] Establish core constructor/default/initializer patterns for the ported module data structures in `src/module_src.rs` where required by the original C module’s setup flow. Depends on: T003, T004

## Phase 3: Core module lifecycle and setup functions

- [ ] T006 [Story] Implement the module initialization and top-level setup functions from `src/shc.c` in `src/module_src.rs`, mapping the original lifecycle entry behavior onto the Rust data structures. Depends on: T005
- [ ] T007 [Story] Implement the module teardown, reset, or finalization functions from `src/shc.c` in `src/module_src.rs`, preserving cleanup semantics within Rust ownership rules. Depends on: T006
- [ ] T008 [P] [Story] Implement any direct configuration or state preparation functions that support module startup in `src/module_src.rs`, grouped with initialization behavior but kept separate from runtime processing logic. Depends on: T005

## Phase 4: Core processing and state-transition functions

- [ ] T009 [Story] Implement the primary stateful processing functions from `src/shc.c` in `src/module_src.rs` that operate on the module’s main data structures during normal execution. Depends on: T006, T008
- [ ] T010 [P] [Story] Implement secondary state-transition or update functions from `src/shc.c` in `src/module_src.rs` that mutate existing module state but are not lifecycle entry points. Depends on: T006, T008
- [ ] T011 [Story] Integrate the grouped processing and update functions in `src/module_src.rs` so shared invariants across the ported state transitions match the original C module behavior. Depends on: T009, T010

## Phase 5: Queries, helpers, and remaining module functions

- [ ] T012 [P] [Story] Implement read-only query, lookup, or accessor-style functions from `src/shc.c` in `src/module_src.rs`, using Rust borrowing to preserve the original inspection behavior. Depends on: T003, T004
- [ ] T013 [P] [Story] Implement internal calculation, conversion, or utility helper functions from `src/shc.c` in `src/module_src.rs` that support the already grouped lifecycle and processing logic. Depends on: T009, T010
- [ ] T014 [Story] Implement any remaining standalone module functions from `src/shc.c` in `src/module_src.rs` that do not belong to earlier lifecycle, processing, or query groups, ensuring each of the 18 original functions is ported exactly once. Depends on: T011, T012, T013

## Final Phase: Polish

- [ ] T015 [Story] Refine `src/module_src.rs` for idiomatic Rust within the established port scope by removing redundant C-style patterns, tightening visibility, and clarifying ownership/borrowing without changing module behavior. Depends on: T014
- [ ] T016 [Story] Review the completed `src/module_src.rs` port for function coverage, structure completeness, and alignment with `src/shc.c`, resolving any remaining gaps in the module migration. Depends on: T015