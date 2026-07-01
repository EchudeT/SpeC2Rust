# Tasks: module_src_parseopt_optset.c_12

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/parseopt/optset.c` port on branch `109-module_src_parseopt_optset.c_12-rust-port`, adding the target source file at `src/parseopt/optset.rs`.
- [T002] [P] [Story] Register the new Rust module so `src/parseopt/optset.rs` is compiled from the existing parseopt module tree. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and port the 14 data structures evidenced in `src/parseopt/optset.c` into Rust definitions in `src/parseopt/optset.rs`, preserving the module-local ownership and layout relationships needed by the two module functions. Depends on: T001.
- [T004] [P] [Story] Define the Rust enums, structs, type aliases, and constant representations required by the option-set domain model in `src/parseopt/optset.rs`, keeping names and field roles aligned with the C module semantics. Depends on: T003.
- [T005] [P] [Story] Implement constructors, default state helpers, and internal conversion helpers needed to initialize and connect the ported option-set data structures in `src/parseopt/optset.rs`. Depends on: T004.
- [T006] [Story] Resolve ownership, borrowing, and collection representation for linked or grouped option-set data carried by the ported structures in `src/parseopt/optset.rs`, so the later function ports can operate without placeholder types. Depends on: T005.

## Phase 3: Option-set Function Port

- [T007] [Story] Port the first `src/parseopt/optset.c` function into `src/parseopt/optset.rs`, implementing its option-set creation or initialization behavior against the completed Rust data structures. Depends on: T006.
- [T008] [Story] Port the second `src/parseopt/optset.c` function into `src/parseopt/optset.rs`, implementing its option-set update, lookup, or lifecycle behavior without duplicating logic already introduced in T007. Depends on: T006.
- [T009] [Story] Refactor shared internal logic used by both ported functions into private helpers within `src/parseopt/optset.rs`, keeping each original C function mapped once and only once. Depends on: T007, T008.

## Final Phase: Polish

- [T010] [Story] Review `src/parseopt/optset.rs` for idiomatic Rust cleanup, remove temporary migration scaffolding, and tighten visibility and type usage while preserving the behavior of the two ported functions. Depends on: T009.
- [T011] [Story] Perform a final module integration pass for `src/parseopt/optset.rs`, ensuring imports, module wiring, and compile-facing interfaces are consistent with the parseopt subtree. Depends on: T010.