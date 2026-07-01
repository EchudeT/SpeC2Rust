# Task List: `main_root_stat_04`

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffolding for the `main_root_stat_04` port in `src/bin/pwd.rs` and `src/bin/root-dev-ino.rs`, aligning each target file with the source C files `pwd.c` and `root-dev-ino.c`.
- [T002] [P] [Story] Add shared internal module declarations needed by `src/bin/pwd.rs` and `src/bin/root-dev-ino.rs` so the ported code can host the module-local data structures and function implementations. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Identify and define the data structures required by the `pwd.c` port inside `src/bin/pwd.rs`, preserving the C module’s structure and field intent closely enough for the later function migration. Depends on: T001.
- [T004] [P] [Story] Identify and define the data structures required by the `root-dev-ino.c` port inside `src/bin/root-dev-ino.rs`, preserving the C module’s structure and field intent closely enough for the later function migration. Depends on: T001.
- [T005] [Story] Reconcile any shared or duplicated structure definitions between `src/bin/pwd.rs` and `src/bin/root-dev-ino.rs`, keeping only directly needed definitions in the inferred Rust target files and adjusting call sites accordingly. Depends on: T003, T004.

## Phase 3: Functions

- [T006] [Story] Port the function implementation from `pwd.c` into `src/bin/pwd.rs`, wiring it to the Rust data structures defined for this module and keeping the migration limited to the behavior evidenced by the source file. Depends on: T003, T005.
- [T007] [P] [Story] Port the function implementation from `root-dev-ino.c` into `src/bin/root-dev-ino.rs`, wiring it to the Rust data structures defined for this module and keeping the migration limited to the behavior evidenced by the source file. Depends on: T004, T005.
- [T008] [Story] Resolve integration points between `src/bin/pwd.rs` and `src/bin/root-dev-ino.rs` so the migrated function logic compiles cleanly with the intended module boundaries and file-local structure ownership. Depends on: T006, T007.

## Final Phase: Polish

- [T009] [Story] Refine the migrated implementations in `src/bin/pwd.rs` and `src/bin/root-dev-ino.rs` by removing redundant translation artifacts, tightening type usage, and ensuring the final Rust code remains faithful to the C module scope without adding new behaviors. Depends on: T008.