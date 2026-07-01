# Tasks: module_gnu_progname.c_42

## Phase 1: Setup

- [T001] [Story] Create the Rust module target for `gnu/progname.c` by adding a dedicated source file at `src/gnu/progname.rs` and declaring it from the existing Rust module tree so the ported module can compile.
- [T002] [P] [Story] Establish the module file skeleton in `src/gnu/progname.rs` with the public/private item layout needed to host the ported logic from `gnu/progname.c`. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/progname.c` and define the foundational Rust representation in `src/gnu/progname.rs` for any module-level state or constants directly required by the function port, keeping the implementation scoped to constructs evidenced by the source file. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Port the function from `gnu/progname.c` into `src/gnu/progname.rs`, preserving its module-level behavior and adapting C-specific program-name handling into idiomatic Rust while staying faithful to the source semantics. Depends on: T003.
- [T005] [P] [Story] Integrate the exported function from `src/gnu/progname.rs` into the surrounding Rust module surface so other translated code can call it through the corresponding `gnu` module path. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/progname.rs` to remove migration scaffolding, tighten visibility to the minimum required by callers, and verify the file remains focused on the behavior sourced from `gnu/progname.c`. Depends on: T005.