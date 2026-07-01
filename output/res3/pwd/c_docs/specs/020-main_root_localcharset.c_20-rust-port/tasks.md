# Tasks: main_root_localcharset.c_20

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `localcharset.c` port in `src/localcharset.rs`, and expose it from the crate root in `src/lib.rs` or `src/main.rs` according to the existing project layout on branch `020-main_root_localcharset.c_20-rust-port`.
- [T002] [P] [Story] Add placeholder Rust items in `src/localcharset.rs` for the module’s 8 data structures and 1 function so the file layout matches the C module migration target. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the 8 data structure definitions required by `localcharset.c` in `src/localcharset.rs`, translating the C module’s foundational types, constants, and table-like representations into Rust equivalents before any function logic is added. Depends on: T002.
- [T004] [P] [Story] Refine visibility, ownership, and module-local organization for the `localcharset.c` data structures in `src/localcharset.rs` so the upcoming function implementation can consume them directly without adding extra compatibility layers. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the single `localcharset.c` function in `src/localcharset.rs`, using the completed Rust data structures and preserving the original module-local behavior within the `main_cluster` scope. Depends on: T004.
- [T006] [P] [Story] Integrate the implemented `localcharset.c` function into the crate surface through `src/lib.rs` or `src/main.rs`, keeping the exported API aligned with the Rust project layout and limiting changes to this module migration. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Perform a module polish pass on `src/localcharset.rs` to remove placeholder code, tighten type usage, and simplify control flow introduced during the port while preserving the implemented `localcharset.c` behavior. Depends on: T005.
- [T008] [Story] Clean up crate-level wiring in `src/lib.rs` or `src/main.rs` so the `localcharset` module is consistently named, reachable, and free of redundant migration scaffolding. Depends on: T006, T007.