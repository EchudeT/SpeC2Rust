# Tasks: main_root_stat_04

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for the `main_root_stat_04` port on branch `004-main_root_stat_04-rust-port`, creating target files `src/bin/pwd.rs` and `src/root_dev_ino.rs` to receive logic migrated from `pwd.c` and `root-dev-ino.c`.
- [T002] [P] [Story] Add module wiring so `src/bin/pwd.rs` can import `src/root_dev_ino.rs`, keeping the migration scope limited to code directly required by `pwd.c` and `root-dev-ino.c`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the Rust data structures and type aliases needed by the migrated `root-dev-ino.c` logic in `src/root_dev_ino.rs`, covering only the C-side data representations evidenced by this module’s 21 data structures. Depends on: T002
- [T004] [P] [Story] Define the Rust data structures and local state representations needed by `pwd.c` in `src/bin/pwd.rs`, reusing shared definitions from `src/root_dev_ino.rs` where applicable and avoiding duplicate model definitions. Depends on: T003

## Phase 3: Root device/inode functionality

- [T005] [Story] Implement the function migrated from `root-dev-ino.c` in `src/root_dev_ino.rs`, preserving the original module responsibility of obtaining and exposing root device/inode state for the `pwd` main flow. Depends on: T003
- [T006] [P] [Story] Integrate the exported root device/inode functionality from `src/root_dev_ino.rs` into `src/bin/pwd.rs` through a minimal interface matching the needs of the main module. Depends on: T005

## Phase 4: Main pwd functionality

- [T007] [Story] Implement the function migrated from `pwd.c` in `src/bin/pwd.rs`, including command entry flow and filesystem state usage required by this module, using the shared root device/inode support from `src/root_dev_ino.rs`. Depends on: T004, T006

## Final Phase: Polish

- [T008] [Story] Refine the migrated code in `src/bin/pwd.rs` and `src/root_dev_ino.rs` to remove redundant C-style patterns, simplify interfaces introduced during migration, and ensure the final Rust code remains scoped to the original module behavior. Depends on: T007