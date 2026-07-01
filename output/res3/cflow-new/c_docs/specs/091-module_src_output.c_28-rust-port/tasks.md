# Tasks: module_src_output.c_28 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `src/output.c` in `src/output.rs`, declare it from the crate root or parent module file already governing `src/`, and add placeholder item visibility matching expected module use.
- [T002] [P] [Story] Review `src/output.c` and map the 10 referenced C data structures and 2 functions into a Rust port outline documented inline in `src/output.rs` as implementation placeholders and TODO anchors. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the first group of foundational Rust data structures ported from `src/output.c` in `src/output.rs`, covering core structs/enums/type aliases that other output-related items depend on. Depends on: T002
- [T004] [P] [Story] Implement the remaining independent Rust data structures from `src/output.c` in `src/output.rs`, including field mapping, ownership/borrowing decisions, and visibility needed by the module functions. Depends on: T002
- [T005] [Story] Reconcile the full set of 10 ported data structures in `src/output.rs`, resolving cross-references, constructor/default patterns if directly needed by the C design, and compile-level consistency for later function work. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Implement the first output-related function from `src/output.c` in `src/output.rs`, using the ported Rust data structures and preserving the original module-local behavior. Depends on: T005
- [T007] [Story] Implement the second output-related function from `src/output.c` in `src/output.rs`, completing the module’s functional port with signatures and internal logic aligned to the Rust data model. Depends on: T005
- [T008] [P] [Story] Integrate and reconcile both ported functions in `src/output.rs`, resolving shared helper logic inline where directly required by the original file migration and ensuring the module builds cleanly without duplicating behavior. Depends on: T006, T007

## Final Phase: Polish

- [T009] [Story] Perform module polish in `src/output.rs` by removing temporary placeholders, tightening type usage and visibility, and refining comments/documentation left from the migration so the ported module is consistent and maintainable. Depends on: T008
- [T010] [Story] Run a final compile-focused pass on the `091-module_src_output.c_28-rust-port` branch for the `src/output.rs` migration, fixing remaining module-local warnings or mismatches introduced during the port without expanding scope beyond `src/output.c`. Depends on: T009