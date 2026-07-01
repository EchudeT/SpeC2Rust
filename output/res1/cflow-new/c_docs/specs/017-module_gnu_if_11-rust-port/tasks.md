# Tasks: module_gnu_if_11

## Phase 1: Setup

- [T001] [Story] Create the module scaffold for the `gnu/vasnprintf.c` port on branch `017-module_gnu_if_11-rust-port`, adding the Rust target file at `src/gnu/vasnprintf.rs` and wiring it into the existing `src/gnu/mod.rs` or nearest inferable module entry point.
- [T002] [P] [Story] Define the Rust-facing migration scope for `src/gnu/vasnprintf.rs` by mapping the C module contents from `gnu/vasnprintf.c` into one module-level implementation plan covering the single data structure and the two functions. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the module’s foundational data structure from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, translating its fields and ownership model into idiomatic Rust types while preserving the C module’s role boundaries. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the core formatting/allocation function group from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, using the Phase 2 data structure where required and keeping behavior aligned with the original module contract. Depends on: T003
- [T005] [Story] Implement the remaining helper or entry function from `gnu/vasnprintf.c` in `src/gnu/vasnprintf.rs`, completing the two-function migration for this module and integrating it with the core function logic without duplicating responsibilities. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/vasnprintf.rs` for Rust-module completeness by removing migration scaffolding, tightening signatures and visibility to module needs, and ensuring the final structure remains limited to the ported data structure and two functions. Depends on: T005