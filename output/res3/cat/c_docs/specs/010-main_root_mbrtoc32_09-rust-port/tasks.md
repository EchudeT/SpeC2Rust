# Tasks: main_root_mbrtoc32_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module file for this port at `src/main_root_mbrtoc32_09.rs` and register it from the existing crate root so the `mbrtoc32.c` migration target is available on branch `010-main_root_mbrtoc32_09-rust-port`.
- [T002] [P] [Story] Add function skeletons in `src/main_root_mbrtoc32_09.rs` for the two functions migrated from `mbrtoc32.c`, preserving the original C entry-point names or their established Rust-port equivalents for later implementation.

## Phase 2: Foundational

- [T003] [Story] Review `mbrtoc32.c` and establish any module-local type aliases, constants, or helper signatures directly required by the two migrated functions inside `src/main_root_mbrtoc32_09.rs`, keeping the scope limited to constructs evidenced by the source file. Depends on: T001, T002

## Phase 3: Functions

- [T004] [Story] Implement the primary `mbrtoc32` conversion routine in `src/main_root_mbrtoc32_09.rs`, porting the source-file logic for multibyte-to-32-bit character conversion and any state handling evidenced in `mbrtoc32.c`. Depends on: T003
- [T005] [Story] Implement the remaining support function from `mbrtoc32.c` in `src/main_root_mbrtoc32_09.rs`, grouping it with the conversion logic only where the C file shows a direct functional relationship. Depends on: T003

## Final Phase: Polish

- [T006] [P] [Story] Refine `src/main_root_mbrtoc32_09.rs` for Rust idioms and module-local clarity, removing migration-only scaffolding, aligning signatures and visibility with the surrounding main-cluster layout, and ensuring no duplicated logic remains across the two ported functions. Depends on: T004, T005