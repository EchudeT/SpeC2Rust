# Task List: main_root_close_stdout_07

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/closeout.rs` for the port of `closeout.c`, and declare it from the existing crate entry point so the `main_root_close_stdout_07` module is compiled on branch `007-main_root_close_stdout_07-rust-port`.
- [T002] [Story] Define the public function signatures in `src/closeout.rs` for the three functions migrated from `closeout.c`, matching the C module’s responsibility around stdout closing and closeout handling so later implementation tasks have stable Rust targets. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Establish any module-local foundational items required by the `closeout.c` port in `src/closeout.rs`, limited to constants, helper type aliases, or internal helper declarations directly evidenced by the migrated functions; if no such items are needed, keep this phase as a no-op confirmation during implementation. Depends on: T002

## Phase 3: Functions

- [T004] [P] [Story] Implement the standalone closeout helper logic in `src/closeout.rs` for the function from `closeout.c` that performs generic output stream finalization without main-program termination behavior. Depends on: T003
- [T005] [P] [Story] Implement the stdout-specific close handling function in `src/closeout.rs` for the function from `closeout.c` responsible for closing or flushing standard output and detecting write/close failures. Depends on: T003
- [T006] [Story] Implement the main/root-facing closeout function in `src/closeout.rs` that integrates the module’s closeout behavior for the `pwd` main cluster, coordinating the helper functions ported from `closeout.c`. Depends on: T004, T005

## Final Phase: Polish

- [T007] [Story] Refine `src/closeout.rs` to remove migration-only scaffolding, align naming and visibility with crate conventions, and verify the three ported functions form a minimal, coherent Rust module matching `closeout.c` scope. Depends on: T006