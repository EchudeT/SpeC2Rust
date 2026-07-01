# Implementation Plan

## Summary
Port `close-stream.c` into a Rust module that preserves the existing responsibility of `close_stream`: finalize an output/input stream, detect close-time failures, and report them through Rust’s `Result`-based error flow rather than C-style return codes and global error state.

The Rust implementation should stay narrow in scope:
- migrate the logic of `close_stream` only;
- use standard-library I/O abstractions where possible;
- preserve the distinction between write/flush failures and close/finalization failures;
- avoid introducing broader stream-management abstractions beyond what is needed to replace the existing C function.

The expected Rust shape is a small helper function in the main binary crate, likely operating on a generic writer or owned file handle and returning `io::Result<()>`. Any behavior in the C function that depends on flushing before close should be made explicit in Rust by calling `flush()` before the handle is dropped, because Rust drop does not report close errors.

## Technical Context
- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fs`, `std::os::unix` only if required by existing project code paths)
- **Testing**: `cargo test`
- **Performance Goals**:
  - No meaningful regression versus the C helper for normal file-stream closure paths
  - Keep overhead minimal: at most one explicit `flush()` plus ownership drop/finalization
  - Avoid unnecessary allocation or buffering beyond what the caller already uses

## Module Mapping
- **C source**: `close-stream.c`
- **Rust target**: `src/main_root_close_stream.rs` or integration into the existing main-cluster source file layout if this branch already consolidates helpers into `src/main.rs`-adjacent modules
- **C function to Rust function**:
  - `close_stream` -> `close_stream(...) -> std::io::Result<()>`

Recommended placement:
- Declare a dedicated internal module for this migrated helper if the branch layout keeps one C-file-to-one-Rust-module correspondence.
- Re-export nothing unless already required by the current main-cluster organization.

## Data Model
This module has no dedicated persistent data structures in the provided analysis.

C-to-Rust mapping for relevant concepts:
- `FILE *` -> Rust owned stream type, selected from:
  - `std::fs::File` when the caller works with files directly
  - generic `W: std::io::Write` only if close semantics are not required beyond flush
- integer status return / `EOF` -> `std::io::Result<()>`
- `errno`-based failure propagation -> `std::io::Error`

Important ownership decision:
- If the original C function is responsible for closure, the Rust function should take ownership of the stream/file handle rather than `&mut` borrow where feasible. This ensures the close path is tied to function completion and mirrors C lifetime responsibility more closely.

## Implementation Phases

### Phase 1: Inspect and define the exact Rust signature
- Review the original `close_stream` body and identify:
  - whether it operates specifically on `FILE *`;
  - whether it checks prior stream error state before/after close;
  - whether it distinguishes flush/write failure from close failure;
  - whether callers expect closure of files only, or generic streams.
- Choose the narrowest Rust signature that matches actual call sites:
  - prefer `fn close_stream(file: std::fs::File) -> std::io::Result<()>` if the C usage is file-specific;
  - use a generic `Write`-based signature only if the function is semantically just “flush and finish” and call sites do not rely on true close reporting.
- Map C return conventions to `Result<()>` without adding new error categories.

### Phase 2: Implement the migrated helper
- Create the Rust module/file corresponding to `close-stream.c`.
- Translate the C logic into Rust with explicit attention to close-time behavior:
  - call `flush()` when the original function’s semantics require pending buffered output to be committed;
  - ensure the handle is consumed so the stream is finalized immediately after the function returns;
  - if true OS-level close error reporting is required and `File` is used, structure the implementation to surface the available error path before drop, while acknowledging that Rust drop itself cannot return errors.
- Preserve the original behavior as closely as Rust allows:
  - no silent swallowing of flush errors;
  - no replacement of I/O failure with boolean status;
  - no extra logging or recovery logic unless the original function already performs it.

### Phase 3: Migrate call sites in the main cluster
- Replace uses of the C helper with the Rust function.
- Update calling code to handle `io::Result<()>` via propagation (`?`) or the existing main-program error exit path.
- Ensure ownership is transferred correctly at the call boundary so the stream is not used after closure.
- Remove any now-redundant C-style status checks that were only needed for `EOF`/`errno`.

### Phase 4: Add focused tests for closure and error propagation
- Add unit tests for the chosen Rust helper shape:
  - successful flush-and-close path;
  - flush failure propagation using a custom test writer if the function is generic over `Write`;
  - file-based success case if the function takes `File`.
- Add integration coverage through existing main-cluster tests only where this helper is exercised by real program flow.
- Keep tests limited to observed semantics from the original C function; do not add broader stream framework tests.