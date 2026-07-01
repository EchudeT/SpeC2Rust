# Tasks: module_include

## Phase 1: Setup

- [T001] [Story] Initialize Rust module scaffolding for the `include/safe-read.c` port on branch `001-module_include-rust-port`, creating the target module file at `src/module_include/safe_read.rs`.
- [T002] [P] [Story] Wire the new module into the Rust crate module tree by declaring `src/module_include/safe_read.rs` from its parent module file under `src/module_include/mod.rs`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `include/safe-read.c` and define any module-local foundational Rust aliases, constants, or helper signatures required by the safe read port directly in `src/module_include/safe_read.rs`, keeping scope limited to items evidenced by the C source. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Port the safe-read function from `include/safe-read.c` into Rust in `src/module_include/safe_read.rs`, preserving the source module’s read behavior and error/return semantics as closely as supported by the Rust project structure. Depends on: T003
- [T005] [P] [Story] Expose the ported safe-read function through the module boundary in `src/module_include/mod.rs` if required by existing crate organization, without introducing extra API surface beyond the C module mapping. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/module_include/safe_read.rs` and `src/module_include/mod.rs` for idiomatic Rust naming, imports, and minimal cleanup while preserving the one-to-one module migration scope. Depends on: T004, T005