# Implementation Plan: main_root_version_etc_06

## Summary

This module ports the logic from `version-etc.c` into Rust for the `pwd` project branch `006-main_root_version_etc_06-rust-port`. The scope is limited to migrating the existing version-reporting entry points:

- `version_etc_arn`
- `version_etc_ar`
- `version_etc_va`
- `version_etc`

The Rust implementation should preserve the current formatting and output behavior expected from the C code while adapting variadic and array-based interfaces into idiomatic, memory-safe Rust functions. The technical approach is to implement a small Rust module responsible for assembling and writing version/help text to a generic output target using the standard library, with thin function mappings that preserve the original call shapes as closely as practical in Rust.

The migration should avoid introducing new abstraction layers beyond what is needed to replace C string handling, array traversal, and output writing. Memory ownership will move from raw pointer and variadic C conventions to borrowed Rust string slices and slice-based author lists. Error handling should be explicit where writing can fail.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep output generation linear in the number of authors and emitted bytes.
  - Avoid unnecessary string cloning where borrowed `&str` or streaming writes are sufficient.
  - Match C behavior closely without adding buffering or allocation-heavy formatting layers unless required by Rust APIs.

## Module Mapping

### C to Rust File Mapping

- `version-etc.c` → `src/version_etc.rs`

### Function Mapping

The Rust module should provide direct replacements for the C entry points, adapted to Rust calling conventions:

- `version_etc_arn`
  → Rust function handling:
  - output target as `&mut impl std::io::Write` or a concrete internal helper over `Write`
  - command/package/version as `&str`
  - author list as a borrowed slice
  - explicit author count is not needed separately if represented by a slice

- `version_etc_ar`
  → Rust wrapper over the same internal implementation, using a slice of authors

- `version_etc_va`
  → Rust helper replacing variadic traversal with slice-based input; no Rust variadic interface should be introduced

- `version_etc`
  → Rust convenience wrapper calling the shared implementation with the provided fixed inputs

### Suggested Rust Module Surface

Keep the public surface narrowly aligned to migration needs:

- one shared internal formatter/writer function
- small wrapper functions corresponding to the original C functions
- no extra support modules unless required by compilation or existing crate layout

## Data Model

This module has no standalone C struct definitions in the provided analysis.

### C to Rust Data Representation

- C string inputs (`char *`, `const char *`)
  → `&str` for borrowed UTF-8 text when inputs are already normalized in Rust
  → if existing crate boundaries require byte-preserving conversion, use `&std::ffi::OsStr` only where explicitly needed by surrounding code; otherwise default to `&str`

- C author array (`const char *const *` or equivalent)
  → `&[&str]`

- C `FILE *` output target
  → `&mut dyn std::io::Write` or generic `W: Write`

- C variadic arguments (`va_list`, `...`)
  → `&[&str]` passed through explicit wrapper functions

### Memory Management Notes

- Eliminate raw-pointer ownership concerns by using borrowed Rust references.
- Do not build temporary owned `String` values unless formatting convenience requires it; prefer `write!`/`writeln!` to stream directly to the output target.
- Keep helper signatures borrowing all inputs for the duration of the call only.

### Error Handling Notes

- C output routines may ignore or defer write failures; in Rust, writing returns `std::io::Result<()>`.
- Internal writer functions should return `io::Result<()>`.
- If the surrounding crate expects infallible top-level behavior, wrappers may convert write errors according to existing crate conventions, but no new recovery mechanism should be added.

## Implementation Phases

## Phase 1: Establish Rust module and shared formatter

- Create `src/version_etc.rs`.
- Identify the exact emitted text and branch logic in `version-etc.c`.
- Implement one shared internal function that:
  - accepts an output writer
  - accepts command/package/version text inputs
  - accepts an author slice
  - emits the version text in the same order and formatting as the C implementation
- Use only standard-library formatting and `std::io::Write`.
- Keep all logic in this module rather than introducing auxiliary formatting layers.

### Deliverables

- Rust module file with internal shared implementation
- Initial unit tests for basic single-author and multi-author formatting behavior

## Phase 2: Port the C entry points into Rust wrappers

- Add Rust functions corresponding to:
  - `version_etc_arn`
  - `version_etc_ar`
  - `version_etc_va`
  - `version_etc`
- Map all wrappers onto the shared implementation.
- Replace C count-plus-pointer handling with slice-based inputs.
- Replace variadic behavior with explicit slice passing in the Rust-facing API.
- Ensure wrapper naming and visibility align with how the rest of the `pwd` crate references this functionality.

### Deliverables

- Complete function coverage for all four analyzed entry points
- Wrapper-level tests confirming equivalent dispatch into the shared implementation

## Phase 3: Validate formatting parity and edge handling

- Add tests for:
  - zero authors if permitted by the original C logic
  - one author
  - two authors
  - larger author lists
  - command/package/version combinations that affect line formatting
- Verify no extra trailing spaces, missing newlines, or altered separators are introduced.
- Confirm all write paths return or propagate `io::Result<()>` consistently.
- Remove any temporary allocation-heavy code if direct writing can replace it without changing behavior.

### Deliverables

- Finalized test coverage for formatting branches
- Cleaned module with stable error propagation and no unnecessary allocations

## Phase 4: Integrate into crate layout and finalize migration

- Wire `src/version_etc.rs` into the crate with the minimal required `mod`/`pub use` changes.
- Replace any references to the C implementation in the Rust branch with calls to the new module.
- Run `cargo test` and fix any integration mismatches in signatures or visibility.
- Keep final scope limited to this module migration only.

### Deliverables

- Integrated Rust module in branch `006-main_root_version_etc_06-rust-port`
- Passing test suite for the migrated functionality

## Acceptance Criteria

- `version-etc.c` functionality is represented in Rust by `src/version_etc.rs`.
- All four listed functions are migrated with Rust-compatible interfaces.
- Output formatting matches the original C behavior for covered author-count cases.
- Memory handling uses borrowed Rust references instead of raw pointers.
- Write failures are handled through `std::io::Result` without adding non-required infrastructure.
- `cargo test` passes for module-level and integration-level checks.