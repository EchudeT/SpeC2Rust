# Tasks: module_src_output.c_28

## Phase 1: Setup

- [T001] [Story] Create the Rust port target for `src/output.c` on branch `091-module_src_output.c_28-rust-port` by adding the module file `src/output.rs` and wiring it into the crate module tree from the existing `src` entrypoint.
- [T002] [P] [Story] Establish the initial `src/output.rs` skeleton with placeholders for the 10 data structures and 2 function ports from `src/output.c`, preserving the C module scope and migration boundary.
- [T003] [Story] Review `src/output.c` and map its exported/internal items into the Rust module layout in `src/output.rs`, documenting which data structures and functions will be ported in this module file before implementation. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Implement the first group of foundational Rust data structures in `src/output.rs` for the direct C type equivalents that are required to express the module’s output state and configuration. Depends on: T003
- [T005] [P] [Story] Implement the second group of foundational Rust data structures in `src/output.rs` for helper/container types used by the module’s output routines, keeping field names and semantics aligned with `src/output.c`. Depends on: T003
- [T006] [Story] Implement the remaining Rust data structures in `src/output.rs` to complete all 10 C-to-Rust type migrations for `src/output.c`, including any enums, structs, or aliases evidenced by the source module. Depends on: T004, T005
- [T007] [Story] Refine the data structure definitions in `src/output.rs` so shared fields, ownership choices, and visibility match actual use by the two ported functions without introducing module-external concerns. Depends on: T006

## Phase 3: Function Implementation

- [T008] [Story] Implement the first output-related function from `src/output.c` in `src/output.rs`, using the completed Rust data structures and keeping behavior scoped to the original module logic. Depends on: T007
- [T009] [Story] Implement the second output-related function from `src/output.c` in `src/output.rs`, completing the function port for this module and reusing the same foundational types where applicable. Depends on: T007
- [T010] [Story] Reconcile the interaction between the two ported functions in `src/output.rs`, ensuring call flow, shared state access, and argument/return conventions remain consistent with the original C module. Depends on: T008, T009

## Final Phase: Polish

- [T011] [Story] Perform a module-level cleanup pass on `src/output.rs` to remove porting placeholders, tighten signatures and visibility, and keep the Rust code idiomatic without changing established behavior. Depends on: T010
- [T012] [Story] Run a final compile-oriented refinement for `src/output.rs`, resolving Rust ownership/borrowing issues and simplifying local implementation details needed to complete the `src/output.c` migration. Depends on: T011