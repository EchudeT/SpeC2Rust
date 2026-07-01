# Tasks: main_root_full-write.c_27

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module scaffold for the `full-write.c` port on branch `028-main_root_full_write.c_27-rust-port`, adding the target source file at `src/full_write.rs` and wiring its module declaration from `src/main.rs` or `src/lib.rs` as already used by the Rust project.
- [T002] [P] [Story] Review the C source behavior in `full-write.c` and map the exported function surface to the Rust target file `src/full_write.rs`, documenting the intended one-to-one migration scope in code comments for the port. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Establish foundational Rust definitions in `src/full_write.rs` needed by the `full-write.c` migration, including imports, type aliases, and internal helper signatures directly required by the module’s function implementation, without introducing new data structures not evidenced by the C module. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the full write routine from `full-write.c` in `src/full_write.rs`, preserving the C module’s write-loop semantics, partial-write handling, and return behavior in idiomatic Rust within the project’s existing error-handling style. Depends on: T003
- [T005] [P] [Story] Integrate the ported full write function into the main-cluster call path by updating the existing Rust entry module file (`src/main.rs` or `src/lib.rs`, whichever owns the module graph) to expose and use `src/full_write.rs` consistently with the original C module role. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/full_write.rs` for parity and maintainability by removing migration-only placeholders, tightening signatures and visibility to the minimum required by the main cluster, and aligning naming and comments with the surrounding Rust project conventions. Depends on: T004, T005