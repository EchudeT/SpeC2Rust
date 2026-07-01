# Tasks: main_root_localcharset.c_20

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `localcharset.c` in `src/localcharset.rs` and expose it from the crate root or module tree used by branch `020-main_root_localcharset.c_20-rust-port`.
- [T002] [P] [Story] Add migration placeholders and module-level documentation in `src/localcharset.rs` describing the port scope for `localcharset.c` and listing the data structures and function to be implemented.

## Phase 2: Foundational

- [T003] [Story] Define the first group of core data structures migrated from `localcharset.c` in `src/localcharset.rs`, preserving C layout intent only where required by the module logic. Depends on: T001.
- [T004] [P] [Story] Define the second group of supporting data structures migrated from `localcharset.c` in `src/localcharset.rs`, completing the module’s 8 data-structure ports without introducing unrelated abstractions. Depends on: T001.
- [T005] [Story] Reconcile and finalize all 8 data structures in `src/localcharset.rs`, including field typing, visibility, and intra-module relationships needed by the function implementation. Depends on: T003, T004.

## Phase 3: Functions

- [T006] [Story] Implement the single function migrated from `localcharset.c` in `src/localcharset.rs`, using the completed local data structures and preserving the original module behavior. Depends on: T005.
- [T007] [P] [Story] Integrate the function’s public or module-visible API surface in `src/localcharset.rs` so it matches the surrounding Rust project organization for the main cluster port. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/localcharset.rs` by removing migration placeholders, tightening signatures and internal helper usage, and resolving compile-time issues introduced during the `localcharset.c` port. Depends on: T007.