# Tasks: main_root_progname.c_30

## Phase 1: Setup

- [T001] [Story] Create or update the Rust module file `src/progname.rs` for the `progname.c` port target on branch `031-main_root_progname.c_30-rust-port`.
- [T002] [Story] Wire the new module into the crate from `src/main.rs` or `src/lib.rs`, exposing `src/progname.rs` so the ported implementation can be called. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `src/progname.rs` and define the minimal Rust-level constants, type aliases, or private helper state directly required to support the single function migrated from `progname.c`. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Port the function implemented in `progname.c` into `src/progname.rs`, preserving the module-local behavior and interfaces required by the cat main-cluster program-name handling flow. Depends on: T003
- [T005] [P] [Story] Update the calling integration in `src/main.rs` or `src/lib.rs` to use the Rust implementation from `src/progname.rs` at the appropriate main-cluster entry path. Depends on: T002, T004

## Final Phase: Polish

- [T006] [Story] Refine `src/progname.rs` and its integration points in `src/main.rs` or `src/lib.rs` to remove migration scaffolding, align naming and visibility with crate conventions, and ensure the port remains limited to behavior evidenced by `progname.c`. Depends on: T005