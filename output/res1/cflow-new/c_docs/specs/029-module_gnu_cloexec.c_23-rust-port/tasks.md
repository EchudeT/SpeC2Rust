# Tasks: module_gnu_cloexec.c_23

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the port of `gnu/cloexec.c` in `src/module_gnu_cloexec.rs`, and wire it into the crate from the existing module tree on branch `029-module_gnu_cloexec.c_23-rust-port`.
- [T002] [P] [Story] Add the minimal item scaffolding in `src/module_gnu_cloexec.rs` for the two ported functions from `gnu/cloexec.c`, preserving module-local naming and signatures needed for later implementation. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/cloexec.c` and establish any module-level constants, imports, and internal helper aliases directly required by the two functions in `src/module_gnu_cloexec.rs`; keep this limited to items evidenced by the source file. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the close-on-exec descriptor handling function from `gnu/cloexec.c` in `src/module_gnu_cloexec.rs`, mapping the original file-descriptor flag update behavior into Rust. Depends on: T003.
- [T005] [Story] Implement the companion query/set support function from `gnu/cloexec.c` in `src/module_gnu_cloexec.rs`, keeping its behavior aligned with the original module and reusing the shared module-level setup from `src/module_gnu_cloexec.rs`. Depends on: T003.

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/module_gnu_cloexec.rs` to remove unused scaffolding, tighten imports, and ensure the two ported functions are consistently organized with the original `gnu/cloexec.c` migration scope. Depends on: T004, T005.