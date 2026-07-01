# Tasks: module_src_output_symbol_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffolding for the ported symbol-output cluster by adding target module files aligned to the source inputs: `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`, and wire them into the crate module tree on branch `072-module_src_output_symbol_09-rust-port`.
- [T002] [P] [Story] Establish shared symbol-output module boundaries and imports across `src/gnu.rs`, `src/output.rs`, and `src/posix.rs` so later data structure and function ports have a stable placement. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and port the 12 data structures used by this module cluster into Rust definitions, placing each structure in the most directly corresponding target file among `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`. Depends on: T002.
- [T004] [P] [Story] Add shared Rust enums, type aliases, and field visibility needed to let the ported data structures interoperate cleanly across `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`. Depends on: T003.
- [T005] [Story] Refine ownership/borrowing and initialization patterns for the ported data structures so the function ports can use them without placeholder C-style mutation assumptions in `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`. Depends on: T004.

## Phase 3: Functions

- [T006] [Story] Port the function implementation from `src/output.c` into `src/output.rs`, adapting its logic to the Rust data structures introduced for this module cluster. Depends on: T005.
- [T007] [P] [Story] Port the function implementation from `src/gnu.c` into `src/gnu.rs`, preserving GNU-specific symbol output behavior while binding to the shared Rust structures. Depends on: T005.
- [T008] [P] [Story] Port the function implementation from `src/posix.c` into `src/posix.rs`, preserving POSIX-specific symbol output behavior while binding to the shared Rust structures. Depends on: T005.
- [T009] [Story] Reconcile cross-file call sites and shared interfaces among `src/output.rs`, `src/gnu.rs`, and `src/posix.rs` so the three ported functions compile together without duplicated adaptation code. Depends on: T006, T007, T008.

## Final Phase: Polish

- [T010] [Story] Remove temporary porting scaffolds, simplify duplicated conversion logic, and align naming and module-local organization across `src/gnu.rs`, `src/output.rs`, and `src/posix.rs` for a maintainable final Rust port. Depends on: T009.
- [T011] [Story] Perform a final compile-oriented cleanup of the module cluster, resolving warnings and tightening signatures only where required by the migrated code in `src/gnu.rs`, `src/output.rs`, and `src/posix.rs`. Depends on: T010.