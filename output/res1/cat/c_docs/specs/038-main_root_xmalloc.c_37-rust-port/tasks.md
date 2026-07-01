# Task List: main_root_xmalloc.c_37

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `xmalloc.c` port in `src/xmalloc.rs`, and expose it from the crate root in `src/lib.rs` or `src/main.rs` on branch `038-main_root_xmalloc.c_37-rust-port`.
- [T002] [P] [Story] Establish the module-level API surface in `src/xmalloc.rs` for the 15 functions identified from `xmalloc.c`, including placeholder signatures and internal organization comments for allocation, duplication, and cleanup helpers. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define foundational Rust-side type aliases, internal helper result conventions, and module constants needed to support the `xmalloc.c` function port in `src/xmalloc.rs`, keeping the implementation scoped to behaviors evidenced by the source module. Depends on: T002.
- [T004] [Story] Add shared internal allocation/size conversion helpers in `src/xmalloc.rs` to centralize low-level memory sizing and failure handling patterns used by the exported `xmalloc.c` function set. Depends on: T003.

## Phase 3: Core allocation functions

- [T005] [Story] Implement the primary allocation function group from `xmalloc.c` in `src/xmalloc.rs`, covering the direct allocation entry points and wiring them to the shared helpers introduced for the module. Depends on: T004.
- [T006] [P] [Story] Implement the zero-initializing allocation function group from `xmalloc.c` in `src/xmalloc.rs`, keeping the behavior aligned with the original module’s allocation semantics. Depends on: T004.
- [T007] [Story] Implement the reallocation function group from `xmalloc.c` in `src/xmalloc.rs`, including the module’s size-aware reallocation behavior and integration with common failure handling. Depends on: T005, T004.

## Phase 4: String and memory duplication functions

- [T008] [P] [Story] Implement the memory block duplication function group from `xmalloc.c` in `src/xmalloc.rs`, using the shared allocation helpers and preserving source-module copying behavior. Depends on: T004.
- [T009] [P] [Story] Implement the string duplication function group from `xmalloc.c` in `src/xmalloc.rs`, covering the module’s owned string copy routines in a Rust-appropriate form within `src/xmalloc.rs`. Depends on: T004.
- [T010] [Story] Reconcile the duplication functions with the core allocation and reallocation routines so all duplication entry points consistently use the same internal failure and size handling paths in `src/xmalloc.rs`. Depends on: T008, T009, T005, T007.

## Phase 5: Failure handling and remaining support functions

- [T011] [Story] Implement the module’s allocation failure handling function group from `xmalloc.c` in `src/xmalloc.rs`, including the canonical out-of-memory path used by the allocation APIs. Depends on: T004.
- [T012] [Story] Implement any remaining support functions from `xmalloc.c` that do not belong to the primary allocation or duplication groups, keeping them colocated in `src/xmalloc.rs` and mapped directly from the source module. Depends on: T011, T010.
- [T013] [Story] Complete integration of all 15 ported functions in `src/xmalloc.rs` by replacing placeholders with finalized implementations and ensuring crate-level visibility/export wiring in `src/lib.rs` or `src/main.rs`. Depends on: T012.

## Final Phase: Polish

- [T014] [Story] Refine `src/xmalloc.rs` to remove placeholder comments, tighten internal helper usage, and simplify any duplicated logic introduced during the staged port while preserving the original `xmalloc.c` behavior. Depends on: T013.
- [T015] [Story] Perform final module-level compile and lint cleanup for the `xmalloc.c` Rust port across `src/xmalloc.rs` and `src/lib.rs` or `src/main.rs`, resolving any visibility, import, or idiomatic Rust issues without expanding scope beyond the source module. Depends on: T014.