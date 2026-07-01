# Tasks: main_root_xmalloc.c_37

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/xmalloc.rs` and declare it from the crate entry point used by the `cat` project on branch `038-main_root_xmalloc.c_37-rust-port`.
- [T002] [P] [Story] Define the migration surface in `src/xmalloc.rs` by adding Rust function stubs for the 15 functions ported from `xmalloc.c`, preserving the C module grouping and names or mapped Rust-facing equivalents as needed for the port.
- [T003] [Story] Wire `src/xmalloc.rs` into the main module organization so other `main_cluster` code can call the allocation helpers after porting. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Establish module-level foundational items in `src/xmalloc.rs` needed by the `xmalloc.c` port, including shared imports, internal helper type aliases if required by the C signatures, and a consistent error-handling approach for allocation failure paths. Depends on: T003.

## Phase 3: Core allocation wrappers

- [T005] [Story] Implement the primary memory allocation wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, covering the direct allocation entry points and their overflow-aware size handling where present in the source module. Depends on: T004.
- [T006] [P] [Story] Implement the reallocation-related wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, keeping their behavior aligned with the original module’s failure semantics. Depends on: T004.
- [T007] [P] [Story] Implement the zero-initializing allocation wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, grouped from the corresponding `xmalloc.c` routines. Depends on: T004.
- [T008] [Story] Reconcile shared internal logic across allocation, zero-allocation, and reallocation wrappers inside `src/xmalloc.rs` so each of the migrated functions is implemented exactly once without duplicated failure-path code. Depends on: T005, T006, T007.

## Phase 4: String and duplication helpers

- [T009] [P] [Story] Implement string duplication helper functions from `xmalloc.c` in `src/xmalloc.rs`, preserving the source module’s allocation-on-copy behavior. Depends on: T008.
- [T010] [P] [Story] Implement bounded or length-aware duplication helper functions from `xmalloc.c` in `src/xmalloc.rs`, matching the original size and termination rules present in the C module. Depends on: T008.
- [T011] [Story] Implement any generic memory/block duplication helper functions from `xmalloc.c` in `src/xmalloc.rs`, grouped with the copy-oriented helpers from the same file. Depends on: T008.
- [T012] [Story] Align the duplication helpers in `src/xmalloc.rs` with the allocation wrappers so shared overflow checks and failure handling remain consistent across all migrated functions. Depends on: T009, T010, T011.

## Phase 5: Failure handling and module integration

- [T013] [Story] Implement the allocation failure reporting or termination function(s) defined by `xmalloc.c` in `src/xmalloc.rs`, preserving the original module-level contract used by callers. Depends on: T012.
- [T014] [Story] Update call sites within the crate entry/module wiring that depend on `src/xmalloc.rs` so the migrated `xmalloc.c` API is used consistently from Rust in the `main_cluster` port. Depends on: T013.

## Final Phase: Polish

- [T015] [Story] Review `src/xmalloc.rs` for idiomatic Rust cleanup, remove dead stubs left from setup, and ensure the 15 functions from `xmalloc.c` are fully migrated with no duplicated placeholders remaining. Depends on: T014.