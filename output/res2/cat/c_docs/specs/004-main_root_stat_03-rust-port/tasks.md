# Tasks: main_root_stat_03

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for this port on branch `004-main_root_stat_03-rust-port`, adding module files aligned to the source inputs: `src/cat.rs` for logic migrated from `cat.c` and `src/fcntl.rs` for logic migrated from `fcntl.c`.
- [T002] [P] [Story] Wire the new module files into the crate root so the `main_root_stat_03` implementation is buildable from the Rust project entry points, updating only directly related Rust module declarations.
- [T003] [Story] Establish the shared migration placeholders in `src/cat.rs` and `src/fcntl.rs` for the 3 data structures and 2 functions identified in this module, preserving clear ownership of items by source file. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Implement the data structures migrated from `cat.c` in `src/cat.rs`, translating the C layout and fields needed by this module’s functions.
- [T005] [P] [Story] Implement the data structures migrated from `fcntl.c` in `src/fcntl.rs`, translating the C layout and fields needed by this module’s functions.
- [T006] [Story] Reconcile the 3 migrated data structures so shared types and visibility between `src/cat.rs` and `src/fcntl.rs` match actual function usage in this module without introducing unrelated abstractions. Depends on: T004, T005

## Phase 3: Functions

- [T007] [Story] Implement the function logic from `cat.c` in `src/cat.rs`, using the migrated data structures and keeping behavior scoped to the original module responsibilities. Depends on: T006
- [T008] [Story] Implement the function logic from `fcntl.c` in `src/fcntl.rs`, using the migrated data structures and keeping behavior scoped to the original module responsibilities. Depends on: T006
- [T009] [Story] Integrate the 2 migrated functions across `src/cat.rs` and `src/fcntl.rs` so call boundaries, type usage, and module visibility align with the original `main_root_stat_03` relationships. Depends on: T007, T008

## Final Phase: Polish

- [T010] [Story] Refine the migrated Rust code in `src/cat.rs` and `src/fcntl.rs` to remove translation scaffolding, tighten signatures and visibility, and resolve compile-time issues introduced during the port. Depends on: T009
- [T011] [Story] Perform a final module review for `main_root_stat_03` to confirm the Rust port remains limited to behavior evidenced by `cat.c` and `fcntl.c`, with no duplicate implementation or unsupported expansion. Depends on: T010