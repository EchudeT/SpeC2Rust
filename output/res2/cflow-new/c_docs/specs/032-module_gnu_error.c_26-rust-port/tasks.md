# Task List: module_gnu_error.c_26

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/error.c` port in `src/gnu/error.rs`.
- [T002] [P] [Story] Expose the new module from the Rust crate module tree by updating the nearest inferred module entry point to include `src/gnu/error.rs`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Establish the foundational Rust definitions in `src/gnu/error.rs` for the `gnu/error.c` port, including imports, module-level constants, and internal helper declarations required by the module’s 5 functions. Depends on: T001

## Phase 3: Error Reporting Core Functions

- [T004] [Story] Implement the core non-variadic error-reporting function from `gnu/error.c` in `src/gnu/error.rs`, preserving the original module behavior and integration points. Depends on: T003
- [T005] [Story] Implement the variadic-style companion error-reporting function from `gnu/error.c` in `src/gnu/error.rs`, translated into the Rust-appropriate argument handling model used by this port. Depends on: T003
- [T006] [P] [Story] Implement the module function in `src/gnu/error.rs` that prints or formats the program-identification prefix used by the error-reporting flow. Depends on: T003

## Phase 4: Error Output Control Functions

- [T007] [Story] Implement the function from `gnu/error.c` in `src/gnu/error.rs` that emits the finalized error message path, wiring it to the core reporting behavior. Depends on: T004, T005, T006
- [T008] [P] [Story] Implement the remaining support function from `gnu/error.c` in `src/gnu/error.rs` that manages module-level error output state or counters required by the reporting API. Depends on: T003

## Final Phase: Polish

- [T009] [Story] Review `src/gnu/error.rs` for parity with `gnu/error.c`, remove porting scaffolding that is no longer needed, and tighten signatures/imports without changing module behavior. Depends on: T004, T005, T006, T007, T008