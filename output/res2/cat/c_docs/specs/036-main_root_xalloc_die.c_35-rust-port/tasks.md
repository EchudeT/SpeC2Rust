# Tasks: main_root_xalloc-die.c_35

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module file for this port in `src/xalloc_die.rs`, establishing the target location for logic migrated from `xalloc-die.c`.
- [T002] [Story] Wire the new module into the crate from `src/lib.rs` or `src/main.rs` (whichever currently owns module declarations in this branch) so `src/xalloc_die.rs` is compiled and available to the rest of the project. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review the C module surface in `xalloc-die.c` and define the minimal Rust-facing function signature in `src/xalloc_die.rs` needed to represent the module’s single exported behavior, without introducing new data structures or unrelated APIs. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Implement the migrated allocation-failure termination function from `xalloc-die.c` in `src/xalloc_die.rs`, preserving the original module responsibility and behavior expected by callers. Depends on: T003
- [T005] [P] [Story] Update direct call sites in `src/lib.rs` or `src/main.rs` that should use the Rust implementation from `src/xalloc_die.rs`, keeping integration limited to references required for this module port. Depends on: T002, T004

## Final Phase: Polish

- [T006] [Story] Refine `src/xalloc_die.rs` and its module exposure for idiomatic Rust naming, visibility, and minimal imports, ensuring the port remains scoped to the original `xalloc-die.c` behavior. Depends on: T004, T005