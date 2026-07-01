# Tasks: module_src_parser.c_31 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/parser.c` in `src/parser.rs`, and wire the module into the crate root or existing module tree on branch `094-module_src_parser.c_31-rust-port`.
- [T002] [P] [Story] Review `src/parser.c` and map the 15 functions and 11 data structures to Rust items to be implemented in `src/parser.rs`; record the implementation order as comments or TODO markers in that file. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the core Rust data structure definitions corresponding to the 11 parser-related C structures from `src/parser.c` in `src/parser.rs`, preserving their module-local responsibilities and ownership relationships. Depends on: T002
- [T004] [P] [Story] Add foundational enums, type aliases, and constant definitions required by the parser data structures and function signatures in `src/parser.rs`, based only on items directly used by `src/parser.c`. Depends on: T002
- [T005] [Story] Refine the data structure layout in `src/parser.rs` so all parser state, node, and helper structures needed by the function groups can be constructed and passed without placeholder fields. Depends on: T003, T004

## Phase 3: Parser state and lifecycle functions

- [T006] [Story] Implement the parser/module initialization and teardown functions from `src/parser.c` in `src/parser.rs`, including Rust ownership handling for parser state creation, reset, and cleanup behavior. Depends on: T005
- [T007] [Story] Implement helper functions that prepare, update, or validate parser state during setup/lifecycle flow in `src/parser.rs`, grouped with the lifecycle behavior they support. Depends on: T006

## Phase 4: Input scanning and token-processing functions

- [T008] [Story] Implement the functions from `src/parser.c` responsible for consuming source input, advancing parser position, and handling low-level token or character processing in `src/parser.rs`. Depends on: T005
- [T009] [P] [Story] Implement closely related helper functions from `src/parser.c` that classify, normalize, or stage parsed input elements for higher-level parsing in `src/parser.rs`. Depends on: T008

## Phase 5: Parse construction and result assembly functions

- [T010] [Story] Implement the functions from `src/parser.c` that build parser outputs, assemble intermediate parse structures, or connect parsed elements into final module-level results within `src/parser.rs`. Depends on: T007, T009
- [T011] [Story] Implement any remaining parser control-flow functions from `src/parser.c` that coordinate the full parse operation across lifecycle, scanning, and result assembly in `src/parser.rs`. Depends on: T010

## Final Phase: Polish

- [T012] [Story] Review `src/parser.rs` and eliminate C-centric implementation artifacts by tightening Rust signatures, ownership, and mutability while preserving the behavior of all 15 ported functions. Depends on: T011
- [T013] [P] [Story] Perform a final pass on `src/parser.rs` to remove duplicated helpers, consolidate related internal logic, and ensure the module remains scoped strictly to behavior evidenced by `src/parser.c`. Depends on: T012