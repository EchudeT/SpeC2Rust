# Tasks: cflow-new / module_src_linked_list_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the ported linked-list cluster by adding or updating `src/linked_list.rs` and `src/symbol.rs`, and wire both modules into the crate so later data-structure and function migration can be implemented in the Rust branch `068-module_src_linked_list_05-rust-port`.
- [T002] [P] [Story] Review `src/linked-list.c` and `src/symbol.c` and map the 53 referenced C data structures into a Rust-facing inventory documented in code comments or placeholder type declarations inside `src/linked_list.rs` and `src/symbol.rs` to establish the migration surface for this module cluster. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust data structures that directly correspond to linked-list node, link ownership, traversal state, and list container concepts from `src/linked-list.c` in `src/linked_list.rs`, preserving relationships needed by all 3 migrated functions. Depends on: T002.
- [T004] [P] [Story] Implement the foundational Rust data structures that directly correspond to symbol-side records, references, and list-linked symbol state from `src/symbol.c` in `src/symbol.rs`, including fields required to interoperate with the linked-list types defined in `src/linked_list.rs`. Depends on: T002.
- [T005] [Story] Reconcile shared or cross-file structure usage between `src/linked_list.rs` and `src/symbol.rs`, deduplicating overlapping type definitions and finalizing the module-visible interfaces required for the function ports. Depends on: T003, T004.

## Phase 3: Linked-list core functions

- [T006] [Story] Port the linked-list construction and mutation function group from `src/linked-list.c` into `src/linked_list.rs`, implementing the C module’s list-insertion/list-linking behavior against the Rust data structures completed in Phase 2. Depends on: T005.
- [T007] [Story] Port the linked-list traversal or extraction function group from `src/linked-list.c` into `src/linked_list.rs`, keeping function behavior aligned with the original C control flow and list semantics without re-splitting work already covered by T006. Depends on: T006.

## Phase 4: Symbol integration functions

- [T008] [Story] Port the symbol-facing function from `src/symbol.c` into `src/symbol.rs`, integrating it with the Rust linked-list APIs and shared structures finalized in `src/linked_list.rs` and `src/symbol.rs`. Depends on: T005, T007.

## Final Phase: Polish

- [T009] [Story] Refine the migrated implementation in `src/linked_list.rs` and `src/symbol.rs` by removing temporary placeholders, tightening ownership/borrowing around the migrated list and symbol structures, and aligning naming and module visibility with the completed Rust port. Depends on: T008.