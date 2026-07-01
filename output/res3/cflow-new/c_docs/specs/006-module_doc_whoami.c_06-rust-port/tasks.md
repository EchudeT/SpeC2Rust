# Tasks: module_doc_whoami.c_06 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `doc/whoami.c` port on branch `006-module_doc_whoami.c_06-rust-port`, adding the target source file `src/module_doc_whoami.rs` and wiring it into the crate module tree from `src/lib.rs`.
- [T002] [P] [Story] Add a porting placeholder comment block in `src/module_doc_whoami.rs` documenting the source mapping from `doc/whoami.c` and reserving sections for the module data structure and function implementation.

## Phase 2: Foundational

- [T003] [Story] Define the single module data structure from `doc/whoami.c` in `src/module_doc_whoami.rs`, preserving its role and field layout semantics as closely as Rust allows for the port. Depends on: T001, T002.

## Phase 3: Functions

- [T004] [Story] Implement the module’s single function from `doc/whoami.c` in `src/module_doc_whoami.rs`, using the Phase 2 data structure where required and keeping behavior aligned with the original C module intent. Depends on: T003.

## Final Phase: Polish

- [T005] [Story] Refine `src/module_doc_whoami.rs` by removing porting placeholders that are no longer needed, tightening item visibility to the minimum required by the crate, and confirming naming/documentation consistency with the `doc/whoami.c` source mapping. Depends on: T004.