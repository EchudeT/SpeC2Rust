# Tasks: main_root_fflush.c_25

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/fflush.rs` for the port of `fflush.c`, and register it from the existing crate entry point used by the `cat` main cluster on branch `026-main_root_fflush.c_25-rust-port`.
- [T002] [P] [Story] Add the public function stubs for the 4 `fflush.c` functions in `src/fflush.rs`, matching the C module scope and leaving `todo!()` placeholders for later implementation. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `fflush.c` and define any module-local Rust type aliases, constants, or helper signatures required by all 4 ported functions directly in `src/fflush.rs`; keep this limited to constructs evidenced by the source module. Depends on: T002.

## Phase 3: Flush stream state and validation functions

- [T004] [Story] Implement the function group in `src/fflush.rs` that performs stream lookup, argument validation, and shared precondition handling for the `fflush.c` port, preserving the original module behavior. Depends on: T003.
- [T005] [P] [Story] Implement the function group in `src/fflush.rs` that handles per-stream flush execution and return-value/error propagation for the `fflush.c` port. Depends on: T004.

## Phase 4: Global flush coordination functions

- [T006] [Story] Implement the remaining function group in `src/fflush.rs` that coordinates whole-process or all-stream flushing behavior from `fflush.c`, reusing the shared logic established earlier without duplicating function work. Depends on: T004.
- [T007] [P] [Story] Complete any remaining standalone helper function implementation in `src/fflush.rs` that is specific to the `fflush.c` module and not yet covered by the earlier grouped tasks. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/fflush.rs` to remove placeholder code, resolve borrow/signature issues, and align naming and visibility with the surrounding Rust main-cluster modules while preserving the `fflush.c` behavior. Depends on: T007.
- [T009] [Story] Run a final pass on `src/fflush.rs` to simplify duplicated logic introduced during porting and keep the implementation minimal and module-scoped. Depends on: T008.