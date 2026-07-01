# Tasks: main_root_mbrtoc32_10

## Phase 1: Setup

- [ ] [T001] [Story] Initialize the Rust port scaffold for this module on branch `010-main_root_mbrtoc32_10-rust-port`, creating or updating the module entry points needed to host the port of `mbrtoc32.c` in `src/main_root_mbrtoc32_10.rs`.
- [ ] [T002] [P] [Story] Wire the new module file into the crate module tree so the `main_cluster` port code is reachable from the existing Rust project structure via `src/lib.rs`. Depends on: T001

## Phase 2: Foundational

- [ ] [T003] [Story] Review `mbrtoc32.c` and define the Rust-side foundational state and type aliases required by its two functions, keeping them in `src/main_root_mbrtoc32_10.rs` and limiting the implementation to data representations directly evidenced by the C module. Depends on: T002

## Phase 3: Functions

- [ ] [T004] [Story] Port the core UTF multibyte-to-`char32` conversion routine from `mbrtoc32.c` into `src/main_root_mbrtoc32_10.rs`, using the foundational state/types from this module and preserving the original function-level behavior. Depends on: T003
- [ ] [T005] [Story] Port the remaining helper or wrapper function from `mbrtoc32.c` into `src/main_root_mbrtoc32_10.rs`, keeping it grouped with the conversion logic and aligned with the original C module responsibilities. Depends on: T004

## Final Phase: Polish

- [ ] [T006] [Story] Refine the Rust implementation in `src/main_root_mbrtoc32_10.rs` by removing obvious duplication introduced during porting, tightening signatures and internal visibility, and ensuring the module remains focused on the `mbrtoc32.c` migration scope. Depends on: T005