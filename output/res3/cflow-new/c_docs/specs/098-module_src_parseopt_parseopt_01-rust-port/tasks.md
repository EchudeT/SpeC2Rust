# Tasks: module_src_parseopt_parseopt_01

## Phase 1: Setup

- [T001] [Story] Create the Rust module file skeletons for the parseopt port in `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs`, matching the source C module split.
- [T002] [Story] Wire the parseopt module into the crate module tree by declaring the Rust modules that correspond to `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs`. Depends on: T001.
- [T003] [P] [Story] Add placeholder public/internal item declarations in `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs` for all migrated parseopt-facing structures and functions so later implementation can proceed without reshaping file boundaries. Depends on: T002.

## Phase 2: Foundational

- [T004] [Story] Define the foundational option-related data structures and type aliases required by the parseopt module in `src/parseopt/optset.rs`, porting the C-side option set, option descriptor, and related storage/state representations before any function logic. Depends on: T003.
- [T005] [Story] Define the parser runtime state and argument-scanning data structures used by parse option processing in `src/parseopt/parseopt.rs`, keeping ownership and borrowing boundaries aligned with the original parse flow. Depends on: T003.
- [T006] [Story] Define the help/usage formatting data structures and supporting enums/constants used by parseopt help generation in `src/parseopt/help.rs`. Depends on: T003.
- [T007] [P] [Story] Normalize shared field names, visibility, and cross-module references among `src/parseopt/optset.rs`, `src/parseopt/parseopt.rs`, and `src/parseopt/help.rs` so all 3 files compile against a stable foundational API. Depends on: T004, T005, T006.

## Phase 3: Option Set Construction and Lookup

- [T008] [Story] Implement the option-set initialization and teardown-equivalent routines in `src/parseopt/optset.rs`, translating the C module’s setup/reset behavior into Rust-managed construction and clear state transitions. Depends on: T007.
- [T009] [Story] Implement option registration and insertion routines in `src/parseopt/optset.rs` that populate the option set from descriptors and preserve the original grouping/order semantics. Depends on: T008.
- [T010] [Story] Implement option lookup and selection helpers in `src/parseopt/optset.rs` for resolving short options, long options, and internal descriptor access during parsing. Depends on: T009.

## Phase 4: Argument Parsing Flow

- [T011] [Story] Implement low-level argument cursor advancement and token classification helpers in `src/parseopt/parseopt.rs` to recognize positional arguments, short-option clusters, long options, separators, and attached values. Depends on: T007, T010.
- [T012] [Story] Implement option argument extraction and validation helpers in `src/parseopt/parseopt.rs`, including required/optional value handling based on descriptors supplied from `src/parseopt/optset.rs`. Depends on: T011.
- [T013] [Story] Implement the main parse loop and dispatch routines in `src/parseopt/parseopt.rs`, connecting runtime parser state with option-set lookup and descriptor-driven result updates. Depends on: T012.
- [T014] [Story] Implement parse completion and final state reporting routines in `src/parseopt/parseopt.rs`, preserving the original module’s end-of-input and leftover-argument behavior. Depends on: T013.

## Phase 5: Help and Usage Output

- [T015] [P] [Story] Implement option-to-display transformation helpers in `src/parseopt/help.rs` that prepare names, argument markers, and metadata for user-facing help text. Depends on: T007, T010.
- [T016] [Story] Implement help text layout and line formatting routines in `src/parseopt/help.rs`, porting spacing, alignment, and wrapping behavior required for parseopt usage output. Depends on: T015.
- [T017] [Story] Implement the public help/usage emission routines in `src/parseopt/help.rs`, wiring formatted output to the option-set definitions used by the parser. Depends on: T016.

## Final Phase: Polish

- [T018] [P] [Story] Refine cross-file interfaces among `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs` to remove migration-only placeholders and ensure each function/data structure lives in its final file without duplicate responsibilities. Depends on: T014, T017.
- [T019] [Story] Perform a final pass on the Rust port in `src/parseopt/help.rs`, `src/parseopt/optset.rs`, and `src/parseopt/parseopt.rs` to simplify ownership, eliminate unnecessary mutable state, and align behavior with the original C module semantics without expanding scope. Depends on: T018.