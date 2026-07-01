# Tasks: module_src Rust port

**Input module:** `src/shc.c`
**Target branch:** `001-module_src-rust-port`

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module target for the `src/shc.c` port by creating `src/module_src.rs` and wiring its module declaration from the crate root entry already used by the project.
- [T002] [Story] Establish the initial public/internal item layout in `src/module_src.rs` for the 5 data structures and 18 functions identified from `src/shc.c`, preserving the C module’s scope boundaries for the Rust port. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port the foundational data structures from `src/shc.c` into Rust definitions in `src/module_src.rs`, including their fields, ownership model, and visibility required by the module functions. Depends on: T002
- [T004] [P] [Story] Add core associated constructors/default initialization helpers in `src/module_src.rs` for the ported data structures where required to support direct translation of module state setup from `src/shc.c`. Depends on: T003
- [T005] [P] [Story] Add shared enums, type aliases, and constant values in `src/module_src.rs` only where directly required to represent the original `src/shc.c` structure fields and function signatures. Depends on: T003

## Phase 3: Core module lifecycle and state functions

- [T006] [Story] Implement the function group in `src/module_src.rs` responsible for module initialization, teardown, and primary state preparation from `src/shc.c`, using the ported data structures as the execution base. Depends on: T004, T005
- [T007] [Story] Implement the function group in `src/module_src.rs` responsible for resetting, clearing, or reinitializing module-held state from `src/shc.c` without duplicating initialization logic. Depends on: T006

## Phase 4: Input parsing and command processing functions

- [T008] [Story] Implement the function group in `src/module_src.rs` responsible for parsing input, arguments, or command text handled by `src/shc.c`, preserving the original control flow and data updates. Depends on: T006
- [T009] [P] [Story] Implement helper functions in `src/module_src.rs` that support token handling, intermediate parsing state, or validation paths directly used by the parsing/command-processing functions from `src/shc.c`. Depends on: T008

## Phase 5: Transformation, execution, and output functions

- [T010] [Story] Implement the function group in `src/module_src.rs` responsible for the core transformation or execution behavior performed by `src/shc.c`, translating each remaining mainline function exactly once into Rust. Depends on: T007, T008, T009
- [T011] [P] [Story] Implement the function group in `src/module_src.rs` responsible for formatting, emitting, or returning final module results handled by `src/shc.c`, matching the original data flow. Depends on: T010
- [T012] [P] [Story] Implement any remaining localized utility functions from `src/shc.c` in `src/module_src.rs` that are used by the execution/output path and were not already covered in earlier phases. Depends on: T010

## Final Phase: Polish

- [T013] [Story] Refine `src/module_src.rs` to remove temporary placeholders, align naming and signatures with the completed Rust port of `src/shc.c`, and ensure all 18 functions and 5 data structures are represented once with no duplicated logic. Depends on: T011, T012
- [T014] [Story] Perform final pass optimization and idiomatic cleanup in `src/module_src.rs`, reducing unnecessary cloning/borrowing friction introduced during translation while preserving the original module behavior. Depends on: T013