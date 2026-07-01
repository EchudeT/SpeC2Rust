# Tasks: `main_root_localcharset.c_29`

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `localcharset.c` in `src/localcharset.rs` and expose it from the crate root if needed by the `cat` binary on branch `030-main_root_localcharset.c_29-rust-port`.
- [T002] [P] [Story] Review `localcharset.c` and map the single exported/internal function plus the 8 associated C data structures/constants to Rust items to be implemented in `src/localcharset.rs`; record the migration outline in code comments or TODO markers in that file. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Implement the foundational Rust representations for the 8 data structures/static layout elements inferred from `localcharset.c` in `src/localcharset.rs`, preserving the C module’s ownership, mutability, and lookup-table intent. Depends on: T002
- [T004] [Story] Add constructors/default initializers or static declarations required to make the `localcharset.c` data representations usable by the function port in `src/localcharset.rs`. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Port the module’s single function from `localcharset.c` to idiomatic Rust in `src/localcharset.rs`, wiring it to the previously implemented data structures and preserving the original charset-resolution behavior. Depends on: T004
- [T006] [Story] Integrate the ported `src/localcharset.rs` function into the `cat` main-cluster call path at the existing Rust entry/use site, replacing any placeholder or stub logic for this module. Depends on: T005

## Final Phase: Polish

- [T007] [Story] Refine `src/localcharset.rs` for parity and maintainability by removing migration placeholders, tightening visibility, and simplifying any direct C-style patterns that are no longer needed after the port. Depends on: T006
- [T008] [P] [Story] Run a final compile-focused cleanup on `src/localcharset.rs` and adjacent module wiring touched by this port, resolving warnings introduced by the migration without expanding scope beyond `localcharset.c`. Depends on: T007