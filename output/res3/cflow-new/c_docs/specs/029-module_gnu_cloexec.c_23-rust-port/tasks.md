# Tasks: module_gnu_cloexec.c_23

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `gnu/cloexec.c` port in `src/module_gnu_cloexec.rs`, and expose it from the crate root module file already used by the project branch.
- [T002] [P] [Story] Review `gnu/cloexec.c` and map its 2 exported or internal functions to Rust function stubs in `src/module_gnu_cloexec.rs`, preserving current module-local scope and naming intent where applicable. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Establish the foundational module-level imports, constants, and minimal helper layout required by the `gnu/cloexec.c` port in `src/module_gnu_cloexec.rs`, limited to items directly needed by the identified functions. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the close-on-exec descriptor handling function from `gnu/cloexec.c` in `src/module_gnu_cloexec.rs`, translating the original file-descriptor flag manipulation logic into Rust. Depends on: T003.
- [T005] [Story] Implement the companion function from `gnu/cloexec.c` in `src/module_gnu_cloexec.rs`, keeping behavior aligned with the C module and reusing the same module-local foundations where applicable. Depends on: T003.

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/module_gnu_cloexec.rs` to remove redundant scaffolding, tighten visibility to module-local use, and ensure both ported functions are organized consistently with the source module’s responsibilities. Depends on: T004, T005.