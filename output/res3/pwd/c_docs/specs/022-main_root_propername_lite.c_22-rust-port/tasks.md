# Task List: `main_root_propername-lite.c_22`

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/propername_lite.rs` to host the port of `propername-lite.c`.
- [T002] [Story] Wire the new module into the crate from `src/main.rs` so the `propername_lite` implementation is compiled and available.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `propername-lite.c` and define the minimal Rust function signature(s) in `src/propername_lite.rs` needed for the single exported behavior of this module, without introducing new data structures.
  - Depends on: T001

## Phase 3: Functions

- [T004] [Story] Implement the core function port from `propername-lite.c` in `src/propername_lite.rs`, preserving the original module behavior in Rust.
  - Depends on: T003
- [T005] [P] [Story] Update call sites in `src/main.rs` to invoke the Rust implementation from `src/propername_lite.rs` in place of the migrated C module logic.
  - Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/propername_lite.rs` and `src/main.rs` to remove migration scaffolding, align naming with Rust conventions, and keep the port limited to the original `propername-lite.c` scope.
  - Depends on: T005