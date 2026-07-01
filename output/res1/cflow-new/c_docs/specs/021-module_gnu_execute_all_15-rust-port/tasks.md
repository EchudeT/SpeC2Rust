# Tasks: module_gnu_execute_all_15 Rust port

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for the `gnu/fd-hook.c` port on branch `021-module_gnu_execute_all_15-rust-port`, adding the target source files `src/gnu/mod.rs` and `src/gnu/fd_hook.rs`.
- [ ] T002 [Story] Wire the new `fd_hook` module into the Rust crate module tree from `src/gnu/mod.rs` so later data structures and functions can be implemented in the inferred target file `src/gnu/fd_hook.rs`. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Define the seven foundational data structures required by the `gnu/fd-hook.c` port in `src/gnu/fd_hook.rs`, keeping names and responsibilities aligned with the C module analysis. Depends on: T002
- [ ] T004 [P] [Story] Add core constructors, default state initialization, and internal field organization for the foundational data structures in `src/gnu/fd_hook.rs` where directly needed to support the module functions. Depends on: T003
- [ ] T005 [P] [Story] Establish internal type aliases, enums, or helper representations in `src/gnu/fd_hook.rs` only where required to express the `fd-hook` data layout and prepare for function migration. Depends on: T003

## Phase 3: Function implementation

- [ ] T006 [Story] Implement the first `gnu/fd-hook.c` function in `src/gnu/fd_hook.rs`, migrating its logic against the completed foundational data structures and preserving module-local behavior. Depends on: T004, T005
- [ ] T007 [Story] Implement the second `gnu/fd-hook.c` function in `src/gnu/fd_hook.rs`, completing the functional port for this module and reusing the same foundational structures without duplicating logic. Depends on: T006

## Final Phase: Polish

- [ ] T008 [Story] Refine `src/gnu/fd_hook.rs` for Rust-idiomatic organization, remove any migration-only redundancy, and ensure the module remains limited to the behavior evidenced by `gnu/fd-hook.c`. Depends on: T007