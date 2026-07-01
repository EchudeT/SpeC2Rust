# Tasks: Port `gnu/xmalloc.c` to Rust

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/xmalloc.c` on branch `062-module_gnu_xmalloc.c_56-rust-port`, adding the target source file `src/gnu/xmalloc.rs` and wiring it into the existing Rust module tree from the nearest parent `src/gnu/mod.rs`.
- [T002] [P] [Story] Review `gnu/xmalloc.c` and map all 15 exported/internal functions into a Rust implementation plan inside `src/gnu/xmalloc.rs`, preserving the C module scope and identifying shared allocation/error-handling helpers needed for the port. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust module-level aliases, imports, and internal helper layout in `src/gnu/xmalloc.rs` needed to support C-style allocation wrappers, size handling, and common failure paths before porting individual functions. Depends on: T002
- [T004] [Story] Implement the module’s shared out-of-memory handling path in `src/gnu/xmalloc.rs`, including the central failure routine used by the allocation wrappers so later function ports reuse one consistent behavior. Depends on: T003

## Phase 3: Core allocation wrappers

- [T005] [Story] Port the base memory allocation wrapper functions from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, covering the primary allocate and reallocate entry points as one functional group built on the shared failure path. Depends on: T004
- [T006] [P] [Story] Port the zero-initializing and duplication-oriented allocation wrappers into `src/gnu/xmalloc.rs`, grouping the functions that allocate initialized memory or duplicate existing memory regions/strings around the same allocation conventions. Depends on: T004
- [T007] [Story] Port the free-or-resize convenience wrappers from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, keeping the semantics aligned with the C module’s handling of null pointers and zero-sized requests. Depends on: T005, T006

## Phase 4: Overflow-checked size helpers

- [T008] [Story] Port the size computation helpers from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, grouping the functions that calculate element-count and byte-size products with overflow checking before allocation. Depends on: T004
- [T009] [P] [Story] Port the allocation entry points that consume checked size computations into `src/gnu/xmalloc.rs`, grouping the functions that combine count-based sizing with malloc/realloc/calloc-style behavior. Depends on: T005, T008
- [T010] [Story] Integrate the checked-size helpers with the previously ported duplication and convenience wrappers in `src/gnu/xmalloc.rs` wherever the original C module routes through common count/size logic. Depends on: T006, T007, T008, T009

## Final Phase: Polish

- [T011] [Story] Perform a module-level conformance pass on `src/gnu/xmalloc.rs` to ensure all 15 functions from `gnu/xmalloc.c` are ported exactly once, signatures are consistent within the Rust module tree, and shared helper usage is deduplicated. Depends on: T005, T006, T007, T008, T009, T010
- [T012] [Story] Refine `src/gnu/xmalloc.rs` for Rust idioms without changing module behavior, removing redundant code paths introduced during migration and tightening imports, visibility, and internal helper organization. Depends on: T011