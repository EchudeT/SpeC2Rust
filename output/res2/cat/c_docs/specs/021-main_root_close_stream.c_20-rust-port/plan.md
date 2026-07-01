# Implementation Plan

## Summary
Port the C module `close-stream.c` into a focused Rust module that preserves the existing responsibility of `close_stream`: finalize and close an output stream while surfacing write/flush/close failures in a way that matches the current program behavior.

The Rust implementation should stay narrow and use the standard library I/O model. The core approach is to migrate the stream-closing logic into a small Rust function that operates on standard output abstractions where possible, explicitly handles buffered output finalization before close, and converts C-style status/error reporting into idiomatic `Result`-based control flow at the module boundary. No additional facilities or generalized stream framework should be introduced.

## Technical Context
- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::io`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - No meaningful regression versus the C helper for stream finalization paths
  - Constant-space handling during close/flush
  - Avoid extra allocations in the close path
  - Keep error checks limited to the same close-time conditions handled by the C module

## Module Mapping
- **C source file**: `close-stream.c`
- **Rust target module**: `src/close_stream.rs`
- **C function -> Rust function**
  - `close_stream` -> `close_stream(...) -> std::io::Result<()>`

If the current Rust crate entry point already has a central module file, expose this as a private or crate-visible helper only to the extent required by the existing `cat` port. Do not create additional adapter layers unless needed by current call sites.

## Data Model
This module does not define persistent C data structures.

### Function and type mapping
- `FILE *`-based stream handling -> Rust writer/stream type using standard library traits and concrete standard output handles as required by the surrounding port
- C integer status return -> `Result<(), std::io::Error>`
- `errno`-driven failure signaling -> `std::io::Error`

### Ownership and lifetime notes
- C manually closes/flushed streams with explicit error inspection
- Rust should rely on ownership and scoped resource management, but the migrated helper must still perform explicit finalization checks rather than depending only on `Drop`, since `Drop` does not report I/O errors
- Where a buffered writer is used, flush must be explicit before resource release so close-time failures remain observable

## Implementation Phases

### Phase 1: Establish module and signature
- Create `src/close_stream.rs`
- Introduce the Rust `close_stream` helper with a minimal signature shaped by the current `cat` port’s actual writer usage
- Map the C return convention to `std::io::Result<()>`
- Keep the helper narrowly scoped to existing stream-closing behavior; do not generalize beyond current needs

### Phase 2: Port close/flush semantics
- Translate the C logic for detecting pending output failure during stream finalization into explicit Rust flush/close checks
- Ensure the implementation does not depend on destructor behavior for observable errors
- Preserve the ordering of operations relevant to correctness:
  - finalize buffered output
  - surface any write/flush failure
  - release the stream cleanly
- Use standard library facilities only

### Phase 3: Integrate with call sites
- Replace existing or placeholder close handling in the Rust `cat` branch with calls to the new helper
- Adjust call sites to propagate `Result` cleanly to the program’s existing error-reporting path
- Keep migration limited to the current module boundary and immediate users of `close_stream`

### Phase 4: Validate behavior with tests
- Add focused unit tests for:
  - successful close/finalization path
  - failure propagation when flush/finalization returns an error
- Prefer small test doubles using custom `Write` implementations to simulate close-time/flush-time errors
- Run `cargo test` and confirm the helper’s behavior matches expected status propagation without adding unrelated test infrastructure