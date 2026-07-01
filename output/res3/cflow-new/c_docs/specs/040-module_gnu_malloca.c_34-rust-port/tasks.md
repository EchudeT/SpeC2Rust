# Tasks: module_gnu_malloca.c_34

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/malloca.c` in `src/gnu/malloca.rs`, and expose it from the existing parent module file that owns `src/gnu/malloca.rs`.
- [T002] [P] [Story] Add placeholder signatures in `src/gnu/malloca.rs` for the 2 functions ported from `gnu/malloca.c`, matching the C module scope and intended Rust visibility.

## Phase 2: Foundational

- [T003] [Story] Review `gnu/malloca.c` and define any module-local constants, type aliases, or small internal helpers required by both functions directly in `src/gnu/malloca.rs`; avoid introducing new files or shared abstractions not evidenced by this module. Depends on: T001, T002

## Phase 3: Function Implementation

- [T004] [Story] Implement the stack-or-heap allocation support function from `gnu/malloca.c` in `src/gnu/malloca.rs`, preserving the original allocation-path behavior and any module-local bookkeeping needed for later release. Depends on: T003
- [T005] [Story] Implement the corresponding release/free function from `gnu/malloca.c` in `src/gnu/malloca.rs`, using the same module-local bookkeeping conventions as the allocation function so heap-backed allocations are released correctly while stack-like cases remain no-op as appropriate. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/gnu/malloca.rs` to remove placeholder code, align naming and visibility with surrounding Rust module conventions, and verify the two-function port remains self-contained to the `gnu/malloca.c` migration scope. Depends on: T005