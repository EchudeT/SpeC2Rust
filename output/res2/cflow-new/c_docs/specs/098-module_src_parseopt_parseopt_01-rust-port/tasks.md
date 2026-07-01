# Tasks: module_src_parseopt_parseopt_01

## Phase 1: Setup

- [T001] [Story] Create the Rust module layout for the parseopt port by adding `src/parseopt/mod.rs`, `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs`, and wire the module exports needed for this module cluster.
- [T002] [Story] Register the new parseopt module in the Rust crate entry points that already expose migrated source modules so `src/parseopt/mod.rs` is compiled on branch `098-module_src_parseopt_parseopt_01-rust-port`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the foundational option-related data structures, enums, constants, and type aliases from `src/parseopt/optset.c` into `src/parseopt/optset.rs`, preserving the C module’s option-set layout and ownership model.
- [T004] [P] [Story] Port the help/usage formatting data structures, enums, constants, and supporting static definitions from `src/parseopt/help.c` into `src/parseopt/help.rs`, aligned with the option definitions introduced in `src/parseopt/optset.rs`. Depends on: T003.
- [T005] [P] [Story] Port the parser state data structures, enums, constants, and internal record types from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, using the shared option-set definitions from `src/parseopt/optset.rs`. Depends on: T003.
- [T006] [Story] Reconcile cross-file structure usage by exposing the shared parseopt types through `src/parseopt/mod.rs` and resolving references between `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs`. Depends on: T004, T005.

## Phase 3: Option-set construction and lookup functions

- [T007] [Story] Implement the option-set creation, initialization, and teardown functions from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, using the foundational option-set types already ported. Depends on: T006.
- [T008] [Story] Implement the option registration, insertion, and configuration update functions from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, keeping behavior grouped around building the in-memory option catalog. Depends on: T007.
- [T009] [Story] Implement the option lookup, matching, and retrieval functions from `src/parseopt/optset.c` in `src/parseopt/optset.rs`, completing the reusable option-set API required by parsing and help generation. Depends on: T008.

## Phase 4: Help and usage rendering functions

- [T010] [Story] Implement the help text preparation and layout helper functions from `src/parseopt/help.c` in `src/parseopt/help.rs`, using the shared option-set structures and preserving output-oriented grouping from the C module. Depends on: T006, T009.
- [T011] [Story] Implement the public help/usage rendering functions from `src/parseopt/help.c` in `src/parseopt/help.rs`, completing the translated help-generation flow on top of the layout helpers. Depends on: T010.

## Phase 5: Argument parsing functions

- [T012] [Story] Implement the parser initialization and parser-state setup functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, wiring them to the option-set APIs from `src/parseopt/optset.rs`. Depends on: T006, T009.
- [T013] [Story] Implement the core argument scanning and option dispatch functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, covering token consumption, option recognition, and parse-state advancement. Depends on: T012.
- [T014] [Story] Implement the parse completion, operand handling, and parser finalization functions from `src/parseopt/parseopt.c` in `src/parseopt/parseopt.rs`, finishing the functional port of the parse flow. Depends on: T013.

## Final Phase: Polish

- [T015] [P] [Story] Refine the Rust implementations in `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs` to remove migration leftovers, tighten module visibility, and align signatures and internal ownership with idiomatic Rust while preserving C-module behavior. Depends on: T011, T014.
- [T016] [Story] Perform final integration cleanup in `src/parseopt/mod.rs` and the parseopt Rust source files to ensure the module cluster builds cleanly with consistent exports and no unused cross-module items. Depends on: T015.