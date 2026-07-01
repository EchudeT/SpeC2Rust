# Task List: `main_root_pwd.c_23` Rust Port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `pwd.c` port on branch `023-main_root_pwd.c_23-rust-port`, adding the target source file `src/pwd.rs` and exposing it from `src/lib.rs` or `src/main.rs` according to the existing crate entry layout.
- [T002] [P] [Story] Establish the module-level Rust item layout in `src/pwd.rs` for the `pwd.c` migration, reserving sections for translated data structures and the two function implementations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Translate the C module’s 18 data structures into Rust definitions in `src/pwd.rs`, preserving only the shapes and relationships evidenced by `pwd.c` and preparing them for use by the module functions. Depends on: T002.
- [T004] [P] [Story] Refine the translated data-structure definitions in `src/pwd.rs` with Rust-appropriate visibility, ownership, and type aliases required for internal module use, without expanding beyond the structures evidenced by `pwd.c`. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the first `pwd.c` function in `src/pwd.rs`, wiring it to the translated data structures and preserving the original module-local behavior expected from the C source. Depends on: T004.
- [T006] [Story] Implement the second `pwd.c` function in `src/pwd.rs`, completing the function migration for `main_root_pwd.c_23` and integrating it with the module data structures and any prior translated function logic. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Perform a module pass on `src/pwd.rs` to remove migration scaffolding, resolve Rust compiler and lint issues introduced by the port, and simplify obvious C-to-Rust translation artifacts while keeping behavior aligned with `pwd.c`. Depends on: T006.
- [T008] [P] [Story] Review `src/lib.rs` or `src/main.rs` and `src/pwd.rs` for final module integration cleanup, ensuring the `pwd.c` port is consistently exposed and no unused migration-only declarations remain. Depends on: T007.