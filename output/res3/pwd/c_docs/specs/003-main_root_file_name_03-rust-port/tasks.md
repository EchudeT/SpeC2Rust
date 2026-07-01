# Tasks: main_root_file_name_03

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `pwd.c` port in `src/main_root_file_name_03.rs` and expose it from `src/lib.rs` on branch `003-main_root_file_name_03-rust-port`.
- [T002] [P] [Story] Define the migration surface for this module in `src/main_root_file_name_03.rs`, including placeholders for the 6 ported functions and the 18 required data structures, keeping names aligned with the C module analysis. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the foundational data structure set required by `pwd.c` in `src/main_root_file_name_03.rs`, covering module-local structs, enums, type aliases, and constants that represent the analyzed 18 C data structures. Depends on: T002
- [T004] [Story] Refine ownership, borrowing, and field typing for the migrated `pwd.c` data structures in `src/main_root_file_name_03.rs` so later function ports can use them directly without placeholder types. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Implement the entry-point oriented function group from `pwd.c` in `src/main_root_file_name_03.rs`, porting the main control-flow functions that coordinate module execution against the migrated data structures. Depends on: T004
- [T006] [P] [Story] Implement the path and root-file-name handling function group from `pwd.c` in `src/main_root_file_name_03.rs`, porting the functions responsible for deriving, normalizing, or selecting the root file name inputs used by the main flow. Depends on: T004
- [T007] [Story] Implement the remaining support/helper function group from `pwd.c` in `src/main_root_file_name_03.rs`, porting the leftover internal functions and integrating them with the entry-point and path-handling groups so all 6 analyzed functions are covered exactly once. Depends on: T005, T006

## Final Phase: Polish

- [T008] [Story] Remove porting placeholders and align signatures, visibility, and module organization in `src/main_root_file_name_03.rs` and `src/lib.rs` with the completed `pwd.c` migration. Depends on: T007
- [T009] [Story] Perform a final cleanup pass on `src/main_root_file_name_03.rs` to simplify obvious translation artifacts from `pwd.c`, reduce unnecessary mutability, and ensure the port is idiomatic Rust without changing module behavior. Depends on: T008