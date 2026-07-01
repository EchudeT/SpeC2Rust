# Tasks: module_gnu_execute_all_15

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffolding for `gnu/fd-hook.c` in `src/gnu/fd_hook.rs` and expose it from `src/gnu/mod.rs` for the `021-module_gnu_execute_all_15-rust-port` branch.
- [ ] T002 [Story] Review `gnu/fd-hook.c` and map its 7 data structures and 2 functions into Rust module-local items to define the migration surface in `src/gnu/fd_hook.rs`. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Implement the Rust representations for the data structures required by `gnu/fd-hook.c` in `src/gnu/fd_hook.rs`, preserving the original module-local ownership and relationships. Depends on: T002
- [ ] T004 [P] [Story] Add constructor, defaulting, and internal helper methods needed to initialize and maintain the migrated `fd-hook` data structures in `src/gnu/fd_hook.rs`. Depends on: T003
- [ ] T005 [P] [Story] Define any module-local constants, enums, or type aliases directly required by the `fd-hook` data structures and function signatures in `src/gnu/fd_hook.rs`. Depends on: T003

## Phase 3: Functions

- [ ] T006 [Story] Implement the first `gnu/fd-hook.c` function in `src/gnu/fd_hook.rs`, wiring it to the migrated data structures and preserving the original module behavior. Depends on: T004, T005
- [ ] T007 [Story] Implement the second `gnu/fd-hook.c` function in `src/gnu/fd_hook.rs`, completing the module’s function migration against the shared `fd-hook` state. Depends on: T004, T005, T006

## Final Phase: Polish

- [ ] T008 [Story] Refine `src/gnu/fd_hook.rs` to remove migration scaffolding, tighten visibility to module-appropriate scope, and align naming and organization with the completed Rust port of `gnu/fd-hook.c`. Depends on: T007
- [ ] T009 [Story] Update `src/gnu/mod.rs` imports and exports as needed so the migrated `fd_hook` module integrates cleanly without unused or placeholder declarations. Depends on: T008