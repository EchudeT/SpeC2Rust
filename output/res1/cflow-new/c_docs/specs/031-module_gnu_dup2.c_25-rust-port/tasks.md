# Tasks: module_gnu_dup2.c_25

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/dup2.c` port on branch `031-module_gnu_dup2.c_25-rust-port`, adding the target source file `src/gnu/dup2.rs` and exposing it from the existing `src/gnu/mod.rs`.
- [T002] [P] [Story] Add the initial item declarations in `src/gnu/dup2.rs` for the module-local data structure and the four function signatures identified from `gnu/dup2.c`, preserving a migration-oriented layout for later implementation. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the single module-local data structure required by `gnu/dup2.c` in `src/gnu/dup2.rs`, translating its C layout into an idiomatic Rust representation only to the extent required by the module functions. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the core file-descriptor duplication logic group in `src/gnu/dup2.rs`, covering the primary `dup2` replacement behavior and any tightly coupled helper logic from `gnu/dup2.c` that directly supports descriptor validation and duplication semantics. Depends on: T003
- [T005] [Story] Implement the fallback and error-handling function group in `src/gnu/dup2.rs`, covering the remaining functions from `gnu/dup2.c` that complete the module’s alternative execution paths and errno-compatible failure behavior. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/dup2.rs` for parity with `gnu/dup2.c` by removing migration scaffolding, tightening visibility, and ensuring the final function/data-structure organization matches the completed Rust port without altering module scope. Depends on: T005