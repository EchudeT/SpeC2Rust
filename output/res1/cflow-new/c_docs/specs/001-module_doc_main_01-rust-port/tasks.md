# Tasks: module_doc_main_01 Rust port

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module layout for the `doc` cluster on branch `001-module_doc_main_01-rust-port`, creating target source files `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs`.
- [T002] [Story] Declare and wire the new Rust module files into the crate module tree so `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs` are compiled from the project entry modules. Depends on: T001.

## Phase 2: Foundational

- [T003] [P] [Story] Port the first documented shared data structure from the C module analysis into an idiomatic Rust definition in the directly corresponding target file among `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, or `src/doc/whoami.rs`, preserving the original module-local ownership and field layout intent. Depends on: T002.
- [T004] [P] [Story] Port the second documented shared data structure from the C module analysis into an idiomatic Rust definition in the directly corresponding target file among `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, or `src/doc/whoami.rs`, preserving the original module-local ownership and field layout intent. Depends on: T002.
- [T005] [P] [Story] Port the third documented shared data structure from the C module analysis into an idiomatic Rust definition in the directly corresponding target file among `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, or `src/doc/whoami.rs`, preserving the original module-local ownership and field layout intent. Depends on: T002.
- [T006] [Story] Reconcile shared type usage across `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs` so the three ported data structures are referenced consistently by the upcoming function implementations. Depends on: T003, T004, T005.

## Phase 3: Functions

- [T007] [P] [Story] Implement the function port for `doc/d.c` in `src/doc/d.rs`, translating its control flow and use of the ported data structures without extending scope beyond the original module behavior. Depends on: T006.
- [T008] [P] [Story] Implement the function port for `doc/foo.c` in `src/doc/foo.rs`, translating its control flow and use of the ported data structures without extending scope beyond the original module behavior. Depends on: T006.
- [T009] [P] [Story] Implement the function port for `doc/wc.c` in `src/doc/wc.rs`, translating its control flow and use of the ported data structures without extending scope beyond the original module behavior. Depends on: T006.
- [T010] [P] [Story] Implement the function port for `doc/whoami.c` in `src/doc/whoami.rs`, translating its control flow and use of the ported data structures without extending scope beyond the original module behavior. Depends on: T006.
- [T011] [Story] Integrate the four function ports across `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs`, resolving call signatures, shared imports, and module visibility to match the original C module relationships. Depends on: T007, T008, T009, T010.

## Final Phase: Polish

- [T012] [Story] Refine the Rust port in `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs` by removing translation-only scaffolding, tightening ownership/borrowing, and resolving compiler warnings while preserving the original module behavior. Depends on: T011.