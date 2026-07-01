# Tasks: module_doc_main_01 Rust port

## Phase 1: Setup

- [ ] T001 [Story] Initialize the Rust module layout for the `doc` cluster on branch `001-module_doc_main_01-rust-port`, creating target source files `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs`.
- [ ] T002 [P] [Story] Wire the new module files into the Rust crate module tree so `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs` are compiled as part of the project. Depends on: T001.

## Phase 2: Foundational

- [ ] T003 [Story] Implement the first inferred foundational data structure needed by the `doc` cluster in its owning Rust target file, keeping the definition colocated with the corresponding ported logic in one of `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, or `src/doc/whoami.rs`. Depends on: T002.
- [ ] T004 [P] [Story] Implement the second inferred foundational data structure needed by the `doc` cluster in its owning Rust target file, keeping the definition colocated with the corresponding ported logic in one of `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, or `src/doc/whoami.rs`. Depends on: T002.
- [ ] T005 [P] [Story] Implement the third inferred foundational data structure needed by the `doc` cluster in its owning Rust target file, keeping the definition colocated with the corresponding ported logic in one of `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, or `src/doc/whoami.rs`. Depends on: T002.
- [ ] T006 [Story] Reconcile shared field types, ownership, and constructor patterns across the three `doc` cluster data structures so the function ports can use them consistently. Depends on: T003, T004, T005.

## Phase 3: Functions

- [ ] T007 [Story] Port the function logic from `doc/d.c` into `src/doc/d.rs`, adapting it to the established Rust data structures and module interfaces. Depends on: T006.
- [ ] T008 [P] [Story] Port the function logic from `doc/foo.c` into `src/doc/foo.rs`, adapting it to the established Rust data structures and module interfaces. Depends on: T006.
- [ ] T009 [P] [Story] Port the function logic from `doc/wc.c` into `src/doc/wc.rs`, adapting it to the established Rust data structures and module interfaces. Depends on: T006.
- [ ] T010 [P] [Story] Port the function logic from `doc/whoami.c` into `src/doc/whoami.rs`, adapting it to the established Rust data structures and module interfaces. Depends on: T006.
- [ ] T011 [Story] Align cross-module call signatures and imports among `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs` so the four ported functions integrate cleanly within the crate. Depends on: T007, T008, T009, T010.

## Final Phase: Polish

- [ ] T012 [Story] Refine the `doc` cluster Rust port for idiomatic naming, remove porting leftovers, and simplify obvious duplication across `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs` without changing behavior. Depends on: T011.
- [ ] T013 [Story] Perform a final compile-pass cleanup for the `doc` cluster, resolving warnings and import hygiene in `src/doc/d.rs`, `src/doc/foo.rs`, `src/doc/wc.rs`, and `src/doc/whoami.rs`. Depends on: T012.