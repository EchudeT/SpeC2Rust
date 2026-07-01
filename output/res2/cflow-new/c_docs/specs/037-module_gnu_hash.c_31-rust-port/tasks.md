# Tasks: module_gnu_hash.c_31

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `gnu/hash.c` in `src/module_gnu_hash.rs`, and expose it from the crate root or parent module so the ported implementation has a dedicated target file on branch `037-module_gnu_hash.c_31-rust-port`.
- [T002] [P] [Story] Add the initial item layout in `src/module_gnu_hash.rs` for the module port, including placeholders for the module data structures and the 9 function signatures identified from `gnu/hash.c`.
  - Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Port the foundational data structures from `gnu/hash.c` into Rust in `src/module_gnu_hash.rs`, defining the core structs, enums, type aliases, and constants needed to represent the module state and hash-related records.
  - Depends on: T002
- [T004] [Story] Complete the remaining supporting data-structure definitions from `gnu/hash.c` in `src/module_gnu_hash.rs`, covering helper records, internal field layouts, and ownership/borrowing choices required before function bodies can be implemented.
  - Depends on: T003
- [T005] [P] [Story] Normalize the foundational APIs in `src/module_gnu_hash.rs` by adding constructors, default initialization paths, and internal helper methods only where directly required to instantiate the ported data structures safely.
  - Depends on: T004

## Phase 3: Core Hash State and Initialization Functions

- [T006] [Story] Implement the function group in `src/module_gnu_hash.rs` responsible for creating, initializing, or resetting GNU hash module state from `gnu/hash.c`, using the Phase 2 data structures without expanding behavior beyond the source module.
  - Depends on: T005
- [T007] [Story] Implement the related internal helper functions in `src/module_gnu_hash.rs` that prepare table metadata, counters, or field defaults used immediately by the initialization flow from `gnu/hash.c`.
  - Depends on: T006

## Phase 4: Hash Computation and Lookup Functions

- [T008] [Story] Implement the function group in `src/module_gnu_hash.rs` that performs GNU hash value computation and any direct key-processing logic defined in `gnu/hash.c`.
  - Depends on: T005
- [T009] [P] [Story] Implement the function group in `src/module_gnu_hash.rs` that performs table lookup, bucket/chain traversal, or entry matching behavior defined in `gnu/hash.c`.
  - Depends on: T008
  - Depends on: T006

## Phase 5: Mutation, Insert, and Cleanup Functions

- [T010] [Story] Implement the function group in `src/module_gnu_hash.rs` that mutates the hash state, including insertion, update, or removal-style operations present in `gnu/hash.c`.
  - Depends on: T009
- [T011] [Story] Implement the function group in `src/module_gnu_hash.rs` that finalizes, clears, or frees module-managed GNU hash state in parity with the cleanup logic from `gnu/hash.c`.
  - Depends on: T010

## Final Phase: Polish

- [T012] [Story] Refine `src/module_gnu_hash.rs` for Rust correctness and module-local clarity by removing temporary placeholders, tightening visibility, and aligning naming and ownership decisions with the completed `gnu/hash.c` port.
  - Depends on: T011
- [T013] [P] [Story] Perform a final pass on `src/module_gnu_hash.rs` to simplify obvious redundant allocations or control flow introduced during the port, while preserving source-module behavior and keeping scope limited to this file.
  - Depends on: T012