# Tasks: module_src_parser.c_31 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/parser.c` in `src/parser.rs`, and register it from the crate root or existing module tree so the `094-module_src_parser.c_31-rust-port` branch has a dedicated target file for this port.
- [T002] [P] [Story] Review `src/parser.c` and map its 11 data structures and 15 functions into a Rust implementation outline inside `src/parser.rs`, identifying function groupings and structure ownership needed for the port. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the first foundational set of parser-related Rust data structures in `src/parser.rs`, translating the corresponding C structs/enums from `src/parser.c` that define parser state and core parsing context. Depends on: T002
- [T004] [P] [Story] Implement the second foundational set of parser-related Rust data structures in `src/parser.rs`, translating the corresponding C structs/enums from `src/parser.c` that define parser tokens, node records, or intermediate parsing values used by multiple functions. Depends on: T002
- [T005] [P] [Story] Implement the remaining support data structures from `src/parser.c` in `src/parser.rs`, including any helper enums, flags, or aggregate records required before function porting begins. Depends on: T002
- [T006] [Story] Reconcile the 11 translated data structures in `src/parser.rs` so their field types, ownership model, and visibility support all 15 parser functions without placeholder C-style patterns. Depends on: T003, T004, T005

## Phase 3: Parser initialization and lifecycle functions

- [T007] [Story] Port the parser initialization and teardown function group from `src/parser.c` into `src/parser.rs`, covering functions that create, reset, or release parser state. Depends on: T006
- [T008] [P] [Story] Port helper functions that prepare parser context, configure defaults, or connect parser-owned working data used immediately after initialization in `src/parser.rs`. Depends on: T006
- [T009] [Story] Integrate the initialization/lifecycle function group in `src/parser.rs` so shared state transitions match the original `src/parser.c` control flow without duplicating setup logic. Depends on: T007, T008

## Phase 4: Token and input handling functions

- [T010] [Story] Port the token-reading and input-consumption function group from `src/parser.c` into `src/parser.rs`, covering functions that advance through source input and update parser position/state. Depends on: T009
- [T011] [P] [Story] Port related token classification, lookahead, or low-level parse helper functions from `src/parser.c` into `src/parser.rs` that are directly used by the input-consumption path. Depends on: T009
- [T012] [Story] Wire the token/input handling functions together in `src/parser.rs`, ensuring shared parser state and helper return values follow the same sequencing as in `src/parser.c`. Depends on: T010, T011

## Phase 5: Core parsing functions

- [T013] [Story] Port the primary syntax parsing function group from `src/parser.c` into `src/parser.rs`, covering the main routines that build or update parsed structures from consumed input. Depends on: T012
- [T014] [P] [Story] Port subsidiary parse-step functions from `src/parser.c` into `src/parser.rs` that handle nested grammar branches, element parsing, or parser decision points called by the main parsing routines. Depends on: T012
- [T015] [Story] Integrate the core parsing function group in `src/parser.rs` so parser-state mutation, intermediate values, and returned parse results remain consistent across the full call chain. Depends on: T013, T014

## Phase 6: Finalization and result functions

- [T016] [Story] Port the parser finalization and output/result handling function group from `src/parser.c` into `src/parser.rs`, covering functions that complete parsing, expose parse results, or clean up final intermediate state. Depends on: T015
- [T017] [P] [Story] Port any remaining parser utility functions from `src/parser.c` into `src/parser.rs` that are not part of earlier groups but are required to complete the 15-function port. Depends on: T015
- [T018] [Story] Resolve remaining call sites inside `src/parser.rs` so all translated parser functions are connected and no C-only helper assumptions remain. Depends on: T016, T017

## Final Phase: Polish

- [T019] [Story] Refine the Rust implementation in `src/parser.rs` by removing temporary porting scaffolds, tightening type usage, and simplifying control flow where the direct C translation can be made idiomatic without changing module behavior. Depends on: T018
- [T020] [Story] Perform a final module pass over `src/parser.rs` to confirm all 11 data structures and 15 functions from `src/parser.c` are represented exactly once in the Rust port and that task-group boundaries have been fully implemented. Depends on: T019