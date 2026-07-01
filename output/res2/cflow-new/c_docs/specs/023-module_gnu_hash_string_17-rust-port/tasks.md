# Tasks: module_gnu_hash_string_17

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/hash.c` in `src/gnu/hash.rs`, and expose it from `src/gnu/mod.rs` and `src/lib.rs` for branch `023-module_gnu_hash_string_17-rust-port`.
- [T002] [P] [Story] Establish the module-local item layout in `src/gnu/hash.rs` for the ported data structures and the 2 function implementations, keeping names and organization aligned with `gnu/hash.c`. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port the foundational data-structure definitions referenced by `gnu/hash.c` into Rust in `src/gnu/hash.rs`, covering the module-owned structs, enums, type aliases, constants, and related field mappings required before function translation. Depends on: T002
- [T004] [Story] Normalize the Rust representations in `src/gnu/hash.rs` so the ported 49 data structures have compile-ready ownership, mutability, and visibility choices appropriate for internal module use by the hash implementation. Depends on: T003

## Phase 3: Functions

- [T005] [Story] Implement the GNU hash string helper functionality from `gnu/hash.c` in `src/gnu/hash.rs`, translating the string-oriented hash computation function and wiring it to the foundational data structures established for this module. Depends on: T004
- [T006] [Story] Implement the remaining GNU hash support function from `gnu/hash.c` in `src/gnu/hash.rs`, grouping it with the string hash logic where shared constants or structure access are required. Depends on: T005

## Final Phase: Polish

- [T007] [Story] Refine `src/gnu/hash.rs` for Rust idioms by removing porting-only inconsistencies, tightening signatures and internal visibility, and ensuring the completed `gnu/hash.c` migration is cohesive with `src/gnu/mod.rs` and `src/lib.rs`. Depends on: T006