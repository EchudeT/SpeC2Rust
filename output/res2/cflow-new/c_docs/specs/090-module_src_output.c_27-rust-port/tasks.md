# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `src/output.c` port in `src/output.rs`, and register it from the crate root or existing module tree so `module_src_output.c_27` has a dedicated Rust implementation target.
- [T002] [Story] Review the 15 functions and 10 data structures from `src/output.c` and map them into Rust items to be implemented in `src/output.rs`, documenting the porting surface in code comments or module-level notes. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port the core data structures defined and used by `src/output.c` into Rust representations in `src/output.rs`, preserving field layout intent and ownership relationships needed by the output workflow. Depends on: T002
- [T004] [P] [Story] Add foundational enums, type aliases, and constant definitions required by the `src/output.c` port into `src/output.rs`, aligned with the data structures introduced for the module. Depends on: T003
- [T005] [Story] Implement shared constructor/helper methods on the ported output-related data structures in `src/output.rs` where needed to support the later function groups without duplicating setup logic. Depends on: T003

## Phase 3: Output state and lifecycle functions

- [T006] [Story] Implement the functions from `src/output.c` that initialize, configure, or tear down module-level output state in `src/output.rs`, using the foundational data structures and helpers established earlier. Depends on: T005
- [T007] [Story] Implement the functions from `src/output.c` that open, close, or switch output destinations/resources in `src/output.rs`, keeping behavior grouped around lifecycle transitions only. Depends on: T006

## Phase 4: Formatting and emission functions

- [T008] [Story] Implement the functions from `src/output.c` responsible for formatting output records, labels, or textual fragments in `src/output.rs`, translating C string-building logic into Rust string or buffer handling. Depends on: T007
- [T009] [P] [Story] Implement the functions from `src/output.c` that write or emit formatted data to the selected output target in `src/output.rs`, reusing the formatting path rather than duplicating record assembly. Depends on: T008

## Phase 5: Output coordination and module entry functions

- [T010] [Story] Implement the remaining coordination functions from `src/output.c` in `src/output.rs` that orchestrate data-structure traversal with lifecycle, formatting, and emission helpers to complete the module’s functional port. Depends on: T009
- [T011] [Story] Reconcile public and internal function visibility in `src/output.rs` so the Rust port exposes only the interfaces required by the surrounding crate while keeping helper routines module-private. Depends on: T010

## Final Phase: Polish

- [T012] [Story] Refine the `src/output.rs` implementation to remove C-specific porting artifacts, simplify ownership/borrowing paths, and consolidate duplicated helper logic introduced during migration. Depends on: T011
- [T013] [Story] Perform a final pass on `src/output.rs` for idiomatic Rust naming, error-path consistency already implied by the C behavior, and inline documentation of non-obvious output-flow decisions. Depends on: T012