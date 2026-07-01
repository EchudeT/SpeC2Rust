# Tasks: main_root_xmalloc.c_29

## Phase 1: Setup

- [T001] [Story] Initialize the Rust module file for the `xmalloc.c` port in `src/xmalloc.rs`, and expose it from the crate root if needed by the `pwd` binary flow.
- [T002] [P] [Story] Review the C functions in `xmalloc.c` and map each one to planned Rust function signatures in `src/xmalloc.rs`, keeping naming and responsibility aligned with the source module.

## Phase 2: Foundational

- [T003] [Story] Define the foundational allocation/error-handling helpers required by `xmalloc.c` in `src/xmalloc.rs`, including any shared internal result or abort path shape needed by multiple allocation wrapper functions. Depends on: T001, T002

## Phase 3: Allocation wrapper functions

- [T004] [Story] Implement the basic allocation wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, covering the core memory-allocation entry points as a single grouped migration unit. Depends on: T003
- [T005] [P] [Story] Implement the reallocation-oriented wrapper functions from `xmalloc.c` in `src/xmalloc.rs`, grouped around resize behavior and shared failure handling. Depends on: T003
- [T006] [P] [Story] Implement the remaining duplication/convenience allocation helpers from `xmalloc.c` in `src/xmalloc.rs`, keeping behavior consistent with the original module’s allocation semantics. Depends on: T003

## Phase 4: Module integration

- [T007] [Story] Integrate all six ported `xmalloc.c` functions in `src/xmalloc.rs` so their visibility, internal helper usage, and call patterns are consistent within the `main_cluster` Rust port. Depends on: T004, T005, T006

## Final Phase: Polish

- [T008] [Story] Refine `src/xmalloc.rs` for idiomatic Rust structure, remove redundant migration scaffolding, and ensure error paths and allocation semantics remain aligned with the original `xmalloc.c` behavior. Depends on: T007