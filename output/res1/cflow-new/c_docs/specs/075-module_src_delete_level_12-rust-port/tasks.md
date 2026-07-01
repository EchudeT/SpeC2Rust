# Tasks: module_src_delete_level_12

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/symbol.c` migration in `src/symbol.rs`, and register it from the crate root so the `module_src_delete_level_12` port has a dedicated target file on branch `075-module_src_delete_level_12-rust-port`.
- [T002] [P] [Story] Review `src/symbol.c` and map the 27 C data structures and 2 functions into a Rust migration inventory documented inline in `src/symbol.rs` as implementation placeholders and ordering notes. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the core Rust type definitions in `src/symbol.rs` for the data structures from `src/symbol.c`, translating the primary symbol-related records and their direct field layouts before any function porting begins. Depends on: T002
- [T004] [P] [Story] Implement the supporting Rust type definitions in `src/symbol.rs` for the remaining auxiliary, nested, and link/handle-style data structures from `src/symbol.c`, keeping names and relationships aligned with the C module. Depends on: T002
- [T005] [Story] Reconcile cross-structure references in `src/symbol.rs`, completing enums, aliases, option-like pointer representations, and shared field types so all 27 migrated data structures compile together as one coherent module. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Port the first symbol-management function from `src/symbol.c` into `src/symbol.rs`, wiring it against the migrated Rust data structures without extending behavior beyond the original module scope. Depends on: T005
- [T007] [Story] Port the second symbol-management function from `src/symbol.c` into `src/symbol.rs`, completing the function migration for this module and preserving the original relationship with the shared symbol data structures. Depends on: T005
- [T008] [Story] Integrate and compile-check the two migrated functions in `src/symbol.rs`, resolving any signature, ownership, or module-local consistency issues introduced by the full `src/symbol.c` port. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Refine `src/symbol.rs` for module-local clarity and minimal Rust idiomatic cleanup, removing temporary placeholders from the migration and ensuring the final file is focused on the ported structures and functions only. Depends on: T008
- [T010] [Story] Perform a final pass on `src/symbol.rs` to verify the `src/symbol.c` migration is complete for this module, with stable item visibility and no leftover incomplete stubs tied to `module_src_delete_level_12`. Depends on: T009