# Tasks: module_src_parseopt_wordwrap_set_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the word wrapping port in `src/parseopt/wordwrap.rs`, and expose it from the existing `src/parseopt/mod.rs` module tree for branch `106-module_src_parseopt_wordwrap_set_09-rust-port`.
- [T002] [P] [Story] Review `src/parseopt/wordwrap.c` and map the 18 C data structures and 2 functions into Rust-owned module items to be implemented in `src/parseopt/wordwrap.rs`; record the implementation inventory as code comments or TODO markers colocated with the target items. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust data structures corresponding to the C module’s core word-wrap state and configuration records in `src/parseopt/wordwrap.rs`, preserving field intent and module-local visibility as inferred from `src/parseopt/wordwrap.c`. Depends on: T002
- [T004] [P] [Story] Implement the remaining supporting Rust data structures corresponding to helper records, iterators, buffers, and formatting context types defined in `src/parseopt/wordwrap.c` within `src/parseopt/wordwrap.rs`. Depends on: T002
- [T005] [Story] Reconcile all 18 ported data structures in `src/parseopt/wordwrap.rs` so shared field types, ownership rules, and cross-references compile cleanly as one consistent module foundation. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Implement the word-wrap state initialization and setter logic from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, wiring it to the ported configuration and state data structures. Depends on: T005
- [T007] [Story] Implement the word-wrap formatting/application routine from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, using the completed buffer, context, and state structures without expanding behavior beyond the source module. Depends on: T005
- [T008] [P] [Story] Integrate the two ported functions in `src/parseopt/wordwrap.rs` by aligning signatures, internal helper usage, and module visibility with the surrounding `src/parseopt` Rust API surface. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Perform a module-local cleanup pass in `src/parseopt/wordwrap.rs` and `src/parseopt/mod.rs` to remove stale porting placeholders, tighten type/visibility choices, and ensure the migrated word-wrap module builds cleanly on branch `106-module_src_parseopt_wordwrap_set_09-rust-port`. Depends on: T008