# Tasks: module_src_parseopt_wordwrap.c_14

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the ported word-wrap implementation in `src/parseopt/wordwrap.rs`, and expose it from the existing `src/parseopt/mod.rs` module tree on branch `111-module_src_parseopt_wordwrap.c_14-rust-port`.
- [T002] [P] [Story] Review `src/parseopt/wordwrap.c` and map its 4 functions and 18 data-structure usages into Rust module-level placeholders and comments in `src/parseopt/wordwrap.rs` to keep the migration aligned to the source file.
- [T003] [Story] Define the Rust-facing module boundaries in `src/parseopt/wordwrap.rs`, including internal visibility, imports, and placeholders for state/data types needed by the C file port. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Implement the foundational Rust data structures inferred from `src/parseopt/wordwrap.c` in `src/parseopt/wordwrap.rs`, covering the module-local state, option/word metadata, wrap configuration, and transient formatting context required before function porting. Depends on: T003.
- [T005] [P] [Story] Add Rust enums, constants, and type aliases in `src/parseopt/wordwrap.rs` for the control values and mode/state markers used by the C implementation so function logic can be translated without ad hoc primitives. Depends on: T003.
- [T006] [Story] Wire the foundational structures together in `src/parseopt/wordwrap.rs` by adding constructors/default initializers and internal helper methods strictly required to represent the C module’s data flow. Depends on: T004, T005.

## Phase 3: Core formatting and wrapping functions

- [T007] [Story] Port the low-level text accumulation and width/accounting function group from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, using the Phase 2 structures to preserve the original wrap-state transitions. Depends on: T006.
- [T008] [Story] Port the line-break and wrap-decision function group from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, keeping the logic colocated with the core formatting state rather than duplicating decisions across helpers. Depends on: T006.
- [T009] [Story] Reconcile shared state handling between the accumulation and wrap-decision groups in `src/parseopt/wordwrap.rs` so the translated functions operate over one consistent internal model. Depends on: T007, T008.

## Phase 4: Public/module entry functions

- [T010] [Story] Port the remaining higher-level entry-point function group from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, connecting input processing to the lower-level wrapping helpers without re-splitting already migrated function logic. Depends on: T009.
- [T011] [P] [Story] Port any module-scope initialization/finalization or reset-style function from `src/parseopt/wordwrap.c` into `src/parseopt/wordwrap.rs`, if present among the 4 source functions, and bind it to the Rust data-structure lifecycle. Depends on: T006.
- [T012] [Story] Integrate all 4 migrated functions in `src/parseopt/wordwrap.rs` so the Rust module matches the original `src/parseopt/wordwrap.c` responsibilities and call relationships. Depends on: T010, T011.

## Final Phase: Polish

- [T013] [Story] Refine `src/parseopt/wordwrap.rs` by removing migration placeholders, collapsing redundant intermediate state introduced during porting, and ensuring the final Rust implementation stays faithful to `src/parseopt/wordwrap.c`. Depends on: T012.
- [T014] [Story] Perform a final module-level cleanup of `src/parseopt/wordwrap.rs` and `src/parseopt/mod.rs`, confirming visibility, naming, and file organization are consistent with the Rust project structure for this single-file migration. Depends on: T013.