# Tasks: module_gnu_itold.c_32 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for the `gnu/itold.c` migration in `src/gnu/itold.rs`, establishing the target location for the ported implementation.
- [T002] [Story] Wire the new module into the Rust project by declaring it from the existing GNU module tree in `src/gnu/mod.rs` so `src/gnu/itold.rs` is compiled.
- [T003] [P] [Story] Add a migration stub for the exported functionality in `src/gnu/itold.rs`, preserving the C module scope and documenting the source file mapping from `gnu/itold.c`. Depends on: T001.

## Phase 2: Foundational

- [T004] [Story] Review `gnu/itold.c` for any module-local aliases, constants, or helper representations required by the integer-to-`long double` conversion logic, and define only those foundational items directly in `src/gnu/itold.rs`. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Port the sole function from `gnu/itold.c` into `src/gnu/itold.rs`, translating the integer-to-`long double` conversion behavior into Rust while preserving the original module semantics. Depends on: T004.
- [T006] [P] [Story] Replace the migration stub in `src/gnu/itold.rs` with the completed function signature and implementation details, keeping the public/private visibility aligned with the original C module usage. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/itold.rs` for idiomatic Rust within the strict bounds of the original C behavior, removing temporary migration comments or placeholders left from the port. Depends on: T006.
- [T008] [Story] Perform a final module integration pass across `src/gnu/mod.rs` and `src/gnu/itold.rs` to confirm the new file path wiring and function placement match the `gnu/itold.c` migration scope without adding extra functionality. Depends on: T007.