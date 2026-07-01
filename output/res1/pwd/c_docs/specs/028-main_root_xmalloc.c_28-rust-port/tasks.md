# Tasks: `main_root_xmalloc.c_28` Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `xmalloc.c` port on branch `028-main_root_xmalloc.c_28-rust-port`, adding the target source file `src/xmalloc.rs` and exposing it from `src/lib.rs` or `src/main.rs` according to the existing `pwd` crate layout.
- [T002] [P] [Story] Add the initial function stubs in `src/xmalloc.rs` for all 15 functions migrated from `xmalloc.c`, preserving C-level grouping and names/signatures as closely as the Rust project conventions allow. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Establish shared allocation/error-handling foundations in `src/xmalloc.rs` needed by the `xmalloc.c` port, including internal helper routines/constants for size validation, overflow checking, and fatal allocation failure reporting reused by the module’s functions. Depends on: T002.
- [T004] [Story] Define the module-local Rust ownership and pointer-handling conventions inside `src/xmalloc.rs` for translating the C allocation helpers, so all migrated functions use one consistent approach for raw allocation results, null checks, and returned buffers. Depends on: T003.

## Phase 3: Core allocation functions

- [T005] [Story] Implement the primary allocation entrypoints from `xmalloc.c` in `src/xmalloc.rs`, covering the functions responsible for allocating new memory blocks and failing through the module’s common fatal path on invalid or unsuccessful allocation. Depends on: T004.
- [T006] [P] [Story] Implement the paired zero-initialized allocation functions from `xmalloc.c` in `src/xmalloc.rs`, reusing the shared overflow and failure helpers established for the module. Depends on: T004.
- [T007] [P] [Story] Implement the reallocation-oriented functions from `xmalloc.c` in `src/xmalloc.rs`, including the variants that resize existing buffers while preserving the module’s common error semantics. Depends on: T004.

## Phase 4: String and duplication helpers

- [T008] [Story] Implement the string duplication functions from `xmalloc.c` in `src/xmalloc.rs`, covering the helpers that allocate and copy NUL-terminated strings through the module’s allocation wrappers. Depends on: T005, T006.
- [T009] [P] [Story] Implement the bounded or length-aware string/memory duplication helpers from `xmalloc.c` in `src/xmalloc.rs`, grouping the functions that allocate based on explicit lengths and copy source content into newly allocated buffers. Depends on: T005, T006.
- [T010] [Story] Align the duplication helpers in `src/xmalloc.rs` with the module’s shared size and overflow validation so all copied-allocation functions fail consistently with the core allocators. Depends on: T008, T009.

## Phase 5: Remaining support functions

- [T011] [Story] Implement the module’s explicit allocation-failure reporting function(s) from `xmalloc.c` in `src/xmalloc.rs`, preserving their central role in emitting the fatal error path used by the allocation and duplication wrappers. Depends on: T003.
- [T012] [P] [Story] Implement any remaining utility functions from `xmalloc.c` in `src/xmalloc.rs` that support size computation, wrapper dispatch, or compatibility behavior not yet covered by the core allocator and duplication groups. Depends on: T005, T006, T007, T008, T009, T011.
- [T013] [Story] Replace the initial stubs in `src/xmalloc.rs` by wiring all 15 migrated functions to their final implementations and ensuring no placeholder bodies remain. Depends on: T007, T010, T011, T012.

## Final Phase: Polish

- [T014] [Story] Review `src/xmalloc.rs` for Rust-idiomatic cleanup that does not change module behavior, removing duplication between allocation wrappers and tightening internal helper visibility after the full `xmalloc.c` migration is complete. Depends on: T013.
- [T015] [Story] Verify the module integration points in `src/lib.rs` or `src/main.rs` for the `xmalloc` port, ensuring exported items from `src/xmalloc.rs` match the needs of the `pwd` crate and the migrated file compiles cleanly in branch `028-main_root_xmalloc.c_28-rust-port`. Depends on: T014.