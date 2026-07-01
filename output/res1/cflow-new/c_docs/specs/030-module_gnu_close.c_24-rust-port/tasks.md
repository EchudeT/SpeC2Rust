# Tasks: module_gnu_close.c_24

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/gnu/close.rs` for the port of `gnu/close.c`.
- [T002] [Story] Register the new module in the Rust module tree by updating the nearest inferable module declarations so `src/gnu/close.rs` is compiled. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/close.c` and define the direct Rust-level constants, type aliases, and helper signatures required inside `src/gnu/close.rs` to support the two exported function ports without introducing extra abstractions. Depends on: T001.

## Phase 3: Close wrappers

- [T004] [Story] Port the first function from `gnu/close.c` into `src/gnu/close.rs`, preserving its close-related control flow and error-handling semantics. Depends on: T003.
- [T005] [P] [Story] Port the second function from `gnu/close.c` into `src/gnu/close.rs`, preserving its close-related control flow and error-handling semantics and sharing only the helpers established in Phase 2. Depends on: T003.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/close.rs` to remove duplication between the two ported functions, align naming and visibility with the surrounding Rust module layout, and verify the file remains a faithful migration of `gnu/close.c`. Depends on: T004, T005.