# Tasks: main_root_xmalloc.c_28

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `xmalloc.c` port on branch `028-main_root_xmalloc.c_28-rust-port`, adding the target implementation file at `src/xmalloc.rs`.
- [T002] [P] [Story] Wire the new module into the crate root so `src/xmalloc.rs` is compiled and accessible from the existing Rust project entry points in `src/lib.rs` or `src/main.rs`, depending on the current crate layout.
- [T003] [Story] Establish the migration boundary for this module by adding the initial public/internal item layout in `src/xmalloc.rs` for the 15 functions from `xmalloc.c`, keeping names and grouping aligned with the source module. Depends on: T001, T002

## Phase 2: Foundational

- [T004] [Story] Implement the shared allocation error-handling foundation in `src/xmalloc.rs` that all translated allocation helpers will use, preserving the module-local behavior expected from `xmalloc.c`. Depends on: T003
- [T005] [Story] Add shared internal helpers in `src/xmalloc.rs` for size validation and Rust allocation pathway normalization so the function ports can consistently map C allocation semantics into Rust. Depends on: T004

## Phase 3: Core allocation functions

- [T006] [Story] Port the primary allocation entry points from `xmalloc.c` into `src/xmalloc.rs`, covering the function group responsible for basic allocation behavior and failure-on-error semantics. Depends on: T005
- [T007] [P] [Story] Port the zero-initializing allocation function group from `xmalloc.c` into `src/xmalloc.rs`, keeping their behavior consistent with the shared allocation foundation. Depends on: T005
- [T008] [P] [Story] Port the reallocation function group from `xmalloc.c` into `src/xmalloc.rs`, including the variants that preserve the original `xmalloc.c` failure behavior. Depends on: T005

## Phase 4: Duplication and string-oriented helpers

- [T009] [Story] Port the memory duplication helper function group from `xmalloc.c` into `src/xmalloc.rs`, using the shared allocation and size-validation helpers already established. Depends on: T006, T008
- [T010] [P] [Story] Port the string duplication helper function group from `xmalloc.c` into `src/xmalloc.rs`, preserving null-terminated C-oriented semantics as required by the original module behavior. Depends on: T006
- [T011] [P] [Story] Port any length-bounded string duplication helper function group from `xmalloc.c` into `src/xmalloc.rs`, ensuring its allocation and copy behavior matches the original module. Depends on: T006, T010

## Phase 5: Integration completion

- [T012] [Story] Reconcile the full set of 15 translated functions in `src/xmalloc.rs`, removing placeholder stubs and ensuring each function from `xmalloc.c` is implemented exactly once within this module. Depends on: T006, T007, T008, T009, T010, T011
- [T013] [Story] Align visibility and signatures in `src/xmalloc.rs` with actual crate usage needs so the ported allocation helpers integrate cleanly with the rest of the `pwd` Rust project. Depends on: T012

## Final Phase: Polish

- [T014] [Story] Perform a final pass on `src/xmalloc.rs` to simplify duplicated internal logic across the translated allocation helpers without changing module behavior. Depends on: T013
- [T015] [Story] Review the `xmalloc.c` to `src/xmalloc.rs` migration for naming consistency, unsafe boundary minimization, and idiomatic Rust organization while preserving original semantics. Depends on: T014