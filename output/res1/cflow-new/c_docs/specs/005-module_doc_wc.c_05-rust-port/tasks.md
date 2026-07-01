# Tasks: module_doc_wc.c_05

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for the `doc/wc.c` port on branch `005-module_doc_wc.c_05-rust-port`, adding the target source file at `src/doc/wc.rs`.
- [ ] T002 [P] [Story] Wire the new module into the Rust crate module tree so `src/doc/wc.rs` is compiled and reachable from the existing `src/doc/mod.rs` or nearest inferable parent module file.
  - Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Review `doc/wc.c` and define the foundational Rust-side internal types, aliases, constants, and helper signatures needed by its 7 functions directly in `src/doc/wc.rs`, keeping the design minimal and limited to constructs evidenced by the C file.
  - Depends on: T001

## Phase 3: Word-count core functions

- [ ] T004 [Story] Implement the primary word-count state initialization and reset-style functions from `doc/wc.c` in `src/doc/wc.rs`, preserving the C module’s behavior and local state flow.
  - Depends on: T003
- [ ] T005 [P] [Story] Implement the input-processing and counting functions from `doc/wc.c` in `src/doc/wc.rs`, translating the core token/line/word/character accumulation logic as a single functional group.
- [ ] T006 [Story] Implement the result finalization and output/reporting functions from `doc/wc.c` in `src/doc/wc.rs`, keeping formatting and emitted values aligned with the original module behavior.
  - Depends on: T004, T005

## Phase 4: Entry/dispatch integration

- [ ] T007 [Story] Implement the remaining top-level control/dispatch function from `doc/wc.c` in `src/doc/wc.rs`, connecting initialization, processing, and reporting paths without expanding behavior beyond the source module.
  - Depends on: T004, T005, T006

## Final Phase: Polish

- [ ] T008 [Story] Refine `src/doc/wc.rs` for idiomatic Rust within the established port scope, removing redundant translation artifacts and aligning naming, visibility, and internal helpers with the completed module implementation.
  - Depends on: T007