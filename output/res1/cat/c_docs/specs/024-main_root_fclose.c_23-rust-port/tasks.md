# Task List: main_root_fclose.c_23

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for the `fclose.c` port on branch `024-main_root_fclose.c_23-rust-port`, creating or updating `src/fclose.rs` and wiring the module entry from `src/main.rs` if not already present.
- [T002] [P] [Story] Define the Rust-facing function signatures in `src/fclose.rs` for the 2 functions migrated from `fclose.c`, preserving the original module scope and call boundaries. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `fclose.c` for any module-local constants, aliases, or helper-level foundational definitions required by both migrated functions, and implement only those directly evidenced items in `src/fclose.rs`. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the primary file-close behavior from `fclose.c` in `src/fclose.rs`, translating the core logic and preserving the original return and state-update semantics. Depends on: T003.
- [T005] [Story] Implement the remaining closely related support function from `fclose.c` in `src/fclose.rs`, keeping it aligned with the same file-close workflow and module-local behavior. Depends on: T003.
- [T006] [Story] Integrate the completed `fclose.c`-derived function flow into `src/main.rs` only where this module is directly invoked, ensuring the Rust port uses the new `src/fclose.rs` implementation without expanding scope. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/fclose.rs` and any direct call sites in `src/main.rs` to remove migration-only placeholders, tighten error/return mapping to the C module behavior, and ensure the port remains minimal and consistent with `fclose.c`. Depends on: T006.