# Tasks: module_gnu_fd_hook_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/fd-hook.c` port on branch `012-module_gnu_fd_hook_06-rust-port`, adding the target source files `src/gnu/mod.rs` and `src/gnu/fd_hook.rs`, and wire `fd_hook` into the `gnu` module tree.

## Phase 2: Foundational

- [T002] [Story] Define the Rust data structures needed by the `fd-hook` port in `src/gnu/fd_hook.rs`, translating the 7 module-local C data structures into Rust types with module-scoped visibility and ownership/borrowing choices suitable for the later function port. Depends on: T001
- [T003] [P] [Story] Add foundational constants, type aliases, and internal helper enums required to support the `fd-hook` data structures in `src/gnu/fd_hook.rs`, keeping them limited to elements directly evidenced by `gnu/fd-hook.c`. Depends on: T001

## Phase 3: Hook state and registration functions

- [T004] [Story] Implement the function group responsible for initializing and maintaining the module’s file-descriptor hook state in `src/gnu/fd_hook.rs`, using the foundational Rust data structures defined for the port. Depends on: T002, T003
- [T005] [Story] Implement the function group responsible for registering, updating, or removing file-descriptor hook entries in `src/gnu/fd_hook.rs`, keeping the behavior aligned with the original `gnu/fd-hook.c` logic. Depends on: T004

## Phase 4: Descriptor notification and traversal functions

- [T006] [Story] Implement the function group responsible for traversing registered hook entries and dispatching the appropriate file-descriptor notifications in `src/gnu/fd_hook.rs`. Depends on: T005
- [T007] [P] [Story] Implement any remaining module-local helper function needed to complete the 4-function port from `gnu/fd-hook.c` in `src/gnu/fd_hook.rs`, assigning it to the same internal ownership model as the other ported functions and avoiding duplicate scheduling of already implemented logic. Depends on: T005

## Final Phase: Polish

- [T008] [Story] Refine `src/gnu/fd_hook.rs` for idiomatic Rust within the existing port scope, removing migration scaffolding, tightening visibility, and simplifying control flow without changing the behavior of the ported `fd-hook` module. Depends on: T006, T007