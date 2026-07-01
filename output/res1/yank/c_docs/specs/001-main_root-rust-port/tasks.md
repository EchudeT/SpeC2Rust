# Tasks: main_root Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust entry-point module scaffold for the `main_root` port in `src/main.rs`, mapping the C source file `yank.c` into the Rust project structure on branch `001-main_root-rust-port`.
- [T002] [P] [Story] Add internal module sections in `src/main.rs` for the `main_root` data structures and function groups so later migration work from `yank.c` has stable placement.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Identify and implement the foundational Rust representations for the 15 C data structures used by `yank.c` directly in `src/main.rs`, preserving field intent and ownership semantics needed by the module entry flow.
  - Depends on: T001
- [T004] [Story] Add shared constructors, default initializers, and helper enums/constants in `src/main.rs` required to instantiate and pass the migrated `main_root` data structures through the function implementations.
  - Depends on: T003

## Phase 3: Startup and argument flow functions

- [T005] [Story] Port the startup-oriented functions from `yank.c` into `src/main.rs`, grouping the functions responsible for process entry, argument intake, and initial runtime state construction around the Rust `main` flow.
  - Depends on: T004
- [T006] [P] [Story] Port the related helper functions from `yank.c` that normalize or transform command-line or initial input values for the startup flow in `src/main.rs`.
- [T007] [Story] Integrate the startup and argument helper function group in `src/main.rs` so the migrated entry path invokes the same sequence as the C module.
  - Depends on: T005, T006

## Phase 4: Main control and operational functions

- [T008] [Story] Port the core control-flow functions from `yank.c` into `src/main.rs`, grouping functions that drive the module’s primary execution path after initialization.
  - Depends on: T007
- [T009] [P] [Story] Port supporting operational helper functions from `yank.c` into `src/main.rs` that are called by the main control-flow group and operate on the foundational data structures.
  - Depends on: T004
- [T010] [Story] Wire the operational helper functions into the main control-flow implementation in `src/main.rs`, completing the migration of the remaining `main_root` execution logic.
  - Depends on: T008, T009

## Phase 5: Shutdown and finalization functions

- [T011] [Story] Port the cleanup and finalization functions from `yank.c` into `src/main.rs`, grouping the functions that release, reset, or finalize module state at the end of execution.
  - Depends on: T010
- [T012] [Story] Connect the finalization function group to the Rust main execution path in `src/main.rs` so termination behavior matches the original module ordering.
  - Depends on: T011

## Final Phase: Polish

- [T013] [Story] Refine `src/main.rs` to remove redundant temporary migration scaffolding, simplify ownership/borrowing across the migrated data structures and 13 function implementations, and ensure the file remains aligned with the original `yank.c` module scope.
  - Depends on: T012