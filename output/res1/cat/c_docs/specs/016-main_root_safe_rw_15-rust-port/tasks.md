# Tasks: main_root_safe_rw_15

## Phase 1: Setup

- [T001] [Story] Create the module port target files for `include/safe-read.c` and `safe-read.c` in the Rust branch by adding the Rust implementation file at `src/safe_read.rs`.
- [T002] [P] [Story] Wire the new module into the Rust crate by declaring and exposing `src/safe_read.rs` from `src/lib.rs` or `src/main.rs`, depending on the existing crate entrypoint.

## Phase 2: Foundational

- [T003] [Story] Review the C module surface from `include/safe-read.c` and `safe-read.c` and define the Rust module API in `src/safe_read.rs`, including the function signatures, return types, and shared imports needed by both translated functions. Depends on: T001, T002

## Phase 3: Safe read function implementation

- [T004] [Story] Port the core safe-read routine from `safe-read.c` into `src/safe_read.rs`, preserving the original read-loop behavior, partial-read handling, and error/EOF result mapping required by the C module contract. Depends on: T003
- [T005] [Story] Port the companion safe-read entry/helper function from `include/safe-read.c` and `safe-read.c` into `src/safe_read.rs`, reusing the shared API established for the module and keeping the implementation aligned with the original module-level behavior. Depends on: T003

## Final Phase: Polish

- [T006] [Story] Refine `src/safe_read.rs` to remove duplicated logic between the two ported functions, tighten Rust idioms around I/O and error propagation, and confirm the final module remains scoped to the behavior defined by `include/safe-read.c` and `safe-read.c`. Depends on: T004, T005