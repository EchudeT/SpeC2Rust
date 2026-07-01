# Tasks: main_root_xalloc-die.c_26

## Phase 1: Setup

- [T001] [Story] Inspect the current Rust crate layout on branch `026-main_root_xalloc_die.c_26-rust-port` and map the C source `xalloc-die.c` to a Rust target file, creating or reserving `src/xalloc_die.rs` for this module port.
- [T002] [Story] Wire the new module file `src/xalloc_die.rs` into the crate entry points that already correspond to the main cluster so the ported implementation can be referenced from the Rust binary path. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the module-level Rust API surface in `src/xalloc_die.rs` for the `xalloc-die.c` port, including the function signature(s) needed by this module and any required imports from existing crate code. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the single function from `xalloc-die.c` in `src/xalloc_die.rs`, preserving its main-cluster behavior and integrating it with the Rust crate’s existing error/termination path instead of leaving a stub. Depends on: T003
- [T005] [P] [Story] Update the calling site(s) within the existing main-cluster Rust entry path to use the implemented function from `src/xalloc_die.rs` wherever the C module behavior is now expected. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Review `src/xalloc_die.rs` and any touched main-cluster module files for naming, visibility, and unused imports, and reduce the implementation to the smallest idiomatic Rust form that still matches the original `xalloc-die.c` role. Depends on: T005