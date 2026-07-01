# Task List: main_root_binary-io.c_17

## Phase 1: Setup

- [T001] [Story] Create the Rust module scaffold for `binary-io.c` on branch `018-main_root_binary_io.c_17-rust-port`, adding the target source file at `src/binary_io.rs` and wiring its module declaration into the existing crate entry integration used by the `cat` main cluster.
- [T002] [P] [Story] Review `binary-io.c` and map its single exported or local function into the Rust target file `src/binary_io.rs`, documenting the intended Rust function signature and its call boundary for later implementation. Depends on: T001

## Phase 2: Foundational

- [T003] [Story] Confirm that `binary-io.c` introduces no module-specific data structures requiring Rust translation; keep `src/binary_io.rs` limited to function-oriented porting scaffolding so later work does not invent unsupported types. Depends on: T002

## Phase 3: Functions

- [T004] [Story] Implement the binary I/O behavior from the single function in `binary-io.c` inside `src/binary_io.rs`, preserving the source module’s control flow and observable behavior within the Rust `cat` main-cluster port. Depends on: T003
- [T005] [Story] Integrate the implemented function from `src/binary_io.rs` into the Rust crate path that replaces the original `binary-io.c` usage, updating the relevant existing main-cluster call site wiring so the ported function is invoked instead of leaving the module disconnected. Depends on: T004

## Final Phase: Polish

- [T006] [Story] Refine `src/binary_io.rs` for idiomatic Rust without changing behavior, removing temporary porting scaffolds, tightening imports, and aligning naming and module visibility with the rest of the `cat` Rust port. Depends on: T005