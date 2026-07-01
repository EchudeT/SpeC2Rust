# Tasks: main_root Rust port

**Input branch**: `001-main_root-rust-port`
**Module**: `main_root`
**Category**: `main`

## Phase 1: Setup

- [T001] [Story] Initialize the Rust binary project structure for the `main_root` port in `Cargo.toml` and `src/main.rs`, mapping the C entrypoint files `c4.c` and `hello.c` into a single executable target.
- [T002] [P] [Story] Create Rust source placeholders for the migrated main-module logic in `src/main.rs`, separating code regions for functionality groups derived from `c4.c` and `hello.c`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the foundational module-level constants, type aliases, and global-state representations required by the `c4.c` and `hello.c` function set in `src/main.rs`, preserving the original main-module layout before function migration. Depends on: T002

## Phase 3: Entry and top-level execution flow

- [T004] [Story] Port the `hello.c` top-level printing/execution function(s) into `src/main.rs`, keeping behavior aligned with the original standalone entry logic. Depends on: T003
- [T005] [Story] Port the primary `main`-path control flow function(s) from `c4.c` into `src/main.rs`, wiring argument handling and top-level execution against the Rust binary entrypoint. Depends on: T003

## Phase 4: Core interpreter/compiler function group

- [T006] [P] [Story] Implement the lexical/parsing-related function group from `c4.c` in `src/main.rs`, migrating the tokenizer/parser logic as one cohesive unit. Depends on: T003
- [T007] [P] [Story] Implement the code-generation/compilation-related function group from `c4.c` in `src/main.rs`, keeping it grouped around the original compilation pipeline responsibilities. Depends on: T003
- [T008] [P] [Story] Implement the execution/virtual-machine-related function group from `c4.c` in `src/main.rs`, preserving the original runtime behavior and module-local state usage. Depends on: T003

## Phase 5: Integration of function groups

- [T009] [Story] Integrate the parser, compilation, and execution function groups in `src/main.rs` so the migrated `c4.c` workflow runs end-to-end from the Rust entry path. Depends on: T005, T006, T007, T008
- [T010] [Story] Reconcile the `hello.c` and `c4.c` entry behaviors in `src/main.rs` so the final Rust binary exposes the intended single executable flow for the `main_root` module. Depends on: T004, T009

## Final Phase: Polish

- [T011] [Story] Refine `src/main.rs` to remove migration placeholders, align naming and control flow with idiomatic Rust where it does not change behavior, and ensure the final file organization cleanly reflects the ported `main_root` module. Depends on: T010