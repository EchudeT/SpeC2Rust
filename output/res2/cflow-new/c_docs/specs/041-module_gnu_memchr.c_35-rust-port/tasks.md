# Tasks: module_gnu_memchr.c_35

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port workspace for this module on branch `041-module_gnu_memchr.c_35-rust-port`, and create the target module file `src/gnu/memchr.rs` mapped from `gnu/memchr.c`.
- [T002] [P] [Story] Wire the new module into the Rust crate module tree by declaring `src/gnu/memchr.rs` from the existing Rust module entry points needed for `src/gnu/memchr.rs` to compile.
- [T003] [Story] Add a migration stub in `src/gnu/memchr.rs` for the function port from `gnu/memchr.c`, preserving the original function boundary and intended public/internal visibility for later implementation. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Establish the foundational Rust-level byte-slice and pointer-access conventions needed by `src/gnu/memchr.rs` so the ported implementation can express the original `gnu/memchr.c` scan logic without introducing new module-level data structures. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the `memchr` function in `src/gnu/memchr.rs`, porting the search behavior from `gnu/memchr.c` and preserving the original semantics for locating the first matching byte within the provided memory range. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/memchr.rs` to remove migration scaffolding, tighten the implementation to idiomatic Rust where it does not change `gnu/memchr.c` behavior, and ensure the final file is cleanly integrated in the crate. Depends on: T005.