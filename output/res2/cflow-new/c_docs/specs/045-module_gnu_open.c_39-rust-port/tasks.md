# Task List: module_gnu_open.c_39

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/open.c` in `src/module_gnu_open.rs`, and expose it from the crate root or module tree used by branch `045-module_gnu_open.c_39-rust-port`.
- [T002] [Story] Establish the file-migration mapping for this port in `src/module_gnu_open.rs`, documenting that it is the Rust target for `gnu/open.c`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the first data structure required by `gnu/open.c` in `src/module_gnu_open.rs`, translating its fields and ownership model into Rust-native representations. Depends on: T002
- [T004] [P] [Story] Implement the second data structure required by `gnu/open.c` in `src/module_gnu_open.rs`, translating its fields and ownership model into Rust-native representations. Depends on: T002
- [T005] [Story] Integrate the two foundational data structures in `src/module_gnu_open.rs`, resolving any shared type usage and constructor/helper needs required before function porting. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Port the sole function from `gnu/open.c` into `src/module_gnu_open.rs`, using the implemented data structures and preserving the original module behavior. Depends on: T005
- [T007] [Story] Refine the function integration in `src/module_gnu_open.rs` so its signature, internal helpers, and module visibility align with the surrounding Rust project structure for `045-module_gnu_open.c_39-rust-port`. Depends on: T006

## Final Phase: Polish

- [T008] [Story] Perform module-level cleanup in `src/module_gnu_open.rs`, removing migration scaffolding no longer needed after the port and simplifying obvious C-to-Rust translation artifacts. Depends on: T007
- [T009] [Story] Review `src/module_gnu_open.rs` for idiomatic Rust improvements that do not expand scope, including small naming, ownership, and control-flow refinements within the migrated module. Depends on: T008