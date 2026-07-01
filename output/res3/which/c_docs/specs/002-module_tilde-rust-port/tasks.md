# Tasks: module_tilde Rust port

## Phase 1: Setup

- [T001] [Story] Initialize Rust module files for the tilde port by creating `src/tilde/mod.rs` and placeholder submodules mapped from `tilde/shell.c` and `tilde/tilde.c`.
- [T002] [Story] Wire the new tilde module into the crate module tree from the existing Rust project entry points so `src/tilde/mod.rs` is compiled on branch `002-module_tilde-rust-port`.
  **Depends on:** T001

## Phase 2: Foundational

- [T003] [Story] Define the tilde-related core data structures and type aliases in `src/tilde/mod.rs`, porting the 5 C module data structures into Rust representations before any function implementation.
  **Depends on:** T002
- [T004] [P] [Story] Add shared constructors, default state, and internal helpers needed by the foundational tilde data structures in `src/tilde/mod.rs`.
  **Depends on:** T003

## Phase 3: Shell integration functions

- [T005] [Story] Port the shell-facing tilde functionality from `tilde/shell.c` into `src/tilde/shell.rs`, implementing the related function group against the foundational Rust data structures.
  **Depends on:** T003
- [T006] [Story] Expose the shell-related APIs from `src/tilde/mod.rs` so the crate can call the `src/tilde/shell.rs` function group.
  **Depends on:** T005

## Phase 4: Tilde expansion core functions

- [T007] [P] [Story] Port the core tilde expansion functionality from `tilde/tilde.c` into `src/tilde/tilde.rs`, implementing the related function group using the shared structures from `src/tilde/mod.rs`.
  **Depends on:** T003
- [T008] [Story] Connect the core tilde expansion APIs through `src/tilde/mod.rs` for use by the rest of the crate.
  **Depends on:** T007

## Final Phase: Polish

- [T009] [Story] Refine cross-module interactions among `src/tilde/mod.rs`, `src/tilde/shell.rs`, and `src/tilde/tilde.rs` by removing migration placeholders, tightening visibility, and resolving any redundant conversions introduced during the C-to-Rust port.
  **Depends on:** T004, T006, T008
- [T010] [P] [Story] Perform a final pass on the tilde module implementation in `src/tilde/mod.rs`, `src/tilde/shell.rs`, and `src/tilde/tilde.rs` to simplify ownership/borrowing boundaries and reduce unnecessary allocations without changing module behavior.
  **Depends on:** T009