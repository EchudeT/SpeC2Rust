# Task List: module_gnu_cloexec.c_23

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/cloexec.rs` and register it from the existing parent module declaration so the port of `gnu/cloexec.c` has a direct target location.
- [T002] [P] [Story] Review `gnu/cloexec.c` and map its 2 exported/internal functions to Rust function stubs in `src/gnu/cloexec.rs`, preserving function-level scope and signatures as closely as Rust allows.
- [T003] [Story] Verify the branch `029-module_gnu_cloexec.c_23-rust-port` builds with the new module wiring in place before function implementation begins. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Establish any module-local constants, imports, and helper type aliases directly required by the `gnu/cloexec.c` function bodies inside `src/gnu/cloexec.rs`, limiting foundational work to items evidenced by that source file. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the close-on-exec descriptor handling function from `gnu/cloexec.c` in `src/gnu/cloexec.rs`, translating the original control flow and system-call interaction without expanding behavior beyond the C module. Depends on: T004.
- [T006] [Story] Implement the remaining `gnu/cloexec.c` function in `src/gnu/cloexec.rs`, grouping it with the same close-on-exec functionality and reusing shared module-local foundations where applicable. Depends on: T004.

## Final Phase: Polish

- [T007] [P] [Story] Refine `src/gnu/cloexec.rs` to remove porting scaffolding, consolidate duplicated logic between the 2 implemented functions, and align naming and visibility with the surrounding Rust module structure. Depends on: T005, T006.
- [T008] [Story] Run a final module-level build verification for the `gnu/cloexec.c` port and resolve any compile issues introduced in `src/gnu/cloexec.rs`. Depends on: T007.