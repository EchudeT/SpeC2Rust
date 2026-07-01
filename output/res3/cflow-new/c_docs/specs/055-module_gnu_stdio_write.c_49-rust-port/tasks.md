# Tasks: Port `gnu/stdio-write.c` to Rust

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for this port at `src/module_gnu_stdio_write_c_49.rs`, and register it from the crate entry point so the module is compiled on branch `055-module_gnu_stdio_write.c_49-rust-port`.
- [T002] [P] [Story] Establish the file-level port scaffold in `src/module_gnu_stdio_write_c_49.rs`, including module documentation referencing `gnu/stdio-write.c`, imports, internal visibility boundaries, and placeholders for the 9 function ports. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Analyze `gnu/stdio-write.c` and define the foundational Rust aliases, constants, and helper signatures directly required by the function ports inside `src/module_gnu_stdio_write_c_49.rs`; keep this limited to items evidenced by the source file. Depends on: T002.
- [T004] [Story] Implement shared low-level write/stdout-stderr helper logic in `src/module_gnu_stdio_write_c_49.rs` that multiple ported functions can call, preserving the control-flow structure of `gnu/stdio-write.c` without introducing unevidenced abstractions. Depends on: T003.

## Phase 3: Core write path functions

- [T005] [Story] Port the primary stdio write-path function group from `gnu/stdio-write.c` into `src/module_gnu_stdio_write_c_49.rs`, covering the central buffered/unbuffered write flow and direct call relationships within that core path. Depends on: T004.
- [T006] [P] [Story] Port the companion status/error-propagation helper functions used by the core write path into `src/module_gnu_stdio_write_c_49.rs`, keeping behavior aligned with the C source and avoiding duplication of logic already implemented in T005. Depends on: T004.
- [T007] [Story] Integrate the functions from T005 and T006 so their shared state transitions, return values, and helper usage match the original `gnu/stdio-write.c` behavior. Depends on: T005, T006.

## Phase 4: Remaining support functions

- [T008] [P] [Story] Port the remaining support/helper function group from `gnu/stdio-write.c` into `src/module_gnu_stdio_write_c_49.rs`, assigning each of the still-unported functions exactly once and keeping them near the original file organization. Depends on: T007.
- [T009] [Story] Resolve any cross-calls among all 9 ported functions in `src/module_gnu_stdio_write_c_49.rs`, ensuring signatures, visibility, and helper reuse are consistent with the original module boundaries. Depends on: T008.

## Final Phase: Polish

- [T010] [Story] Refine `src/module_gnu_stdio_write_c_49.rs` to remove placeholder code, tighten control-flow and naming where needed for idiomatic Rust, and preserve the semantics of `gnu/stdio-write.c` without expanding scope. Depends on: T009.
- [T011] [Story] Perform a final compile-focused pass for the module registration and implementation files touched by this port, resolving warnings or mismatches caused by the `gnu/stdio-write.c` migration only. Depends on: T010.