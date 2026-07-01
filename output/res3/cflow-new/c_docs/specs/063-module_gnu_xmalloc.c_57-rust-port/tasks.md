# Tasks: module_gnu_xmalloc.c_57 Rust port

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port workspace for this module branch by adding the target module file corresponding to `gnu/xmalloc.c` at `src/gnu/xmalloc.rs` and declaring it from the existing Rust module tree so the migrated implementation has a dedicated compilation unit.
- [T002] [P] [Story] Review `gnu/xmalloc.c` and map its 5 exported/internal functions into Rust function stubs in `src/gnu/xmalloc.rs`, preserving original grouping and naming intent to prepare direct migration.
- [T003] [Story] Define module-level allocation/error-handling imports and any shared constants needed by the migrated functions in `src/gnu/xmalloc.rs`. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Implement the foundational internal helpers in `src/gnu/xmalloc.rs` needed to centralize memory-size validation and allocation failure reporting that are shared across the migrated `xmalloc` function family. Depends on: T003.

## Phase 3: Core allocation functions

- [T005] [Story] Port the primary allocation function from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, wiring it to the shared validation and failure path helpers established for this module. Depends on: T004.
- [T006] [P] [Story] Port the zero-initializing allocation function from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, reusing the common size checks and failure behavior. Depends on: T004.
- [T007] [P] [Story] Port the reallocation function from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, preserving module-consistent overflow and allocation-failure handling. Depends on: T004.

## Phase 4: String/array allocation helpers

- [T008] [P] [Story] Port the string duplication/allocation helper from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, implementing it in terms of the migrated allocation primitives where applicable. Depends on: T005.
- [T009] [P] [Story] Port the remaining size-multiplying or convenience allocation helper from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, ensuring it uses the shared overflow validation path before allocation. Depends on: T004, T005, T007.

## Final Phase: Polish

- [T010] [Story] Refine `src/gnu/xmalloc.rs` to remove migration scaffolding, align function signatures and visibility with actual module usage, and ensure the 5 migrated functions consistently share the same internal error-handling path. Depends on: T005, T006, T007, T008, T009.