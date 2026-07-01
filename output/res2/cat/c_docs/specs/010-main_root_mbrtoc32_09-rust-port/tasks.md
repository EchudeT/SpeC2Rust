# Tasks: main_root_mbrtoc32_09

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `mbrtoc32.c` in `src/main_root_mbrtoc32_09.rs`, and expose it from the existing crate entry point needed by branch `010-main_root_mbrtoc32_09-rust-port`.
- [T002] [P] [Story] Review `mbrtoc32.c` and map its 2 functions to Rust function stubs in `src/main_root_mbrtoc32_09.rs`, keeping names/signatures aligned with the C module’s migration scope. Depends on: T001.

## Phase 2: Foundational

- [T003] [Story] Confirm that `mbrtoc32.c` introduces no module-specific data structures and keep `src/main_root_mbrtoc32_09.rs` free of unnecessary struct or enum additions before function porting. Depends on: T002.

## Phase 3: Functions

- [T004] [Story] Implement the primary multibyte-to-`char32` conversion logic from `mbrtoc32.c` in `src/main_root_mbrtoc32_09.rs`, preserving the original control flow and return semantics expected by the C module. Depends on: T003.
- [T005] [Story] Implement the remaining helper/wrapper function from `mbrtoc32.c` in `src/main_root_mbrtoc32_09.rs`, wiring it to the primary conversion logic without duplicating behavior. Depends on: T004.

## Final Phase: Polish

- [T006] [Story] Refine `src/main_root_mbrtoc32_09.rs` to remove migration scaffolding, verify imports and visibility, and ensure the two ported functions form a minimal, module-local Rust translation of `mbrtoc32.c`. Depends on: T005.