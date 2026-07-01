# Tasks: module_gnu_is_infinite_18

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module scaffold for the `gnu/vasnprintf.c` port in `src/gnu/vasnprintf.rs`, and expose it from the existing Rust module tree used by this branch.
- [ ] T002 [P] [Story] Add the initial item layout in `src/gnu/vasnprintf.rs` for the module-local data structure and the two function signatures required by the `gnu/vasnprintf.c` migration, keeping names and responsibilities aligned with the source module.

## Phase 2: Foundational

- [ ] T003 [Story] Implement the single data structure inferred from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, translating its fields and Rust ownership/borrowing model as the foundation for the function port.
- [ ] T004 [Story] Wire the foundational data structure into the module API in `src/gnu/vasnprintf.rs` so the later function implementations can use it directly without introducing placeholder duplicates. Depends on: T003.

## Phase 3: Floating-point infinity handling functions

- [ ] T005 [Story] Port the first infinity-related function from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, preserving its original role in GNU `vasnprintf` floating-point formatting and adapting its logic to Rust idioms. Depends on: T004.
- [ ] T006 [Story] Port the second infinity-related function from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, keeping behavior aligned with the source implementation and integrating it with the shared module data structure where needed. Depends on: T004.
- [ ] T007 [Story] Integrate both ported functions within `src/gnu/vasnprintf.rs` so their shared assumptions, visibility, and call relationships match the original `gnu/vasnprintf.c` module without duplicating logic. Depends on: T005, T006.

## Final Phase: Polish

- [ ] T008 [Story] Review `src/gnu/vasnprintf.rs` for Rust-level cleanup after the port, removing temporary migration scaffolding, tightening signatures and visibility, and ensuring the translated data structure and infinity-handling functions remain cohesive. Depends on: T007.
- [ ] T009 [P] [Story] Perform a final pass on `src/gnu/vasnprintf.rs` to simplify obvious C-to-Rust mechanical patterns in the completed port while preserving the original `gnu/vasnprintf.c` behavior. Depends on: T008.