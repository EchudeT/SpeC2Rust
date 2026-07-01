# Implementation Plan: module_src_parseopt_wordwrap.c_13

## Summary

This module ports `src/parseopt/wordwrap.c` into Rust with a direct, file-scoped translation focused on preserving current wrapping behavior, margin handling, buffered output, and error propagation. The Rust implementation should keep the existing operational model: a stateful word-wrap context that accepts text fragments, tracks left/right margins, rescans line state as needed, and emits wrapped output through a writer-backed sink.

The implementation approach should stay close to the C layout rather than redesigning the API. The main work is to:

- translate the word-wrap state object into a Rust struct with owned buffers and explicit counters,
- replace raw file-descriptor/file callbacks with standard-library `Write`-based output,
- preserve multibyte-aware scanning behavior using Rust string/byte processing with explicit fallbacks for invalid sequences,
- map C integer/status error reporting into `Result` while retaining module-local error state where the original flow depends on it,
- migrate functions in dependency order so internal state transitions can be verified incrementally.

The resulting Rust code should live in a single corresponding Rust module for this C file, without introducing extra abstraction layers beyond what is required for safe ownership and I/O.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.75+

### Primary Dependencies

Use the Rust standard library by default:

- `std::io::{Write, Error, ErrorKind, Result}`
- `std::borrow::Cow` only if needed for temporary text handling
- `std::mem` / `std::cmp` for buffer and counter operations

Third-party crates are not required by the provided evidence. In particular, avoid adding unicode-width or terminal crates unless later source review shows the C module depends on behavior that cannot be reproduced acceptably with the standard library alone.

### Testing

- `cargo test`

Testing should cover:

- line initialization and reset behavior,
- right-margin detection paths,
- write/flush/close sequencing,
- wrapping across spaces and prefixes,
- left-margin advancement,
- error propagation from the writer,
- handling of valid and invalid multibyte input boundaries.

### Performance Goals

- Preserve streaming behavior without unnecessary whole-input copies.
- Keep buffering localized to the wrap state, similar to the C implementation.
- Avoid per-character heap allocation during scanning and wrapping.
- Maintain comparable asymptotic behavior to the C module for continuous writes and flushes.
- Use byte-slice traversal where possible, escalating to scalar decoding only where multibyte handling is required.

## Module Mapping

### Source File Mapping

- C: `src/parseopt/wordwrap.c`
- Rust: `src/parseopt/wordwrap.rs`

If the existing Rust project already exposes a `parseopt` module tree, this file should be added there directly:

- `src/parseopt/mod.rs` updated to include `pub(crate) mod wordwrap;`

No additional module split is planned; all migrated functions should remain in this single Rust file unless an existing project layout already dictates otherwise.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `wordwrap_line_init` | `fn wordwrap_line_init(state: &mut WordWrap)` | Internal state reset helper. |
| `detect_right_margin` | `fn detect_right_margin(state: &mut WordWrap) -> usize` or `fn ... -> Result<usize>` | Return type depends on whether the C path can fail due to I/O/environmental lookup. |
| `_ww_fd_writer` | `fn ww_fd_writer<W: Write>(writer: &mut W, buf: &[u8]) -> io::Result<usize>` or folded into writer calls | Prefer collapsing into direct `Write` use unless a callback-compatible helper is needed. |
| `wordwrap_open` | `fn wordwrap_open(...) -> WordWrap` or `-> io::Result<WordWrap>` | Constructor for writer-backed state. |
| `wordwrap_fdopen` | `fn wordwrap_fdopen<W: Write + 'a>(writer: W, ...) -> WordWrap<W>` | Rust equivalent of opening on a descriptor/stream. |
| `wordwrap_close` | `fn wordwrap_close(state: &mut WordWrap) -> io::Result<()>` | Flush buffered content and finalize state; no manual free. |
| `full_write` | `fn full_write<W: Write>(writer: &mut W, buf: &[u8]) -> io::Result<()>` | Loop until all bytes are written or error. |
| `safe_mbrtowc` | `fn safe_mbrtowc(input: &[u8]) -> DecodeStep` | Internal decoding helper replacing C multibyte conversion state. |
| `wsprefix` | `fn wsprefix(s: &[u8]) -> usize` | Count/locate leading whitespace prefix. |
| `wordwrap_rescan` | `fn wordwrap_rescan(state: &mut WordWrap)` | Recompute wrap boundaries within buffered line content. |
| `wordwrap_flush` | `fn wordwrap_flush(state: &mut WordWrap) -> io::Result<()>` | Emit pending line/buffer content. |
| `wordwrap_error` | `fn wordwrap_error(state: &WordWrap) -> Option<&io::Error>` or status accessor | Preserve “sticky error” semantics if present in C. |
| `wordwrap_next_left_margin` | `fn wordwrap_next_left_margin(state: &mut WordWrap)` | Advance left-margin state for next line. |
| `wordwrap_write` | `fn wordwrap_write(state: &mut WordWrap, buf: &[u8]) -> io::Result<()>` | Primary streaming entry point. |
| `wordwrap_puts` | `fn wordwrap_puts(state: &mut WordWrap, s: &str) -> io::Result<()>` | Convenience wrapper over `wordwrap_write`. |

## Data Model

The analysis output lists anonymous C data structures only, so the Rust plan should infer a restrained data model from function usage rather than inventing new public types.

### Core State Object

The C module almost certainly centers around a mutable word-wrap context. Map that to a single Rust struct:

