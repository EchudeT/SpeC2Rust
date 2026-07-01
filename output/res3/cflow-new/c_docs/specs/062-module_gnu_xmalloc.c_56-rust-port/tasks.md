# Tasks: Port `gnu/xmalloc.c` to Rust

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, and wire it into the crate module tree from the existing root module file so the ported implementation has a dedicated compilation unit.
- [T002] [P] [Story] Review `gnu/xmalloc.c` and map its 15 functions into Rust implementation groups inside `src/module_gnu_xmalloc.rs`, keeping a one-time migration plan for allocation, reallocation, duplication, and failure-handling helpers without duplicating function scheduling. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Establish the foundational Rust imports, internal type aliases, and module-level helper signatures needed to support the `gnu/xmalloc.c` port in `src/module_gnu_xmalloc.rs`, matching the C module’s allocation-oriented role without introducing unrelated abstractions. Depends on: T001
- [T004] [Story] Define the internal failure-reporting and size-conversion helper layer in `src/module_gnu_xmalloc.rs` that the ported allocation functions will share, so later function groups can use a consistent base implementation. Depends on: T003

## Phase 3: Core allocation functions

- [T005] [Story] Implement the primary allocation entry-point group from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, covering the functions responsible for direct memory allocation and their immediate failure path behavior. Depends on: T004
- [T006] [P] [Story] Implement the zero-initializing allocation function group from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, keeping them aligned with the shared failure helpers established earlier. Depends on: T004
- [T007] [P] [Story] Implement the basic reallocation function group from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, including the shared resizing behavior expected by the original module. Depends on: T004

## Phase 4: Sized and array-oriented allocation functions

- [T008] [Story] Implement the overflow-aware array allocation function group from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, covering functions that combine element count and element size before allocation. Depends on: T005, T006, T004
- [T009] [P] [Story] Implement the overflow-aware array reallocation function group from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, reusing the foundational size helpers and reallocation behavior from earlier phases. Depends on: T007, T004
- [T010] [P] [Story] Implement any `n`/`nz`-style allocation variants from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs` as a single grouped migration task, keeping all count-based wrappers together and mapped once. Depends on: T008, T009

## Phase 5: Duplication and ownership-transfer helpers

- [T011] [Story] Implement the memory duplication helper function group from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, covering functions that allocate new storage and copy existing byte content. Depends on: T005, T008
- [T012] [P] [Story] Implement the string duplication helper function group from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs`, covering the module’s string-copying allocation wrappers in one pass. Depends on: T011
- [T013] [Story] Implement the ownership-transfer or growth-helper wrapper functions from `gnu/xmalloc.c` in `src/module_gnu_xmalloc.rs` that complete the module’s remaining exported allocation API, ensuring each remaining C function is migrated exactly once. Depends on: T007, T010, T011, T012

## Final Phase: Polish

- [T014] [Story] Review `src/module_gnu_xmalloc.rs` for API completeness against `gnu/xmalloc.c`, ensuring all 15 functions are present, grouped as planned, and use the shared helper layer consistently. Depends on: T005, T006, T007, T008, T009, T010, T011, T012, T013
- [T015] [Story] Refine `src/module_gnu_xmalloc.rs` for idiomatic Rust organization, removing migration-only duplication inside the module, tightening internal visibility, and preserving the original module behavior. Depends on: T014