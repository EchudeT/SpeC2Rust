# Tasks: module_src_output_symbol_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module file layout for the ported scope covered by `src/gnu.c`, `src/output.c`, and `src/posix.c` in `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`.
- [T002] [Story] Wire the new module files into the crate module tree so `src/gnu.rs`, `src/output.rs`, and `src/posix.rs` are compiled on branch `072-module_src_output_symbol_09-rust-port`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and define the Rust representations for the 12 data structures required by this module cluster, placing GNU-specific structures in `src/gnu.rs`, output-specific structures in `src/output.rs`, and POSIX-specific structures in `src/posix.rs`.
- [T004] [P] [Story] Implement the GNU-side foundational data structures and associated field mappings in `src/gnu.rs` based on the `src/gnu.c` scope. Depends on: T003.
- [T005] [P] [Story] Implement the output-side foundational data structures and associated field mappings in `src/output.rs` based on the `src/output.c` scope. Depends on: T003.
- [T006] [P] [Story] Implement the POSIX-side foundational data structures and associated field mappings in `src/posix.rs` based on the `src/posix.c` scope. Depends on: T003.
- [T007] [Story] Reconcile shared structure usage across `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`, adjusting visibility and type references so function implementations can consume the new data structures without duplicating definitions. Depends on: T004, T005, T006.

## Phase 3: Functions

- [T008] [P] [Story] Implement the GNU symbol-output function group in `src/gnu.rs`, porting the function logic from `src/gnu.c` against the completed Rust data structures. Depends on: T007.
- [T009] [P] [Story] Implement the generic output symbol-formatting function group in `src/output.rs`, porting the function logic from `src/output.c` against the completed Rust data structures. Depends on: T007.
- [T010] [P] [Story] Implement the POSIX symbol-output function group in `src/posix.rs`, porting the function logic from `src/posix.c` against the completed Rust data structures. Depends on: T007.
- [T011] [Story] Integrate cross-module call sites among `src/gnu.rs`, `src/output.rs`, and `src/posix.rs` so the three ported functions use consistent interfaces and preserve the original module-cluster behavior. Depends on: T008, T009, T010.

## Final Phase: Polish

- [T012] [Story] Refine the Rust implementations in `src/gnu.rs`, `src/output.rs`, and `src/posix.rs` by removing porting-only scaffolding, tightening signatures and visibility, and resolving compile-time warnings introduced during the module migration. Depends on: T011.