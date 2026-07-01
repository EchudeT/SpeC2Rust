# Tasks: module_gnu_itold.c_32

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/itold.c` migration in `src/gnu/itold.rs`.
- [T002] [Story] Expose the migrated module from the Rust module tree by updating the nearest parent module declaration in `src/gnu/mod.rs`. Depends on: T001
- [T003] [P] [Story] Add a placeholder implementation stub for the migrated function in `src/gnu/itold.rs` to establish the target API surface before porting logic. Depends on: T001

## Phase 2: Foundational

- [T004] [Story] Review `gnu/itold.c` and define any module-local Rust aliases, constants, or helper signatures required to support the function port directly in `src/gnu/itold.rs`, keeping them limited to items evidenced by the source migration. Depends on: T003

## Phase 3: Function Port

- [T005] [Story] Port the integer-to-long-double conversion behavior from `gnu/itold.c` into the Rust implementation in `src/gnu/itold.rs`, preserving the original function semantics and module-local logic. Depends on: T004
- [T006] [P] [Story] Refine the function signature and internal numeric conversions in `src/gnu/itold.rs` so they align with the surrounding Rust crate conventions while staying faithful to the original `gnu/itold.c` behavior. Depends on: T005

## Final Phase: Polish

- [T007] [Story] Remove migration placeholders and simplify the final module layout in `src/gnu/itold.rs` so only the completed ported implementation remains. Depends on: T006
- [T008] [Story] Perform a final integration pass on `src/gnu/mod.rs` and `src/gnu/itold.rs` to verify the module is consistently wired into the Rust project branch `038-module_gnu_itold.c_32-rust-port`. Depends on: T002, T007