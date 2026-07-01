# Tasks: module_src_parseopt_wordwrap.c_13

## Phase 1: Setup

- [ ] [T001] [Story] Create the Rust module file `src/parseopt/wordwrap.rs` to host the port of `src/parseopt/wordwrap.c`.
- [ ] [T002] [Story] Wire `src/parseopt/wordwrap.rs` into the existing Rust module tree from `src/parseopt/mod.rs` or the nearest parseopt parent module so the ported module builds. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [Story] Port the module-level constants, type aliases, and core scalar definitions from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs` as Rust equivalents. Depends on: T001
- [ ] [T004] [P] [Story] Define the foundational Rust structs corresponding to the C module's persistent word-wrapping state in `src/parseopt/wordwrap.rs`. Depends on: T003
- [ ] [T005] [P] [Story] Define the auxiliary Rust structs and enums corresponding to the C module's helper records, flags, and temporary wrapping metadata in `src/parseopt/wordwrap.rs`. Depends on: T003
- [ ] [T006] [Story] Establish ownership, lifetime, and mutability relationships among the 18 ported data structures in `src/parseopt/wordwrap.rs`, including constructor/default patterns needed by later function ports. Depends on: T004, T005

## Phase 3: Initialization and state-management functions

- [ ] [T007] [Story] Implement the word-wrap initialization and reset-related functions from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, preserving the original module state setup behavior. Depends on: T006
- [ ] [T008] [Story] Implement the configuration/update functions that adjust width, indentation, margins, or other wrap-state parameters in `src/parseopt/wordwrap.rs`. Depends on: T007
- [ ] [T009] [Story] Implement the cleanup/finalization functions from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, translating any C resource-state teardown into Rust state transitions. Depends on: T007

## Phase 4: Core wrapping and buffer-processing functions

- [ ] [T010] [Story] Implement the core line-building and word-placement functions from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, covering accumulation of words into the active output line. Depends on: T008
- [ ] [T011] [Story] Implement the line-break and flush/output-transition functions in `src/parseopt/wordwrap.rs`, preserving the original conditions for emitting completed wrapped lines. Depends on: T010
- [ ] [T012] [Story] Implement the helper functions that classify input text spans, whitespace, separators, or token boundaries for the wrapping logic in `src/parseopt/wordwrap.rs`. Depends on: T006
- [ ] [T013] [Story] Integrate helper classification logic into the core wrapping path so the main wrap functions in `src/parseopt/wordwrap.rs` match the C module's token-processing flow. Depends on: T010, T011, T012

## Phase 5: Public entrypoints and module integration functions

- [ ] [T014] [Story] Implement the high-level public entrypoint functions from `src/parseopt/wordwrap.c` that accept caller input and drive wrapping in `src/parseopt/wordwrap.rs`. Depends on: T013
- [ ] [T015] [P] [Story] Implement any remaining formatting helper functions that support public entrypoints, such as indentation-prefix or spacing emission helpers, in `src/parseopt/wordwrap.rs`. Depends on: T013
- [ ] [T016] [Story] Connect the public entrypoints with the remaining helper functions so all 15 functions from `src/parseopt/wordwrap.c` are fully represented once in `src/parseopt/wordwrap.rs`. Depends on: T014, T015

## Final Phase: Polish

- [ ] [T017] [Story] Refine `src/parseopt/wordwrap.rs` to remove C-specific control-flow artifacts, simplify borrowing and state mutation, and keep behavior aligned with the original module. Depends on: T009, T016
- [ ] [T018] [Story] Review the Rust port in `src/parseopt/wordwrap.rs` for idiomatic naming, visibility, and internal organization without expanding beyond the original `wordwrap.c` module scope. Depends on: T017