# Tasks: `main_root_fadvise.c_22`

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `fadvise.c` port on branch `023-main_root_fadvise.c_22-rust-port`, adding the target source file at `src/fadvise.rs` and wiring the module into the existing crate entry points where `cat` main-cluster modules are registered.
  - Depends on: none

## Phase 2: Foundational

- [T002] [Story] Define the foundational Rust items needed by the `fadvise.c` port in `src/fadvise.rs`, including any module-local constants, type aliases, and helper signatures directly required to express the two C functions without introducing unrelated abstractions.
  - Depends on: [T001]

## Phase 3: Fadvise Function Port

- [T003] [P] [Story] Port the lower-level file-advice operation from `fadvise.c` into `src/fadvise.rs`, translating the direct advisory call logic and preserving the original function’s return/error behavior as used by the module.
  - Depends on: [T002]

- [T004] [Story] Port the higher-level `fadvise.c` entry function into `src/fadvise.rs`, connecting it to the lower-level advisory helper and preserving the original module control flow for `cat`.
  - Depends on: [T003]

## Final Phase: Polish

- [T005] [Story] Refine `src/fadvise.rs` for integration quality by removing porting scaffolding, aligning naming and visibility with the crate’s existing main-cluster conventions, and verifying that the two migrated functions are contained cleanly within the module without duplicate logic.
  - Depends on: [T004]