# Tasks: module_gnu_fd_hook_06

## Phase 1: Setup

- [ ] [T001] [Story] Create the Rust module scaffold for the `gnu/fd-hook.c` port in `src/gnu/fd_hook.rs`, and expose it from `src/gnu/mod.rs` and the crate root module file if needed for the existing project layout.
- [ ] [T002] [P] [Story] Review `gnu/fd-hook.c` and map its 7 data structures and 4 functions into Rust items to be implemented in `src/gnu/fd_hook.rs`, documenting the planned Rust names and ownership model in code comments before implementation. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [Story] Implement the core Rust definitions for all 7 data structures required by `gnu/fd-hook.c` in `src/gnu/fd_hook.rs`, including the field layouts, enums, internal state holders, and any type aliases directly evidenced by the C module. Depends on: T002
- [ ] [T004] [Story] Add foundational constructors, default state initialization, and internal helper methods that are necessary to support the module’s function implementations in `src/gnu/fd_hook.rs`, staying limited to helpers directly required by the C module logic. Depends on: T003

## Phase 3: Function Implementation

- [ ] [T005] [Story] Implement the function group in `src/gnu/fd_hook.rs` responsible for module state initialization and hook registration logic, porting the corresponding function or closely related functions from `gnu/fd-hook.c`. Depends on: T004
- [ ] [T006] [P] [Story] Implement the function group in `src/gnu/fd_hook.rs` responsible for descriptor hook lookup, update, or dispatch behavior, porting the corresponding related function or functions from `gnu/fd-hook.c`. Depends on: T004
- [ ] [T007] [Story] Integrate the 4 ported functions within `src/gnu/fd_hook.rs` so shared state, structure usage, and call flow match the original `gnu/fd-hook.c` module behavior without duplicating function work across phases. Depends on: T005, T006

## Final Phase: Polish

- [ ] [T008] [Story] Refine `src/gnu/fd_hook.rs` by removing C-centric artifacts, tightening Rust visibility and ownership, and simplifying internal helpers while preserving the behavior established by the `gnu/fd-hook.c` port. Depends on: T007