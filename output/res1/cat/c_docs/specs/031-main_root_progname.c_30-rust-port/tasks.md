# Tasks: main_root_progname.c_30

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `progname.c` in `src/progname.rs`, defining the public surface needed for the `main_cluster` port.
- [T002] [Story] Wire the new module into the crate from `src/lib.rs` or `src/main.rs` by declaring `mod progname;` / `pub mod progname;` as appropriate for the existing project layout. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Establish the foundational Rust representation for the `progname.c` migration in `src/progname.rs`, including any module-level state or simple aliases directly required to support the C function port. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Port the single function from `progname.c` into idiomatic Rust in `src/progname.rs`, preserving the original `main_cluster` behavior and keeping the implementation scoped to this module’s responsibility. Depends on: T003
- [T005] [P] [Story] Integrate call sites that rely on the `progname.c` function by updating the crate entry wiring in `src/main.rs` or `src/lib.rs` to use the Rust implementation from `src/progname.rs`. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/progname.rs` and its crate wiring for consistency with project conventions, removing migration scaffolding and simplifying the final API without changing behavior. Depends on: T005