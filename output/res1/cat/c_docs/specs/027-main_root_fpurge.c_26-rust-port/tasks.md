# Tasks: main_root_fpurge.c_26

## Phase 1: Setup

- [T001] [Story] Initialize the Rust port workspace on branch `027-main_root_fpurge.c_26-rust-port` and create the target module file `src/fpurge.rs` corresponding to `fpurge.c`.
- [T002] [Story] Wire the new module into the existing crate entry points by declaring `src/fpurge.rs` from the nearest applicable root module file so the ported implementation is build-visible. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `fpurge.c` and define the minimal Rust-side module API surface in `src/fpurge.rs`, including any internal type aliases or signatures needed to support the ported function, without introducing unevidenced new data structures. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Port the function implemented in `fpurge.c` into `src/fpurge.rs`, preserving the original module behavior and limiting the implementation scope to the single evidenced function in this module. Depends on: T002, T003
- [T005] [P] [Story] Update any direct call sites or module references affected by the `fpurge.c` migration so they use the Rust implementation exposed from `src/fpurge.rs`. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/fpurge.rs` for idiomatic Rust within the bounds of the original C module behavior, removing migration-only scaffolding and resolving integration issues introduced by the port. Depends on: T004, T005