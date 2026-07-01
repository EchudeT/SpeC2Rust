# Tasks: main_root_quoting_options_01

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `quotearg.c` port in `src/quotearg.rs`, and expose it from the crate root via `src/lib.rs` or `src/main.rs` according to the existing project layout on branch `001-main_root_quoting_options_01-rust-port`.
- [T002] [P] [Story] Add the initial module API surface in `src/quotearg.rs` for quoting options and quoting entry points, keeping names and grouping aligned with the `quotearg.c` migration target. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational quoting configuration data structures migrated from `quotearg.c` in `src/quotearg.rs`, including the primary quoting options structure and its directly associated enums, flags, and constant tables required by the module. Depends on: T002.
- [T004] [P] [Story] Implement the supporting state-holder and argument-slot data structures used to retain per-call and reusable quoting results in `src/quotearg.rs`, matching the C module’s storage responsibilities without adding unrelated behavior. Depends on: T003.
- [T005] [P] [Story] Define Rust representations for the remaining helper data structures from `quotearg.c` that are required by the module’s 15 functions, consolidating them in `src/quotearg.rs` and wiring their relationships to the main quoting options model. Depends on: T003.

## Phase 3: Core option accessors and state management functions

- [T006] [Story] Implement the functions that create, retrieve, or reset quoting option state in `src/quotearg.rs`, grouping the direct option accessors and default-option helpers together so later quoting functions can consume a stable configuration API. Depends on: T003, T004.
- [T007] [P] [Story] Implement the functions that mutate quoting option fields in `src/quotearg.rs`, including setter-style behaviors for style selection and option flags, keeping them grouped as the module’s option mutation layer. Depends on: T006.
- [T008] [P] [Story] Implement the functions that manage reusable quoted-argument storage in `src/quotearg.rs`, covering the slot/state updates needed to support repeated quoting calls without duplicating option logic. Depends on: T004, T006.

## Phase 4: Quoting engine and quoting entry-point functions

- [T009] [Story] Implement the internal quoting engine functions in `src/quotearg.rs` that transform input bytes or strings according to the configured quoting options, using the foundational data structures and avoiding expansion beyond behavior evidenced by `quotearg.c`. Depends on: T005, T006, T007.
- [T010] [P] [Story] Implement the public quoting entry-point functions in `src/quotearg.rs` that wrap the internal engine for the module’s main call patterns, including variants that use explicit options versus default options. Depends on: T008, T009.
- [T011] [P] [Story] Implement the remaining convenience wrapper functions in `src/quotearg.rs` that adapt argument forms or predefined styles to the core quoting engine, ensuring each C function from `quotearg.c` is ported exactly once in this phase. Depends on: T009.

## Final Phase: Polish

- [T012] [Story] Refine `src/quotearg.rs` to remove migration duplication, tighten ownership and borrowing around quoting buffers and option state, and ensure the module exports are minimal and consistent with the completed `quotearg.c` port. Depends on: T007, T008, T010, T011.