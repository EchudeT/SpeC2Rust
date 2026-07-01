# Implementation Plan

## Summary

Port the `cat.c` main-cluster logic to Rust as a single focused module that preserves the existing file- and stream-copy behavior, line-number state handling, buffered output flushing, and command-line usage emission already present in the C implementation.

The Rust implementation should stay close to the current C structure and migrate the existing functions directly rather than redesigning behavior. The technical approach is:

- keep one Rust source module corresponding to `cat.c`
- translate the procedural control flow of `usage`, `next_line_num`, `simple_cat`, `write_pending`, `cat`, and `copy_cat` into Rust functions with minimal restructuring
- represent transient C state with small Rust structs where needed, especially for output buffering and line-number tracking
- use `std::io::{Read, Write, BufRead}` and explicit byte-buffer processing to preserve behavior around raw copying and line-oriented state transitions
- replace C error-code and manual buffer management with `Result`-based propagation and owned stack/heap buffers from the standard library

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve streaming behavior for stdin and file inputs without loading whole files into memory
  - use reusable byte buffers for copy paths corresponding to `simple_cat` and `copy_cat`
  - avoid unnecessary UTF-8 conversion; process bytes directly
  - keep output buffering explicit enough to match C behavior where pending output must be flushed in order

## Module Mapping

- **C source**: `cat.c`
- **Rust target**: `src/main.rs`

Function migration mapping:

- `usage` -> `fn usage(...) -> !` or `fn usage(...) -> ExitCode` depending on surrounding CLI flow already used in the Rust port
- `next_line_num` -> `fn next_line_num(state: &mut LineNumberState)`
- `simple_cat` -> `fn simple_cat<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()>`
- `write_pending` -> `fn write_pending<W: Write>(output: &mut W, state: &mut OutputState) -> io::Result<()>`
- `cat` -> `fn cat<R: Read, W: Write>(input: &mut R, output: &mut W, state: &mut CatState) -> io::Result<()>`
- `copy_cat` -> `fn copy_cat(...) -> io::Result<()>` as the top-level per-input dispatch for the copied C logic

Module scope should remain restrained: one main Rust file is sufficient unless the branch already has a split layout that this port must match.

## Data Model

The C analysis only exposes anonymous structures, so the Rust plan should introduce small named structs only where needed to carry migrated state from function boundaries.

### C anonymous struct -> Rust mapping

- **anonymous output/pending-buffer state** -> `struct OutputState`
  - likely fields:
    - pending byte buffer: `Vec<u8>` or fixed reusable buffer if the C code used a bounded region
    - current pending length/index counters as `usize`
  - purpose:
    - support `write_pending`
    - preserve ordered writes and partial-write retry handling through `Write::write_all`

- **anonymous line-number state** -> `struct LineNumberState`
    - current line counter digits or numeric counter
    - preformatted line-number buffer if the C code mutates digit characters in place
  - preferred Rust representation:
    - first choice: numeric counter plus formatting into a small stack buffer when needed
    - if exact in-place digit carry behavior is important to preserve, model the digit array directly as `[u8; N]` and advance it in `next_line_num`

### Aggregate migrated state

To avoid widening function signatures while staying close to the C code, combine related state only if the C code shares it broadly:

```rust
struct CatState {
    line_number: LineNumberState,
    output: OutputState,
    // boolean flags and mode switches migrated from existing cat.c locals/options
}
```

This should only include fields already implied by the current module’s functions. Do not introduce unrelated abstraction layers.

### Memory management and error handling decisions

- replace raw C buffers with Rust-owned buffers (`[u8; N]`, `Vec<u8>`)
- replace manual lifetime/alias management with mutable borrows scoped to function calls
- replace C integer status returns and write checks with `io::Result<()>`
- convert top-level operational failures into exit status handling at the `main` path already used in the Rust branch
- avoid `unsafe` unless a specific C behavior cannot be translated with safe I/O APIs; expected plan is fully safe Rust

## Implementation Phases

### Phase 1: Skeleton and direct function port

- create or update `src/main.rs` for this module’s port
- add Rust equivalents for:
  - `usage`
  - `next_line_num`
  - `simple_cat`
  - `write_pending`
  - `cat`
  - `copy_cat`
- define minimal state structs needed to replace the anonymous C structures
- map C primitive types and counters to Rust types:
  - sizes/indices -> `usize`
  - byte data -> `u8`
  - booleans/flags -> `bool`
- keep control flow close to the original C implementation to reduce migration risk

### Phase 2: I/O path and state-behavior completion

- implement byte-oriented read/write loops for the simple copy path
- implement the main `cat` path using explicit state transitions for:
  - line numbering advancement
  - pending-output accumulation and flush behavior
  - per-byte or per-line processing as required by the original logic
- ensure `write_pending` centralizes flush behavior and partial-write handling through standard-library writes
- verify EOF handling and repeated processing across multiple inputs match the C structure

### Phase 3: CLI integration and error propagation

- connect `usage` and `copy_cat` into the existing Rust program entry flow on branch `020-main_root_cat.c_19-rust-port`
- preserve the current module’s exit behavior without adding new CLI features
- route all operational errors through `Result` and emit user-facing failures only at the outer command level
- ensure stdin and named-file handling follow the same dispatch shape as the C code

### Phase 4: Focused tests and parity cleanup

- add `cargo test` unit tests for:
  - line-number state advancement in `next_line_num`
  - pending-buffer flush behavior in `write_pending`
  - raw byte copying in `simple_cat`
  - representative `cat`/`copy_cat` cases using in-memory readers/writers
- add regression tests for edge conditions visible from the ported logic:
  - empty input
  - final line without trailing newline
  - multiple sequential inputs
  - write of buffered pending data at EOF
- perform final cleanup to remove C-style artifacts that are no longer needed, while keeping function boundaries and behavior aligned with the original module