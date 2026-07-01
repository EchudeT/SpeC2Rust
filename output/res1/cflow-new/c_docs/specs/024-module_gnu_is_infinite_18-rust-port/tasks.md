# Tasks: module_gnu_is_infinite_18

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/vasnprintf.c` migration in `src/gnu/vasnprintf.rs`, defining the module entry points needed for `module_gnu_is_infinite_18`.
- [T002] [P] [Story] Wire the new module into the crate module tree by updating the nearest Rust module declaration file for `src/gnu/vasnprintf.rs` so the migrated implementation is compiled.
- [T003] [Story] Document the C-to-Rust migration boundary for `gnu/vasnprintf.c` inside `src/gnu/vasnprintf.rs`, listing the 1 data structure and 2 functions to be ported as implementation placeholders.

## Phase 2: Foundational

- [T004] [Story] Implement the module’s single foundational data structure in `src/gnu/vasnprintf.rs`, preserving the role and field layout required by the `gnu/vasnprintf.c` logic.
- [T005] [Story] Refine constructor/helper methods directly associated with the migrated data structure in `src/gnu/vasnprintf.rs` so the later function ports can use it without adding duplicate state handling. Depends on: T004

## Phase 3: Floating-point infinity handling functions

- [T006] [Story] Port the first `gnu_is_infinite`-related function from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, adapting its control flow to idiomatic Rust while keeping behavior aligned with the original module data structure. Depends on: T004, T005
- [T007] [P] [Story] Port the second `gnu_is_infinite`-related function from `gnu/vasnprintf.c` into `src/gnu/vasnprintf.rs`, grouping it with the same floating-point classification behavior and reusing the shared foundational data structure. Depends on: T004, T005
- [T008] [Story] Integrate the two migrated infinity-handling functions within `src/gnu/vasnprintf.rs` so shared constants, internal visibility, and call relationships match the original `gnu/vasnprintf.c` module. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Review `src/gnu/vasnprintf.rs` for duplication introduced during the port and consolidate local helpers, pattern matches, and naming so the final Rust module remains minimal and faithful to `gnu/vasnprintf.c`. Depends on: T008
- [T010] [Story] Perform a final compile-focused cleanup in `src/gnu/vasnprintf.rs` and the corresponding module declaration file, removing unused placeholders from setup while keeping only the migrated data structure and 2 functions required for `module_gnu_is_infinite_18`. Depends on: T009