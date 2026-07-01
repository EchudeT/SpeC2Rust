# Tasks: module_gnu_stat_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module file structure for the ported GNU stat module in `src/gnu/stat.rs` and shared allocation support in `src/gnu/xmalloc.rs`, and register them from the existing `src/gnu/mod.rs` or equivalent module index.
- [T002] [P] [Story] Add placeholder public items in `src/gnu/stat.rs` and `src/gnu/xmalloc.rs` matching the analyzed module scope so later data structure and function work can compile incrementally. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the 2 data structures required by `gnu/stat.c` in `src/gnu/stat.rs`, preserving the C module’s field layout and ownership semantics as closely as idiomatic Rust allows. Depends on: T002.
- [T004] [P] [Story] Add constructors or local helper initialization routines for the data structures in `src/gnu/stat.rs` only where directly needed to support the upcoming function ports from `gnu/stat.c`. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Port the function logic from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, keeping its allocation-focused behavior grouped in one implementation pass and adjusting call patterns for Rust memory management without expanding module scope. Depends on: T002.
- [T006] [Story] Port the stat-related function logic from `gnu/stat.c` into `src/gnu/stat.rs`, rewriting it to use the Phase 2 data structures and any allocation support exposed from `src/gnu/xmalloc.rs`. Depends on: T003, T005.
- [T007] [P] [Story] Wire internal visibility and imports between `src/gnu/stat.rs` and `src/gnu/xmalloc.rs` so the ported functions compile cleanly under the project’s module layout without introducing new cross-module APIs beyond this module’s evident needs. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/gnu/stat.rs` and `src/gnu/xmalloc.rs` for idiomatic Rust cleanup, remove temporary placeholders, and resolve remaining compile-time issues caused by the C-to-Rust migration while preserving the original module behavior. Depends on: T004, T007.