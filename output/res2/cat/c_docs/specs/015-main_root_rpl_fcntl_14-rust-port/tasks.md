# Task List: main_root_rpl_fcntl_14

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `fcntl.c` migration in `src/main_root_rpl_fcntl_14.rs`, and wire it into the crate root so the module can host the ported data structure and functions.
- [T002] [P] [Story] Review `fcntl.c` and map its 2 functions and 1 data structure to Rust items in `src/main_root_rpl_fcntl_14.rs`, documenting the intended item names and relationships in module comments for implementation alignment.

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure from `fcntl.c` in `src/main_root_rpl_fcntl_14.rs`, preserving the C layout semantics needed by the migrated functions. Depends on: T001, T002.

## Phase 3: Functions

- [T004] [Story] Implement the first `fcntl.c` function in `src/main_root_rpl_fcntl_14.rs`, using the migrated data structure where required and keeping behavior aligned with the original module logic. Depends on: T003.
- [T005] [Story] Implement the second `fcntl.c` function in `src/main_root_rpl_fcntl_14.rs`, completing the functional port of this module and reusing the shared foundational items already introduced. Depends on: T003.
- [T006] [P] [Story] Integrate both migrated functions within `src/main_root_rpl_fcntl_14.rs` so shared constants, signatures, and internal call relationships from `fcntl.c` are consistent and compile together. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/main_root_rpl_fcntl_14.rs` by removing migration scaffolding comments, tightening item visibility to the minimum needed, and resolving compile-time warnings introduced during the `fcntl.c` port. Depends on: T006.
- [T008] [Story] Perform a final module pass on `src/main_root_rpl_fcntl_14.rs` to simplify direct C-to-Rust translations where possible without changing behavior, keeping the implementation idiomatic and contained to this module. Depends on: T007.