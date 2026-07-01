# Tasks: module_src_parseopt_parseopt_03

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the parseopt port in `src/parseopt/mod.rs`, declaring the `parseopt` submodule boundary that will host the migration from `src/parseopt/parseopt.c`.
- [T002] [P] [Story] Create the Rust implementation file `src/parseopt/parseopt.rs` and add the initial public/internal item layout needed for the module port. Depends on: T001.
- [T003] [Story] Wire the new module into the existing crate module tree from the nearest parent module file so `src/parseopt/parseopt.rs` is compiled on branch `100-module_src_parseopt_parseopt_03-rust-port`. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Inventory and define the Rust representations for the parseopt module data structures from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, converting the module-local structs, enums, aliases, constants, and state holders needed by the function set. Depends on: T002.
- [T005] [P] [Story] Implement foundational constructors, default state initialization, and internal helper methods for the core parseopt data structures in `src/parseopt/parseopt.rs`, keeping behavior aligned with the C module’s state setup patterns. Depends on: T004.
- [T006] [P] [Story] Define ownership and borrowing boundaries for argument vectors, option descriptors, parser state, and parse results in `src/parseopt/parseopt.rs` so later function ports can reuse a stable internal model. Depends on: T004.
- [T007] [Story] Consolidate shared module-level constants, sentinel values, and internal utility types required across the parseopt implementation in `src/parseopt/parseopt.rs`. Depends on: T004.

## Phase 3: Option Model and Parser State Functions

- [T008] [Story] Port the functions that build, normalize, or expose option-definition and parser-state primitives from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, using the foundational types introduced in Phase 2. Depends on: T005, T006, T007.
- [T009] [P] [Story] Port closely related internal helper functions that classify options, inspect option metadata, or prepare parser traversal state in `src/parseopt/parseopt.rs`. Depends on: T008.

## Phase 4: Argument Scanning and Option Parsing Functions

- [T010] [Story] Port the core argument-scanning and option-consumption functions from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, covering the main control flow for stepping through input arguments and matching them to option definitions. Depends on: T008, T009.
- [T011] [P] [Story] Port the internal helpers that process short options, long options, attached values, and argument advancement rules in `src/parseopt/parseopt.rs`. Depends on: T010.
- [T012] [Story] Integrate the scanning helpers with parser state updates so consumed arguments, option values, and end-of-options transitions follow the original module behavior in `src/parseopt/parseopt.rs`. Depends on: T010, T011.

## Phase 5: Result Handling and Public Entry Functions

- [T013] [Story] Port the remaining public-facing parseopt functions from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, including the module entry points that initialize parsing, drive the parse loop, and expose parse outcomes to callers. Depends on: T012.
- [T014] [P] [Story] Port the remaining result-handling and reporting helpers that finalize parser state, surface selected option/value data, or return completion/error status from `src/parseopt/parseopt.rs`. Depends on: T013.
- [T015] [Story] Reconcile all ten migrated functions against the Rust data structures in `src/parseopt/parseopt.rs`, removing temporary gaps and ensuring each C function from `src/parseopt/parseopt.c` is represented exactly once in the port. Depends on: T013, T014.

## Final Phase: Polish

- [T016] [Story] Refine the Rust implementation in `src/parseopt/parseopt.rs` for idiomatic control flow, reduced duplication, and clearer internal naming without changing the ported behavior. Depends on: T015.
- [T017] [Story] Perform a final module pass on `src/parseopt/mod.rs` and `src/parseopt/parseopt.rs` to remove migration scaffolding, confirm module visibility is minimal, and align the finished port with the crate’s Rust style. Depends on: T016.