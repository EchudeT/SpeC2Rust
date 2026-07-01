# Tasks: main_root_propername-lite.c_31

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/propername_lite.rs` to host the port of `propername-lite.c`.
- [T002] [Story] Wire the new module into the crate from `src/lib.rs` or `src/main.rs`, whichever already owns main-cluster module declarations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `propername-lite.c` and define any module-local Rust type aliases, constants, or helper signatures directly required by the port in `src/propername_lite.rs`.
- [T004] [P] [Story] Add the minimal module-level documentation comments in `src/propername_lite.rs` describing the scope of the `propername-lite.c` migration.

## Phase 3: Functions

- [T005] [Story] Port the single function from `propername-lite.c` into idiomatic Rust in `src/propername_lite.rs`, preserving the C module behavior and keeping the implementation scoped to this module. Depends on: T002, T003.
- [T006] [Story] Resolve the Rust-facing function visibility and signature at the call boundary in `src/propername_lite.rs` and the owning crate entry file (`src/lib.rs` or `src/main.rs`) so the migrated function is reachable where the C module was used. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/propername_lite.rs` to remove migration-only scaffolding, simplify obvious C-to-Rust control-flow artifacts, and ensure the final code matches existing project style. Depends on: T006.
- [T008] [P] [Story] Run a final compile-oriented pass over `src/propername_lite.rs` and the owning crate entry file (`src/lib.rs` or `src/main.rs`) to fix warnings introduced by this module migration without expanding scope beyond `propername-lite.c`. Depends on: T007.