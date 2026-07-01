# Tasks: cflow-new - module_src_parseopt_optset.c_12

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/parseopt/optset.c` in `src/parseopt/optset.rs`, and expose it from the existing parent module so the ported module can compile on branch `109-module_src_parseopt_optset.c_12-rust-port`.
- [T002] [P] [Story] Review `src/parseopt/optset.c` and map its 14 C data structures and 2 functions into a Rust porting checklist documented inline as implementation comments in `src/parseopt/optset.rs` to keep migration scope aligned with the source file. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the Rust equivalents for the core `optset.c` data structures in `src/parseopt/optset.rs`, preserving C layout relationships, ownership expectations, and field semantics required by the module’s functions. Depends on: T002.
- [T004] [P] [Story] Add associated Rust enums, type aliases, and internal helper representations in `src/parseopt/optset.rs` for the remaining `optset.c` data structures that support option-set state, parsing metadata, and internal module bookkeeping. Depends on: T003.
- [T005] [Story] Implement constructors or default initialization paths in `src/parseopt/optset.rs` for the ported optset data structures where the C module relies on zeroed or explicitly initialized state before function execution. Depends on: T003, T004.

## Phase 3: Functions

- [T006] [Story] Implement the first `optset.c` function in `src/parseopt/optset.rs`, wiring it to the ported data structures and preserving the source module’s option-set initialization or mutation behavior. Depends on: T005.
- [T007] [Story] Implement the second `optset.c` function in `src/parseopt/optset.rs`, reusing the shared Rust data structures and preserving the source module’s option-set lookup, update, or traversal semantics as defined in the C module. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/parseopt/optset.rs` to remove migration-only placeholders, tighten visibility to module-local scope where possible, and align naming and control flow with project Rust conventions without changing `optset.c` behavior. Depends on: T007.
- [T009] [P] [Story] Perform a final compile-oriented review of `src/parseopt/optset.rs` and its parent module exports to resolve integration issues introduced by the port of `src/parseopt/optset.c`. Depends on: T008.