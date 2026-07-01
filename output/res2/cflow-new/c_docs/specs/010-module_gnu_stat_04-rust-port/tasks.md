# Tasks: module_gnu_stat_04

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffolding for this port branch under files corresponding to `gnu/dup2.c`, `gnu/fcntl.c`, `gnu/fstat.c`, `gnu/open.c`, `gnu/stat-w32.c`, and `gnu/stat.c`, establishing compilable Rust targets for the module.
- [T002] [P] [Story] Add module declarations and internal file wiring so the Rust project can build the `module_gnu_stat_04` unit from the created target files. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the 11 module-local data structures required by the source set, placing each structure in the closest corresponding Rust target file among `gnu/fcntl.rs`, `gnu/fstat.rs`, `gnu/open.rs`, `gnu/stat_w32.rs`, and `gnu/stat.rs`.
- [T004] [Story] Implement shared constants, flags, and structure field mappings needed by the translated stat/open/fcntl data structures in their owning Rust files, keeping definitions adjacent to the inferred source file migration. Depends on: T003.
- [T005] [P] [Story] Reconcile shared structure usage across `gnu/fstat.rs`, `gnu/open.rs`, `gnu/stat_w32.rs`, and `gnu/stat.rs` so function implementations can consume consistent foundational types without duplicating definitions. Depends on: T003, T004.

## Phase 3: Descriptor and open/fcntl functions

- [T006] [Story] Port the function from `gnu/dup2.c` into the corresponding Rust target file, preserving the module’s descriptor-duplication behavior against the established foundational types. Depends on: T003, T004.
- [T007] [P] [Story] Port the function set from `gnu/fcntl.c` into the corresponding Rust target file, implementing the module’s fcntl-related behavior using the shared flag and descriptor foundations. Depends on: T004, T005.
- [T008] [P] [Story] Port the function set from `gnu/open.c` into the corresponding Rust target file, implementing the module’s open-related behavior with the shared open/stat structures and flags. Depends on: T004, T005.
- [T009] [Story] Resolve integration points among the translated `dup2`, `fcntl`, and `open` Rust files so descriptor and open-path behavior uses one consistent set of flags and structure contracts. Depends on: T006, T007, T008.

## Phase 4: Stat and fstat functions

- [T010] [Story] Port the function set from `gnu/fstat.c` into the corresponding Rust target file, implementing fstat-related behavior using the foundational stat structures. Depends on: T003, T004, T005.
- [T011] [P] [Story] Port the function set from `gnu/stat.c` into the corresponding Rust target file, implementing the primary stat-related behavior with the shared stat data structures. Depends on: T003, T004, T005.
- [T012] [P] [Story] Port the function set from `gnu/stat-w32.c` into the corresponding Rust target file, implementing the Windows-specific stat behavior using the same foundational structure model where applicable. Depends on: T003, T004, T005.
- [T013] [Story] Consolidate cross-file stat behavior between `gnu/fstat.rs`, `gnu/stat.rs`, and `gnu/stat_w32.rs`, ensuring the translated functions share compatible structure layouts and status handling expectations. Depends on: T010, T011, T012.

## Final Phase: Polish

- [T014] [Story] Perform module-level cleanup in the Rust target files migrated from `gnu/dup2.c`, `gnu/fcntl.c`, `gnu/fstat.c`, `gnu/open.c`, `gnu/stat-w32.c`, and `gnu/stat.c`, removing duplication introduced during porting and tightening internal naming while preserving migrated behavior. Depends on: T009, T013.
- [T015] [Story] Review the final Rust module wiring and file-local organization for `module_gnu_stat_04` so all translated files build together cleanly on branch `010-module_gnu_stat_04-rust-port`. Depends on: T014.