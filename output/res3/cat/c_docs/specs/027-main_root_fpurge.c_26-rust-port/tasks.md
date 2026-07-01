# Tasks: cat main_root_fpurge.c_26

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for the `fpurge.c` port on branch `027-main_root_fpurge.c_26-rust-port`, adding the target source file at `src/fpurge.rs` and declaring the module from `src/lib.rs` or `src/main.rs` according to the existing crate entry layout.
- [T002] [P] [Story] Establish the migration boundary in `src/fpurge.rs` with a placeholder public API corresponding to the C module `fpurge.c`, so later function implementation can be added without changing module wiring. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `fpurge.c` usage requirements and define any minimal Rust-side foundational aliases, constants, or helper signatures directly required by the module implementation in `src/fpurge.rs`; if no such foundational items are evidenced by the C source, keep this phase limited to confirming that none are needed. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the single function port from `fpurge.c` in `src/fpurge.rs`, preserving the C module behavior and mapping its logic into idiomatic Rust while staying within the module’s evidenced scope. Depends on: T003.
- [T005] [P] [Story] Integrate the implemented `src/fpurge.rs` function into the crate’s existing call surface in `src/lib.rs` or `src/main.rs`, ensuring the ported module is reachable from the Rust project without expanding beyond the original module contract. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/fpurge.rs` and its crate entry declaration for clarity and minimalism, removing placeholder code, resolving any migration-specific TODOs, and ensuring the final port remains focused on the original `fpurge.c` module scope. Depends on: T005.