# Tasks: main_root_hard-locale.c_28

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/hard_locale.rs` to host the port of `hard-locale.c`.
- [T002] [Story] Expose the new module from the crate root by adding `mod hard_locale;` in `src/main.rs` or `src/lib.rs`, matching the existing project entry layout.
- [T003] [Story] Review the C source `hard-locale.c` and map its single exported function to a Rust function signature in `src/hard_locale.rs` so later implementation stays confined to the original module scope. Depends on: T001.

## Phase 2: Foundational

- [T004] [Story] Confirm that `hard-locale.c` introduces no module-specific data structures and keep `src/hard_locale.rs` free of unnecessary struct or enum scaffolding before function implementation. Depends on: T003.

## Phase 3: Functions

- [T005] [Story] Implement the Rust equivalent of the single function from `hard-locale.c` in `src/hard_locale.rs`, keeping behavior aligned with the original locale-handling logic and limiting dependencies to what is required by this module. Depends on: T004.
- [T006] [P] [Story] Wire call sites in `src/main.rs` or `src/lib.rs` to use the function from `src/hard_locale.rs` if the crate currently expects the C-module behavior through the main cluster integration surface. Depends on: T005.

## Final Phase: Polish

- [T007] [Story] Refine `src/hard_locale.rs` and the corresponding crate-root integration for idiomatic Rust naming, minimal visibility, and removal of any temporary porting placeholders introduced during implementation. Depends on: T005, T006.