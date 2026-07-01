# Implementation Plan: `module_src_parseopt_wordwrap.c_13`

## Summary

This module ports `src/parseopt/wordwrap.c` into Rust as a focused text-wrapping component that preserves the existing behavior and function boundaries as closely as practical. The C file appears to combine three concerns: wrapping state management, margin/prefix scanning, and output writing to either a file descriptor-like sink or a generic wrapper-owned stream. The Rust implementation should keep these concerns in a single module and migrate the existing functions in a direct, traceable way rather than redesigning the API.

The technical approach is to replace C-managed buffers, manual multibyte parsing, and raw write loops with safe Rust ownership and I/O primitives from the standard library. A central wrapper state struct will hold the current line buffer, margin configuration, output sink, and error/flush state. Functions that in C relied on pointer arithmetic and mutable byte buffers will become private helper methods over slices and `String`/`Vec<u8>` state, while externally visible operations remain close to the original call sequence: open/fdopen, write/puts, flush, close, error query, and left-margin advancement.

Unicode and multibyte behavior should be ported conservatively. Since the C module includes `safe_mbrtowc`, the Rust version should make character-boundary handling explicit and rely on UTF-8 decoding via the standard library. Any behavior that depends on byte-by-byte rescanning should be preserved using byte slices plus checked decoding, not broadened into grapheme-aware wrapping or locale frameworks that are not present in the source scope.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
  - `std::io::{self, Write}`
  - `std::fs::File` if file-backed construction is needed by the surrounding crate
  - `std::borrow::Cow` only if needed for temporary borrowed/owned prefix text, otherwise avoid
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear scanning over input text and line buffers.
  - Avoid unnecessary string reallocations by reusing internal buffers.
  - Preserve streaming-style writes instead of accumulating whole-output copies.
  - Match C behavior closely enough that wrapping decisions and flush boundaries remain stable for existing callers.

## Module Mapping

### Source File Mapping
- `src/parseopt/wordwrap.c`
  → `src/parseopt/wordwrap.rs`

### Function Mapping
Keep the migration close to the original file layout, using free functions only where constructor-style functions are required and converting stateful helpers into `impl` methods when that reduces unsafe-style parameter passing.

- `wordwrap_line_init`
  → `fn wordwrap_line_init(...)` or private `WordWrap::line_init(&mut self, ...)`
- `detect_right_margin`
  → private helper `fn detect_right_margin(...) -> usize`
- `_ww_fd_writer`
  → private sink write helper over `Write`: `fn ww_writer(...) -> io::Result<()>`
- `wordwrap_open`
  → `pub fn wordwrap_open(...) -> WordWrap`
- `wordwrap_fdopen`
  → `pub fn wordwrap_fdopen<W: Write + 'static>(writer: W, ...) -> WordWrap`
- `wordwrap_close`
  → `pub fn close(&mut self) -> io::Result<()>`
- `full_write`
  → private helper `fn full_write<W: Write>(...) -> io::Result<()>`
- `safe_mbrtowc`
  → private UTF-8 decoding helper `fn safe_mbrtowc(...) -> DecodeStep`
- `wsprefix`
  → private helper `fn wsprefix(...) -> ...`
- `wordwrap_rescan`
  → private `fn rescan(&mut self, ...)`
- `wordwrap_flush`
  → `pub fn flush(&mut self) -> io::Result<()>`
- `wordwrap_error`
  → `pub fn error(&self) -> Option<&io::Error>` or boolean error state if original interface is status-only
- `wordwrap_next_left_margin`
  → `pub fn next_left_margin(&mut self, ...)`
- `wordwrap_write`
  → `pub fn write_str(&mut self, input: &str) -> io::Result<()>`
- `wordwrap_puts`
  → `pub fn puts(&mut self, input: &str) -> io::Result<()>`

### Rust Module Placement
Use the existing crate structure and place the port directly under the corresponding path:
- `src/parseopt/mod.rs` updated to expose `wordwrap`
- `src/parseopt/wordwrap.rs` contains all migrated logic from the C file

Do not split scanning, sink, or margin logic into additional Rust modules unless the existing crate layout already requires it.

## Data Model

Because the C analysis only exposes anonymous structures, the Rust plan should map them by role rather than by guessed names. The implementation should introduce a restrained set of explicit Rust structs/enums mirroring the state clusters found in the C file.

### Core State Mapping

- **C anonymous wrapper state struct**
  → `pub struct WordWrap`
  - Holds line state, margins, sink, pending error, and wrapping configuration.
  - Owns all buffers directly; no borrowed internal pointers.

Suggested fields:
- `writer: Box<dyn Write>`
- `line_buf: String` or `Vec<u8>` depending on whether byte-exact buffering is needed
- `left_margin: usize`
- `right_margin: usize`
- `current_prefix_width: usize`
- `pending_ws_prefix: usize` or equivalent rescanned indentation state
- `error: Option<io::Error>`
- `closed: bool`

### Output Sink Mapping

- **C function pointer + opaque data for writer**
  → `Box<dyn Write>`

If the C module distinguishes an fd writer callback from other writer forms, keep only one internal sink abstraction in Rust and adapt construction through `wordwrap_fdopen`/`wordwrap_open`.

