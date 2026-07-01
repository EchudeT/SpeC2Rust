# Tasks: `main_root_fadvise.c_22` Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the ported implementation at `src/fadvise.rs`, aligned to the source module `fadvise.c`.
- [T002] [Story] Wire the new module into the crate from the existing Rust entry/module declarations so `src/fadvise.rs` is compiled on branch `023-main_root_fadvise.c_22-rust-port`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `fadvise.c` and define the minimal Rust-side constants, type aliases, and helper signatures needed by its two functions directly in `src/fadvise.rs`. Depends on: T002

## Phase 3: Functions

- [T004] [P] [Story] Implement the advisory helper function from `fadvise.c` in `src/fadvise.rs`, preserving its CLI-facing behavior and return semantics. Depends on: T003
- [T005] [P] [Story] Implement the supporting `fadvise.c` function that performs the underlying advice/dispatch logic in `src/fadvise.rs`, keeping behavior grouped with the module’s advisory helper. Depends on: T003

## Final Phase: Polish

- [T006] [Story] Refine `src/fadvise.rs` by removing migration scaffolding, aligning names and visibility with crate conventions, and ensuring the two ported functions are cleanly integrated without changing behavior. Depends on: T004, T005