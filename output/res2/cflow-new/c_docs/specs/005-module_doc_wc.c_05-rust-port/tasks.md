# Tasks: module_doc_wc.c_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `doc/wc.c` port on branch `005-module_doc_wc.c_05-rust-port`, adding the target source file at `src/doc/wc.rs`.
- [T002] [Story] Wire the new module into the Rust project module tree so `src/doc/wc.rs` is compiled and reachable from the existing `src/doc/mod.rs` or nearest inferable parent module file. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Analyze `doc/wc.c` and define the shared Rust-side foundational types, aliases, constants, and internal helper signatures required by its 7 functions directly in `src/doc/wc.rs`. Depends on: T002.
- [T004] [P] [Story] Establish the internal module-level state layout and visibility boundaries needed by multiple `doc/wc.c` function ports, keeping all shared items colocated in `src/doc/wc.rs`. Depends on: T003.

## Phase 3: Core word-count flow functions

- [T005] [Story] Port the primary `doc/wc.c` entry and orchestration functions that drive the word-count behavior into `src/doc/wc.rs`, preserving the original call flow and using the shared foundations from Phase 2. Depends on: T004.
- [T006] [P] [Story] Port the closely related input-processing and line/token counting helper functions from `doc/wc.c` into `src/doc/wc.rs`, grouping only helpers used by the core word-count flow. Depends on: T005.
- [T007] [P] [Story] Port the output-formatting and result-reporting functions from `doc/wc.c` into `src/doc/wc.rs`, keeping formatting behavior aligned with the C module. Depends on: T005.
- [T008] [Story] Integrate the remaining support functions from `doc/wc.c` into `src/doc/wc.rs` so all 7 module functions are implemented exactly once and connected through the Rust module flow. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine the `src/doc/wc.rs` implementation to remove porting duplication, align naming and visibility with project conventions, and ensure the final Rust module remains a faithful, minimal-scope migration of `doc/wc.c`. Depends on: T008.