### Line/Scan Temporary State

- **C anonymous line-tracking struct**
  → private `struct LineState`
  - Stores current visible width, break candidate position, and whether line start rules are active.

Suggested fields:
- `byte_len: usize`
- `display_width: usize`
- `break_byte: Option<usize>`
- `break_width: Option<usize>`
- `at_line_start: bool`

This can either be embedded into `WordWrap` or used as a temporary helper during rescans.

### Multibyte Decode Result

- **C `safe_mbrtowc` return/status pattern**
  → private enum:
```rust
enum DecodeStep {
    Char(char, usize),
    Invalid(usize),
    Incomplete,
}
```
This preserves explicit handling of malformed or partial byte sequences without introducing unsafe conversions.

### Error State

- **C integer/errno-style error state**
  → `Option<io::Error>` plus `io::Result<()>` returns from mutating operations.

The Rust port should not silently discard write failures. If the original C object retains sticky error state, preserve that by storing the first encountered error and making subsequent operations no-ops or repeated failures, matching the original control flow as closely as possible.

### Buffer Representation Decision

Prefer:
- `String` for text that is guaranteed valid UTF-8 within the Rust-facing API.
- `Vec<u8>` if preserving exact byte-oriented scanning is necessary due to invalid multibyte handling inherited from C.

Because `safe_mbrtowc` exists in the original code, the safer migration plan is:
- external API: `&str` for `write_str`/`puts`
- internal scan input: bytes from `&str`
- line buffer: `String` if no invalid byte states are stored internally

If existing callers in the Rust project must pass arbitrary bytes, revisit this to `&[u8]`, but do not assume that need unless the surrounding code shows it.

## Implementation Phases

## Phase 1: Skeleton Port and State Definition

- Create `src/parseopt/wordwrap.rs`.
- Define `WordWrap` and the minimum private helper structs/enums required to represent:
  - sink ownership
  - line buffer
  - left/right margin state
  - sticky error state
  - rescan bookkeeping
- Implement constructor-style entry points corresponding to:
  - `wordwrap_open`
  - `wordwrap_fdopen`
- Implement close/error surface:
  - `wordwrap_close`
  - `wordwrap_error`
- Implement low-level sink helpers:
  - `_ww_fd_writer`
  - `full_write`
- Decide and document whether close consumes the object logically via a `closed` flag or simply flushes and marks unusable; keep behavior aligned with existing call sites rather than inventing `Drop`-driven semantics.

### Phase 1 Acceptance
- Module compiles with placeholder wrapping logic.
- Writer ownership and error propagation are established.
- No unsafe code is introduced unless forced by surrounding APIs.

## Phase 2: Margin, Prefix, and Scan Logic Migration

- Port scanning helpers directly from the C file into private Rust functions/methods:
  - `wordwrap_line_init`
  - `detect_right_margin`
  - `safe_mbrtowc`
  - `wsprefix`
  - `wordwrap_rescan`
  - `wordwrap_next_left_margin`
- Preserve the original control flow for:
  - detecting effective right margin
  - identifying whitespace prefix/indentation
  - rescanning buffered text after margin changes or line breaks
- Replace pointer arithmetic with:
  - byte indices over `&[u8]`
  - checked `str` boundary handling when converting back to text
- Keep display-width logic limited to what the C file already implements; do not add unicode-width crates or terminal-width dependencies without direct evidence they are already needed.

### Phase 2 Acceptance
- Internal tests cover:
  - empty input
  - whitespace-only prefixes
  - exact-right-margin boundary behavior
  - multibyte UTF-8 characters in normal valid input
- Rescan behavior is deterministic and avoids panics on character boundaries.

## Phase 3: Write/Flush Operations and Behavioral Parity

- Implement the main output path:
  - `wordwrap_write`
  - `wordwrap_puts`
  - `wordwrap_flush`
- Ensure line wrapping decisions use the migrated scan state from Phase 2.
- Preserve semantics around:
  - pending buffered text before flush
  - line break insertion points
  - left-margin continuation handling
  - repeated writes across multiple calls
- Ensure all writes route through the same sink helper and sticky error path.

### Phase 3 Acceptance
- `cargo test` passes for streaming write sequences and flush/close order.
- Write failures are retained in object state and surfaced consistently.
- Output is stable for representative multi-call wrapping cases.

## Phase 4: Verification Against C Behavior and Cleanup

- Compare the Rust implementation against the C file function-by-function and remove remaining placeholder deviations.
- Normalize naming so the Rust file remains traceable to the C source while still following idiomatic visibility:
  - public API only for externally used entry points
  - private helpers for scan/write internals
- Add focused regression tests for:
  - margin transitions
  - continuation indentation
  - final flush without trailing newline
  - close after prior error
- Keep the module self-contained; do not expand into auxiliary abstractions beyond what is needed to complete the port.

### Phase 4 Acceptance
- All migrated functions from `wordwrap.c` are represented in the Rust file.
- The implementation is memory-safe, avoids raw-pointer lifetime risks, and uses standard-library I/O/error types consistently.
- The port is ready for integration on branch `110-module_src_parseopt_wordwrap.c_13-rust-port`.