# Tasks: module_src_parseopt_parseopt_02

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the parseopt port in `src/parseopt/mod.rs` and `src/parseopt/parseopt.rs`, and expose the module from the existing crate entry so the migrated code has a dedicated target location.
- [T002] [P] [Story] Review `src/parseopt/parseopt.c` and map the 53 C data structures and 15 functions into a Rust-side implementation inventory inside `src/parseopt/parseopt.rs`, defining the Rust item layout and migration order for this module.
- [T003] [Story] Define the module-level Rust imports, visibility boundaries, and internal organization comments in `src/parseopt/parseopt.rs` so subsequent data-structure and function ports land in stable sections. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Port the core parse-option state structures from `src/parseopt/parseopt.c` into Rust definitions in `src/parseopt/parseopt.rs`, covering the primary parser context, option descriptor, parse result, and shared state carriers required by multiple functions. Depends on: T003
- [T005] [P] [Story] Port the supporting constant-like and flag-bearing data structures from `src/parseopt/parseopt.c` into Rust definitions in `src/parseopt/parseopt.rs`, preserving relationships to the core parser state without implementing function behavior yet. Depends on: T003
- [T006] [P] [Story] Port the remaining helper, intermediate, and callback-related data structures from `src/parseopt/parseopt.c` into Rust definitions in `src/parseopt/parseopt.rs`, including placeholder representations needed for later function groups. Depends on: T003
- [T007] [Story] Reconcile the full set of migrated data structures in `src/parseopt/parseopt.rs`, wiring references, enums, aliases, and default construction helpers so all 53 structures form a coherent base for function implementation. Depends on: T004, T005, T006

## Phase 3: Option Model and Argument State Functions

- [T008] [Story] Implement the functions in `src/parseopt/parseopt.rs` that construct, initialize, or reset parse-option state and option records, using the foundational Rust data structures as the source of truth. Depends on: T007
- [T009] [P] [Story] Implement the functions in `src/parseopt/parseopt.rs` that classify option kinds, flags, or argument expectations, grouping the C helpers that operate only on option metadata and local state. Depends on: T007
- [T010] [Story] Integrate the state-initialization and option-classification function group in `src/parseopt/parseopt.rs`, aligning signatures and shared helper usage so downstream parsing functions can call them consistently. Depends on: T008, T009

## Phase 4: Token Scanning and Option Matching Functions

- [T011] [Story] Implement the functions in `src/parseopt/parseopt.rs` that scan command-line tokens and identify whether an input item should be treated as an option, operand, or terminator, based on the original logic in `src/parseopt/parseopt.c`. Depends on: T010
- [T012] [P] [Story] Implement the functions in `src/parseopt/parseopt.rs` that match scanned tokens against option definitions, including short/long option lookup and any local comparison helpers present in `src/parseopt/parseopt.c`. Depends on: T010
- [T013] [Story] Integrate token scanning with option matching in `src/parseopt/parseopt.rs`, ensuring the grouped functions share common state updates and produce the same option-selection flow as the C module. Depends on: T011, T012

## Phase 5: Parse Execution and Result Handling Functions

- [T014] [Story] Implement the main parse-execution functions in `src/parseopt/parseopt.rs` that iterate through arguments, dispatch matched options, and advance parser state according to the behavior of `src/parseopt/parseopt.c`. Depends on: T013
- [T015] [P] [Story] Implement the functions in `src/parseopt/parseopt.rs` that finalize parse results, propagate option arguments, and report completion or parse-status outcomes defined by the original module. Depends on: T013
- [T016] [Story] Integrate the full parse-execution path with result-handling functions in `src/parseopt/parseopt.rs`, completing coverage of the 15 migrated functions without duplicating responsibilities across phases. Depends on: T014, T015

## Final Phase: Polish

- [T017] [Story] Refine `src/parseopt/parseopt.rs` for idiomatic Rust within the established port scope by removing dead migration scaffolding, tightening internal visibility, and simplifying ownership/borrowing patterns that became clear after the full function port. Depends on: T016
- [T018] [Story] Perform a final module-level consistency pass across `src/parseopt/mod.rs` and `src/parseopt/parseopt.rs`, verifying exported items, function grouping, and file organization match the completed port of `src/parseopt/parseopt.c`. Depends on: T017