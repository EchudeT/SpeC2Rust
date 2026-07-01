# Tasks: module_src_parser.c_32 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/parser.c` in `src/parser.rs`, declare the module from the crate root, and add placeholder items for the parser port on branch `095-module_src_parser.c_32-rust-port`.
- [T002] [P] [Story] Review `src/parser.c` and map the 11 C data structures and 2 functions to Rust items in `src/parser.rs`, recording direct name and responsibility correspondences as implementation notes in code comments for the port.
- [T003] [Story] Define the public/private item boundaries in `src/parser.rs` so the upcoming data structures and functions can be added without further file layout changes. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Port the first group of parser-related C data structures from `src/parser.c` into Rust data type definitions in `src/parser.rs`, preserving field intent and module-local ownership semantics. Depends on: T003.
- [T005] [P] [Story] Port the second group of parser-related C data structures from `src/parser.c` into Rust data type definitions in `src/parser.rs`, preserving field intent and replacing raw C representations with idiomatic Rust equivalents where directly inferable. Depends on: T003.
- [T006] [P] [Story] Port the remaining parser-related C data structures from `src/parser.c` into Rust data type definitions in `src/parser.rs`, completing all 11 structure definitions required by this module. Depends on: T003.
- [T007] [Story] Reconcile and integrate the structure groups in `src/parser.rs`, resolving cross-references, lifetime/ownership relationships, and shared enums or aliases needed for the full parser data model. Depends on: T004, T005, T006.

## Phase 3: Functions

- [T008] [Story] Implement the first parser function from `src/parser.c` in `src/parser.rs`, wiring it to the completed Rust data structures and preserving the original control flow and state mutations. Depends on: T007.
- [T009] [Story] Implement the second parser function from `src/parser.c` in `src/parser.rs`, completing the functional port and reusing the shared parser data model established earlier. Depends on: T007.
- [T010] [Story] Resolve any direct call ordering, shared helper logic, or state coupling between the two ported parser functions in `src/parser.rs` so the module behavior matches the original C module without duplicating logic. Depends on: T008, T009.

## Final Phase: Polish

- [T011] [Story] Refine `src/parser.rs` by removing temporary placeholders, tightening visibility, and simplifying any C-style representations that can be safely expressed with clearer Rust constructs without changing module behavior. Depends on: T010.
- [T012] [Story] Perform a final module pass on `src/parser.rs` to confirm the `src/parser.c` migration is complete, consistent, and isolated to this module’s Rust port scope. Depends on: T011.