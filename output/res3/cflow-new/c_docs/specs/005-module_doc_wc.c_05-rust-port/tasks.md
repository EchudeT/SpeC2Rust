# Tasks: module_doc_wc.c_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `doc/wc.c` port on branch `005-module_doc_wc.c_05-rust-port`, adding the target source file at `src/doc/wc.rs`.
- [T002] [P] [Story] Wire the new module file `src/doc/wc.rs` into the existing Rust project module tree so the `doc/wc.c` migration unit is reachable from the crate root. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `doc/wc.c` and define the foundational Rust-side constants, type aliases, and internal helper signatures required to support all 7 migrated functions, implementing them in `src/doc/wc.rs`.
- [T004] [Story] Establish the module-level state flow and shared internal utility routines in `src/doc/wc.rs` that are needed by multiple `doc/wc.c` functions before porting function bodies. Depends on: T003.

## Phase 3: Word-count core functions

- [T005] [Story] Port the core counting and input-processing functions from `doc/wc.c` into `src/doc/wc.rs`, preserving original control flow and behavior for the main word/line/character counting path. Depends on: T004.
- [T006] [P] [Story] Port the output-formatting and result-reporting functions from `doc/wc.c` into `src/doc/wc.rs`, keeping formatting logic aligned with the C module behavior. Depends on: T004.
- [T007] [Story] Integrate the counting and reporting function groups inside `src/doc/wc.rs`, resolving shared state usage and function call ordering to match the original `doc/wc.c` module behavior. Depends on: T005, T006.

## Phase 4: Entry and option handling functions

- [T008] [Story] Port the remaining entry-point, argument-handling, and module-orchestration functions from `doc/wc.c` into `src/doc/wc.rs`, limiting scope to the functions present in this module. Depends on: T007.
- [T009] [Story] Complete the direct migration coverage check in `src/doc/wc.rs` to ensure all 7 functions from `doc/wc.c` are implemented once and only once, with no leftover C-only placeholders. Depends on: T008.

## Final Phase: Polish

- [T010] [Story] Refine `src/doc/wc.rs` for Rust idioms that do not alter behavior, simplifying internal ownership/borrowing and removing migration scaffolding introduced during the port. Depends on: T009.
- [T011] [Story] Perform a final module-level compile and integration cleanup for `src/doc/wc.rs`, resolving warnings and import issues introduced by the `doc/wc.c` migration. Depends on: T010.