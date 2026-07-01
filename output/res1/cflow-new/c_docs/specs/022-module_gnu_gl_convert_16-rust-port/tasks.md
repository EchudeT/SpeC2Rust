# Tasks: module_gnu_gl_convert_16

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the port of `gnu/stat-w32.c` in `src/module_gnu_gl_convert_16.rs`, and expose it from the crate root in `src/lib.rs`.
- [T002] [P] [Story] Add the branch module wiring and placeholder public items in `src/module_gnu_gl_convert_16.rs` so later data structure and function migration can land without changing the file layout. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and port the 3 data structures used by `gnu/stat-w32.c` into Rust-native equivalents in `src/module_gnu_gl_convert_16.rs`, preserving field layout intent and module-local visibility/publicity as required by the C module. Depends on: T002.
- [T004] [Story] Add foundational constructors/default helpers or constant initializers for the migrated data structures in `src/module_gnu_gl_convert_16.rs` only where required to support the function ports from `gnu/stat-w32.c`. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Port the first function from `gnu/stat-w32.c` into `src/module_gnu_gl_convert_16.rs`, implementing its logic against the migrated Rust data structures and keeping behavior aligned with the original module responsibility. Depends on: T004.
- [T006] [Story] Port the second function from `gnu/stat-w32.c` into `src/module_gnu_gl_convert_16.rs`, completing the module’s function migration and reusing the shared foundational types already introduced. Depends on: T004.
- [T007] [P] [Story] Reconcile shared helper usage, signatures, and internal visibility between the two migrated functions in `src/module_gnu_gl_convert_16.rs` so the module presents a coherent Rust API surface equivalent to the C module scope. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Review `src/module_gnu_gl_convert_16.rs` for Rust idioms, remove leftover scaffolding from setup, and simplify direct C-style patterns where possible without changing the behavior of the migrated `gnu/stat-w32.c` module. Depends on: T007.
- [T009] [Story] Finalize module exports in `src/lib.rs` so `module_gnu_gl_convert_16` is exposed consistently with the rest of the Rust project branch and matches the completed migration scope. Depends on: T008.