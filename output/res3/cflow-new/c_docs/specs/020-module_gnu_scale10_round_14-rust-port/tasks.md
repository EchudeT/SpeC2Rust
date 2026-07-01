# Tasks: module_gnu_scale10_round_14

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `module_gnu_scale10_round_14` by adding the target source file `src/gnu/vasnprintf.rs` and wiring its module declaration from the existing Rust crate entry points needed for this branch.
- [T002] [P] [Story] Review `gnu/vasnprintf.c` and map the 3 in-scope functions plus the 1 in-scope data structure into Rust items to be implemented in `src/gnu/vasnprintf.rs`; record the migration boundaries in code comments or TODO markers to keep the port limited to this module. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the module’s single foundational data structure in `src/gnu/vasnprintf.rs`, preserving the C module’s field layout and ownership semantics as closely as practical in Rust so the later function ports can use it directly. Depends on: T002
- [T004] [Story] Add any minimal internal helper types, enums, or aliases in `src/gnu/vasnprintf.rs` that are directly required to express the ported data structure and function signatures from `gnu/vasnprintf.c`, without expanding beyond the analyzed module scope. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Port the subset of closely related support routines from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs` that operate directly on the foundational data structure and prepare state for formatting/rounding behavior. Implement this function group as one cohesive migration unit covering the relevant in-scope functions. Depends on: T003, T004
- [T006] [Story] Port the remaining formatting/rounding-oriented function group from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, reusing the data structure and support routines introduced earlier and keeping each of the 3 analyzed functions implemented exactly once. Depends on: T005
- [T007] [P] [Story] Reconcile function signatures, visibility, and call flow inside `src/gnu/vasnprintf.rs` so the migrated functions match the intended internal module boundaries from `gnu/vasnprintf.c` and do not expose unevidenced public API surface. Depends on: T006

## Final Phase: Polish

- [T008] [Story] Refine the Rust implementation in `src/gnu/vasnprintf.rs` by removing redundant migration scaffolding, tightening ownership/borrowing around the ported data structure and functions, and ensuring the final file remains a faithful, scope-limited translation of `gnu/vasnprintf.c`. Depends on: T007