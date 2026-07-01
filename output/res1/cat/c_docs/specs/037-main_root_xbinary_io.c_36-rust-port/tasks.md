# Tasks: cat — main_root_xbinary-io.c_36

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `xbinary-io.c` port in `src/xbinary_io.rs` and expose it from `src/lib.rs` or `src/main.rs` according to the current crate entry layout on branch `037-main_root_xbinary_io.c_36-rust-port`.
- [T002] [P] [Story] Add a migration placeholder in `src/xbinary_io.rs` that documents the source mapping from `xbinary-io.c` and identifies the single function to be ported in this module.

## Phase 2: Foundational

- [T003] [Story] Review `xbinary-io.c` for module-local constants, type aliases, or helper definitions required by the target function, and recreate only those directly needed in `src/xbinary_io.rs`. Depends on: T001, T002

## Phase 3: Functions

- [T004] [Story] Port the single function implemented in `xbinary-io.c` into `src/xbinary_io.rs`, preserving the original control flow and I/O-facing behavior while adapting it to idiomatic Rust signatures and error propagation as supported by the crate. Depends on: T003
- [T005] [Story] Wire the ported function into the crate surface from `src/lib.rs` or `src/main.rs` so the migrated `xbinary-io.c` behavior is reachable through the Rust module structure. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/xbinary_io.rs` to remove migration placeholders, align naming and visibility with the surrounding Rust crate conventions, and ensure the final file contains only the definitions required for the `xbinary-io.c` port. Depends on: T005