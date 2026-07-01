# Tasks: module_tilde

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module layout for the tilde port by creating `src/tilde/mod.rs` and wiring the module into the crate root so code from `tilde/shell.c` and `tilde/tilde.c` has a dedicated Rust target location.
- [T002] [P] [Story] Create Rust source files `src/tilde/shell.rs` and `src/tilde/tilde.rs` to mirror the original C module file split and establish the migration targets for shell-related and tilde-expansion logic. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the module-local data structure definitions from `tilde/shell.c` and `tilde/tilde.c` into Rust types in `src/tilde/shell.rs` and `src/tilde/tilde.rs`, preserving the original ownership boundaries and shared field layout needed by the 4 module functions. Depends on: T002.
- [T004] [P] [Story] Add internal enums, type aliases, and helper value containers required to support the 5 migrated data structures in `src/tilde/shell.rs` and `src/tilde/tilde.rs`, keeping them scoped to the module and aligned with the original C usage. Depends on: T003.

## Phase 3: Shell integration functions

- [T005] [Story] Implement the shell-facing function group from `tilde/shell.c` in `src/tilde/shell.rs`, translating control flow and data access against the migrated module data structures. Depends on: T004.
- [T006] [P] [Story] Connect any shared tilde module interfaces needed by the shell-facing functions through `src/tilde/mod.rs`, exposing only the Rust items required for cross-file calls between `src/tilde/shell.rs` and `src/tilde/tilde.rs`. Depends on: T005.

## Phase 4: Tilde expansion functions

- [T007] [Story] Implement the core tilde-expansion function group from `tilde/tilde.c` in `src/tilde/tilde.rs`, completing the direct port of the remaining module functions using the already-migrated data structures. Depends on: T004.
- [T008] [Story] Reconcile shared behavior between `src/tilde/shell.rs` and `src/tilde/tilde.rs` so the 4 migrated functions operate consistently across module boundaries without duplicating logic. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine the `src/tilde/shell.rs` and `src/tilde/tilde.rs` implementations to remove C-specific artifacts, simplify Rust ownership and borrowing where possible, and ensure the final module organization in `src/tilde/mod.rs` is minimal and idiomatic. Depends on: T008.