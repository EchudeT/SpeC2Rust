# Tasks: main_root_fpurge.c_26

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port workspace for this module on branch `027-main_root_fpurge.c_26-rust-port`, and create the target source file `src/fpurge.rs` mapped from `fpurge.c`.
- [T002] [P] [Story] Wire the module into the crate entry points by declaring and exposing `src/fpurge.rs` from the existing Rust project module tree.

## Phase 2: Foundational

- [T003] [Story] Review `fpurge.c` and define the minimal Rust-side foundational items needed by its function implementation directly in `src/fpurge.rs`, avoiding new data structures unless the C source requires them.
- [T004] [Story] Establish any required imports, type aliases, and function signatures in `src/fpurge.rs` so the module can host the ported `fpurge.c` function cleanly. Depends on: T001, T002.

## Phase 3: Functions

- [T005] [Story] Port the single function implemented in `fpurge.c` into `src/fpurge.rs`, preserving the original module behavior and keeping the implementation scoped to the source evidence. Depends on: T003, T004.
- [T006] [P] [Story] Integrate the ported `src/fpurge.rs` function with any existing call sites or exports required by the Rust crate so the module is usable from the main cluster. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/fpurge.rs` for idiomatic Rust within the constraints of the original C behavior, removing unnecessary scaffolding introduced during porting. Depends on: T005.
- [T008] [Story] Perform a final compile-level validation of the `src/fpurge.rs` module and its crate integration, resolving any remaining module-scope issues without expanding functionality. Depends on: T006, T007.