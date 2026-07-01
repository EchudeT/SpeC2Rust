# Tasks: module_gnu_stat-w32.c_46

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/stat_w32.rs` to host the port of `gnu/stat-w32.c`.
- [T002] [Story] Wire `src/gnu/stat_w32.rs` into the existing Rust module tree by updating the nearest inferable module declaration file under `src/gnu/` so the new module is compiled.
- [T003] [P] [Story] Add initial item stubs in `src/gnu/stat_w32.rs` for the module's 3 data structures and 1 function to establish the migration skeleton.

## Phase 2: Foundational

- [T004] [Story] Implement the first required data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, preserving the C module's field layout and role as needed for the Rust port.
- [T005] [P] [Story] Implement the second required data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, keeping its fields and visibility aligned with module usage.
- [T006] [P] [Story] Implement the third required data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, completing the module-local type definitions needed by the function port.
- [T007] [Story] Reconcile the three data structures in `src/gnu/stat_w32.rs` by adding the required derives, constructors, or helper associations directly evidenced by their use in `gnu/stat-w32.c`.

## Phase 3: Functions

- [T008] [Story] Port the module's single function from `gnu/stat-w32.c` into `src/gnu/stat_w32.rs`, translating its control flow and interactions to use the implemented Rust data structures. Depends on: T004, T005, T006, T007.
- [T009] [Story] Integrate the completed function and type definitions in `src/gnu/stat_w32.rs` by resolving signatures, internal visibility, and call compatibility within the Rust module boundary. Depends on: T008.

## Final Phase: Polish

- [T010] [Story] Refine `src/gnu/stat_w32.rs` by removing placeholder code, tightening naming and comments to match the migrated implementation, and ensuring the module is cleanly organized after the port. Depends on: T009.