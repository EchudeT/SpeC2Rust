# Tasks: module_gnu_close.c_24 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/close.c` port in `src/gnu/close.rs`, and declare the module from the existing parent module file that owns `src/gnu/close.rs` on branch `030-module_gnu_close.c_24-rust-port`.
- [T002] [P] [Story] Add the initial public/private item skeleton in `src/gnu/close.rs` for the 2 functions identified in `gnu/close.c`, preserving module-local scope decisions needed for the port. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/close.c` for module-local constants, type aliases, and helper definitions, and mirror only the required foundational items in `src/gnu/close.rs` before function bodies are implemented. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the primary close-related function from `gnu/close.c` in `src/gnu/close.rs`, translating its return-value handling and error propagation semantics from the C module. Depends on: T003.
- [T005] [Story] Implement the remaining close-related function from `gnu/close.c` in `src/gnu/close.rs`, keeping behavior aligned with the original module and reusing the shared foundations established earlier. Depends on: T003.

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/gnu/close.rs` to remove porting scaffolding, tighten visibility, and align naming and inline documentation with surrounding Rust module conventions after both functions are in place. Depends on: T004, T005.
- [T007] [Story] Perform a final module-level verification pass on `src/gnu/close.rs` to ensure the Rust port fully covers the 2 functions from `gnu/close.c` with no duplicate or deferred implementation gaps. Depends on: T006.