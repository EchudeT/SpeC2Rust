# Tasks: main_root_version_etc_06

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for this migration in `src/version_etc.rs`, wiring it into the crate from the existing root module layout for branch `006-main_root_version_etc_06-rust-port`.
- [T002] [P] [Story] Establish the public function signatures in `src/version_etc.rs` for the four functions migrated from `version-etc.c`, keeping names and grouping aligned with the source module. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Define the foundational constants, helper aliases, and internal formatting scaffolding required by `version-etc.c` in `src/version_etc.rs`, limited to support directly needed by the migrated functions. Depends on: T002.

## Phase 3: Version text emission functions

- [T004] [Story] Implement the core version/banner output function from `version-etc.c` in `src/version_etc.rs`, using the Phase 2 formatting scaffolding. Depends on: T003.
- [T005] [P] [Story] Implement the companion variant that emits version information with explicit package/program metadata in `src/version_etc.rs`, sharing the same internal formatting path as the core output function. Depends on: T003.
- [T006] [Story] Implement the author-list handling function from `version-etc.c` in `src/version_etc.rs`, covering the argument shaping needed by the version output entry points. Depends on: T003.

## Phase 4: Variadic/front-end wrapper functions

- [T007] [Story] Implement the remaining wrapper/front-end function from `version-etc.c` in `src/version_etc.rs`, connecting it to the author-list handling and version text emission logic without duplicating formatting behavior. Depends on: T004, T005, T006.

## Final Phase: Polish

- [T008] [Story] Refine `src/version_etc.rs` to remove duplicated formatting paths, align visibility to actual module use, and ensure the migrated functions remain contained to the `version-etc.c` port scope. Depends on: T007.