# Tasks: module_src_print_function_13

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for this port branch by creating or updating `src/gnu.rs` and `src/output.rs` to host the migrated code from `src/gnu.c` and `src/output.c`.
- [T002] [P] [Story] Wire the new Rust module files into the crate module tree so `src/gnu.rs` and `src/output.rs` are compiled and accessible to the rest of the project. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the 11 module-scoped data structures required by the migrated functionality, placing GNU-related definitions in `src/gnu.rs` and output-related definitions in `src/output.rs`, while preserving the original module boundaries inferred from `src/gnu.c` and `src/output.c`. Depends on: T001.
- [T004] [Story] Add shared field types, enums, and internal helper representations needed to let the migrated structures compile cleanly across `src/gnu.rs` and `src/output.rs`. Depends on: T003.
- [T005] [P] [Story] Implement constructors, default state, or direct Rust initialization patterns for the newly introduced data structures in `src/gnu.rs` and `src/output.rs` where required by the two migrated functions. Depends on: T004.

## Phase 3: Functions

- [T006] [Story] Port the function implementation from `src/gnu.c` into `src/gnu.rs`, adapting its logic to the Rust data structures introduced in Phase 2 and keeping its behavior scoped to the original GNU-side module responsibilities. Depends on: T005.
- [T007] [Story] Port the function implementation from `src/output.c` into `src/output.rs`, adapting its logic to the Rust data structures introduced in Phase 2 and preserving its original output-side module behavior. Depends on: T005.
- [T008] [Story] Resolve the call relationships and data flow between `src/gnu.rs` and `src/output.rs` so the two migrated functions interoperate correctly through Rust module interfaces without reintroducing C-style globals unnecessarily. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine the migrated code in `src/gnu.rs` and `src/output.rs` by removing temporary placeholders, tightening type usage, and simplifying ownership and borrowing where possible without changing module behavior. Depends on: T008.
- [T010] [Story] Perform a final compile-focused cleanup of `src/gnu.rs` and `src/output.rs`, resolving warnings directly introduced by this module port and ensuring the new module code is consistently formatted and integrated. Depends on: T009.