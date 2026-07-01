# Tasks: `main_root_xmalloc.c_28` Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `xmalloc.c` on branch `028-main_root_xmalloc.c_28-rust-port`, adding the target source file `src/xmalloc.rs` and declaring it from the crate root if not already exposed.
- [T002] [P] [Story] Inspect `xmalloc.c` and enumerate its 15 functions into the Rust module plan, mapping each C function to a single Rust implementation location in `src/xmalloc.rs` to avoid duplicate scheduling.
  **Depends on:** T001

## Phase 2: Foundational

- [T003] [Story] Define the foundational allocation API surface in `src/xmalloc.rs`, including shared Rust-level type aliases, result/panic conventions, and internal helper signatures needed by the translated `xmalloc.c` functions.
  **Depends on:** T002
- [T004] [P] [Story] Establish the common error-reporting and size-handling helpers in `src/xmalloc.rs` that will be reused by the allocation and string/vector utility functions ported from `xmalloc.c`.
  **Depends on:** T003

## Phase 3: Core allocation functions

- [T005] [Story] Implement the primary memory allocation functions from `xmalloc.c` in `src/xmalloc.rs`, covering the core allocate and zero-initialized allocate behaviors as one grouped porting task.
  **Depends on:** T004
- [T006] [Story] Implement the reallocation and resize-related functions from `xmalloc.c` in `src/xmalloc.rs`, keeping all resize semantics grouped with the core allocation API.
  **Depends on:** T005
- [T007] [P] [Story] Implement the allocation failure handling routine(s) from `xmalloc.c` in `src/xmalloc.rs`, including the shared fatal-path behavior used by the allocation entry points.
  **Depends on:** T004

## Phase 4: String and duplication helpers

- [T008] [Story] Implement the string duplication functions from `xmalloc.c` in `src/xmalloc.rs`, grouping the direct string-copy allocation helpers together.
  **Depends on:** T005, T007
- [T009] [P] [Story] Implement bounded or length-aware string duplication helpers from `xmalloc.c` in `src/xmalloc.rs`, preserving the original grouped semantics for partial-copy allocation utilities.
  **Depends on:** T008
- [T010] [P] [Story] Implement memory block duplication helper functions from `xmalloc.c` in `src/xmalloc.rs`, grouping non-string copy-allocation routines separately from string-specific helpers.
  **Depends on:** T005, T007

## Phase 5: Argument/vector allocation helpers

- [T011] [Story] Implement vector or pointer-array allocation helpers from `xmalloc.c` in `src/xmalloc.rs`, covering grouped functions that allocate expandable argument or pointer lists.
  **Depends on:** T006, T007
- [T012] [P] [Story] Implement vector growth or duplication helper functions from `xmalloc.c` in `src/xmalloc.rs`, keeping related pointer-array resize/copy routines together.
  **Depends on:** T011
- [T013] [P] [Story] Implement any remaining utility allocation wrappers from `xmalloc.c` in `src/xmalloc.rs` that do not fit the core allocation or string groups, ensuring each unscheduled function is ported exactly once.
  **Depends on:** T006, T007

## Final Phase: Polish

- [T014] [Story] Review `src/xmalloc.rs` for parity with `xmalloc.c`, confirming that all 15 functions are present, grouped appropriately, and use consistent Rust-side naming and internal helper reuse.
  **Depends on:** T005, T006, T007, T008, T009, T010, T011, T012, T013
- [T015] [Story] Refine `src/xmalloc.rs` to remove redundant logic introduced during porting, simplify helper boundaries, and ensure the module remains focused on the migrated `xmalloc.c` behavior only.
  **Depends on:** T014