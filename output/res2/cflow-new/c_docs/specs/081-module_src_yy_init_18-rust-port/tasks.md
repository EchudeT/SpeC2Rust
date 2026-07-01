# Tasks: module_src_yy_init_18

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module skeleton for the `src/c.c` migration in `src/module_src_yy_init_18.rs`, and expose it from the crate root on branch `081-module_src_yy_init_18-rust-port`.
- [ ] T002 [P] [Story] Review `src/c.c` and enumerate the 13 module-local data structures and 2 functions to be ported, then record the implementation mapping as module comments in `src/module_src_yy_init_18.rs`. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Port the first subset of foundational data structures from `src/c.c` into Rust definitions in `src/module_src_yy_init_18.rs`, preserving field layout and ownership semantics required by the module. Depends on: T002
- [ ] T004 [Story] Port the remaining data structures from `src/c.c` into Rust definitions in `src/module_src_yy_init_18.rs`, completing all 13 structures needed by the module. Depends on: T003
- [ ] T005 [Story] Add inherent constructors, default initializers, or helper enums/constants in `src/module_src_yy_init_18.rs` where the C initialization patterns in `src/c.c` require them for the upcoming function ports. Depends on: T004

## Phase 3: Functions

- [ ] T006 [Story] Implement the initialization-oriented function from `src/c.c` in `src/module_src_yy_init_18.rs`, wiring it to the ported data structures and preserving the original setup behavior. Depends on: T005
- [ ] T007 [Story] Implement the remaining companion function from `src/c.c` in `src/module_src_yy_init_18.rs`, completing the functional port for this module and reusing the same Rust data model. Depends on: T006

## Final Phase: Polish

- [ ] T008 [Story] Refine `src/module_src_yy_init_18.rs` by removing C-specific translation artifacts, tightening visibility, and simplifying internal initialization flow without changing module behavior. Depends on: T007
- [ ] T009 [Story] Run a final consistency pass on `src/module_src_yy_init_18.rs` to confirm every structure and both functions from `src/c.c` are represented exactly once and that dependency ordering remains clean. Depends on: T008