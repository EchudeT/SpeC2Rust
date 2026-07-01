# Implementation Plan: module_doc_wc.c_05

## Summary

This module ports `doc/wc.c` to Rust as a single focused module that preserves the existing command-line word-count style behavior and internal processing flow. The Rust implementation should keep the original function responsibilities recognizable: input scanning, word detection, per-input counting, reporting, and simple error emission.

The technical approach is to translate the current file-oriented and character-oriented C logic into idiomatic Rust using the standard library, while avoiding unnecessary restructuring. Counting logic should remain local to the module, with explicit helper functions replacing the C utility functions. Error handling should move from C-style printing helpers and implicit global state patterns toward `Result`-based internal flow, while still preserving user-visible diagnostics at the top-level reporting boundary.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the practical throughput of the C implementation for typical text inputs.
  - Process input in a streaming manner without loading full files into memory.
  - Keep allocation minimal, especially in the word-scanning path.
  - Maintain predictable behavior for small and large files alike.

## Module Mapping

### Source File Mapping

- `doc/wc.c` → `src/module_doc_wc_05.rs`

If the crate already exposes modules through `src/lib.rs` or `src/main.rs`, add only the minimal module declaration needed to compile this port.

### Function Mapping

- `error_print` → `fn error_print(...)`
  - Preserve as a local diagnostic formatting helper.
  - Prefer taking `&str`/`&Path`/error references rather than raw C-style message fragments.

- `errf` → `fn errf(...)`
  - Map to a small helper for non-OS errors.
  - Likely writes to standard error and returns an error indicator or is used only at the outer reporting layer.

- `perrf` → `fn perrf(...)`
  - Map to a helper for I/O or OS-related errors.
  - In Rust, this should typically format `std::io::Error` rather than depend on external errno state.

- `report` → `fn report(...)`
  - Preserve as the output formatting function for counts.
  - Keep formatting logic separate from counting logic.

- `isword` → `fn isword(ch: u8) -> bool` or `fn isword(ch: char) -> bool`
  - Prefer byte-based classification if the original C behavior is byte-oriented.
  - Do not widen semantics beyond the original classification rules.

- `getword` → `fn getword(...)`
  - Preserve as the scanning helper that advances through input and identifies words.
  - Implement over buffered input primitives from the standard library.

- `counter` → `fn counter(...) -> Result<..., std::io::Error>`
  - Main counting routine for a single input source.
  - Return a compact count structure instead of mutating dispersed local state where practical.

## Data Model

The input analysis provides no explicit C structs, so the Rust port should introduce only the minimum internal structures needed to carry the original state cleanly.

### New Rust Structures

#### `Counts`
A compact internal struct for the per-input totals produced by `counter`.

```rust
struct Counts {
    lines: u64,
    words: u64,
    bytes: u64,
}
```

Use `u64` for counters to avoid narrowing relative to common C integer usage and to support large files safely.

#### `ReportTarget`
If the original code distinguishes named files from standard input in reporting, use a minimal enum only if needed by formatting logic.

```rust
enum ReportTarget<'a> {
    Stdin,
    Path(&'a std::path::Path),
}
```

This is optional; if a simple `Option<&Path>` is enough, prefer that instead.

### C-to-Rust Data Mapping

- C local integral counters → Rust `u64`
- C character/input values from `getc`-style APIs → Rust `u8` or `Option<u8>` during buffered scanning
- C string/file-name pointers → Rust `&str`, `String`, or `&Path`
- C FILE-based stream handling → Rust `std::io::BufRead` / `Read`
- C errno-derived error reporting → Rust `std::io::Error`

## Implementation Phases

## Phase 1: Establish the Rust Module Skeleton

- Create `src/module_doc_wc_05.rs`.
- Add only the required module declaration in the crate root.
- Define the minimal internal `Counts` type.
- Stub the direct function counterparts:
  - `error_print`
  - `errf`
  - `perrf`
  - `report`
  - `isword`
  - `getword`
  - `counter`
- Decide the function signatures based on actual call relationships in `doc/wc.c`, keeping them narrow and file-local unless external callers require visibility.
- Preserve the original execution order and helper layering rather than redesigning control flow.

### Deliverables
- Compiling module skeleton.
- Basic type definitions and signatures aligned with the C source structure.

## Phase 2: Port Character Classification and Counting Logic

- Implement `isword` with behavior matching the C classification rules exactly; do not substitute broader Unicode word semantics unless the source clearly does so.
- Implement `getword` as the scanning primitive over buffered input.
- Implement `counter` to:
  - read incrementally from the input source,
  - track bytes, lines, and words,
  - transition between inside-word and outside-word states as in the C logic,
  - return a `Counts` value.
- Use standard-library buffered reading to avoid per-character system calls where possible, while keeping logic close to the original algorithm.
- Keep memory ownership simple: no retained references into temporary buffers, no unnecessary string construction during scanning.

### Deliverables
- Working counting path for one input source.
- Unit tests for:
  - empty input,
  - whitespace-only input,
  - single word,
  - multiple lines,
  - punctuation or delimiter handling according to `isword`.

## Phase 3: Port Reporting and Error Paths

- Implement `report` to print counts in the same field order and general formatting style as the C module.
- Implement `error_print`, `errf`, and `perrf` as small stderr helpers, but keep actual failure propagation in `Result` form internally.
- At the outermost module boundary, translate `Result` failures into user-visible diagnostics that match the original behavior closely.
- Ensure standard input and named-file cases are both handled if present in the C file.
- Avoid introducing a broader error abstraction unless the source usage requires it.

### Deliverables
- End-to-end output path with diagnostics.
- Tests covering:
  - successful reporting for representative inputs,
  - I/O failure propagation and stderr formatting where practical.

## Phase 4: Final Compatibility Pass

- Compare the Rust module behavior against `doc/wc.c` function-by-function:
  - word boundary behavior,
  - line counting behavior,
  - byte counting behavior,
  - report formatting,
  - error messaging shape.
- Remove any helper abstractions that are not justified by the original file.
- Confirm that all resource management is handled through Rust ownership and RAII:
  - file handles close automatically,
  - no manual cleanup branches remain,
  - no borrowed data outlives read buffers.
- Run `cargo test` and fix any mismatches in edge-case behavior.

### Deliverables
- Finalized Rust port of `doc/wc.c`.
- Behavior-aligned tests and cleaned module interface.