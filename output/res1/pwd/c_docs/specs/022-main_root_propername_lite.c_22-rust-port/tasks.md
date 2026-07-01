# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for this migration in `src/propername_lite.rs` and declare it from the crate root used by the branch so `propername-lite.c` logic has a dedicated target location.
- [T002] [P] [Story] Add a migration stub in `src/propername_lite.rs` for the function port from `propername-lite.c`, preserving the C function boundary as a Rust TODO entry for later implementation.

## Phase 2: Foundational

- [T003] [Story] Review `propername-lite.c` and define any module-local type aliases, constants, or helper signatures directly required by the single exported/internal function in `src/propername_lite.rs`; skip this task if the function ports without additional foundational items. Depends on: T001, T002

## Phase 3: Function Implementation

- [T004] [Story] Port the function from `propername-lite.c` into idiomatic Rust in `src/propername_lite.rs`, preserving its current behavior and call shape as closely as practical within the Rust crate. Depends on: T003
- [T005] [P] [Story] Wire the migrated function for use from the crate root/module tree by finalizing the necessary `mod`/`pub` visibility in the directly affected Rust source file(s), limited to enabling access to the implementation in `src/propername_lite.rs`. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/propername_lite.rs` to remove leftover migration placeholders, simplify obvious C-to-Rust translation artifacts, and ensure the module matches existing project style without changing behavior. Depends on: T004, T005