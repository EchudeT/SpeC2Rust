# Task List: module_src_parseopt_help_context_05

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the help-context port in `src/parseopt/help.rs`, and expose it from the existing `src/parseopt/mod.rs` if needed for module wiring.
- [T002] [P] [Story] Review `src/parseopt/help.c` and map the 46 C data structures and 2 functions to Rust-owned definitions and implementation sections in `src/parseopt/help.rs`; record the migration inventory as code comments or TODO markers adjacent to the target sections.
- [T003] [Story] Define the Rust module layout inside `src/parseopt/help.rs` for this port, separating data-structure declarations from function implementations to preserve the migration order. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Port the foundational help-context data structures from `src/parseopt/help.c` into Rust type definitions in `src/parseopt/help.rs`, covering the core structs, enums, aliases, and constant-backed layout required before any function implementation. Depends on: T003
- [T005] [P] [Story] Port the remaining support data structures from `src/parseopt/help.c` into Rust in `src/parseopt/help.rs`, including helper records and nested representations used by the help-context logic. Depends on: T003
- [T006] [Story] Reconcile the full set of 46 migrated data structures in `src/parseopt/help.rs`, resolving cross-references, ownership/borrowing choices, and visibility so the module compiles as a complete foundational unit. Depends on: T004, T005

## Phase 3: Functions

- [T007] [Story] Implement the function that builds or initializes parse-option help context behavior from `src/parseopt/help.c` in `src/parseopt/help.rs`, using the completed Rust data structures without changing module scope. Depends on: T006
- [T008] [Story] Implement the function that formats, updates, or emits parse-option help context behavior from `src/parseopt/help.c` in `src/parseopt/help.rs`, preserving the original module-level responsibility and using the same migrated Rust model. Depends on: T006
- [T009] [Story] Integrate the two migrated help-context functions with the Rust module surface in `src/parseopt/help.rs`, ensuring shared internal types and helper usage are consistent and that no function logic remains duplicated or stubbed. Depends on: T007, T008

## Final Phase: Polish

- [T010] [Story] Refine `src/parseopt/help.rs` by removing migration placeholders, tightening type/function visibility, and simplifying obvious C-to-Rust translation artifacts while keeping behavior and file scope aligned with `src/parseopt/help.c`. Depends on: T009
- [T011] [Story] Perform a final compile-focused pass on `src/parseopt/help.rs` and any direct module export in `src/parseopt/mod.rs`, fixing remaining signature mismatches, import issues, and module wiring gaps introduced by the port. Depends on: T010