# Tasks: module_src_collect_data_10

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/symbol.c` port on branch `073-module_src_collect_data_10-rust-port`, adding the target Rust source file at `src/symbol.rs` and wiring it into the crate module tree where the C module is being migrated.
- [T002] [P] [Story] Define the migration surface in `src/symbol.rs` by listing the C-backed data structures and the 2 function entry points as Rust items/placeholders so later tasks can fill them in without changing file boundaries.

## Phase 2: Foundational

- [T003] [Story] Port the foundational type definitions from `src/symbol.c` into Rust in `src/symbol.rs`, introducing Rust `struct`, `enum`, `type`, and constant definitions for the module-owned data structures first, with field layouts and visibility aligned to actual module use. Depends on: T001, T002
- [T004] [P] [Story] Implement shared constructors, default state initialization, and helper methods required by the ported data structures in `src/symbol.rs`, limited to helpers directly evidenced by `src/symbol.c`. Depends on: T003
- [T005] [Story] Resolve ownership/borrowing representation for the ported `src/symbol.c` data graph inside `src/symbol.rs`, replacing C pointer/state relationships with Rust-safe internal references or containers only where required by the module’s 27 data structures. Depends on: T003

## Phase 3: Functions

- [T006] [Story] Implement the first function from `src/symbol.c` in `src/symbol.rs`, covering symbol/data collection behavior that primarily initializes, populates, or updates the module’s core structures. Depends on: T003, T004, T005
- [T007] [Story] Implement the second function from `src/symbol.c` in `src/symbol.rs`, covering the remaining symbol/data collection behavior and reusing the foundational structures established earlier without redefining them. Depends on: T003, T004, T005
- [T008] [Story] Integrate the two ported functions in `src/symbol.rs` by aligning shared helper usage, call ordering, and internal state transitions so the Rust module matches the original `src/symbol.c` functional grouping without duplicate logic. Depends on: T006, T007

## Final Phase: Polish

- [T009] [P] [Story] Refine `src/symbol.rs` for idiomatic Rust within the existing port scope by removing placeholder items, tightening visibility, and simplifying control flow introduced during migration. Depends on: T008
- [T010] [Story] Perform a final module pass on `src/symbol.rs` to verify completeness of the `src/symbol.c` migration for this module, ensuring all evidenced data structures and both functions are represented in their final Rust locations. Depends on: T009