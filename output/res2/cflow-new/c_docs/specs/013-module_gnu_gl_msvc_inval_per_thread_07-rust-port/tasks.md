# Tasks: module_gnu_gl_msvc_inval_per_thread_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/msvc-inval.c` in `src/gnu/msvc_inval.rs`, and expose it from the existing crate module tree so the ported module has a dedicated target file.
- [T002] [P] [Story] Add the module file wiring needed for `src/gnu/msvc_inval.rs` in the nearest existing Rust module declaration file under `src/gnu/`, keeping the integration limited to this module’s namespace.

## Phase 2: Foundational

- [T003] [Story] Define the seven data structures required by `gnu/msvc-inval.c` in `src/gnu/msvc_inval.rs`, preserving the C module’s per-thread invalid-parameter support layout and keeping all type definitions in place before function translation.
- [T004] [Story] Add any associated constants, aliases, and internal field representations directly needed by those seven data structures in `src/gnu/msvc_inval.rs`. Depends on: T003.

## Phase 3: Function implementation

- [T005] [Story] Implement the module’s setup and state-access function group in `src/gnu/msvc_inval.rs`, covering the functions that initialize and retrieve the per-thread invalid-parameter handling state. Depends on: T003, T004.
- [T006] [P] [Story] Implement the module’s handler-update function group in `src/gnu/msvc_inval.rs`, covering the functions that install, replace, or restore the per-thread invalid-parameter handler state defined by the C module. Depends on: T003, T004.
- [T007] [Story] Implement the remaining dispatch/helper function in `src/gnu/msvc_inval.rs`, completing the four-function port for invalid-parameter handling behavior in this module. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Review `src/gnu/msvc_inval.rs` for idiomatic Rust cleanup, remove redundant C-port scaffolding, and ensure naming and visibility match the module’s intended internal use. Depends on: T005, T006, T007.
- [T009] [Story] Verify that the final module integration remains limited to `src/gnu/msvc_inval.rs` and its required `src/gnu/` module declaration updates, and confirm all four functions and seven data structures from `gnu/msvc-inval.c` are represented exactly once. Depends on: T008.