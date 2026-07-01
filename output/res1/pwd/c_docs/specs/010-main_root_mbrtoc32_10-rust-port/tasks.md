# Task List

## Phase 1: Setup

- [ ] T001 [Story] Create the Rust module file `src/main_root_mbrtoc32_10.rs` and declare its integration point from the crate root so the ported `mbrtoc32.c` logic has a dedicated target location.
- [ ] T002 [P] [Story] Add the initial public function stubs in `src/main_root_mbrtoc32_10.rs` for the two functions ported from `mbrtoc32.c`, preserving the C module’s function boundaries for incremental migration.

## Phase 2: Foundational

- [ ] T003 [Story] Review `mbrtoc32.c` state handling and define any module-local Rust aliases or helper representations needed in `src/main_root_mbrtoc32_10.rs` to support the function ports without introducing new data structures not evidenced by the source. Depends on: T001

## Phase 3: Functions

- [ ] T004 [Story] Implement the core multibyte-to-`char32` conversion function from `mbrtoc32.c` in `src/main_root_mbrtoc32_10.rs`, translating the source control flow and result semantics onto Rust-compatible types. Depends on: T002, T003
- [ ] T005 [Story] Implement the remaining helper or entry-point function from `mbrtoc32.c` in `src/main_root_mbrtoc32_10.rs`, keeping its behavior aligned with the original module and reusing the same module-local foundations. Depends on: T002, T003
- [ ] T006 [P] [Story] Reconcile shared logic between the two ported functions in `src/main_root_mbrtoc32_10.rs` so duplicated conversion-path behavior from the initial port is consolidated without changing the original module scope. Depends on: T004, T005

## Final Phase: Polish

- [ ] T007 [Story] Perform a final polish pass on `src/main_root_mbrtoc32_10.rs` to tighten signatures, visibility, and inline documentation comments so the Rust module is consistent with the surrounding `main_cluster` port structure. Depends on: T006