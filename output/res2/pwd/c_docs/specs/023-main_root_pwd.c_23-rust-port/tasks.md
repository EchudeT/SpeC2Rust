# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `main_root_pwd` port in `src/pwd.rs`, establishing the target file migrated from `pwd.c`.
- [T002] [Story] Wire the new `src/pwd.rs` module into the crate entry points used by the `pwd` project so the `023-main_root_pwd.c_23-rust-port` branch can compile the migrated module. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust data structures, enums, and type aliases required by the `pwd.c` migration in `src/pwd.rs`, covering the module-local representations evidenced by the source analysis before any function porting begins. Depends on: T001.
- [T004] [P] [Story] Add constructors, default/initial-state helpers, and internal layout comments for the `pwd.c`-derived data structures in `src/pwd.rs` where needed to support direct function translation. Depends on: T003.
- [T005] [P] [Story] Add shared constant definitions and internal utility values in `src/pwd.rs` that are required by the migrated data structures and will be referenced by the module functions. Depends on: T003.

## Phase 3: Functions

- [T006] [Story] Port the module entry/control function from `pwd.c` into `src/pwd.rs`, using the foundational data structures and preserving the original main-cluster behavior. Depends on: T003, T004, T005.
- [T007] [Story] Port the remaining helper/support function from `pwd.c` into `src/pwd.rs`, grouping it with the entry/control flow implementation so shared state and data-structure usage are completed together. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/pwd.rs` for idiomatic Rust within the migrated scope by removing translation leftovers, tightening visibility, and simplifying data-structure/function interactions without changing behavior. Depends on: T007.
- [T009] [Story] Perform a final compile-focused review of the migrated `pwd.c` coverage in `src/pwd.rs` and crate wiring touched by this module to ensure the port is complete and internally consistent. Depends on: T002, T008.