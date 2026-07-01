# Tasks: module_gnu_GL_ATTRIBUTE_12

## Phase 1: Setup

- [T001] [Story] Create the Rust module file scaffold for the C sources by adding `src/gnu/error.rs` and `src/gnu/hash.rs`, and register them from the existing `src/gnu/mod.rs` or equivalent module entry point for branch `018-module_gnu_GL_ATTRIBUTE_12-rust-port`.
- [T002] [P] [Story] Establish the per-file migration skeleton in `src/gnu/error.rs` and `src/gnu/hash.rs` with placeholder Rust items matching the source file split, so later data structures and functions can be added without changing module layout. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Inventory and port the data structures required by `gnu/error.c` into Rust in `src/gnu/error.rs`, defining Rust `struct`, `enum`, `type`, `const`, and static representations only for items evidenced by that source file. Depends on: T002.
- [T004] [P] [Story] Inventory and port the data structures required by `gnu/hash.c` into Rust in `src/gnu/hash.rs`, defining Rust `struct`, `enum`, `type`, `const`, and static representations only for items evidenced by that source file. Depends on: T002.
- [T005] [Story] Reconcile shared foundational definitions used by both `src/gnu/error.rs` and `src/gnu/hash.rs`, keeping the definitions in the directly inferable target files and aligning signatures, ownership, and visibility needed for the later function ports. Depends on: T003, T004.

## Phase 3: Error-related functions

- [T006] [Story] Implement the function group from `gnu/error.c` in `src/gnu/error.rs`, porting its error-state and message handling behavior against the Phase 2 Rust data structures without expanding scope beyond the original module responsibilities. Depends on: T003, T005.

## Phase 4: Hash-related functions

- [T007] [Story] Implement the function group from `gnu/hash.c` in `src/gnu/hash.rs`, porting the hash table or hash utility behavior against the Phase 2 Rust data structures without re-splitting the same function work across later phases. Depends on: T004, T005.

## Phase 5: Cross-module integration

- [T008] [P] [Story] Integrate any direct usage linkage between `src/gnu/error.rs` and `src/gnu/hash.rs`, adjusting imports, visibility, and call sites required by the original module relationships while keeping each migrated function in its owning target file. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine the migrated code in `src/gnu/error.rs` and `src/gnu/hash.rs` to remove placeholder scaffolding, tighten type usage, and ensure the final Rust module layout is clean and minimal for `module_gnu_GL_ATTRIBUTE_12`. Depends on: T008.