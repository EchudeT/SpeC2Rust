# Tasks: main_root_progname.c_30

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/progname.rs` for the port of `progname.c`, and declare it from the crate root so the module is compiled on branch `031-main_root_progname.c_30-rust-port`.
- [T002] [Story] Establish the public API surface in `src/progname.rs` for the single function ported from `progname.c`, matching the C module responsibility for program name handling. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define any module-local foundational state or simple type aliases needed by the `progname.c` port directly in `src/progname.rs`, keeping the implementation minimal because no standalone C data structures are present. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the program-name handling function from `progname.c` in `src/progname.rs`, preserving the original module behavior and limiting the port to this module’s single function responsibility. Depends on: T003.
- [T005] [P] [Story] Wire any required visibility or call-site-facing exports for the implemented program-name function so other Rust code can use it through `src/progname.rs` without expanding beyond the source module scope. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/progname.rs` for idiomatic Rust clarity, remove any placeholder setup left from the migration, and verify the final file remains narrowly aligned with the `progname.c` source behavior. Depends on: T005.