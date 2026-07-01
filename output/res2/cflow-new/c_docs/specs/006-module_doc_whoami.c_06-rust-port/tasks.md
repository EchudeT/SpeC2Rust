# Task List: module_doc_whoami.c_06

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module port scaffold for `doc/whoami.c` on branch `006-module_doc_whoami.c_06-rust-port`, creating the target source file at `src/doc/whoami.rs`.
- [T002] [Story] Register the new Rust module file `src/doc/whoami.rs` in the crate’s module tree so the ported implementation is reachable from the project build.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port the single data structure described by `doc/whoami.c` into a Rust type definition in `src/doc/whoami.rs`, preserving the C module’s field layout and usage semantics as closely as Rust allows.
  - Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the module’s single function from `doc/whoami.c` in `src/doc/whoami.rs`, using the Rust data structure defined for this module and keeping behavior aligned with the original C logic.
  - Depends on: T003

## Final Phase: Polish

- [T005] [Story] Refine `src/doc/whoami.rs` for idiomatic Rust within the scope of the C port, removing migration scaffolding, tightening signatures and visibility, and ensuring the module remains cleanly integrated in the crate.
  - Depends on: T004