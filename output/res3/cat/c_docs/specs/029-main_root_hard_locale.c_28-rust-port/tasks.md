# Tasks: main_root_hard-locale.c_28

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/hard_locale.rs` and declare it from the crate entry point used by the `cat` project so the port of `hard-locale.c` has a dedicated target file.
- [T002] [Story] Establish the module skeleton in `src/hard_locale.rs` for the `hard-locale.c` migration, including public/private item visibility placeholders needed for the module function implementation. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `hard-locale.c` and map its module-level constants, imports, and helper declarations into Rust equivalents inside `src/hard_locale.rs`, keeping scope limited to items directly required by the module function. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the `hard-locale.c` exported function logic in `src/hard_locale.rs`, preserving the original behavior and adapting any locale-related system/library interactions to idiomatic Rust within the module boundary. Depends on: T003
- [T005] [P] [Story] Integrate the new `src/hard_locale.rs` function into the existing main-cluster call sites or module wiring already present in the Rust `cat` crate entry path, ensuring the ported function is reachable from the application flow. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/hard_locale.rs` for parity and maintainability by removing migration placeholders, tightening signatures/imports, and verifying that the module contains only the items required by `hard-locale.c`. Depends on: T005