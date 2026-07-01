# Tasks: main_root_stat_04

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for the `main_root_stat_04` port on branch `004-main_root_stat_04-rust-port`, creating and wiring target files inferred from `pwd.c` and `root-dev-ino.c` such as `src/pwd.rs` and `src/root_dev_ino.rs`.
  - Dependencies: None

- [T002] [P] [Story] Add module declarations and integration points so the new Rust files are reachable from the crate entry layout, keeping `src/pwd.rs` and `src/root_dev_ino.rs` aligned with the original C module split.
  - Dependencies: T001

## Phase 2: Foundational

- [T003] [Story] Identify and define the Rust representations for the module’s required foundational data structures from `pwd.c` and `root-dev-ino.c` in `src/pwd.rs`, including ownership/borrowing decisions needed before function migration.
  - Dependencies: T002

- [T004] [P] [Story] Implement the remaining root-device/inode related data structures needed by `root-dev-ino.c` in `src/root_dev_ino.rs`, matching the C module responsibilities closely without expanding beyond evidenced structures.

- [T005] [Story] Reconcile shared structure usage between `src/pwd.rs` and `src/root_dev_ino.rs`, consolidating imports, visibility, and type boundaries so function ports can consume the data structures without duplication.
  - Dependencies: T003, T004

## Phase 3: Functions

- [T006] [Story] Port the function logic from `root-dev-ino.c` into `src/root_dev_ino.rs`, using the completed root device/inode structures and preserving the original module behavior.
  - Dependencies: T004, T005

- [T007] [Story] Port the `pwd.c` function logic into `src/pwd.rs`, updating it to use the Rust data structures and any helper exposed from `src/root_dev_ino.rs`.
  - Dependencies: T003, T005, T006

## Final Phase: Polish

- [T008] [P] [Story] Refine the migrated code in `src/pwd.rs` and `src/root_dev_ino.rs` by removing redundant conversions, tightening visibility, and simplifying control flow while keeping behavior unchanged.
  - Dependencies: T006, T007

- [T009] [Story] Perform final module-level integration cleanup for `main_root_stat_04`, ensuring `src/pwd.rs` and `src/root_dev_ino.rs` build cleanly together and reflect the original C file boundaries.
  - Dependencies: T008