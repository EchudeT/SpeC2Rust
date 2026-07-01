# Tasks: module_gnu_stat-w32.c_46

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/stat-w32.c` port on branch `052-module_gnu_stat_w32.c_46-rust-port`, adding a dedicated module file at `src/gnu/stat_w32.rs`.
- [T002] [Story] Register the new module in the Rust crate module tree so `src/gnu/stat_w32.rs` is compiled and reachable from the existing `src/gnu/mod.rs`.
- [T003] [P] [Story] Add placeholder item declarations in `src/gnu/stat_w32.rs` for the 3 translated data structures and the 1 function to establish the migration surface before implementation. Depends on: T001

## Phase 2: Foundational

- [T004] [Story] Implement the first translated data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, preserving field layout and Rust-side ownership/borrowing semantics required by the module logic. Depends on: T003
- [T005] [P] [Story] Implement the second translated data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, keeping its representation aligned with the source module’s usage. Depends on: T003
- [T006] [P] [Story] Implement the third translated data structure from `gnu/stat-w32.c` in `src/gnu/stat_w32.rs`, including any enum/flag-style modeling directly evidenced by the source module. Depends on: T003
- [T007] [Story] Reconcile the three translated data structures in `src/gnu/stat_w32.rs` so shared field types, visibility, and module-local helper constants are consistent with the original `gnu/stat-w32.c` interactions. Depends on: T004, T005, T006

## Phase 3: Functions

- [T008] [Story] Port the module’s single function from `gnu/stat-w32.c` into `src/gnu/stat_w32.rs`, wiring it to the translated data structures and preserving the original control flow and return behavior. Depends on: T007
- [T009] [Story] Refine the function implementation in `src/gnu/stat_w32.rs` to replace placeholder translations with idiomatic Rust error/value handling where directly supported by the source module behavior, without expanding module scope. Depends on: T008

## Final Phase: Polish

- [T010] [Story] Review `src/gnu/stat_w32.rs` for naming consistency, removal of scaffolding placeholders, and local documentation comments that clarify the correspondence to `gnu/stat-w32.c`. Depends on: T009
- [T011] [Story] Perform a final compile-focused cleanup for the migrated module by resolving unused items introduced during porting and ensuring the registered module path in `src/gnu/mod.rs` remains minimal and consistent. Depends on: T010