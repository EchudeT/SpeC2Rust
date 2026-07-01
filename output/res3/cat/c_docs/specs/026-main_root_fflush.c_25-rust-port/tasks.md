# Tasks: main_root_fflush.c_25

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/fflush.rs` for the port of `fflush.c` and wire it into the crate from `src/main.rs` or `src/lib.rs`, matching the existing project entry structure on branch `026-main_root_fflush.c_25-rust-port`.
- [T002] [P] [Story] Add the initial public/internal function stubs in `src/fflush.rs` for the 4 functions identified from `fflush.c`, preserving the C module-level grouping for later implementation. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Establish shared module-local aliases, imports, and helper constants in `src/fflush.rs` required by the `fflush.c` port, limited to items directly evidenced by the source file’s function implementations. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the primary flush control function group in `src/fflush.rs`, covering the main exported `fflush.c` behavior and any directly paired internal helper needed for the same control flow. Depends on: T003.
- [T005] [P] [Story] Implement the remaining state/stream handling helper function group in `src/fflush.rs` for the other functions from `fflush.c` that support or complete module-local flush behavior. Depends on: T003.
- [T006] [Story] Integrate and reconcile all 4 ported functions in `src/fflush.rs`, ensuring signatures, visibility, and intra-module calls match the original `fflush.c` relationships without duplicating function work. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/fflush.rs` to remove placeholder code, unused imports, and migration scaffolding introduced during the port, keeping the implementation aligned with `fflush.c` semantics. Depends on: T006.
- [T008] [Story] Perform a final crate-level compile cleanup for the `fflush.c` migration by resolving module inclusion issues in `src/main.rs` or `src/lib.rs` caused by `src/fflush.rs` integration. Depends on: T007.