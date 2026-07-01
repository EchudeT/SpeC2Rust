# Tasks: cat main_root_xbinary-io.c_36

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `xbinary-io.c` port on branch `037-main_root_xbinary_io.c_36-rust-port`, adding the target source file at `src/xbinary_io.rs`.
- [T002] [Story] Wire the new module into the crate entry so the `src/xbinary_io.rs` unit is compiled and reachable from the main cluster migration path. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `xbinary-io.c` and define the minimal Rust-side foundational items required by its single exported/internal function, placing only directly evidenced type aliases, constants, or helper signatures into `src/xbinary_io.rs`. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Port the function implemented in `xbinary-io.c` into `src/xbinary_io.rs`, preserving the C module’s input/output behavior and keeping the implementation scoped to this module’s binary I/O responsibilities. Depends on: T003.
- [T005] [P] [Story] Update any directly affected call sites in the main cluster to invoke the Rust implementation from `src/xbinary_io.rs` once the ported function is available. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/xbinary_io.rs` for idiomatic Rust within the existing port scope, removing migration-only rough edges and confirming the module remains limited to behavior evidenced by `xbinary-io.c`. Depends on: T004, T005.