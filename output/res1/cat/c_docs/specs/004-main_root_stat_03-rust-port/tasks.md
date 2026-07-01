# Tasks: main_root_stat_03

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for the `main_root_stat_03` port on branch `004-main_root_stat_03-rust-port`, creating or updating `src/main.rs` and adding module files `src/cat.rs` and `src/fcntl.rs` to mirror the C source split from `cat.c` and `fcntl.c`.
- [T002] [P] [Story] Wire Rust module declarations and call boundaries so code from `src/main.rs` can access implementations placed in `src/cat.rs` and `src/fcntl.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the 3 module-local data structures required by the `main_root_stat_03` port in `src/cat.rs` and `src/fcntl.rs`, preserving the original ownership and field intent from `cat.c` and `fcntl.c`. Depends on: T002.
- [T004] [P] [Story] Adjust the data structure definitions in `src/cat.rs` and `src/fcntl.rs` to use Rust-native types and visibility suitable for later function migration, without expanding beyond fields evidenced by the C module. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Port the function from `cat.c` into `src/cat.rs`, updating its signature to consume the migrated data structures and fit the Rust main-cluster call flow. Depends on: T004.
- [T006] [Story] Port the function from `fcntl.c` into `src/fcntl.rs`, mapping its file-control behavior into Rust while staying aligned with the original module responsibilities only. Depends on: T004.
- [T007] [Story] Integrate the migrated functions into `src/main.rs`, replacing placeholder wiring with actual calls and ensuring imports between `src/main.rs`, `src/cat.rs`, and `src/fcntl.rs` are consistent. Depends on: T005, T006.

## Final Phase: Polish

- [T008] [Story] Refine the `main_root_stat_03` Rust port for compile cleanliness and minimal idiomatic improvements in `src/main.rs`, `src/cat.rs`, and `src/fcntl.rs`, without changing evidenced behavior or adding new module scope. Depends on: T007.