```rust
pub(crate) struct WordWrap<W: Write> {
    writer: W,
    buffer: Vec<u8>,
    line_start: usize,
    line_width: usize,
    left_margin: usize,
    next_left_margin: usize,
    right_margin: usize,
    pending_space: bool,
    error: Option<std::io::Error>,
    closed: bool,
    // additional counters/flags migrated directly from the C state
}
```

This is illustrative; final field names should follow the actual C variables and only include fields required by the original file.

### Data-Structure Mapping

Because the C analysis does not provide named structs, use the following migration rule:

| C Construct | Rust Mapping | Notes |
|---|---|---|
| Anonymous state struct for word-wrap session | `struct WordWrap<W: Write>` | Owns writer and all mutable wrap state. |
| Anonymous writer callback payload | Generic type parameter `W: Write` or a small internal enum if multiple sink kinds truly exist | Prefer generic `Write` over callback tables. |
| Raw character buffer | `Vec<u8>` | Preserve byte-oriented buffering. |
| C string pointers | `&[u8]` / `&str` | Use `&str` only where input is guaranteed valid UTF-8. |
| Multibyte conversion state | Internal decode helper state, likely small enum | No unsafe locale-state emulation unless source requires it. |
| Error/status integer fields | `Option<io::Error>` and/or boolean flags | Preserve sticky failure behavior. |
| Margin and width counters | `usize` | Convert from C integer types with checked assumptions where needed. |
| Boolean flags | `bool` | Replace integer flag fields directly. |

### Suggested Internal Enums

If needed to preserve scan logic clearly:

```rust
enum DecodeStep {
    Complete { ch: char, len: usize },
    Invalid { len: usize },
    Incomplete,
}
```

and, if the C code distinguishes flush/write modes with flags, a small private enum may replace integer mode constants. Do not introduce enums unless they correspond to existing branching in the C file.

### Memory Management Notes

- C allocation/free behavior should collapse into Rust ownership of `Vec<u8>` and the `WordWrap` struct.
- `wordwrap_close` should not emulate manual deallocation; it should only flush and mark the state finalized if the API needs explicit closure.
- Avoid borrowing internal buffers across write calls to prevent self-referential patterns.
- If the original C API allowed null pointers or uninitialized state, model that as constructor-validated state rather than optional internals wherever possible.

### Error Handling Notes

- Convert write failures into `io::Result<()>`.
- If the C module stores an internal error and suppresses subsequent writes, preserve that with a sticky `error: Option<io::Error>` field and early-return checks.
- For malformed multibyte input, preserve original tolerant behavior: do not panic; treat invalid sequences according to the C function’s fallback path.
- Avoid `unwrap`, especially in decoding and output code.

## Implementation Phases

## Phase 1: Establish the Rust Module Skeleton

Scope:

- Create `src/parseopt/wordwrap.rs`.
- Add the module declaration in `src/parseopt/mod.rs` if required by the existing tree.
- Define the `WordWrap<W: Write>` struct with placeholders for all state directly mirrored from C locals/statics.
- Implement constructors corresponding to `wordwrap_open` and `wordwrap_fdopen`.
- Implement low-level output helpers:
  - `full_write`
  - `_ww_fd_writer` equivalent if still needed
  - `wordwrap_error` status accessor
- Implement `wordwrap_close` with flush delegation and finalized-state handling.

Deliverables:

- Module compiles.
- Writer lifecycle is represented with safe ownership.
- Basic tests for open/write-close with a `Vec<u8>` sink or test writer.

## Phase 2: Port Core Line State and Margin Logic

Scope:

- Implement `wordwrap_line_init`.
- Implement `detect_right_margin`.
- Implement `wsprefix`.
- Implement `wordwrap_next_left_margin`.
- Add all required counters and flags to `WordWrap`.
- Mirror C integer arithmetic carefully, converting to `usize`/`isize` only where semantics remain exact.

Deliverables:

- Internal line-state transitions compile and behave deterministically.
- Unit tests cover initialization, margin updates, and prefix calculations.
- No wrapping output logic yet beyond state preparation.

## Phase 3: Port Scanning and Wrap Processing

Scope:

- Implement `safe_mbrtowc` as an internal decoding helper.
- Implement `wordwrap_rescan`.
- Implement `wordwrap_write`.
- Implement `wordwrap_puts`.

Technical decisions:

- Keep the buffered content byte-oriented.
- Decode only enough to preserve character-boundary-safe wrapping logic.
- If the C logic operates on locale multibyte input rather than Unicode width, preserve byte/character boundary behavior only; do not add display-width features not evidenced by the source.

Deliverables:

- Streaming input is accumulated, rescanned, and wrapped through the configured writer.
- Tests cover:
  - single-line pass-through,
  - wrap at whitespace,
  - left-margin continuation,
  - multibyte valid input,
  - invalid byte-sequence tolerance path if applicable from source.

## Phase 4: Port Flush Semantics and Complete Behavioral Verification

Scope:

- Implement `wordwrap_flush`.
- Reconcile `wordwrap_close` with final flush behavior.
- Verify sticky-error behavior against all public entry points.
- Align any remaining edge cases from the C control flow, especially partial lines and end-of-input handling.

Deliverables:

- Full function coverage for all items listed from `wordwrap.c`.
- Tests for:
  - flush without trailing newline,
  - repeated flush/close behavior,
  - writer error propagation,
  - no writes after terminal error/close if the C module enforces that.
- Final cleanup to keep naming and visibility aligned with the rest of the Rust project.