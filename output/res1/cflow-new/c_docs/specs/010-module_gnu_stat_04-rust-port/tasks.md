# Tasks: module_gnu_stat_04

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the gnu stat cluster in `src/gnu/mod.rs` and add module declarations for target files inferred from `gnu/dup2.c`, `gnu/fcntl.c`, `gnu/fstat.c`, `gnu/open.c`, `gnu/stat.c`, and `gnu/stat_w32.rs`.
- [T002] [Story] Create Rust source files `src/gnu/dup2.rs`, `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat.rs`, and `src/gnu/stat_w32.rs` with placeholder public APIs matching the C module split. Depends on: T001.

## Phase 2: Foundational

- [T003] [P] [Story] Port and define the data structures required by `gnu/fcntl.c` in `src/gnu/fcntl.rs`, preserving the C module-local structure layout and constants needed by later function work. Depends on: T002.
- [T004] [P] [Story] Port and define the data structures required by `gnu/stat.c` in `src/gnu/stat.rs`, preserving the C module-local structure layout and constants needed by later function work. Depends on: T002.
- [T005] [P] [Story] Port and define the data structures required by `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, preserving the C module-local structure layout and constants needed by later function work. Depends on: T002.
- [T006] [P] [Story] Port and define the data structures required by `gnu/fstat.c` in `src/gnu/fstat.rs`, aligned with the shared stat-related representations needed by file-status functions. Depends on: T004, T005.
- [T007] [P] [Story] Port and define the data structures required by `gnu/open.c` in `src/gnu/open.rs`, aligned with the flag and mode representations established in `src/gnu/fcntl.rs`. Depends on: T003.
- [T008] [P] [Story] Port and define the data structures required by `gnu/dup2.c` in `src/gnu/dup2.rs`, aligned with descriptor and flag representations established in `src/gnu/fcntl.rs`. Depends on: T003.

## Phase 3: File Descriptor and Open/Control Functions

- [T009] [P] [Story] Implement the function port from `gnu/dup2.c` in `src/gnu/dup2.rs`, using the foundational descriptor-related structures and constants already defined. Depends on: T008.
- [T010] [P] [Story] Implement the function port from `gnu/fcntl.c` in `src/gnu/fcntl.rs`, covering the file-control behavior represented by this module without duplicating later open/stat work. Depends on: T003.
- [T011] [Story] Implement the function port from `gnu/open.c` in `src/gnu/open.rs`, wiring open-related behavior to the flag and mode structures from `src/gnu/fcntl.rs`. Depends on: T007, T010.

## Phase 4: File Status Functions

- [T012] [P] [Story] Implement the function port from `gnu/stat.c` in `src/gnu/stat.rs`, using the shared stat-related structures defined for this module cluster. Depends on: T004.
- [T013] [P] [Story] Implement the function port from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, preserving the Windows-specific status handling from the source module and aligning it with shared stat representations where needed. Depends on: T005.
- [T014] [Story] Implement the function port from `gnu/fstat.c` in `src/gnu/fstat.rs`, integrating descriptor-based status retrieval with the shared stat foundations and any platform-specific status handling already ported. Depends on: T006, T012, T013.

## Final Phase: Polish

- [T015] [Story] Refine module exports and internal visibility across `src/gnu/mod.rs`, `src/gnu/dup2.rs`, `src/gnu/fcntl.rs`, `src/gnu/fstat.rs`, `src/gnu/open.rs`, `src/gnu/stat.rs`, and `src/gnu/stat_w32.rs` so the Rust port matches the intended module boundaries without redundant public surface. Depends on: T009, T010, T011, T012, T013, T014.
- [T016] [Story] Perform a final consistency pass on flag handling, structure reuse, and duplicated logic across `src/gnu/fcntl.rs`, `src/gnu/open.rs`, `src/gnu/dup2.rs`, `src/gnu/fstat.rs`, `src/gnu/stat.rs`, and `src/gnu/stat_w32.rs`, keeping the port aligned with the original file-level responsibilities. Depends on: T015.