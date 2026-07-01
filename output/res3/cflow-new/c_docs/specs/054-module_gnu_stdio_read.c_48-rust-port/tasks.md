# Tasks: module_gnu_stdio-read.c_48

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/stdio-read.c` port on branch `054-module_gnu_stdio_read.c_48-rust-port`, adding the target source file at `src/gnu/stdio_read.rs`.
- [T002] [P] [Story] Wire the new module file into the existing Rust crate module tree so `src/gnu/stdio_read.rs` is compiled and reachable from the corresponding `src/gnu/mod.rs`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/stdio-read.c` and define the shared Rust-side internal aliases, constants, and helper signatures needed by its 8 functions directly in `src/gnu/stdio_read.rs`, keeping the surface limited to constructs evidenced by the C file. Depends on: T002.

## Phase 3: Stream state and read-position functions

- [T004] [Story] Port the functions in `gnu/stdio-read.c` that determine whether buffered read data is available and that expose the current read position, implementing them in `src/gnu/stdio_read.rs` using the shared definitions from this module. Depends on: T003.
- [T005] [P] [Story] Port the functions in `gnu/stdio-read.c` that compute the number of pending readable bytes from the stream buffer, implementing them in `src/gnu/stdio_read.rs` alongside the read-position logic where they share buffer access rules. Depends on: T003.

## Phase 4: Read buffer pointer and end-of-buffer functions

- [T006] [Story] Port the functions in `gnu/stdio-read.c` that return or derive the current read buffer base and cursor pointers, preserving the C module’s pointer arithmetic semantics in `src/gnu/stdio_read.rs`. Depends on: T004, T005.
- [T007] [P] [Story] Port the functions in `gnu/stdio-read.c` that expose the read buffer end boundary and related limit calculations in `src/gnu/stdio_read.rs`, grouped with the buffer pointer accessors they rely on. Depends on: T004, T005.

## Phase 5: Read buffer mutation helpers

- [T008] [Story] Port the functions in `gnu/stdio-read.c` that advance, reset, or otherwise mutate the stream’s read buffer cursor or counters, implementing the remaining state-transition behavior in `src/gnu/stdio_read.rs`. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine `src/gnu/stdio_read.rs` to remove redundant temporary logic introduced during porting, align naming and visibility with neighboring Rust GNU stdio modules, and verify all 8 functions from `gnu/stdio-read.c` are implemented exactly once with dependencies resolved. Depends on: T008.