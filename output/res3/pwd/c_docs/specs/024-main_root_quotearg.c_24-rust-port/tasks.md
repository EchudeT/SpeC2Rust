# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `quotearg.c` port in `src/quotearg.rs`, and expose it from the crate root in `src/lib.rs` or `src/main.rs` as appropriate for branch `024-main_root_quotearg.c_24-rust-port`.
- [T002] [Story] Define the module migration boundary in `src/quotearg.rs` by adding placeholders for the 29 data structures and 8 functions identified for `quotearg.c`, keeping names and grouping aligned with the source module for incremental porting. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the core quoting configuration data structures in `src/quotearg.rs`, covering the primary option/state types needed to represent quoting style, flags, and active configuration. Depends on: T002
- [T004] [P] [Story] Implement the supporting static/constant data structures in `src/quotearg.rs` that describe quoting tables, style metadata, and default values used by the module. Depends on: T002
- [T005] [P] [Story] Implement the remaining helper and internal data structures in `src/quotearg.rs` required to complete the full set of 29 structures from `quotearg.c`, ensuring field layout and ownership semantics are suitable for Rust usage. Depends on: T003, T004

## Phase 3: Configuration and option access functions

- [T006] [Story] Implement the functions in `src/quotearg.rs` that initialize, clone, or reset quoting option/state values, grouped as the configuration entry points for the ported module. Depends on: T003, T005
- [T007] [P] [Story] Implement the functions in `src/quotearg.rs` that read or update quoting style and related option fields, keeping all option-manipulation behavior together in one migration step. Depends on: T006

## Phase 4: Core quoting and argument formatting functions

- [T008] [Story] Implement the core quoting function group in `src/quotearg.rs` that transforms input text according to the configured quoting options and style rules. Depends on: T004, T005, T007
- [T009] [P] [Story] Implement the wrapper functions in `src/quotearg.rs` that expose argument-oriented quoting entry points built on top of the core quoting logic, without duplicating underlying transformation behavior. Depends on: T008

## Final Phase: Polish

- [T010] [Story] Refine `src/quotearg.rs` to remove porting placeholders, consolidate internal helper visibility, and ensure the migrated data structures and 8 functions form a coherent Rust module with no redundant stubs remaining. Depends on: T009