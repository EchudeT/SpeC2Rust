# Tasks: main_root_progname.c_21

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/progname.rs` to host the port of `progname.c`.
- [T002] [Story] Register the new module in `src/main.rs` or `src/lib.rs` by adding the `progname` module declaration and wiring it into the current crate structure. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `progname.c` usage and define the minimal Rust-visible constants, static state, or helper signatures needed for the single-function port inside `src/progname.rs`. Depends on: T001

## Phase 3: Function Port

- [T004] [Story] Implement the function from `progname.c` in `src/progname.rs`, preserving its main-cluster behavior and progname-related semantics in Rust. Depends on: T003
- [T005] [P] [Story] Update the call site in `src/main.rs` or `src/lib.rs` to use the Rust implementation from `src/progname.rs` in place of the C-module behavior. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/progname.rs` and the integration point in `src/main.rs` or `src/lib.rs` by removing temporary porting scaffolding, tightening visibility, and ensuring naming aligns with the `progname.c` migration. Depends on: T005