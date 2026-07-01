# tasks.md

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffolding for the `src/output.c` port on branch `090-module_src_output.c_27-rust-port`, adding the target Rust source file at `src/output.rs`.
- [T002] [Story] Wire the new `src/output.rs` module into the crate module tree from the existing Rust project entry points so the ported module can be compiled. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the Rust data structures, enums, type aliases, and constant representations required by `src/output.c` in `src/output.rs`, covering the module’s 10 identified data structures. Depends on: T002.
- [T004] [P] [Story] Add foundational constructors, default/value initialization helpers, and internal utility methods for the `src/output.c` data structures in `src/output.rs` where needed to support the function port. Depends on: T003.

## Phase 3: Output stream and state management functions

- [T005] [Story] Port the `src/output.c` functions responsible for module-level output state initialization, reset, and teardown into `src/output.rs`, preserving their shared use of the foundational data structures. Depends on: T003, T004.
- [T006] [P] [Story] Port the `src/output.c` functions that configure output destinations, handles, or writer selection into `src/output.rs`. Depends on: T005.
- [T007] [P] [Story] Port the `src/output.c` functions that manage output mode flags, formatting state, or emission context transitions into `src/output.rs`. Depends on: T005.

## Phase 4: Record and formatted emission functions

- [T008] [Story] Port the `src/output.c` functions that emit primary output records or lines into `src/output.rs`, reusing the output state and destination configuration. Depends on: T006, T007.
- [T009] [P] [Story] Port the `src/output.c` helper functions that format names, prefixes, separators, or field text before emission into `src/output.rs`. Depends on: T003, T004.
- [T010] [Story] Integrate the formatting helpers with the primary emission functions so the full record-generation path from `src/output.c` is represented in `src/output.rs`. Depends on: T008, T009.

## Phase 5: Auxiliary output helpers

- [T011] [Story] Port the remaining auxiliary `src/output.c` functions that support buffering, flushing, or end-of-output finalization behavior into `src/output.rs`. Depends on: T010.
- [T012] [P] [Story] Port the remaining small helper functions from `src/output.c` that are only used internally by the output module and were not covered by earlier groups, keeping each function implemented exactly once in `src/output.rs`. Depends on: T010.

## Final Phase: Polish

- [T013] [Story] Refine `src/output.rs` to remove obvious C-specific patterns made unnecessary by Rust, consolidate duplicated internal logic introduced during the port, and align naming and visibility with the surrounding Rust module conventions. Depends on: T011, T012.
- [T014] [Story] Perform a final compile-focused review of the `src/output.rs` port, resolving integration issues introduced by the migration and ensuring all ported data structures and functions are connected consistently. Depends on: T013.