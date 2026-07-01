# Tasks: module_src_yy_init_18

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/c.c` migration on branch `081-module_src_yy_init_18-rust-port`, adding the target source file `src/c.rs` and declaring the module from the crate entry point if not already exposed.
- [T002] [P] [Story] Establish the module-local porting layout in `src/c.rs`, reserving sections for the 13 data structures and 2 function implementations so subsequent migration work remains localized to the source file.
- [T003] [Story] Review `src/c.c` and map the 13 C data structures and 2 functions to Rust items to be implemented in `src/c.rs`, recording direct naming and ownership decisions as code comments/placeholders adjacent to each item. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Implement the first grouped set of foundational Rust data structures in `src/c.rs` for plain value/state holders from `src/c.c`, preserving field coverage and C layout intent where required by usage. Depends on: T003
- [T005] [P] [Story] Implement the second grouped set of foundational Rust data structures in `src/c.rs` for pointer-linked, nested, or composite records from `src/c.c`, translating references and optional ownership into Rust types. Depends on: T003
- [T006] [P] [Story] Implement the remaining supporting enums, aliases, constants, or helper structs needed to complete all 13 migrated data structures in `src/c.rs`, keeping definitions directly aligned with `src/c.c`. Depends on: T003
- [T007] [Story] Reconcile and finalize all 13 data-structure definitions in `src/c.rs`, resolving cross-references between the groups and ensuring the function signatures can be written against the completed types. Depends on: T004, T005, T006

## Phase 3: Functions

- [T008] [Story] Implement the initialization-oriented function group from `src/c.c` in `src/c.rs`, porting the first function to use the completed Rust data structures and preserving control flow and state setup semantics. Depends on: T007
- [T009] [Story] Implement the remaining closely related function from `src/c.c` in `src/c.rs`, keeping shared state handling and call relationships consistent with the first ported function. Depends on: T007, T008

## Final Phase: Polish

- [T010] [Story] Refine `src/c.rs` for module completion by removing temporary placeholders, tightening visibility to the minimum required for the migrated items, and simplifying any direct C-to-Rust translations without changing behavior. Depends on: T009
- [T011] [Story] Perform a final compile-oriented review of the `src/c.rs` migration to confirm the 13 data structures and 2 functions are consistently integrated and that no `src/c.c` items within this module scope were omitted. Depends on: T010