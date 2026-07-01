# Tasks: module_gnu_stdio-write.c_49 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/stdio-write.c` in `src/gnu/stdio_write.rs`, and expose it from `src/gnu/mod.rs` so the ported implementation has a dedicated target file.
- [T002] [P] [Story] Review `gnu/stdio-write.c` and map its 9 functions into Rust task groups within `src/gnu/stdio_write.rs`, documenting direct function-to-function migration boundaries in code comments or module TODO markers.
- [T003] [Story] Verify the branch-local crate layout can compile with the new module wiring after adding `src/gnu/stdio_write.rs`; depends on: T001.

## Phase 2: Foundational

- [T004] [Story] Establish the foundational Rust imports, internal helper type aliases, and module-local constants needed by the `gnu/stdio-write.c` port inside `src/gnu/stdio_write.rs`, keeping them limited to constructs evidenced by the source file; depends on: T003.
- [T005] [Story] Define the core internal helper routines/signature placeholders that the function groups in `src/gnu/stdio_write.rs` will build on, without implementing any function twice or introducing extra abstractions not evidenced by `gnu/stdio-write.c`; depends on: T004.

## Phase 3: Stream write core functions

- [T006] [Story] Port the primary low-level stdio write path functions from `gnu/stdio-write.c` into `src/gnu/stdio_write.rs`, preserving the original grouped control flow and error propagation behavior; depends on: T005.
- [T007] [P] [Story] Port the closely related buffer advancement and partial-write handling functions from `gnu/stdio-write.c` into `src/gnu/stdio_write.rs`, grouped with the core write path they support; depends on: T005.
- [T008] [Story] Integrate the Phase 3 function group in `src/gnu/stdio_write.rs` so shared helper usage and call ordering match the original module relationships; depends on: T006, T007.

## Phase 4: Flush and completion functions

- [T009] [Story] Port the functions in `gnu/stdio-write.c` responsible for flushing pending output and completing buffered writes into `src/gnu/stdio_write.rs`; depends on: T008.
- [T010] [P] [Story] Port the companion functions in `gnu/stdio-write.c` that update final write state, return values, or end-of-operation status for stdio writes into `src/gnu/stdio_write.rs`; depends on: T008.
- [T011] [Story] Wire the flush/completion function group together with the core write path in `src/gnu/stdio_write.rs`, ensuring each of the 9 source functions is migrated exactly once; depends on: T009, T010.

## Final Phase: Polish

- [T012] [Story] Refine `src/gnu/stdio_write.rs` to remove redundant placeholders, tighten visibility, and align naming/comments with the migrated `gnu/stdio-write.c` behavior now that all function groups are in place; depends on: T011.
- [T013] [Story] Perform a final compile-oriented review of the `src/gnu/stdio_write.rs` and `src/gnu/mod.rs` changes to confirm the module port is self-consistent and limited to the `gnu/stdio-write.c` migration scope; depends on: T012.