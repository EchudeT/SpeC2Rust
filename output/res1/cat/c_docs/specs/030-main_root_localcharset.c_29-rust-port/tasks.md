# Task List: main_root_localcharset.c_29

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port surface for `localcharset.c` on branch `030-main_root_localcharset.c_29-rust-port` by creating or confirming the module file at `src/localcharset.rs` and declaring it from the crate root file that already hosts `main_cluster` modules.
- [T002] [P] [Story] Add the initial item layout in `src/localcharset.rs` for the 8 data structures and the function port so later migration work stays confined to the `localcharset.c` Rust target.

## Phase 2: Foundational

- [T003] [Story] Port the foundational constants, enums, type aliases, and struct declarations represented by the 8 `localcharset.c` data structures into `src/localcharset.rs`, preserving C-to-Rust layout intent only where required by in-module usage. Depends on: T001, T002
- [T004] [P] [Story] Define the associated static tables, mappings, or embedded metadata in `src/localcharset.rs` that are directly required to support the module’s data structures and later function logic. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Implement the single `localcharset.c` function in `src/localcharset.rs`, wiring it to the migrated data structures and static mappings without expanding beyond behavior evidenced by this module. Depends on: T003, T004
- [T006] [P] [Story] Integrate the exported visibility and call surface for the migrated `localcharset.c` function from `src/localcharset.rs` into the existing `main_cluster` crate root/module declaration file so the Rust port is reachable where the C module was used. Depends on: T005

## Final Phase: Polish

- [T007] [Story] Refine `src/localcharset.rs` for idiomatic Rust within the already-ported scope by removing redundant placeholders, tightening type usage, and resolving migration-time compatibility issues introduced during the `localcharset.c` port. Depends on: T005, T006
- [T008] [Story] Perform a final module review of the `localcharset.c` Rust port across `src/localcharset.rs` and the corresponding crate root/module declaration file to confirm the 8 data structures are defined before the function implementation and that no duplicate migration work remains. Depends on: T007