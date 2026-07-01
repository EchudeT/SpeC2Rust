# Tasks: module_src_parseopt_optdef_04

## Phase 1: Setup

- [T001] [Story] Create Rust module scaffolding for the parseopt port in `src/parseopt/help.rs` and `src/parseopt/parseopt.rs`, and wire both files into the crate module tree used by branch `101-module_src_parseopt_optdef_04-rust-port`.
- [T002] [P] [Story] Define the Rust-side file ownership split for migrated content from `src/parseopt/help.c` and `src/parseopt/parseopt.c`, keeping help-related items in `src/parseopt/help.rs` and option-definition / parsing items in `src/parseopt/parseopt.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational option-definition data structures required by this module in `src/parseopt/parseopt.rs`, covering the Rust representations needed for the migrated option tables, flags, value descriptors, and parse-state data referenced by the 2 source files. Depends on: T001.
- [T004] [P] [Story] Implement the foundational help/usage presentation data structures in `src/parseopt/help.rs`, covering the Rust representations needed for help text fragments, option display metadata, and help-generation state referenced by the 2 source files. Depends on: T001.
- [T005] [Story] Add shared constructors, defaults, and internal conversion helpers needed to connect the parse-state structures in `src/parseopt/parseopt.rs` with the help-state structures in `src/parseopt/help.rs`. Depends on: T003, T004.

## Phase 3: Option Definition and Parse Core Functions

- [T006] [Story] Port the option-definition initialization and registration function group from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, using the foundational Rust option-definition structures created in Phase 2. Depends on: T003, T005.
- [T007] [Story] Port the core option parsing and option lookup function group from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, preserving the module-local control flow around argument scanning, option matching, and parse-state updates. Depends on: T006.
- [T008] [P] [Story] Port the option value extraction and validation helper function group from `src/parseopt/parseopt.c` into `src/parseopt/parseopt.rs`, keeping the implementation aligned with the Rust parse-state and option-definition structures. Depends on: T006.

## Phase 4: Help and Usage Functions

- [T009] [Story] Port the help text assembly function group from `src/parseopt/help.c` into `src/parseopt/help.rs`, using the Rust help data structures and preserving the original option-display ordering and formatting logic. Depends on: T004, T005, T006.
- [T010] [P] [Story] Port the usage-line and option-summary rendering function group from `src/parseopt/help.c` into `src/parseopt/help.rs`, keeping output construction local to the migrated help module. Depends on: T004, T009.
- [T011] [Story] Integrate the parse and help paths by porting the cross-module functions that trigger help/usage behavior from parsing code, updating call sites between `src/parseopt/parseopt.rs` and `src/parseopt/help.rs`. Depends on: T007, T008, T009, T010.

## Final Phase: Polish

- [T012] [Story] Refine the migrated Rust implementation in `src/parseopt/parseopt.rs` and `src/parseopt/help.rs` by removing C-oriented migration leftovers, tightening internal APIs, and ensuring the final module layout matches the original file responsibilities without duplicating logic. Depends on: T011.