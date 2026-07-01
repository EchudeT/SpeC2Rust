# Tasks: main_root_close_stdout_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `closeout.c` in `src/closeout.rs` and declare it from the crate root used by the `cat` project branch `007-main_root_close_stdout_06-rust-port`.
- [T002] [Story] Review the C source responsibilities in `closeout.c` and map the 3 exported/internal functions to Rust function stubs in `src/closeout.rs`, preserving the module-local scope each function needs. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Confirm that this module requires no dedicated Rust data structures because the input analysis reports `0` data structures, and keep `src/closeout.rs` limited to function and import definitions only. Depends on: T002

## Phase 3: Closeout function implementation

- [T004] [Story] Implement the Rust equivalent of the stdout-closing helper logic from `closeout.c` in `src/closeout.rs`, keeping behavior aligned with the source module’s close/flush responsibility. Depends on: T003
- [T005] [Story] Implement the Rust equivalent of the write/close status handling helper from `closeout.c` in `src/closeout.rs`, grouped with the stdout closeout flow it supports. Depends on: T003
- [T006] [P] [Story] Implement the remaining module-local reporting or termination helper from `closeout.c` in `src/closeout.rs`, matching how the C module finishes closeout-related failures. Depends on: T003
- [T007] [Story] Wire the 3 Rust functions together in `src/closeout.rs` so the module exposes the same closeout control flow as `closeout.c` without duplicating logic. Depends on: T004, T005, T006

## Final Phase: Polish

- [T008] [Story] Refine `src/closeout.rs` imports, visibility, and error-path clarity so the port stays minimal, idiomatic, and scoped strictly to the original `closeout.c` module behavior. Depends on: T007