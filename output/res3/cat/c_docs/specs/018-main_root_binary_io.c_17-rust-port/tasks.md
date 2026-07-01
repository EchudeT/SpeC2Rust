# Tasks: main_root_binary-io.c_17

## Phase 1: Setup

- [T001] [Story] Create the Rust module target for `binary-io.c` in `src/binary_io.rs`, and wire its declaration from the crate entry so the `018-main_root_binary_io.c_17-rust-port` branch has a dedicated port location for this module.
  - Depends on: none

## Phase 2: Foundational

- [T002] [Story] Review `binary-io.c` and establish the minimal Rust-side internal item layout in `src/binary_io.rs` needed to host the module’s single function port, keeping signatures and helper item placeholders limited to evidence from the C source.
  - Depends on: T001

## Phase 3: Functions

- [T003] [Story] Port the single function implemented by `binary-io.c` into `src/binary_io.rs`, preserving its binary I/O behavior and adapting the C control flow to idiomatic Rust while staying within the original module scope.
  - Depends on: T002

## Final Phase: Polish

- [T004] [Story] Refine `src/binary_io.rs` by removing temporary scaffolding introduced during porting, tightening imports and visibility, and confirming the module integrates cleanly with the crate entry updated in setup.
  - Depends on: T003