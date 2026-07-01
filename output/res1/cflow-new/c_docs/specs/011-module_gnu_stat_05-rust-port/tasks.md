# Tasks: module_gnu_stat_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module files `src/gnu/stat.rs` and `src/gnu/xmalloc.rs`, and register them from the existing crate module tree for branch `011-module_gnu_stat_05-rust-port`.
- [T002] [P] [Story] Establish the shared Rust imports, module visibility, and placeholder item layout in `src/gnu/stat.rs` and `src/gnu/xmalloc.rs` to mirror the migration scope from `gnu/stat.c` and `gnu/xmalloc.c`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Identify and define the two module-local data structures required by `gnu/stat.c` inside `src/gnu/stat.rs`, preserving field intent and ownership semantics needed by the C implementation. Depends on: T002
- [T004] [P] [Story] Add constructors or helper initialization logic for the data structures in `src/gnu/stat.rs` only where directly required to support the migrated functions. Depends on: T003

## Phase 3: Function Implementation

- [T005] [Story] Port the function behavior from `gnu/xmalloc.c` into `src/gnu/xmalloc.rs`, adapting allocation-related logic to idiomatic Rust while keeping the original module responsibility intact. Depends on: T002
- [T006] [Story] Port the first function from `gnu/stat.c` into `src/gnu/stat.rs`, wiring it to the migrated data structures and any allocation support needed from `src/gnu/xmalloc.rs`. Depends on: T004, T005
- [T007] [Story] Port the remaining function from `gnu/stat.c` into `src/gnu/stat.rs`, completing the functional migration of the C module without duplicating earlier implementation work. Depends on: T006

## Final Phase: Polish

- [T008] [Story] Refine `src/gnu/stat.rs` and `src/gnu/xmalloc.rs` to remove placeholder code, tighten signatures and visibility, and ensure the migrated module compiles cleanly as an integrated Rust port. Depends on: T005, T007