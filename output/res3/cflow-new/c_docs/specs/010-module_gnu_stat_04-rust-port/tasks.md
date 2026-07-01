# Tasks: module_gnu_stat_04

**Input**: C module analysis for `module_gnu_stat_04`
**Prerequisites**: Rust project branch `010-module_gnu_stat_04-rust-port`

## Phase 1: Setup

- [T001] [Story] Create Rust module files corresponding to the analyzed C sources: `src/gnu/dup2.rs`, `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat_w32.rs`, and `src/gnu/stat.rs`.
- [T002] [Story] Register the new Rust modules in the nearest Rust module declaration point so `src/gnu/dup2.rs`, `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat_w32.rs`, and `src/gnu/stat.rs` are compiled by branch `010-module_gnu_stat_04-rust-port`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and define the 11 module-local data structures and constant layouts required by the stat/open/fcntl port, placing each definition in the Rust file that matches its source responsibility across `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat_w32.rs`, and `src/gnu/stat.rs`.
- [T004] [P] [Story] Implement foundational Rust representations for data structures primarily used by file-opening and descriptor-control logic in `src/gnu/open.rs`, `src/gnu/fcntl.rs`, and `src/gnu/dup2.rs`. Depends on: T003.
- [T005] [P] [Story] Implement foundational Rust representations for data structures primarily used by file-status and metadata logic in `src/gnu/fstat.rs`, `src/gnu/stat.rs`, and `src/gnu/stat_w32.rs`. Depends on: T003.
- [T006] [Story] Reconcile shared structure usage between descriptor-control and status/metadata modules so cross-module types and field naming are consistent in `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat_w32.rs`, and `src/gnu/stat.rs`. Depends on: T004, T005.

## Phase 3: File Descriptor and Open Semantics

- [T007] [P] [Story] Port the function from `gnu/dup2.c` into `src/gnu/dup2.rs`, using the established descriptor-control structures and preserving the original module scope behavior. Depends on: T006.
- [T008] [P] [Story] Port the function set from `gnu/fcntl.c` into `src/gnu/fcntl.rs`, implementing descriptor flag/control behavior with the foundational structures already defined. Depends on: T006.
- [T009] [Story] Port the function set from `gnu/open.c` into `src/gnu/open.rs`, implementing file-open behavior and integrating the descriptor-control definitions shared with `src/gnu/fcntl.rs` and `src/gnu/dup2.rs`. Depends on: T004, T008.

## Phase 4: File Status and Metadata Semantics

- [T010] [P] [Story] Port the function set from `gnu/fstat.c` into `src/gnu/fstat.rs`, implementing file-status retrieval against the shared metadata structures. Depends on: T006.
- [T011] [P] [Story] Port the function set from `gnu/stat.c` into `src/gnu/stat.rs`, implementing general stat-path behavior with the shared metadata structures. Depends on: T006.
- [T012] [P] [Story] Port the function set from `gnu/stat-w32.c` into `src/gnu/stat_w32.rs`, implementing Windows-specific stat behavior using the same foundational metadata structures where applicable. Depends on: T006.
- [T013] [Story] Align shared status and metadata behavior across `src/gnu/fstat.rs`, `src/gnu/stat.rs`, and `src/gnu/stat_w32.rs` so duplicated stat-field interpretation is consolidated at module level without changing scope. Depends on: T010, T011, T012.

## Final Phase: Polish

- [T014] [Story] Perform module-level cleanup for `module_gnu_stat_04`: remove porting scaffolds, resolve remaining compile issues, and simplify imports/usages across `src/gnu/dup2.rs`, `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat_w32.rs`, and `src/gnu/stat.rs`. Depends on: T009, T013.
- [T015] [Story] Review the completed Rust port for naming, visibility, and file-local organization consistency so the migrated module remains close to the original C file boundaries in `src/gnu/dup2.rs`, `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat_w32.rs`, and `src/gnu/stat.rs`. Depends on: T014.