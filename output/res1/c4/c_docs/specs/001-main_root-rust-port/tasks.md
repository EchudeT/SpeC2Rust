# Tasks: main_root Rust Port

**Input module:** `main_root`
**Category:** `main`
**Branch:** `001-main_root-rust-port`

## Phase 1: Setup

- [ ] T001 [Story] Initialize the Rust entry-point migration for this module by creating `src/main.rs` and mapping C module responsibilities from `c4.c` and `hello.c` into the Rust binary layout.
- [ ] T002 [P] [Story] Create Rust source placeholders in `src/main.rs` for the function groups inferred from `c4.c` and `hello.c`, keeping function stubs organized by source-file origin. Depends on: T001

## Phase 2: Foundational

- [ ] T003 [Story] Define the foundational module-level constants, type aliases, and shared state in `src/main.rs` that are required before porting functions from `c4.c` and `hello.c`. Depends on: T002
- [ ] T004 [P] [Story] Establish the core execution flow scaffold in `src/main.rs` so later function ports can be attached without changing the file structure again. Depends on: T003

## Phase 3: Core main module functions

- [ ] T005 [Story] Port the primary program-entry and top-level control functions from `c4.c` into `src/main.rs`, preserving the original main-module execution order. Depends on: T004
- [ ] T006 [Story] Port the remaining support functions from `c4.c` into `src/main.rs`, grouped with the entry/control logic they directly assist and without duplicating implementation work. Depends on: T005

## Phase 4: hello module functions

- [ ] T007 [P] [Story] Port the `hello.c` function group into `src/main.rs`, keeping the migrated logic isolated in the same Rust target file and wiring it into the main control flow only where required. Depends on: T004
- [ ] T008 [Story] Integrate the `hello.c` ported functions with the `c4.c`-derived execution path in `src/main.rs`, resolving call sites and shared state usage. Depends on: T006, T007

## Final Phase: Polish

- [ ] T009 [Story] Refine `src/main.rs` to remove redundant stubs, align naming and signatures across the migrated functions, and ensure the final file cleanly represents the combined port of `c4.c` and `hello.c`. Depends on: T008
- [ ] T010 [Story] Perform a final pass on `src/main.rs` for Rust compile-readiness improvements tied directly to the migrated main-module logic, including small ownership and mutability adjustments needed by the completed port. Depends on: T009