# Tasks: main_root Rust port

## Phase 1: Setup

- [T001] [Story] Initialize the Rust crate structure for the `main_root` port on branch `001-main_root-rust-port`, creating and wiring `src/main.rs`, `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` to mirror `which.c`, `bash.c`, `getopt.c`, and `getopt1.c`.
- [T002] [P] [Story] Establish module declarations and shared visibility boundaries across `src/main.rs`, `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` so later file migrations can be implemented without circular imports. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the data structures defined and used by `which.c` into Rust-native structs/enums/constants in `src/which.rs`, preserving the state needed by command lookup, option handling, and output flow. Depends on: T002.
- [T004] [P] [Story] Port the data structures defined and used by `bash.c` into Rust-native structs/enums/constants in `src/bash.rs`, preserving shell-related parsing and behavior state required by the module. Depends on: T002.
- [T005] [P] [Story] Port the data structures defined and used by `getopt.c` and `getopt1.c` into Rust-native structs/enums/constants in `src/getopt.rs`, including option descriptor/state representations needed by short and long option parsing. Depends on: T002.
- [T006] [Story] Consolidate shared foundational types and cross-module references so `src/main.rs`, `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` compile together with the migrated data structures and constant definitions. Depends on: T003, T004, T005.

## Phase 3: Option parsing functions

- [T007] [Story] Implement the core short-option parsing functions migrated from `getopt.c` in `src/getopt.rs`, using the Phase 2 option state/data structures as the single parsing foundation. Depends on: T005, T006.
- [T008] [Story] Implement the long-option and extended parsing functions migrated from `getopt1.c` in `src/getopt.rs`, integrating them with the core parser from `getopt.c` without duplicating parsing state. Depends on: T007.
- [T009] [Story] Connect option parsing entry points from `src/which.rs` and `src/main.rs` to the migrated `src/getopt.rs` functionality so command-line arguments flow through the Rust parser consistently with the C module layout. Depends on: T008.

## Phase 4: Shell and path behavior functions

- [T010] [Story] Implement the shell-related helper functions migrated from `bash.c` in `src/bash.rs`, grouping parsing, quoting, and shell behavior helpers together around the already-ported shell data structures. Depends on: T004, T006.
- [T011] [P] [Story] Implement the path-search and command resolution helper functions migrated from `which.c` in `src/which.rs`, grouping filesystem lookup and candidate resolution behavior together around the ported `which.c` state. Depends on: T003, T006.
- [T012] [Story] Integrate shell behavior from `src/bash.rs` into command resolution in `src/which.rs` where the original C module couples shell semantics to lookup/output decisions. Depends on: T010, T011.

## Phase 5: Main command flow functions

- [T013] [Story] Implement the remaining top-level execution and control-flow functions from `which.c` in `src/which.rs`, including argument processing, dispatch, and result reporting built on the migrated option parsing and command resolution helpers. Depends on: T009, T012.
- [T014] [Story] Implement the `main` entry point in `src/main.rs` to delegate to the Rust port of the `which.c` program flow and return process status consistent with the migrated module behavior. Depends on: T013.

## Final Phase: Polish

- [T015] [Story] Refine the Rust port across `src/main.rs`, `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` by removing migration scaffolding, tightening module interfaces, and resolving compile-time warnings introduced during the file-by-file port. Depends on: T014.
- [T016] [Story] Perform a final pass on the migrated main-module files `src/main.rs`, `src/which.rs`, `src/bash.rs`, and `src/getopt.rs` to simplify duplicated logic created during porting and ensure each C source file’s responsibilities remain cleanly mapped in Rust. Depends on: T015.