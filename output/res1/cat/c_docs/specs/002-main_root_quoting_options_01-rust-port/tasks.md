# Tasks: main_root_quoting_options_01

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `quotearg.c` in `src/quotearg.rs`, and wire it into the crate root via the existing main-cluster module entry so the ported quoting code has a dedicated compilation unit.
- [T002] [P] [Story] Define the initial public/internal item layout in `src/quotearg.rs` for quoting options, slot state, and helper types inferred from `quotearg.c`, keeping names grouped to support direct migration of the C module. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Implement the foundational data structures from `quotearg.c` in `src/quotearg.rs`, including the Rust equivalents for quoting option state, quoting style representation, per-call/per-slot buffers, and character-class or flag-carrying configuration records required by the module’s functions. Depends on: T002.
- [T004] [P] [Story] Add associated constructors, defaults, and internal mutation helpers for the quoting-related data structures in `src/quotearg.rs`, so later function ports can reuse a single canonical state model instead of duplicating initialization logic. Depends on: T003.
- [T005] [P] [Story] Establish constant values, static defaults, and internal storage conventions in `src/quotearg.rs` for root quoting behavior and option presets that mirror the source module’s global/default configuration patterns. Depends on: T003.

## Phase 3: Option and State Management Functions

- [T006] [Story] Port the functions that create, clone, fetch, or reset quoting option state from `quotearg.c` into `src/quotearg.rs`, using the Phase 2 structures directly and preserving the original module-level ownership boundaries. Depends on: T004, T005.
- [T007] [P] [Story] Port the functions that mutate quoting option fields such as style selection, flag updates, and per-character quoting configuration into `src/quotearg.rs`, grouping all direct option-editing entry points together. Depends on: T006.
- [T008] [Story] Port the functions responsible for managing reusable quoting slots/buffers and default-option access in `src/quotearg.rs`, keeping slot lifecycle logic in the same implementation group as the state-management API. Depends on: T006.

## Phase 4: Core Quoting Functions

- [T009] [Story] Implement the core internal quoting routine group from `quotearg.c` in `src/quotearg.rs`, covering the main escaping/quoting algorithm that consumes bytes or strings plus quoting options and produces the rendered quoted form. Depends on: T007, T008.
- [T010] [P] [Story] Port the helper functions that support the core quoting algorithm in `src/quotearg.rs`, such as style-specific delimiter selection, character handling, and size/accounting logic that are only used by the main quoting path. Depends on: T009.
- [T011] [Story] Port the public wrapper functions that expose common quoting entry points with default options, explicit options, or slot-backed output into `src/quotearg.rs`, mapping them onto the single core implementation without duplicating algorithm logic. Depends on: T009.

## Phase 5: Finalize Remaining Function Variants

- [T012] [Story] Implement the remaining specialized quoting entry points from `quotearg.c` in `src/quotearg.rs`, including variants for argument/vector-oriented use and root-oriented quoting option application, ensuring each C function is ported exactly once into the Rust module. Depends on: T011.
- [T013] [P] [Story] Consolidate shared internal conversion paths in `src/quotearg.rs` so all function variants reuse the same option normalization and output-buffer preparation behavior introduced by earlier tasks. Depends on: T012.

## Final Phase: Polish

- [T014] [Story] Refine `src/quotearg.rs` for idiomatic Rust within the existing port scope by removing migration-only duplication, tightening visibility on internal helpers and data structures, and aligning function signatures with the completed module implementation. Depends on: T013.
- [T015] [Story] Perform a final module pass on `src/quotearg.rs` to verify that all data structures and all 15 functions from `quotearg.c` are represented in the Rust port and that dependency ordering has not left any dead or duplicate implementation paths. Depends on: T014.