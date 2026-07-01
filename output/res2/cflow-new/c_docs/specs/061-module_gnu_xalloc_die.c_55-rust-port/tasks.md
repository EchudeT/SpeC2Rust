# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/xalloc_die.rs` to host the port of `gnu/xalloc-die.c`.
- [T002] [Story] Expose the new module from `src/gnu/mod.rs` so `src/gnu/xalloc_die.rs` is compiled in the Rust project. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/xalloc-die.c` behavior and define the Rust-visible function signature and module-level imports directly in `src/gnu/xalloc_die.rs`, keeping the port scoped to the single source function. Depends on: T001, T002.

## Phase 3: Functions

- [T004] [Story] Implement the module’s allocation-failure termination function in `src/gnu/xalloc_die.rs`, preserving the C module’s responsibility for emitting the failure path and terminating execution as appropriate for the Rust port. Depends on: T003.

## Final Phase: Polish

- [T005] [P] [Story] Refine `src/gnu/xalloc_die.rs` for idiomatic Rust naming, minimal visibility, and concise in-file documentation aligned with the original `gnu/xalloc-die.c` scope. Depends on: T004.