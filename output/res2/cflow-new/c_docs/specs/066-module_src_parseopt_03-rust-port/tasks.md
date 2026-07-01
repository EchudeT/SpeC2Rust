# Tasks: module_src_parseopt_03

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/main.c` port on branch `066-module_src_parseopt_03-rust-port`, adding the target module files `src/parseopt.rs` and wiring the module from `src/main.rs`.
- [T002] [P] [Story] Establish the initial public API surface in `src/parseopt.rs` for the parse-option port, reserving Rust equivalents for the module-local data structures and function entry points inferred from `src/main.c`.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the foundational data structure definitions required by the parse-option module in `src/parseopt.rs`, translating the C module’s option-related structs, enums, aliases, and constant representations before any function porting begins.
  - Depends on: T002
- [T004] [Story] Add ownership and lifetime-safe Rust representations in `src/parseopt.rs` for the module’s internal state carriers and argument/option descriptor containers so later function groups can share a stable data model.
  - Depends on: T003
- [T005] [Story] Define helper constructors, default-state initialization, and internal conversion utilities in `src/parseopt.rs` for the newly ported parse-option data structures, keeping them limited to behavior directly needed by the original `src/main.c` module.
  - Depends on: T004

## Phase 3: Option and state initialization functions

- [T006] [Story] Port the functions from `src/main.c` that initialize parse-option state, option tables, and per-run parsing context into `src/parseopt.rs`, using the foundational structures already defined.
  - Depends on: T005
- [T007] [P] [Story] Port the functions from `src/main.c` that reset, update, or finalize parse-option state bookkeeping in `src/parseopt.rs`, keeping the behavior grouped around parser state management only.
  - Depends on: T006

## Phase 4: Argument scanning and option classification functions

- [T008] [Story] Port the functions from `src/main.c` responsible for scanning input arguments and classifying tokens as short options, long options, positional arguments, or terminators into `src/parseopt.rs`.
  - Depends on: T006
- [T009] [P] [Story] Port the helper functions from `src/main.c` that match option names, inspect option metadata, and resolve the appropriate descriptor during scanning in `src/parseopt.rs`.
  - Depends on: T008

## Phase 5: Option value handling and dispatch functions

- [T010] [Story] Port the functions from `src/main.c` that consume option values and apply parsed results to the module state in `src/parseopt.rs`, including handling for flags, valued options, and internal parser updates evidenced by the original module.
  - Depends on: T008, T009
- [T011] [P] [Story] Port the dispatch-oriented helper functions from `src/main.c` that connect resolved options to their corresponding update paths or callbacks within `src/parseopt.rs`.
  - Depends on: T010

## Phase 6: Usage, diagnostics, and top-level parse flow functions

- [T012] [Story] Port the functions from `src/main.c` that generate option-related usage text, help output, or parse diagnostics into `src/parseopt.rs`, preserving module-local formatting behavior only as evidenced by the source.
  - Depends on: T005
- [T013] [Story] Port the top-level parse-control functions from `src/main.c` that orchestrate initialization, argument scanning, value handling, and completion into `src/parseopt.rs`.
  - Depends on: T007, T010, T011, T012
- [T014] [Story] Update `src/main.rs` to invoke the new Rust parse-option entry points from `src/parseopt.rs`, replacing the migrated `src/main.c` module responsibilities at the integration boundary.
  - Depends on: T013

## Final Phase: Polish

- [T015] [Story] Refine `src/parseopt.rs` and `src/main.rs` to remove C-centric temporary patterns introduced during porting, simplify control flow, and tighten Rust idioms without changing the behavior established by the migrated `src/main.c` module.
  - Depends on: T014