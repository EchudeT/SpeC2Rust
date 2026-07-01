# Tasks: main_root_version_etc_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module file `src/version_etc.rs` for the port of `version-etc.c`, and declare it from the existing crate entry so the module is compiled on branch `006-main_root_version_etc_06-rust-port`.
- [T002] [Story] Add placeholder public function signatures in `src/version_etc.rs` for the 4 functions analyzed from `version-etc.c`, preserving a grouping suitable for version-information output behavior. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational constants, internal helper types, and shared formatting primitives in `src/version_etc.rs` that are directly needed by the `version-etc.c` port, keeping the implementation limited to structures evidenced by the source file. Depends on: T002.

## Phase 3: Version output functions

- [T004] [Story] Implement the core version-information emission function in `src/version_etc.rs` that prints program/version text and shared header content derived from the `version-etc.c` behavior. Depends on: T003.
- [T005] [P] [Story] Implement the author-list formatting helper function in `src/version_etc.rs`, covering the formatting rules needed by the version output API without introducing unrelated abstractions. Depends on: T003.
- [T006] [Story] Implement the variant wrapper function in `src/version_etc.rs` that accepts an already-prepared author collection and delegates to the core version-information formatter. Depends on: T004, T005.
- [T007] [Story] Implement the remaining public convenience wrapper function in `src/version_etc.rs` that matches the final `version-etc.c` entry point and routes arguments into the shared formatting path. Depends on: T004, T005.

## Final Phase: Polish

- [T008] [Story] Refine `src/version_etc.rs` to remove duplication across the 4 ported functions, finalize visibility, and align naming/documentation with the migrated `version-etc.c` behavior without expanding module scope. Depends on: T006, T007.