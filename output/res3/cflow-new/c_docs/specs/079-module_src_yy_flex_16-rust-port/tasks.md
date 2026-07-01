# Tasks: module_src_yy_flex_16

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for the `src/c.c` migration on branch `079-module_src_yy_flex_16-rust-port`, creating or updating the target source file at `src/c.rs`.
- [T002] [Story] Define the Rust module surface in `src/c.rs` for this port, reserving space for 13 translated data structures and 2 migrated functions from `src/c.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [P] [Story] Translate and implement the first subset of foundational data structures from `src/c.c` into Rust types in `src/c.rs`, preserving C layout intent where required by module usage. Depends on: T002.
- [T004] [P] [Story] Translate and implement the second subset of foundational data structures from `src/c.c` into Rust types in `src/c.rs`, preserving C layout intent where required by module usage. Depends on: T002.
- [T005] [Story] Complete integration of all 13 data structures in `src/c.rs`, including shared field types, aliases, enums, structs, and constants directly evidenced by `src/c.c`. Depends on: T003, T004.

## Phase 3: Function Implementation

- [T006] [Story] Implement the first migrated function from `src/c.c` in `src/c.rs`, wiring it to the translated data structures and preserving the original module behavior. Depends on: T005.
- [T007] [Story] Implement the second migrated function from `src/c.c` in `src/c.rs`, wiring it to the translated data structures and preserving the original module behavior. Depends on: T005.

## Final Phase: Polish

- [T008] [Story] Refine `src/c.rs` to remove migration-only placeholders, resolve ownership and mutability details, and align the completed data structures and 2 functions with idiomatic Rust while preserving the original `src/c.c` semantics. Depends on: T006, T007.