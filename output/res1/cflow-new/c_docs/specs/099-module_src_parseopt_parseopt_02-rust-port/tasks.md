# Tasks: module_src_parseopt_parseopt_02

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the parse option port in `src/parseopt/mod.rs` and `src/parseopt/parseopt.rs`, and expose the new module from the existing crate root as needed for branch `099-module_src_parseopt_parseopt_02-rust-port`.
- [T002] [P] [Story] Inventory the C surface from `src/parseopt/parseopt.c` and map the 15 functions and 53 data items into a Rust port plan documented inline in `src/parseopt/parseopt.rs` as migration placeholders and section comments. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust data structures, enums, type aliases, and constants required by `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, preserving the C module’s parse-option state model before any function bodies are ported. Depends on: T002
- [T004] [P] [Story] Add Rust representations for grouped option descriptors, parser context records, and callback-related signatures inferred from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`. Depends on: T003
- [T005] [P] [Story] Add Rust representations for argument scanning state, option classification helpers, and module-local supporting fields/constants inferred from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`. Depends on: T003
- [T006] [Story] Consolidate the foundational definitions from the prior tasks into the final compile-ready layout in `src/parseopt/parseopt.rs`, resolving ownership, lifetimes, and visibility needed by the later function groups. Depends on: T004, T005

## Phase 3: Core option scanning and classification functions

- [T007] [Story] Implement the low-level option token scanning and argument classification functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, covering the functions that identify option forms and split incoming argument text for further parsing. Depends on: T006
- [T008] [P] [Story] Implement helper functions that locate matching option descriptors and interpret option metadata from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`. Depends on: T006
- [T009] [Story] Integrate the scanning and lookup helpers into a coherent internal parsing flow in `src/parseopt/parseopt.rs` so later handler functions can consume normalized option matches. Depends on: T007, T008

## Phase 4: Option value handling and parser execution functions

- [T010] [Story] Implement the functions from `src/parseopt/parseopt.c` that apply parsed option values to parser state and option records in `src/parseopt/parseopt.rs`, including value extraction and assignment behavior required by the module. Depends on: T009
- [T011] [P] [Story] Implement callback or action-dispatch functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, covering the function group that invokes option-specific handling once a match is resolved. Depends on: T009
- [T012] [Story] Implement the main parse execution functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, wiring together scanning, lookup, value handling, and dispatch into the module’s public parsing entry points. Depends on: T010, T011

## Phase 5: Usage/help formatting and exported module completion

- [T013] [Story] Implement the functions from `src/parseopt/parseopt.c` responsible for usage text, option listing, or other parseopt-facing output support in `src/parseopt/parseopt.rs`, limited to behavior evidenced by the source module. Depends on: T012
- [T014] [Story] Finalize the Rust-visible API surface for the ported parseopt module in `src/parseopt/mod.rs` and `src/parseopt/parseopt.rs`, ensuring the 15 migrated functions are exposed or kept internal consistent with the C module structure. Depends on: T012, T013

## Final Phase: Polish

- [T015] [Story] Refine `src/parseopt/parseopt.rs` for idiomatic Rust without changing module behavior, removing redundant C-style patterns introduced during migration and tightening internal helper boundaries. Depends on: T014
- [T016] [Story] Perform a final compile-pass cleanup for the parseopt port across `src/parseopt/mod.rs` and `src/parseopt/parseopt.rs`, resolving warnings, dead private items created by migration, and phase-to-phase integration issues. Depends on: T015