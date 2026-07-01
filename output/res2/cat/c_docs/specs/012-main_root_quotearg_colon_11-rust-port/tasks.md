# Tasks: main_root_quotearg_colon_11

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for the `quotearg.c` migration in `src/quotearg.rs`, and expose it from the crate root or existing module registry in the Rust project branch `012-main_root_quotearg_colon_11-rust-port`.
- [T002] [P] [Story] Review `quotearg.c` and enumerate the 29 migrated data structures and 2 target functions as Rust items to be placed in `src/quotearg.rs`; document the final item mapping inline as implementation comments for the migration work. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust representations for the `quotearg.c` data structures in `src/quotearg.rs`, preserving the C module’s layout and semantic groupings needed by this module’s 2 functions. Depends on: T002
- [T004] [P] [Story] Add associated enums, constants, option/state carriers, and helper type aliases in `src/quotearg.rs` required to support colon-style quoting behavior and root quotearg state used by this module. Depends on: T002
- [T005] [Story] Integrate the full set of foundational data structures into a coherent module API within `src/quotearg.rs`, resolving references among the migrated structs/enums so the function implementations can be added without placeholder types. Depends on: T003, T004

## Phase 3: Functions

- [T006] [Story] Implement the root quotearg entry-point function from `quotearg.c` in `src/quotearg.rs`, using the migrated quoting option/state data structures and preserving the original module behavior. Depends on: T005
- [T007] [Story] Implement the colon-specific quotearg function from `quotearg.c` in `src/quotearg.rs`, grouping it with the root quotearg logic and reusing the shared foundational types instead of duplicating quoting state handling. Depends on: T005
- [T008] [Story] Reconcile shared internal logic between the two migrated functions in `src/quotearg.rs` so both use the same Rust-side data structure model and module-local helpers implied by the original `quotearg.c` implementation. Depends on: T006, T007

## Final Phase: Polish

- [T009] [P] [Story] Refine `src/quotearg.rs` for idiomatic Rust naming, visibility, and ownership while keeping the migrated `quotearg.c` behavior intact and limiting changes to this module’s ported data structures and 2 functions. Depends on: T008
- [T010] [Story] Perform a final pass on `src/quotearg.rs` to remove migration scaffolding comments that are no longer needed, verify dependency ordering is reflected in the file organization, and ensure the module is ready for integration on branch `012-main_root_quotearg_colon_11-rust-port`. Depends on: T009