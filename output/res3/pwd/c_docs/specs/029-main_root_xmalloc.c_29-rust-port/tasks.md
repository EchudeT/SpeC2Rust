# Task List: `main_root_xmalloc.c_29`

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/xmalloc.rs` and wire it into the crate from the existing Rust entry module so the port of `xmalloc.c` has a dedicated compilation unit.
- [T002] [P] [Story] Establish the public API surface in `src/xmalloc.rs` for the six functions from `xmalloc.c`, preserving a Rust-appropriate naming and visibility plan before implementation begins.

## Phase 2: Foundational

- [T003] [Story] Define the shared foundational allocation/error-handling helpers needed by the `xmalloc.c` port in `src/xmalloc.rs`, so all six functions can reuse one consistent internal mechanism for allocation failure handling. Depends on: T001, T002

## Phase 3: Core allocation functions

- [T004] [Story] Implement the primary memory-allocation function group from `xmalloc.c` in `src/xmalloc.rs`, covering the base allocator behavior and its direct variant(s) that allocate new memory blocks. Depends on: T003
- [T005] [Story] Implement the zero-initializing allocation function group from `xmalloc.c` in `src/xmalloc.rs`, using the shared allocation/error path established for the module. Depends on: T003
- [T006] [Story] Implement the reallocation function group from `xmalloc.c` in `src/xmalloc.rs`, including growth or resize behavior routed through the module’s common failure handling. Depends on: T003

## Phase 4: String/duplication helpers

- [T007] [P] [Story] Implement the memory/string duplication helper function group from `xmalloc.c` in `src/xmalloc.rs`, keeping ownership and allocation semantics aligned with the original module intent. Depends on: T003
- [T008] [Story] Reconcile all six implemented functions in `src/xmalloc.rs` so shared helper usage, signatures, and module exports are consistent and no function logic is duplicated. Depends on: T004, T005, T006, T007

## Final Phase: Polish

- [T009] [Story] Perform a final polish pass on `src/xmalloc.rs` and the crate wiring touched for this module to remove redundant code paths, tighten documentation comments where needed, and ensure the module builds cleanly on branch `029-main_root_xmalloc.c_29-rust-port`. Depends on: T008