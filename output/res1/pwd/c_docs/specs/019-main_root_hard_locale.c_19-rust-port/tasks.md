# Tasks: main_root_hard-locale.c_19

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/hard_locale.rs` for the port of `hard-locale.c`.
- [T002] [Story] Expose the new module from `src/lib.rs` with `mod hard_locale;` and the needed public re-export for its function. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Review `src/hard_locale.rs` and define any module-local constants, helper imports, and function signatures required to support the `hard-locale.c` port without introducing new data structures. Depends on: T001

## Phase 3: Functions

- [T004] [Story] Implement the hard-locale behavior from `hard-locale.c` in `src/hard_locale.rs`, porting the module’s single function and preserving its main-cluster semantics for locale detection. Depends on: T003
- [T005] [P] [Story] Integrate call-site visibility for the ported hard-locale function by aligning its public API in `src/lib.rs` with the implementation in `src/hard_locale.rs`. Depends on: T002, T004

## Final Phase: Polish

- [T006] [Story] Refine `src/hard_locale.rs` and `src/lib.rs` to remove migration scaffolding, ensure naming and comments match the Rust port conventions, and verify dependency wiring remains minimal and module-scoped. Depends on: T004, T005