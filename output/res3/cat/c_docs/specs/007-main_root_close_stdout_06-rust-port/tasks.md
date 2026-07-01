# Tasks: main_root_close_stdout_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `closeout.c` port in `src/closeout.rs`, and declare it from `src/main.rs` or `src/lib.rs` according to the existing crate entry layout on branch `007-main_root_close_stdout_06-rust-port`.
- [T002] [P] [Story] Add the initial public/internal function stubs in `src/closeout.rs` for the three functions represented by `closeout.c`, preserving the C module boundary for later implementation. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `closeout.c` for module-local constants, aliases, or helper state and map any required foundational items into `src/closeout.rs`; if no data structures or persistent state are present, record the module as function-only and keep this phase limited to file-local foundational definitions. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the close-stdout core behavior in `src/closeout.rs` by porting the primary function that finalizes standard output handling from `closeout.c`, keeping error semantics aligned with the C module. Depends on: T003.
- [T005] [P] [Story] Implement the remaining helper function(s) in `src/closeout.rs` that support the close-stdout flow from `closeout.c`, grouped as module-local functionality and aligned to the same error/reporting path. Depends on: T003.
- [T006] [Story] Integrate the three ported functions within `src/closeout.rs` so the exported/mainly used closeout entrypoint calls the required helpers in the same functional grouping as `closeout.c`. Depends on: T004, T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/closeout.rs` for idiomatic Rust within the existing `cat` project style, removing unnecessary placeholders from the port and tightening visibility, imports, and control flow without changing module behavior. Depends on: T006.
- [T008] [Story] Wire the completed `closeout` module usage at the Rust crate entry point in `src/main.rs` or `src/lib.rs`, ensuring the `main_cluster` migration for `main_root_close_stdout_06` is complete and builds cleanly. Depends on: T007.