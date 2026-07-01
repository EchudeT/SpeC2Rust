# Implementation Plan: main_root_cat.c_19

## Summary

Port `cat.c` from the C main cluster into a Rust module that preserves the existing command-line behavior and streaming I/O flow without adding new capabilities. The Rust implementation should keep the same functional split as the source analysis: argument/help handling, line-number progression, simple byte-copy mode, buffered pending-output writes, and the main formatted copy path.

The technical approach is a direct migration to idiomatic but restrained Rust:

- keep the logic concentrated in a single Rust source file for this module area,
- map C file-scope state and helper routines into private functions and compact state structs,
- use `std::io::{Read, Write, BufRead}`-style primitives for streaming input/output,
- represent recoverable failures with `std::io::Result` or a small module-local error path,
- avoid extra abstraction layers unless required by the existing function boundaries.

The implementation should prioritize behavioral equivalence, especially around buffered copying, line-number state progression, and pending-write handling.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum practical compiler target: Rust 1.74+

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:
- None required for this module based on the available input

### Testing

- `cargo test`

Testing focus:
- unit tests for line-number increment behavior
- unit tests for pending-buffer write handling
- integration-style tests for simple copy and formatted copy paths using temporary files and in-memory buffers where practical
- command-path tests for usage/help output shape only if already covered by the surrounding crate structure

### Performance Goals

- Preserve streaming behavior; do not load whole files into memory
- Keep buffered read/write paths comparable to the C implementation
- Avoid unnecessary allocations in copy loops
- Maintain predictable throughput for large file concatenation workloads
- Keep line-number formatting/state updates constant-cost per line

## Module Mapping

### C to Rust File Mapping

- `cat.c` -> `src/bin/cat.rs`

If the repository already uses a different binary entry layout, keep the port in the existing binary file and migrate these functions into that file rather than creating extra modules.

### Function Mapping

- `usage` -> `fn usage(...)`
  - Private helper unless the existing crate entrypoint requires wider visibility
  - Emits usage/help text and returns the appropriate exit path

- `next_line_num` -> `fn next_line_num(...)`
  - Private helper operating on a mutable line-number state buffer/counter

- `simple_cat` -> `fn simple_cat<R: Read, W: Write>(...) -> io::Result<()>`
  - Direct byte-stream copy path for the uncomplicated mode

- `write_pending` -> `fn write_pending<W: Write>(...) -> io::Result<()>`
  - Private helper to flush deferred buffered output slices

- `cat` -> `fn cat<R: Read, W: Write>(...) -> io::Result<()>`
  - Main processing path for option-sensitive output behavior

- `copy_cat` -> `fn copy_cat(...) -> io::Result<()>`
  - Dispatcher coordinating selected copy mode across inputs

### Entrypoint Relationship

If `cat.c` currently hosts the executable’s main logic, Rust should keep:
- argument parsing and top-level dispatch in the binary entry file,
- the migrated helpers as local/private functions in the same file,
- no additional façade modules unless already required by the project layout.

## Data Model

The analysis reports only anonymous C structures. For this port, keep the Rust data model minimal and driven by actual helper-state needs observed during migration.

### Anonymous C Struct Mapping

- `anonymous` -> `struct LineNumberState`
  - Purpose: hold mutable line-number formatting/progression state used by `next_line_num`
  - Likely fields in Rust:
    - numeric counter, e.g. `u64`
    - optional fixed-width ASCII digit buffer, e.g. `[u8; N]`, if the C logic updates a printable number in place
  - Decision rule:
    - use a numeric counter alone if formatting can be reproduced cheaply at write time without changing behavior materially
    - use an in-place byte buffer if the C routine depends on incremental digit mutation for exact output handling/performance

- `anonymous` -> `struct PendingWrite<'a>`
  - Purpose: represent deferred output segments handled by `write_pending`
    - borrowed byte slice for pending data, `&'a [u8]`, or
    - owned temporary buffer `Vec<u8>` only if borrow-based flow is not practical with the final control structure
  - Prefer borrowed slices and explicit offsets to avoid unnecessary allocation

### Ownership and Memory Management

- Replace raw pointers and manual buffer arithmetic with slices and indices
- Keep read buffers as reusable `Vec<u8>` or fixed-size stack arrays depending on the translated logic
- Use mutable references for shared state such as line numbering and pending-output tracking
- Avoid `unsafe` unless a verified one-to-one low-level translation is absolutely necessary; default expectation is safe Rust only

### Error Handling

- Replace C write/read status checks with `io::Result`
- Propagate I/O failures with `?`
- Convert top-level failures into exit status handling in the binary entry path
- Preserve short-write correctness by using `write_all` where equivalent to C intent; if `write_pending` models partial progress explicitly, keep that logic visible rather than hiding it behind unrelated abstractions

## Implementation Phases

### Phase 1: Establish file-local Rust skeleton and shared state

- Create or update the Rust binary file corresponding to `cat.c`
- Add Rust function stubs matching:
  - `usage`
  - `next_line_num`
  - `simple_cat`
  - `write_pending`
  - `cat`
  - `copy_cat`
- Identify anonymous C state usages during translation and define only the required Rust structs:
  - `LineNumberState`
  - `PendingWrite` if needed
- Wire top-level argument flow to the migrated functions without introducing new module layers
- Define common buffer sizes and local constants only where directly required by the C source

### Phase 2: Port the core copy paths

- Implement `simple_cat` as the straightforward buffered read/write loop
- Implement `write_pending` to handle deferred output slices and partial-write correctness
- Implement `cat` by translating the main processing loop as closely as possible:
  - preserve buffer-walking behavior,
  - preserve line-boundary handling,
  - preserve numbering-state updates through `next_line_num`
- Implement `next_line_num` with a direct state mutation model matching the original logic
- Keep all processing streaming and allocation-light

### Phase 3: Complete dispatch and usage handling

- Implement `copy_cat` to iterate over requested inputs and choose the correct copy path
- Integrate stdin/file opening using standard-library file and stdin handles
- Implement `usage` with the existing command interface expectations from the surrounding binary
- Ensure top-level return paths map Rust errors to the crate’s existing exit-status convention
- Verify no functionality beyond the migrated C behavior is introduced

### Phase 4: Validate parity with focused tests

- Add unit tests for:
  - `next_line_num` progression across boundary values
  - `write_pending` behavior with full and segmented writes
- Add copy-path tests for:
  - simple streaming copy from input to output
  - formatted/main `cat` path on representative line-oriented inputs
  - multi-input dispatch in `copy_cat`
- Run `cargo test`
- Fix translation mismatches discovered during parity checks while keeping the file/function structure restrained