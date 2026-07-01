# Tasks: main_root_binary-io.c_17

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/binary_io.rs` to host the port of logic from `binary-io.c`.
- [T002] [Story] Register the new module in `src/main.rs` or `src/lib.rs` by adding the `binary_io` module declaration so the migrated code is compiled.
- [T003] [P] [Story] Add migration placeholders and file-level comments in `src/binary_io.rs` identifying the source module `binary-io.c` and the target branch `018-main_root_binary_io.c_17-rust-port`. Depends on: T001

## Phase 2: Foundational

- [T004] [Story] Review `binary-io.c` and define the Rust function signature scaffold in `src/binary_io.rs` for the single migrated function, preserving the C module responsibility and required inputs/outputs. Depends on: T001
- [T005] [Story] Add any module-local type aliases, constants, or helper declarations directly evidenced by the migrated function into `src/binary_io.rs`, keeping them minimal and scoped to the port. Depends on: T004

## Phase 3: Functions

- [T006] [Story] Implement the binary I/O function from `binary-io.c` in `src/binary_io.rs`, translating its control flow and I/O behavior into idiomatic Rust while preserving module semantics. Depends on: T004, T005
- [T007] [P] [Story] Wire the implemented function into the main-cluster call site in `src/main.rs` if the migrated function is invoked from the Rust entry path, replacing any placeholder usage introduced during setup. Depends on: T006, T002

## Final Phase: Polish

- [T008] [Story] Refine `src/binary_io.rs` to remove migration scaffolding, resolve compile warnings, and align naming/imports with the surrounding Rust main-cluster code without changing behavior. Depends on: T006
- [T009] [Story] Perform a final integration review of `src/binary_io.rs` with `src/main.rs` or `src/lib.rs` to confirm the module builds cleanly and the single migrated function is exposed only where needed. Depends on: T007, T008