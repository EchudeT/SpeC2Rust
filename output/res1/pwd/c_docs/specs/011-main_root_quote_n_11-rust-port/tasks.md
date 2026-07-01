# Tasks: main_root_quote_n_11

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for the `quotearg.c` port in `src/main_root_quote_n_11.rs`, and expose it from the crate root if needed by the `pwd` binary on branch `011-main_root_quote_n_11-rust-port`.
- [ ] T002 [Story] Define the file-local migration boundaries and placeholders in `src/main_root_quote_n_11.rs` for the 29 data structures and 2 functions from `quotearg.c`, keeping the implementation scope limited to this module.

## Phase 2: Foundational

- [ ] T003 [Story] Port the quote-related constant definitions, enums, and simple type aliases from `quotearg.c` into Rust in `src/main_root_quote_n_11.rs`. Depends on: T001, T002
- [ ] T004 [P] [Story] Port the quote option record types and associated field layouts from `quotearg.c` into Rust structs in `src/main_root_quote_n_11.rs`. Depends on: T003
- [ ] T005 [P] [Story] Port the character classification and quoting-state helper structs from `quotearg.c` into Rust structs and internal helper types in `src/main_root_quote_n_11.rs`. Depends on: T003
- [ ] T006 [P] [Story] Port the remaining supporting data structures, static table representations, and module-local storage layouts from `quotearg.c` into Rust definitions in `src/main_root_quote_n_11.rs`. Depends on: T003
- [ ] T007 [Story] Reconcile the 29 migrated data structures into idiomatic Rust ownership and visibility rules while preserving the original module behavior contract in `src/main_root_quote_n_11.rs`. Depends on: T004, T005, T006

## Phase 3: Functions

- [ ] T008 [Story] Implement the primary quote-generation function from `quotearg.c` in `src/main_root_quote_n_11.rs`, wiring it to the migrated option/state structures and static tables. Depends on: T007
- [ ] T009 [Story] Implement the remaining quote-configuration or dispatch function from `quotearg.c` in `src/main_root_quote_n_11.rs`, grouped with the main quoting flow and without duplicating logic. Depends on: T007, T008

## Final Phase: Polish

- [ ] T010 [Story] Refine the migrated `quotearg.c` port in `src/main_root_quote_n_11.rs` by removing placeholder code, tightening signatures and visibility, and ensuring the final Rust module remains aligned with the original main-cluster scope. Depends on: T008, T009