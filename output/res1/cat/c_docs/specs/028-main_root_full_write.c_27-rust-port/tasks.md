# Tasks: main_root_full-write.c_27

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/full_write.rs` for the port of `full-write.c` on branch `028-main_root_full_write.c_27-rust-port`.
- [T002] [Story] Expose the new module from `src/lib.rs` or `src/main.rs`, matching the existing crate entry structure so `src/full_write.rs` is compiled. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review the C module surface from `full-write.c` and define the Rust function signature in `src/full_write.rs`, including required standard library imports and error/return types, without introducing new data structures. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Implement the full-write helper function from `full-write.c` in `src/full_write.rs`, preserving its write-all semantics and partial-write handling in idiomatic Rust. Depends on: T003
- [T005] [P] [Story] Wire call visibility and module-level access for the implemented full-write function so it can be used by the rest of the crate through `src/lib.rs` or `src/main.rs`. Depends on: T002, T004

## Final Phase: Polish

- [T006] [Story] Refine `src/full_write.rs` for idiomatic Rust clarity, minimizing unnecessary mutation and aligning error propagation with crate conventions without changing module scope. Depends on: T004
- [T007] [Story] Perform a final integration pass on `src/full_write.rs` and `src/lib.rs` or `src/main.rs` to remove any unused imports or visibility mismatches introduced during the port. Depends on: T005, T006