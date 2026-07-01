# Implementation Plan: module_src_parseopt_position_06

## Summary

This module ports the position-tracking and line-flushing logic currently implemented in `src/parseopt/wordwrap.c` into Rust, preserving the existing behavior and call relationships without adding new formatting features. The Rust implementation should keep the logic localized to a single Rust module corresponding to the current C file and migrate the functions in dependency order: position helpers first, then whitespace scanning, then line flush behavior.

The main technical approach is:

- represent the C position state with a small Rust struct using integer fields,
- translate the helper functions (`position_init`, `position_incr`, `position_add`, `position_eq`) into straightforward methods or free functions with equivalent mutation semantics,
- port `wordwrap_last_ws` as a slice/string scan routine matching the original whitespace-search behavior,
- port `flush_line` with explicit ownership and borrowing rules so buffer updates are safe and deterministic.

The port should prefer direct translation over redesign. Any anonymous C structs used only within `wordwrap.c` should become private Rust structs scoped to the Rust module. Memory safety should come from owned `String`/`Vec` storage and bounded indexing rather than manual pointer arithmetic.

## Technical Context

### Language / Version

- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies

- Rust standard library only

No third-party crates are recommended from the available evidence. The module appears to be self-contained text-processing logic and should be implemented with:

- `String`
- `&str`
- `Vec`
- integer primitives such as `usize` and possibly signed integers only if required by the C logic

### Testing

- `cargo test`

Testing should include:

- unit tests for all position helper functions,
- targeted tests for whitespace detection behavior in `wordwrap_last_ws`,
- integration-style unit tests for `flush_line` behavior using representative wrapped and non-wrapped line inputs,
- edge cases for empty input, trailing whitespace, no-whitespace lines, and repeated position updates.

### Performance Goals

- preserve linear-time scanning behavior for line and whitespace searches,
- avoid unnecessary string cloning during line flush operations,
- keep heap allocations limited to the existing logical buffering needs of the C implementation,
- maintain performance suitable for command-line text formatting workloads, with no asymptotic regressions from the C code.

## Module Mapping

### Source Mapping

| C Source File | Rust Target |
|---|---|
| `src/parseopt/wordwrap.c` | `src/parseopt/wordwrap.rs` |

If the Rust project already exposes parseopt modules through `mod.rs` or `src/parseopt.rs`, only the minimal module declaration needed to compile `wordwrap.rs` should be added.

### Function Mapping

| C Function | Rust Mapping |
|---|---|
| `position_init` | `Position::init(...)` or `fn position_init(...)` in `src/parseopt/wordwrap.rs` |
| `position_incr` | `Position::incr(...)` or `fn position_incr(...)` |
| `position_add` | `Position::add(...)` or `fn position_add(...)` |
| `position_eq` | `Position::eq_pos(...)` or `fn position_eq(...)` |
| `wordwrap_last_ws` | private helper function in `src/parseopt/wordwrap.rs` |
| `flush_line` | private or crate-visible function in `src/parseopt/wordwrap.rs`, matching current usage |

### Visibility Guidance

- Keep position helpers private unless they are referenced from outside the original C file’s equivalent scope.
- Keep `wordwrap_last_ws` private.
- Expose `flush_line` only if the surrounding Rust port requires it outside the module; otherwise keep it private.

## Data Model

The analysis reports only anonymous C data structures, which strongly suggests file-local structs or transient compound state. The Rust mapping should therefore remain conservative and file-local.

### Data-Structure Mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous position-like struct | `struct Position { ... }` | Use named fields reflecting the C semantics discovered during porting. Integer types should default to `usize` when representing indexes/columns. |
| anonymous line/buffer state structs | private `struct` definitions in `src/parseopt/wordwrap.rs` | Create only the structs that are directly needed to express the existing `flush_line` and wrapping state. |
| anonymous temporary record types | local structs or tuples | If a C anonymous struct is only used for local grouping, prefer a local Rust struct only when it improves direct translation. Otherwise use explicit locals. |

### Core Rust Representation Decisions

#### Position State

The functions `position_init`, `position_incr`, `position_add`, and `position_eq` indicate a small mutable value object. In Rust:

- use a plain `struct Position`,
- derive `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq` if field semantics allow it,
- implement direct mutating methods for increment/add behavior.

This keeps the logic close to the C code while reducing call-site verbosity.

#### Text Buffers

