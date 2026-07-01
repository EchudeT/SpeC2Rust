# Tasks: main_root_clear_ungetc_08

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for the `fflush.c` migration on branch `009-main_root_clear_ungetc_08-rust-port`, adding the target source file at `src/fflush.rs` and exposing it from `src/lib.rs` if not already declared.
- [T002] [P] [Story] Review the C logic in `fflush.c` and map the two module functions to Rust implementation placeholders in `src/fflush.rs`, keeping names and responsibilities aligned with the source migration scope.
- [T003] [Story] Wire any main-cluster call sites that directly depend on the migrated `fflush.c` functionality to the Rust module boundary in `src/lib.rs` or the existing crate root, without expanding beyond this module's usage surface. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Define any module-local foundational types or aliases required by the `fflush.c` port in `src/fflush.rs`, limited to structures directly evidenced by the translated function signatures and shared state handling. Depends on: T002

## Phase 3: Functions

- [T005] [Story] Implement the core `fflush.c` function behavior in `src/fflush.rs`, preserving the original control flow and semantics needed by this module's main-cluster role. Depends on: T004
- [T006] [P] [Story] Implement the companion helper or secondary function from `fflush.c` in `src/fflush.rs`, grouping it with the same file-local migration work and keeping its behavior aligned with the C source. Depends on: T004
- [T007] [Story] Integrate the two migrated functions in `src/fflush.rs` so shared assumptions, return paths, and module visibility match the original `fflush.c` relationships. Depends on: T005, T006

## Final Phase: Polish

- [T008] [Story] Refine `src/fflush.rs` for idiomatic Rust within the existing migration scope, removing placeholder code, tightening signatures, and simplifying control flow without changing the translated behavior. Depends on: T007
- [T009] [Story] Perform a final compile-pass review of `src/fflush.rs` and `src/lib.rs` to confirm the `fflush.c` migration is consistently wired into the Rust project and contains no leftover scaffolding. Depends on: T008