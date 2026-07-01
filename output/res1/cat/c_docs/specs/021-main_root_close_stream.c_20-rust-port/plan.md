# Implementation Plan

## Summary

This module ports the C file `close-stream.c` into a focused Rust implementation that preserves the existing responsibility: closing an output stream and reporting close/write failures in a way compatible with the surrounding `cat` program behavior.

The Rust approach should stay narrow and map the existing logic into a small helper function in the main program area rather than introducing new abstractions. The implementation should use Rust standard library I/O types and error propagation, with explicit handling for buffered output flush and final close/drop behavior. Because Rust does not expose C-style `fclose` semantics directly for arbitrary `FILE *`, the practical migration target is the Rust writer path used by the ported `cat` binary, where flushing buffered output and surfacing any pending write errors is the main technical equivalent.

Key migration goals:

- Move the `close_stream` logic into a Rust module under the main program cluster.
- Preserve observable error reporting points around stream finalization.
- Avoid expanding scope beyond the existing file/function responsibility.
- Keep ownership and lifetime rules explicit so stream finalization is deterministic at the call site.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fs` if needed by callers)
- **Testing**: `cargo test`
- **Performance Goals**:
  - No meaningful regression versus the C helper for normal output paths.
  - Keep stream-finalization overhead constant-time aside from required flush behavior.
  - Avoid extra buffering layers or allocations beyond what the Rust caller already uses.

## Module Mapping

| C File | C Function | Rust Target |
|---|---|---|
| `close-stream.c` | `close_stream` | `src/main_root/close_stream.rs` with `pub(crate) fn close_stream(...)` |

### Proposed Rust placement

- `src/main_root/close_stream.rs`
  - Contains the direct Rust port of `close_stream`.
- `src/main_root/mod.rs` or the existing main-cluster module file
  - Re-exports or wires in the helper for use by the ported `cat` main flow.

### Mapping notes

- Keep this as a single-purpose helper module.
- Do not split error formatting, diagnostics, or stream wrappers into additional support modules unless already required by the existing Rust port structure.
- The function signature should be chosen to fit the actual writer object used by the Rust `cat` main path, likely a generic writer bound by `Write` and ownership-consuming close semantics at the call site.

## Data Model

This C module does not define module-local data structures.

### C to Rust type mapping

| C Concept | Rust Mapping |
|---|---|
| `FILE *` stream parameter | A Rust writer object used by the caller, likely `std::io::Stdout`, `std::fs::File`, or `std::io::BufWriter<W>` |
| integer status return | `std::io::Result<()>` internally, with caller-level conversion to process status if needed |
| `errno`-based failure | `std::io::Error` |

### Data-handling decisions

- Prefer returning `io::Result<()>` from the Rust helper.
- If the surrounding port already uses explicit exit-status accumulation rather than `Result`, keep `close_stream` internally result-based and convert at the immediate call site.
- Use ownership or `&mut` borrowing based on the writer lifecycle:
  - `&mut W` if the caller retains the writer object and only needs flush/finalization semantics.
  - Owned buffered writer if dropping the wrapper is the intended close point.
- No custom Rust struct is needed for this module unless the existing port already wraps output streams in a project-local type.

## Implementation Phases

## Phase 1: Inspect and define the Rust function boundary

### Goals
- Identify the exact call sites in the Rust `cat` port that correspond to the C module’s `close_stream` usage.
- Define the narrowest Rust function signature that matches those call sites.

### Tasks
- Review `close-stream.c` behavior and note:
  - whether it only closes,
  - whether it flushes before close,
  - how it distinguishes prior write failure from close failure,
  - what status form it returns.
- Review the Rust branch’s current output path:
  - direct `Stdout`,
  - `File`,
  - `BufWriter`,
  - generic `Write`.
- Choose one concrete function form, for example:
  - `pub(crate) fn close_stream<W: Write>(stream: &mut W) -> io::Result<()>`
  - or a slightly more concrete variant if the caller uses a buffered file writer.
- Record how the caller will relinquish ownership so Rust drop semantics complete the close after any explicit flush.

### Deliverable
- A compile-ready module skeleton with the final function signature and imports.

## Phase 2: Port close/finalization logic

### Goals
- Implement the Rust equivalent of `close_stream` without adding broader infrastructure.

### Tasks
- Create `src/main_root/close_stream.rs`.
- Implement the helper using standard library I/O:
  - call `flush()` explicitly,
  - return any flush error directly,
  - allow Rust drop/ownership end to handle descriptor closure where applicable.
- If the caller holds an owned `BufWriter<File>`, ensure finalization order is clear:
  - flush before object goes out of scope,
  - do not suppress I/O errors.
- Preserve the distinction between successful writing and finalization failure as far as Rust permits through `io::Result`.
- Keep memory management fully ownership-based:
  - no raw pointers,
  - no manual deallocation,
  - no unsafe code unless forced by already-existing project constraints.

### Deliverable
- Working Rust implementation of `close_stream` integrated into the main module tree.

## Phase 3: Wire module usage into the main flow

### Goals
- Replace the original C helper usage pattern with the Rust helper at the relevant output finalization point.

### Tasks
- Update the ported main execution path to call `close_stream` at the same logical stage as the C code.
- Convert `io::Result<()>` into the program’s existing error/status handling pattern.
- Ensure diagnostics are emitted by the same layer that currently handles command failures; do not duplicate reporting inside `close_stream` unless the surrounding Rust port already centralizes nothing.
- Confirm that ownership and scope end immediately after finalization where close timing matters.

### Deliverable
- End-to-end main-path integration with no unused legacy placeholder for this helper.

## Phase 4: Add focused tests for stream finalization behavior

### Goals
- Validate the ported helper’s observable behavior with minimal, targeted tests.

### Tasks
- Add unit tests for:
  - successful flush/finalization on an in-memory writer where applicable,
  - propagation of a flush failure using a small custom test writer implementing `Write`,
  - caller-visible status conversion if that logic sits adjacent to this module.
- Prefer local test doubles over external crates.
- Run `cargo test` and confirm the helper behaves consistently with the intended C semantics within Rust’s I/O model.

### Deliverable
- Passing tests covering success and error paths for `close_stream`.