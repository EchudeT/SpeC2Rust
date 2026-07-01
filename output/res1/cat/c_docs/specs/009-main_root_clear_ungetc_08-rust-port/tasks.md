# Tasks: main_root_clear_ungetc_08

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for this port in `src/fflush.rs`, establishing the target location for logic migrated from `fflush.c`.
- [T002] [Story] Register the new module from `src/lib.rs` or `src/main.rs` (whichever is the project’s existing module root) so `src/fflush.rs` is compiled on branch `009-main_root_clear_ungetc_08-rust-port`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `fflush.c` and define the minimal Rust-side foundational items required by the two migrated functions directly inside `src/fflush.rs`, limited to helper aliases, imports, and internal utility scaffolding evidenced by that file. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Implement the first `fflush.c` function in `src/fflush.rs`, preserving its original responsibility and mapping its control flow to Rust with only the dependencies evidenced by this module. Depends on: T003
- [T005] [P] [Story] Implement the second `fflush.c` function in `src/fflush.rs`, keeping behavior aligned with the C source and confined to the module-local scope established for this port. Depends on: T003
- [T006] [Story] Integrate the two migrated functions within `src/fflush.rs`, resolving any shared helper usage, call ordering, or visibility adjustments required so the module builds cleanly as one coherent Rust unit. Depends on: T004, T005

## Final Phase: Polish

- [T007] [Story] Refine `src/fflush.rs` for idiomatic Rust without changing migrated behavior, removing redundant scaffolding and tightening imports, naming, and internal organization introduced during the port. Depends on: T006
- [T008] [Story] Verify the module compiles cleanly through the project entry registration in `src/lib.rs` or `src/main.rs`, and perform final file-level cleanup for `src/fflush.rs` specific to this migration. Depends on: T007