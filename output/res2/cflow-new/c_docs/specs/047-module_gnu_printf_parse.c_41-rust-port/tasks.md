# Tasks: module_gnu_printf-parse.c_41

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/printf-parse.c` in `src/gnu/printf_parse.rs`, and expose it from `src/gnu/mod.rs` so the ported implementation has a dedicated target file.
- [T002] [P] [Story] Review the C source `gnu/printf-parse.c` and map its single exported/internal function into the Rust module boundary in `src/gnu/printf_parse.rs`, documenting the intended function signature and immediate helper needs as code comments before implementation. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Define the foundational Rust types directly required by the parsed printf logic in `src/gnu/printf_parse.rs`, translating only the C-local state, enums, and record shapes evidenced by `gnu/printf-parse.c` so the function implementation has stable in-module representations. Depends on: T002
- [T004] [P] [Story] Add any constant tables, flag masks, or small in-module utility helpers directly evidenced by `gnu/printf-parse.c` into `src/gnu/printf_parse.rs`, keeping them scoped to support the later function port without introducing unrelated abstractions. Depends on: T003

## Phase 3: Function Implementation

- [T005] [Story] Port the module’s single printf parsing function from `gnu/printf-parse.c` into `src/gnu/printf_parse.rs`, implementing the main control flow, format-specifier scanning, and updates to the foundational parse-state types defined earlier. Depends on: T003, T004
- [T006] [Story] Integrate any function-local helper logic from `gnu/printf-parse.c` into `src/gnu/printf_parse.rs` as private Rust helpers only where needed to keep the single ported function readable, without splitting the same C behavior across multiple tasks. Depends on: T005

## Final Phase: Polish

- [T007] [Story] Refine the Rust port in `src/gnu/printf_parse.rs` for idiomatic ownership, error/return handling parity with the C implementation, and removal of redundant temporary state introduced during migration while preserving behavior. Depends on: T006
- [T008] [P] [Story] Clean up module visibility and imports in `src/gnu/printf_parse.rs` and `src/gnu/mod.rs`, ensuring only the ported API surface required by this module remains exposed after migration. Depends on: T007