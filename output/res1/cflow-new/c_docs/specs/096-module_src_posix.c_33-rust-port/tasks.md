# Tasks: module_src_posix.c_33

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `src/posix.c` port in `src/posix.rs`, and expose it from the crate root or parent module so the `096-module_src_posix.c_33-rust-port` branch has a dedicated target for this migration.
- [T002] [P] [Story] Add the initial module skeleton in `src/posix.rs`, including placeholders for the one module data structure and the two migrated functions, keeping names and responsibilities aligned with `src/posix.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure in `src/posix.rs`, translating the C layout and ownership semantics needed by the functions from `src/posix.c`. Depends on: T002.
- [T004] [Story] Refine the data structure API in `src/posix.rs` with only the constructors, field visibility, and helper methods directly required by the two migrated functions. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the first POSIX-related function from `src/posix.c` in `src/posix.rs`, using the migrated data structure and preserving the original module-local behavior. Depends on: T004.
- [T006] [Story] Implement the second POSIX-related function from `src/posix.c` in `src/posix.rs`, completing the functional port for this module and reusing shared module types where applicable. Depends on: T004.

## Final Phase: Polish

- [T007] [Story] Review `src/posix.rs` to remove placeholder code, resolve any remaining TODOs from the migration, and ensure the two functions and single data structure form a coherent, idiomatic Rust module without expanding beyond `src/posix.c`. Depends on: T005, T006.
- [T008] [P] [Story] Perform a final pass on `src/posix.rs` to simplify signatures, tighten visibility, and reduce unnecessary allocations or copies only where this is directly supported by the original `src/posix.c` behavior. Depends on: T007.