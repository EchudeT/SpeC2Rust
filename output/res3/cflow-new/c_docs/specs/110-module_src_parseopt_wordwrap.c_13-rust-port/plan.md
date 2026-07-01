# Implementation Plan: module_src_parseopt_wordwrap.c_13

## Summary

This module migrates `src/parseopt/wordwrap.c` into a Rust module that preserves the existing word-wrapping behavior, output flow, margin handling, and error propagation semantics of the C implementation. The Rust implementation should stay narrowly scoped to the current file’s responsibilities: managing a word-wrap state machine over text written to an output target, including left/right margin handling, buffering/rescanning, flush/close behavior, and writer-backed output.

The technical approach is a direct file-level port into a single Rust source module, with C functions translated into private helpers and a small public API matching the current module’s operational boundaries. Memory ownership will move from manual allocation and raw buffers to owned Rust types such as `String`, `Vec<u8>`, and explicit state structs. I/O will be expressed through `std::io::Write`, while preserving deferred write/flush/error behavior expected by the original implementation. Multibyte decoding logic should be implemented with standard-library byte/string handling first, only introducing a small internal compatibility helper where the C code used `mbrtowc`-style scanning.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required by default
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve linear-time processing over input text
  - Avoid unnecessary intermediate allocations beyond the working buffer already implied by the C design
  - Maintain streaming-style writes and flushes without rewriting the module into a batch processor
  - Keep UTF-8 / multibyte scanning efficient enough for command-line text output workloads

## Module Mapping

### Source File Mapping

- **C source**
  - `src/parseopt/wordwrap.c`

- **Rust target**
  - `src/parseopt/wordwrap.rs`

### Function Mapping

The Rust port should retain the existing functional decomposition where it helps preserve behavior, but functions that exist only for C resource management may be folded into methods on a state struct.

- `wordwrap_line_init`
  - Port as a private method on the line/state struct, e.g. `fn line_init(&mut self)`
- `detect_right_margin`
  - Port as a private helper or constructor-time method, e.g. `fn detect_right_margin(...) -> usize`
- `_ww_fd_writer`
  - Port as a private writer adapter helper only if needed; otherwise replace with direct `Write` trait calls
- `wordwrap_open`
  - Port as a public constructor returning the main wrapper state over an owned writer abstraction
- `wordwrap_fdopen`
  - Port as a constructor variant for an already-available writer handle
- `wordwrap_close`
  - Port as a public consuming or finalizing method; ensure flush-before-close semantics
- `full_write`
  - Port as a private helper around `write_all` / repeated writes if exact C semantics require partial-write handling
- `safe_mbrtowc`
  - Port as a private multibyte/UTF-8 step helper used only by scan logic
- `wsprefix`
  - Port as a private helper for whitespace-prefix detection/counting
- `wordwrap_rescan`
  - Port as a private method that recomputes line-wrap state from buffered content
- `wordwrap_flush`
  - Port as a public method calling internal flush logic and writer flush
- `wordwrap_error`
  - Port as a state query method, or eliminate if Rust error returns fully replace sticky error state; if the C API tracks a stored error, preserve that in state
- `wordwrap_next_left_margin`
  - Port as a private/public method depending on current call graph; keep exact role in margin advancement
- `wordwrap_write`
  - Port as the main public write-entry method over `&str` or `&[u8]` depending on current call requirements
- `wordwrap_puts`
  - Port as a convenience public method over string input, likely delegating to `wordwrap_write`

### Rust Module Shape

Keep the Rust implementation contained to one file corresponding to the original C file, with:
- one main state struct for the wrapper
- one small internal line/buffer state struct if the C logic separates these concerns
- private helper functions/methods mirroring the original control flow
- a minimal public API exposing open/write/flush/close/error-equivalent operations

## Data Model

Because the input analysis lists only anonymous C structures, the Rust plan should infer structure boundaries from usage rather than inventing new abstractions. The mapping should stay minimal and local to this module.

### Data-Structure Mapping

- **C anonymous wrapper/context struct**
  - **Rust**: `struct WordWrap<W: Write>`
  - Holds:
    - output writer
    - current line buffer/state
    - margin configuration
    - right-margin width
    - pending/sticky error state if required by original behavior
    - any flags controlling wrap behavior

