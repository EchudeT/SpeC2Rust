# Tasks: main_root_xmalloc.c_37

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `xmalloc.c` migration in `src/xmalloc.rs`, and register it from the crate root file used by the `cat` project branch so the module can host the ported allocation helpers.
- [T002] [P] [Story] Review `xmalloc.c` and map all 15 exported/internal functions into a Rust implementation plan inside `src/xmalloc.rs`, grouping them by allocation role to avoid duplicate or fragmented migration work.
- [T003] [Story] Establish the shared Rust imports, result conventions, and internal helper visibility in `src/xmalloc.rs` needed by all ported functions. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Implement the foundational allocation/error-handling scaffolding in `src/xmalloc.rs` that the migrated `xmalloc.c` functions will share, including common helper aliases or internal utility routines directly evidenced by the source file. Depends on: T003.

## Phase 3: Core allocation functions

- [T005] [Story] Port the primary memory allocation functions from `xmalloc.c` into `src/xmalloc.rs`, covering the base allocate and zero-initializing allocate behaviors as one functional group. Depends on: T004.
- [T006] [P] [Story] Port the primary reallocation functions from `xmalloc.c` into `src/xmalloc.rs`, covering resize-oriented helpers that operate on existing allocations. Depends on: T004.
- [T007] [P] [Story] Port the array/count-aware allocation functions from `xmalloc.c` into `src/xmalloc.rs`, grouping the helpers that combine element count and element size handling. Depends on: T004.

## Phase 4: String and duplication helpers

- [T008] [Story] Port the string duplication and buffer duplication helpers from `xmalloc.c` into `src/xmalloc.rs`, keeping related duplication semantics together in one migration step. Depends on: T005, T006, T007.
- [T009] [P] [Story] Port any string resizing or concatenation-style allocation helpers evidenced in `xmalloc.c` into `src/xmalloc.rs`, grouped with the string-allocation behavior they depend on. Depends on: T008.

## Phase 5: Remaining support functions

- [T010] [Story] Implement the remaining support functions from `xmalloc.c` in `src/xmalloc.rs` that do not fit the earlier allocation or string groups, ensuring all 15 functions are migrated exactly once. Depends on: T005, T006, T007, T008, T009.
- [T011] [Story] Reconcile function signatures and internal call sites across `src/xmalloc.rs` so shared helpers and public-facing allocation routines match the migrated C module behavior. Depends on: T010.

## Final Phase: Polish

- [T012] [Story] Refine `src/xmalloc.rs` to remove migration-only duplication, tighten visibility, and simplify grouped helper usage without changing the module’s ported behavior. Depends on: T011.
- [T013] [Story] Perform a final file-level review of `src/xmalloc.rs` and its crate-root registration to confirm the `xmalloc.c` migration is complete, dependency ordering is respected, and no unevidenced tasks were introduced. Depends on: T012.