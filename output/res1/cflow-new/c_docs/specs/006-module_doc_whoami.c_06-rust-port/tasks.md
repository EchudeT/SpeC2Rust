# Tasks: module_doc_whoami.c_06 Rust port

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for `doc/whoami.c` migration in `src/doc/whoami.rs`.
- [T002] [Story] Wire the new module into the Rust crate module tree so `src/doc/whoami.rs` is compiled, updating the nearest inferable module declaration file under `src/doc/`.
- [T003] [P] [Story] Add placeholder public items in `src/doc/whoami.rs` matching the single data structure and single function identified for this module, to establish the migration surface. Depends on: T001, T002.

## Phase 2: Foundational

- [T004] [Story] Implement the module’s single data structure in `src/doc/whoami.rs`, translating the C structure semantics from `doc/whoami.c` into an idiomatic Rust type. Depends on: T003.
- [T005] [Story] Refine visibility, ownership, and field typing for the data structure in `src/doc/whoami.rs` so the upcoming function implementation can use it directly without C-style patterns. Depends on: T004.

## Phase 3: Functions

- [T006] [Story] Implement the module’s single function in `src/doc/whoami.rs`, preserving the behavior of `doc/whoami.c` and integrating it with the migrated data structure. Depends on: T005.
- [T007] [Story] Complete function-level cleanup in `src/doc/whoami.rs`, resolving signature details, return types, and internal control flow needed to match the original module behavior. Depends on: T006.

## Final Phase: Polish

- [T008] [Story] Remove setup placeholders and dead migration scaffolding from `src/doc/whoami.rs`, leaving only the finalized Rust implementation. Depends on: T007.
- [T009] [Story] Perform a final module pass on `src/doc/whoami.rs` and the related `src/doc/` module declaration file to align naming, documentation comments, and imports with the completed port. Depends on: T008.