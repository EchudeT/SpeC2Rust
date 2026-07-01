# Tasks: main_root Rust port

## Phase 1: Setup

- [T001] [Story] Initialize the Rust entry-point layout for the `main_root` port on branch `001-main_root-rust-port`, creating and wiring `src/main.rs`, `src/bash.rs`, `src/getopt.rs`, `src/getopt1.rs`, and `src/which.rs` to mirror the C module file split.
- [T002] [P] [Story] Add module declarations and cross-module visibility needed by `src/main.rs` to call into logic migrated from `which.c`, with placeholders only for items evidenced by `bash.c`, `getopt.c`, `getopt1.c`, and `which.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Port the data structures defined in `which.c` into Rust types in `src/which.rs`, preserving the main-program state and option/result carrier layouts needed before function migration. Depends on: T002.
- [T004] [P] [Story] Port the data structures defined in `getopt.c` and `getopt1.c` into Rust types, constants, and enums in `src/getopt.rs` and `src/getopt1.rs`, including option-description and parser-state representations required by getopt-style processing. Depends on: T002.
- [T005] [P] [Story] Port the data structures and static state evidenced in `bash.c` into Rust representations in `src/bash.rs`, limited to shell/path/environment support types actually used by the main module flow. Depends on: T002.
- [T006] [Story] Reconcile shared foundational types across `src/which.rs`, `src/getopt.rs`, `src/getopt1.rs`, and `src/bash.rs`, exposing only the interfaces required for later function groups and removing duplicate placeholder definitions. Depends on: T003, T004, T005.

## Phase 3: Option parsing core

- [T007] [Story] Implement the core short-option parsing functions migrated from `getopt.c` in `src/getopt.rs`, grouping the low-level parser routines that advance argv state, decode option characters, and manage parser globals/state. Depends on: T004, T006.
- [T008] [Story] Implement the long-option and extended parsing functions migrated from `getopt1.c` in `src/getopt1.rs`, grouping the routines that interpret long options and integrate with the short-option parser. Depends on: T004, T006, T007.
- [T009] [Story] Connect the parser interfaces from `src/getopt.rs` and `src/getopt1.rs` for consumption by the main program logic in `src/which.rs` and `src/main.rs`, without re-implementing parser behavior elsewhere. Depends on: T007, T008.

## Phase 4: Shell and path support

- [T010] [Story] Implement the shell- and environment-support functions migrated from `bash.c` in `src/bash.rs`, grouped around command-path interpretation, shell-style path handling, and environment lookups evidenced as dependencies of the `which` program. Depends on: T005, T006.
- [T011] [P] [Story] Expose the `bash.c`-derived helpers from `src/bash.rs` to `src/which.rs` through a minimal Rust API that preserves the original module boundary and avoids duplicating path logic in the main module. Depends on: T010.

## Phase 5: Main program behavior

- [T012] [Story] Implement the argument-processing and option-dispatch functions migrated from `which.c` in `src/which.rs`, grouping the routines that interpret parsed options, manage program mode/state, and prepare command lookup requests. Depends on: T003, T009, T011.
- [T013] [Story] Implement the command-search and resolution functions migrated from `which.c` in `src/which.rs`, grouping the routines that inspect PATH-related inputs, evaluate matches, and produce lookup outcomes using helpers from `src/bash.rs`. Depends on: T012.
- [T014] [Story] Implement the output, reporting, and exit-status functions migrated from `which.c` in `src/which.rs`, grouping the routines that format results, handle not-found/reporting branches, and finalize the program result. Depends on: T013.
- [T015] [Story] Implement the top-level `which.c` entry flow in Rust by wiring `src/main.rs` to the migrated coordinator function(s) in `src/which.rs`, replacing setup placeholders with the real main-program execution path. Depends on: T012, T013, T014.

## Final Phase: Polish

- [T016] [P] [Story] Refine the Rust port in `src/bash.rs`, `src/getopt.rs`, `src/getopt1.rs`, `src/which.rs`, and `src/main.rs` by removing temporary migration scaffolding, tightening signatures and ownership/borrowing, and ensuring module boundaries stay aligned with the original C file split. Depends on: T015.
- [T017] [Story] Perform final pass cleanup across `src/main.rs`, `src/bash.rs`, `src/getopt.rs`, `src/getopt1.rs`, and `src/which.rs` to resolve remaining integration issues, eliminate duplicated migration code, and ensure the `main_root` module compiles as a coherent Rust main program. Depends on: T016.