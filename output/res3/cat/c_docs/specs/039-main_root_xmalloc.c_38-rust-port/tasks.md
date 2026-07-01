# Tasks: main_root_xmalloc.c_38

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `xmalloc.c` port on branch `039-main_root_xmalloc.c_38-rust-port`, adding or updating `src/xmalloc.rs` so the module can host the translated allocation helpers.
- [T002] [P] [Story] Wire the new `xmalloc` module into the crate root by updating `src/lib.rs` or `src/main.rs` (whichever already owns module declarations in this project) to expose `src/xmalloc.rs` for use by the rest of the `cat` port.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust-side type aliases, constants, and internal helper signatures needed to support the six functions from `xmalloc.c` in `src/xmalloc.rs`, keeping the design limited to direct allocation-helper migration with no extra abstractions. Depends on: T001, T002

## Phase 3: Core allocation wrappers

- [T004] [Story] Implement the basic memory allocation entry points translated from `xmalloc.c` in `src/xmalloc.rs`, grouping the direct allocation and zero-initializing allocation functions together because they share failure-handling behavior. Depends on: T003
- [T005] [P] [Story] Implement the reallocation-oriented functions from `xmalloc.c` in `src/xmalloc.rs`, grouping the resize helpers together around the same allocation-error path established for the module. Depends on: T003
- [T006] [Story] Implement the duplication and convenience allocation helper functions from `xmalloc.c` in `src/xmalloc.rs`, completing the remaining exported function set without introducing behavior beyond the original file scope. Depends on: T004, T005

## Final Phase: Polish

- [T007] [Story] Review and refine `src/xmalloc.rs` for idiomatic Rust naming, visibility, and internal helper reuse while preserving the original `xmalloc.c` behavior and keeping the module boundaries aligned with this single-file migration. Depends on: T004, T005, T006