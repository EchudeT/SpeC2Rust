# Tasks: module_src_parseopt_wordwrap.c_14

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the wordwrap port in `src/parseopt/wordwrap.rs`, mirroring the source scope from `src/parseopt/wordwrap.c`.
- [T002] [P] [Story] Register the new Rust module from the existing parseopt module tree so `src/parseopt/wordwrap.rs` is compiled on branch `111-module_src_parseopt_wordwrap.c_14-rust-port`.

## Phase 2: Foundational

- [T003] [Story] Analyze the C declarations in `src/parseopt/wordwrap.c` and define the Rust-owned foundational data structures, enums, constants, and type aliases needed for all 4 ported functions in `src/parseopt/wordwrap.rs`. Depends on: T001.
- [T004] [P] [Story] Implement the text-buffer and line-state structures required by the wrapping logic in `src/parseopt/wordwrap.rs`, preserving the C module’s field intent and ownership model. Depends on: T003.
- [T005] [P] [Story] Implement the option/state-holder structures used to carry width, indentation, spacing, and wrapping configuration through the module in `src/parseopt/wordwrap.rs`. Depends on: T003.
- [T006] [Story] Implement any remaining helper structs and internal enums from `src/parseopt/wordwrap.c` so the full set of 18 data structures is represented in `src/parseopt/wordwrap.rs`. Depends on: T004, T005.

## Phase 3: Core word wrapping functions

- [T007] [Story] Port the low-level initialization and state-reset function group from `src/parseopt/wordwrap.c` into idiomatic internal Rust functions in `src/parseopt/wordwrap.rs`, wiring them to the foundational structures. Depends on: T006.
- [T008] [Story] Port the token accumulation and width-accounting function group from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, preserving wrapping decisions and buffer updates. Depends on: T006.
- [T009] [Story] Port the line-break and output-emission function group from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, keeping behavior aligned with the original formatting flow. Depends on: T007, T008.
- [T010] [Story] Port the public/coordinating entry-point function that drives word wrapping in `src/parseopt/wordwrap.rs`, connecting initialization, token processing, and final flush behavior. Depends on: T007, T008, T009.

## Final Phase: Polish

- [T011] [Story] Refine `src/parseopt/wordwrap.rs` for idiomatic Rust cleanup by removing migration scaffolding, consolidating internal helpers, and ensuring signatures and visibility match actual module use. Depends on: T010.
- [T012] [Story] Review the completed port in `src/parseopt/wordwrap.rs` for parity with `src/parseopt/wordwrap.c`, confirming all 4 functions and 18 data structures are covered without adding out-of-scope behavior. Depends on: T011.