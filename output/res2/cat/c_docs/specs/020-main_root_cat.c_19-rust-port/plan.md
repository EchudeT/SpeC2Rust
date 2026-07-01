# Implementation Plan

## Summary

Port the `cat.c` main-cluster logic into a Rust module that preserves the existing control flow and I/O behavior of the C implementation without adding new capabilities. The Rust work should focus on migrating the functions `usage`, `next_line_num`, `simple_cat`, `write_pending`, `cat`, and `copy_cat` into a single idiomatic-but-direct module layout that keeps feature behavior aligned with the source file.

The implementation approach should prefer:
- direct translation of file-processing paths into Rust functions,
- standard-library buffered and unbuffered I/O primitives where they match the original behavior,
- explicit state structs for any anonymous C structures,
- `Result`-based error propagation for internal operations, with top-level formatting consistent with command-line utility expectations,
- careful handling of output buffering and pending writes to preserve observable behavior.

The plan should avoid architectural expansion: keep the Rust port centered on the existing `cat.c` responsibilities and migrate functions in dependency order so that low-level write and line-number state handling are established before higher-level file-copy routines.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended at this stage; the provided analysis does not justify external dependencies
- **Testing**:
  - `cargo test`
  - targeted unit tests for line-number progression and pending-write behavior
  - integration-style tests for file/stdin to stdout copying paths where practical
- **Performance Goals**:
  - match the C module’s streaming behavior closely for large inputs
  - avoid unnecessary allocations inside hot copy loops
  - use reusable buffers for read/write paths
  - preserve straightforward fast-path handling for simple copy behavior
  - ensure pending output is flushed deterministically and without extra buffering layers beyond what is needed for parity

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `cat.c` | `src/cat.rs` or `src/main_root_cat_c_19.rs` | Keep all migrated functions together in one module to mirror the original file scope |
| `cat.c` entry-related logic | `src/main.rs` call sites if needed | Only wire existing program flow to the migrated module; do not introduce new command dispatch layers |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `usage` | `fn usage(...)` | Keep as a dedicated formatting/output function; avoid embedding usage text construction across unrelated logic |
| `next_line_num` | `fn next_line_num(state: &mut LineNumberState)` | Model mutable numbering state explicitly rather than pointer arithmetic on buffers |
| `simple_cat` | `fn simple_cat<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()>` | Implement the fast copy path with a reusable byte buffer |
| `write_pending` | `fn write_pending<W: Write>(state: &mut OutputState, output: &mut W) -> io::Result<()>` | Centralize deferred output emission and partial-write handling |
| `cat` | `fn cat<R: Read, W: Write>(...) -> io::Result<()>` | Port the main transformed-copy path with explicit processing state |
| `copy_cat` | `fn copy_cat(...) -> io::Result<()>` | Keep as the orchestration layer selecting simple vs processed path based on existing flags/state |

## Data Model

The source analysis reports two anonymous structures. In Rust, these should be converted into named private structs with field names chosen from actual usage in `cat.c` during implementation. The goal is not redesign, but replacing unnamed mutable C storage with explicit typed state.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous struct #1 | `struct LineNumberState` | Holds the mutable state used by `next_line_num`; likely line counter digits and/or formatted prefix buffer |
| anonymous struct #2 | `struct OutputState` | Holds pending output bytes, buffering offsets, and any state needed by `write_pending` and transformed output |

### Proposed Rust Data Shapes

These names are placeholders for the migration and should be adjusted to match actual source roles once fields are read from `cat.c`.

```rust
struct LineNumberState {
    // counter and/or rendered number buffer fields derived from C usage
}

struct OutputState {
    // pending byte buffer, current length/offset, and related flags if present
}
```

### Memory Management and Ownership

- Replace raw buffer ownership with `Vec<u8>`, fixed-size arrays, or slices depending on source usage.
- Represent mutable shared state by `&mut` borrows passed through the call chain rather than global mutable storage.
- Use stack allocation for fixed formatting buffers where the C code uses fixed arrays.
- Preserve buffer reuse in loops to avoid repeated allocation.
- Avoid copying input data unnecessarily; write directly from read buffers whenever possible.

### Error Handling Strategy

- Convert low-level I/O failure points into `std::io::Result<()>`.
- Keep helper functions composable by returning `io::Result` rather than terminating directly.
- Reserve user-visible diagnostic formatting for the top-level calling layer already responsible for utility behavior.
- Treat partial writes explicitly in `write_pending` and any direct write loop; do not assume a single `write` completes the operation.
- Avoid `panic!` for expected runtime failures such as unreadable files or broken pipes unless the wider project already mandates special handling.

## Implementation Phases

## Phase 1: Establish module skeleton and state mappings

- Create the Rust destination module for `cat.c` as a single file under `src/`.
- Identify the two anonymous C structs from actual field use and define corresponding private Rust structs.
- Port `usage` as a direct standalone function with minimal signature changes required by Rust.
- Port `next_line_num` using an explicit mutable line-number state struct.
- Define internal helper signatures for `write_pending`, `simple_cat`, `cat`, and `copy_cat` before filling in full logic.
- Decide concrete buffer types based strictly on the C file’s existing local/static storage patterns.

**Exit criteria**:
- Module compiles with placeholder bodies for copy functions.
- State structs exist and support line-number updates without unsafe code.

## Phase 2: Port low-level output and simple copy path

- Implement `write_pending` first, including correct handling of short writes and retained pending bytes.
- Implement `simple_cat` as the direct streaming path using `Read`/`Write` and a reusable byte buffer.
- Keep the implementation close to the C loop structure so later behavioral comparison is straightforward.
- Add unit tests for:
  - pending-buffer flush behavior,
  - repeated short-write handling via a custom test writer,
  - direct copy of empty and non-empty input.

**Exit criteria**:
- Fast-path copying is functional and tested.
- Pending output logic is reusable by the transformed path.

## Phase 3: Port the main transformed copy logic

- Implement `cat` using the Rust state structs and low-level output helpers.
- Preserve existing character-processing order and state transitions from the C source.
- Route all deferred output through `write_pending` where the C implementation uses staged emission.
- Ensure line-number advancement uses `next_line_num` rather than duplicating numbering logic.
- Keep branch structure aligned with the original code to reduce migration risk.

**Exit criteria**:
- Main processing path compiles and passes focused behavior tests derived from the original function responsibilities.
- No extra feature flags or abstractions have been introduced.

## Phase 4: Wire orchestration and finalize verification

- Implement `copy_cat` as the selector/orchestrator between `simple_cat` and `cat`.
- Connect the migrated module to existing Rust entry flow with only the wiring needed to replace the C file’s role.
- Add integration-oriented tests covering:
  - simple copy path selection,
  - transformed path selection,
  - stdin/file-like reader handling as supported by current project structure,
  - final output flush behavior.
- Review all functions for parity in:
  - buffer lifetime,
  - EOF handling,
  - write completion,
  - line-number state mutation,
  - error propagation.

**Exit criteria**:
- All migrated functions are implemented.
- `cargo test` passes.
- The Rust module replaces the `cat.c` logic in scope without expanding project structure or behavior.