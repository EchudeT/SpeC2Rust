# Tasks: module_gnu_execute_all_15 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/fd-hook.c` port in `src/gnu/fd_hook.rs`, and expose it from the existing `src/gnu/mod.rs` module tree on branch `021-module_gnu_execute_all_15-rust-port`.
- [T002] [Story] Review `gnu/fd-hook.c` and map the 7 C data structures and 2 functions to Rust items to be implemented in `src/gnu/fd_hook.rs`, recording direct C-to-Rust naming and ownership decisions in code comments for the port. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the Rust representations for the module-local data structures from `gnu/fd-hook.c` in `src/gnu/fd_hook.rs`, preserving the original module scope and relationships needed by the function port. Depends on: T002
- [T004] [P] [Story] Add shared type aliases, enums, constants, and helper state definitions required by the `gnu/fd-hook.c` data structures in `src/gnu/fd_hook.rs`, keeping them limited to items directly evidenced by the source module. Depends on: T002
- [T005] [Story] Integrate the foundational data structures and helper definitions so the module compiles as a Rust unit with placeholders only where the 2 ported functions will attach in `src/gnu/fd_hook.rs`. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Implement the file-descriptor hook registration/update function group from `gnu/fd-hook.c` in `src/gnu/fd_hook.rs`, using the Phase 2 Rust data structures and preserving the original module behavior. Depends on: T005
- [T007] [Story] Implement the file-descriptor hook lookup/application function group from `gnu/fd-hook.c` in `src/gnu/fd_hook.rs`, completing the remaining function port from the C module with the same module-local state interactions. Depends on: T005
- [T008] [Story] Reconcile shared state access and call relationships between the 2 ported functions in `src/gnu/fd_hook.rs` so the module behavior matches the original `gnu/fd-hook.c` interaction model. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Refine `src/gnu/fd_hook.rs` by removing temporary placeholders, tightening visibility to module-appropriate scope, and simplifying direct C-to-Rust translations without changing behavior. Depends on: T008
- [T010] [Story] Perform a final compile-focused review of the `src/gnu/fd_hook.rs` port and its `src/gnu/mod.rs` exposure to ensure the migrated module is internally consistent and ready for integration. Depends on: T009