Where the C code uses mutable character buffers and pointer offsets:

- prefer `String` for owned line buffers,
- use `&str` and byte indexes carefully,
- use `Vec<u8>` only if the C logic depends on in-place byte-level mutation that is awkward with `String`.

Because whitespace detection and wrapping often operate on textual boundaries, the first implementation should assume byte-oriented compatibility with the original C behavior unless the source clearly depends on multibyte character semantics.

#### Optional / Error States

If the C code signals “not found” or invalid positions through sentinel values:

- map these to `Option<usize>` or `Option<Position>` where practical,
- only preserve sentinel integers internally if needed for a direct migration step.

This improves safety while keeping behavior equivalent.

#### Memory Management

- replace all manual allocation and lifetime management with owned Rust values,
- avoid storing raw references into buffers across mutations,
- recompute indexes after mutations when needed rather than emulating pointer aliasing.

## Implementation Phases

## Phase 1: Establish module skeleton and port position helpers

### Goals

Create the Rust module corresponding to `src/parseopt/wordwrap.c` and migrate the independent position-manipulation logic first.

### Tasks

- Add `src/parseopt/wordwrap.rs`.
- Define the Rust `Position` struct for the C position state.
- Port:
  - `position_init`
  - `position_incr`
  - `position_add`
  - `position_eq`
- Preserve field semantics and mutation order exactly as in the C implementation.
- Add focused unit tests for:
  - initialization,
  - single increment behavior,
  - additive updates,
  - equality comparison.

### Completion Criteria

- Position logic compiles independently.
- Tests confirm behavior parity for normal and edge values.
- No unsafe code is introduced for this phase.

## Phase 2: Port whitespace scan logic from `wordwrap_last_ws`

### Goals

Translate the whitespace-location helper into safe Rust using bounded scans over the current line buffer.

### Tasks

- Inspect the C implementation of `wordwrap_last_ws` and identify:
  - input buffer type,
  - search range,
  - return contract for “not found,”
  - exact whitespace classification used.
- Implement a private Rust helper that mirrors the C scan direction and stop conditions.
- Use explicit index-based scanning over the underlying bytes if the C logic is byte-oriented.
- Convert any sentinel “not found” result into `Option<usize>` internally, unless direct integer parity is required by the surrounding port.
- Add unit tests covering:
  - empty buffer,
  - buffer with no whitespace,
  - first/last character whitespace,
  - multiple whitespace regions,
  - trailing whitespace behavior.

### Completion Criteria

- The helper matches the C function’s search results for representative inputs.
- Return-value handling is clear and does not rely on unchecked indexing.

## Phase 3: Port line flush behavior from `flush_line`

### Goals

Migrate the stateful line emission/update logic while preserving output sequencing and wrap decisions.

### Tasks

- Identify the exact state manipulated by `flush_line` in `wordwrap.c`.
- Introduce only the minimal private Rust structs needed to represent that state.
- Port `flush_line` directly, preserving:
  - buffer clearing/reset behavior,
  - position updates,
  - interaction with whitespace split points,
  - output/write ordering.
- Replace pointer mutation with:
  - owned buffer mutation through `String`/`Vec`,
  - explicit slicing/index calculations,
  - borrow-scoped helper calls.
- Handle any fallible output operations with Rust `Result` if the surrounding API already supports it; otherwise keep behavior aligned with the existing local control flow and contain failures to the current abstraction boundary.

### Completion Criteria

- `flush_line` compiles with the surrounding module state.
- State transitions after a flush match the C code for wrapped and unwrapped cases.
- Tests cover representative flush scenarios and buffer-reset behavior.

## Phase 4: Final integration and parity verification

### Goals

Integrate all migrated pieces into the Rust branch with minimal structural change and verify behavior consistency.

### Tasks

- Wire `wordwrap.rs` into the existing Rust module tree.
- Adjust call sites only as required by Rust ownership and visibility rules.
- Remove any temporary compatibility code introduced during the port.
- Add end-to-end module tests that exercise:
  - repeated position updates through multiple lines,
  - flush operations after whitespace and non-whitespace boundaries,
  - empty and short line handling.
- Run `cargo test` and fix any parity issues discovered during translation.

### Completion Criteria

- The Rust module is the direct replacement for the C logic in scope for this port.
- All tests pass under `cargo test`.
- The implementation remains limited to the migrated file/function scope without extra abstractions or feature expansion.