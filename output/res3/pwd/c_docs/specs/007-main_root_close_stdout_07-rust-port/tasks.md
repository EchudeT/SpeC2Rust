# Tasks: main_root_close_stdout_07

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port scaffolding for this module on branch `007-main_root_close_stdout_07-rust-port`, adding the target module file `src/closeout.rs` and declaring it from `src/main.rs` or `src/lib.rs` according to the existing project entry layout.
- [T002] [P] [Story] Add the migration stub for `closeout.c` logic in `src/closeout.rs`, with placeholder function signatures for the 3 module functions identified from the C source. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `closeout.c` for module-local constants, aliases, or helper state and map any directly needed foundational Rust equivalents into `src/closeout.rs` before function bodies are implemented. Depends on: T002.

## Phase 3: Close-stdout handling functions

- [T004] [Story] Implement the function group in `src/closeout.rs` responsible for module entry-level stdout closeout behavior migrated from `closeout.c`, preserving the original control flow and return/error behavior as evidenced by the source. Depends on: T003.
- [T005] [Story] Implement the function group in `src/closeout.rs` responsible for supporting closeout/error-reporting behavior migrated from `closeout.c`, completing the remaining module functions without changing externally visible semantics. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/closeout.rs` to remove migration stubs, resolve any integration issues with the main cluster wiring, and ensure the port remains idiomatic Rust while staying behaviorally aligned with `closeout.c`. Depends on: T005.