# Tasks: main_root_clear_ungetc_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `fflush.c` migration in `src/fflush.rs`, and expose it from the crate root in `src/lib.rs` or `src/main.rs` according to the existing project layout.
- [T002] [P] [Story] Add placeholder signatures in `src/fflush.rs` for the 2 functions analyzed from `fflush.c`, keeping names and responsibilities aligned with the source module. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `fflush.c` migration needs and define any module-local foundational constants, helper aliases, or internal state handling required by the 2 target functions directly inside `src/fflush.rs`, without introducing unrelated abstractions. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the root clear functionality from `fflush.c` in `src/fflush.rs`, preserving the original module behavior and integrating any required local helpers defined for this module. Depends on: T003
- [T005] [Story] Implement the ungetc-related functionality from `fflush.c` in `src/fflush.rs`, keeping behavior consistent with the original C logic and reusing the same module-local foundations where applicable. Depends on: T003

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/fflush.rs` to remove migration placeholders, resolve any duplicated logic between the 2 implemented functions, and ensure the module is cleanly integrated through `src/lib.rs` or `src/main.rs`. Depends on: T004, T005