- **C anonymous line-tracking struct**
  - **Rust**: `struct LineState`
    - buffered text/bytes for current line
    - current display width / column position
    - left-margin state
    - break position metadata used by rescan/wrap decisions

- **C anonymous writer callback payload / file-descriptor adapter**
  - **Rust**: either omitted in favor of `W: Write`, or represented as a very small internal adapter struct if constructor variants require it

- **C anonymous flag/return-state groupings**
  - **Rust**: plain fields on `WordWrap` or a small internal enum when there is a real state machine distinction

### Scalar and Buffer Mappings

- `char *` buffers
  - `String` when content is valid text and scan logic is character-oriented
  - `Vec<u8>` if exact byte preservation is required during incremental processing
- size/count fields
  - `usize`
- C booleans/flags
  - `bool`
- file descriptor / `FILE *`-style output abstraction
  - `W: std::io::Write`
- error codes / sticky failure indicators
  - `std::io::Result<()>` for immediate failures
  - optional stored `Option<std::io::ErrorKind>` or equivalent if the original API exposes post-failure querying via `wordwrap_error`

### Ownership and Memory Management

- Replace manual allocation/free paths with owned fields on `WordWrap` and `LineState`
- Avoid interior raw pointers entirely
- Keep borrow scopes short during write/flush operations to simplify mutation of both state and writer
- If buffered text must be rescanned repeatedly, prefer reusing the same allocation rather than rebuilding new buffers per write call

### Error Handling Strategy

- Convert direct write/flush failures into `io::Result`
- Preserve sticky-error semantics only where the C module’s API depends on querying error state after an operation
- Treat invalid multibyte sequences conservatively:
  - if the original logic tolerated them during width/wrap scanning, preserve byte-progress guarantees and avoid panics
  - never assume all input is valid UTF-8 unless call sites already enforce that

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Core State

- Create `src/parseopt/wordwrap.rs`
- Define the main `WordWrap<W: Write>` state struct and the internal `LineState`
- Port constructor/finalizer boundaries first:
  - `wordwrap_open`
  - `wordwrap_fdopen`
  - `wordwrap_close`
  - `wordwrap_error`
- Decide whether the public write surface accepts `&str`, `&[u8]`, or both based on the C call patterns and multibyte handling needs
- Encode margin/right-width configuration as plain Rust fields rather than callback-heavy C-style setup
- Add basic unit tests for object lifecycle:
  - create/open
  - close after no writes
  - flush/close propagation of writer errors

## Phase 2: Port Low-Level Write and Scan Helpers

- Port internal helpers that determine output correctness:
  - `full_write`
  - `_ww_fd_writer` only if still needed after `Write` integration
  - `safe_mbrtowc`
  - `wsprefix`
  - `detect_right_margin`
- Implement writer interaction using standard `Write` semantics
- Reproduce byte/character stepping behavior needed for wrap calculations
- Ensure helper logic never panics on malformed or incomplete multibyte input
- Add focused tests for:
  - repeated partial-buffer writes through the public API
  - whitespace-prefix handling
  - right-margin detection defaults/overrides
  - malformed multibyte input handling if supported by the original module

## Phase 3: Port Wrapping State Machine and Buffer Rescan Logic

- Port the line-management logic:
  - `wordwrap_line_init`
  - `wordwrap_rescan`
  - `wordwrap_next_left_margin`
- Translate the C control flow into methods on `WordWrap`/`LineState` without redesigning the algorithm
- Preserve current-buffer mutation order so line breaks, indentation, and rescans match the original behavior as closely as possible
- Keep temporary allocations bounded by reusing internal buffers
- Add unit tests covering:
  - left-margin advancement
  - wrap at right margin
  - lines requiring rescan after new input
  - preservation of whitespace behavior around wrapped boundaries

## Phase 4: Port Public Write/Flush Surface and Final Behavior Validation

- Port the main operational functions:
  - `wordwrap_write`
  - `wordwrap_puts`
  - `wordwrap_flush`
- Connect helper/state-machine logic into the public API
- Verify flush semantics for:
  - pending buffered text
  - explicit newline boundaries
  - close-after-write behavior
- Confirm sticky error behavior if retained
- Add end-to-end tests comparing expected output for:
  - simple unwrapped text
  - multi-line wrapped text
  - mixed whitespace and indentation
  - explicit flush between writes
  - write failure propagation from the underlying writer