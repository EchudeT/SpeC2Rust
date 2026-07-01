# Tasks: module_gnu_memchr.c_35

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/memchr.c` port in `src/gnu/memchr.rs`.
- [T002] [P] [Story] Register the new module in the Rust crate module tree so `src/gnu/memchr.rs` is compiled, updating the nearest existing module declaration file under `src/gnu/`.
- [T003] [Story] Add the public function signature placeholder for the `gnu/memchr.c` port in `src/gnu/memchr.rs`, matching the C module scope and preparing for implementation. Depends on: T001.

## Phase 2: Foundational

- [T004] [Story] Review `src/gnu/memchr.rs` and define any local aliases, constants, or private helper layout strictly required to support the `memchr` port, keeping the implementation self-contained and minimal. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the `memchr` function logic in `src/gnu/memchr.rs`, porting the behavior from `gnu/memchr.c` into idiomatic Rust while preserving the original search semantics. Depends on: T004.
- [T006] [P] [Story] Refine visibility and call surface in `src/gnu/memchr.rs` so the exported Rust function matches the intended module-level usage after the port is complete. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Review `src/gnu/memchr.rs` for simplification and remove any placeholder or transitional code introduced during setup, leaving only the final ported implementation. Depends on: T006.
- [T008] [Story] Run a final module-level compile/readiness pass for the `gnu/memchr.c` migration by checking `src/gnu/memchr.rs` and its module registration updates compile cleanly together. Depends on: T007.