# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_wordwrap.c_13`

## Summary

This module provides buffered word-wrapping output for text streams. It accepts text through a write-oriented handle, tracks line state, applies left and right margin behavior, preserves error state, and emits wrapped output through either a caller-supplied writer callback or a file-descriptor-backed writer.

The Rust rewrite must preserve the module’s observable behavior as evidenced by `src/parseopt/wordwrap.c`, including:

- opening and closing a word-wrap output handle,
- detecting or assigning a right margin for wrapping,
- buffering text until flush or wrap decisions are made,
- wrapping output at word boundaries based on current margins,
- handling newline-driven line resets,
- exposing flush and error reporting,
- supporting left-margin changes for subsequent lines,
- accepting both explicit-length writes and NUL-terminated string writes.

## Scope

In scope:

- Functional behavior represented by the `wordwrap_file` handle and the exported operations:
  - `wordwrap_open`
  - `wordwrap_fdopen`
  - `wordwrap_close`
  - `wordwrap_flush`
  - `wordwrap_error`
  - `wordwrap_next_left_margin`
  - `wordwrap_write`
  - `wordwrap_puts`
- Internal behavior necessary to achieve the public semantics, including line initialization, rescanning buffered text, whitespace-prefix handling, full buffered writes, and right-margin detection.

Out of scope:

- Any public API not evidenced in the source file.
- Concurrency guarantees.
- Serialization or persistence.
- Recovery semantics beyond returned status and stored error state.
- Performance guarantees beyond functional equivalence.

---

## Feature Specification

### 1. Word-wrapping output handle

The module manages a stateful output handle representing an in-progress wrapped text stream. The handle encapsulates:

- an output destination,
- current buffered line content,
- current line position and wrap-related positions,
- configured or detected margins,
- multibyte/text-scanning state needed for width-sensitive processing,
- persistent error status.

The Rust version must provide equivalent stateful behavior so that a caller can incrementally submit text and receive wrapped output in the same logical order.

### 2. Output destination abstraction

The module supports two destination forms:

- a caller-provided writer callback with opaque caller data,
- a file descriptor destination using an internal adapter.

The Rust rewrite must preserve both functional modes:

- custom sink mode, where bytes are emitted through caller-supplied write logic,
- file-descriptor-backed mode, where output is directed to a file descriptor using the module’s default writer behavior.

### 3. Right margin detection and use

When a wrapping handle is opened, the module determines the effective right margin. The C source evidences a right-margin detection step associated with terminal/window sizing and fallback behavior.

The Rust version must:

- determine an effective right margin at open time,
- use that margin when deciding whether buffered text fits on the current line or must wrap,
- preserve fallback behavior when terminal-width detection is unavailable or unsuitable.

This requirement is limited to reproducing the observed functional role of `detect_right_margin`; it does not require exposing margin detection as a public feature.

### 4. Line initialization and reset behavior

The module initializes line state when a handle is created and resets relevant line-tracking state when beginning a fresh line. Source evidence shows dedicated line initialization logic and explicit handling for clearing pending whitespace state.

The Rust version must preserve line-state reset behavior needed for:

- starting a new output stream,
- continuing after a newline,
- beginning a wrapped continuation line,
- applying next-line left-margin settings correctly.

### 5. Buffered writing and complete emission

The module buffers text internally and emits it through a write helper that attempts to write the full buffered amount before considering the operation complete. Flush and close operations depend on this behavior.

The Rust version must ensure:

- buffered content is retained until emitted or an error occurs,
- flush writes pending buffered content to the destination,
- close flushes pending content before releasing resources,
- partial sink writes are handled so that requested buffered output is fully attempted,
- sink failure is reflected in module status.

### 6. Incremental text acceptance

The module accepts text incrementally in two forms:

- byte sequence plus explicit length,
- NUL-terminated string convenience form.

The Rust version must preserve both input styles functionally:

- `write`-style submission for arbitrary byte sequences with known length,
- `puts`-style submission for a full C-string equivalent.

Submitted text may arrive in segments; wrapping decisions must remain consistent across segmented writes.

### 7. Word-boundary wrapping

The core feature of the module is word wrapping. It scans buffered text, tracks positions within the current line, and decides where to break lines relative to margins and whitespace boundaries.

The Rust rewrite must preserve these observable behaviors:

- text that fits within the effective line width remains on the current line,
- when the current line would exceed the right margin, output is wrapped at a suitable earlier break opportunity,
- wrapping prefers word boundaries derived from whitespace-aware scanning,
- continuation output starts as a new line with the appropriate left margin state.

The specification does not require reproducing internal variable layout, only the same functional behavior.

### 8. Whitespace-prefix handling

The source includes dedicated logic for identifying a whitespace prefix and line initialization with optional whitespace clearing. This indicates that leading whitespace handling affects wrapped-line formatting.

The Rust version must preserve whitespace-prefix behavior relevant to wrapping, specifically:

- leading whitespace may be treated specially when forming or continuing lines,
- wrapped continuation behavior must remain consistent with source handling of whitespace prefixes,
- any removal or preservation of pending leading whitespace must match the source-visible output behavior.

### 9. Newline-sensitive behavior

The module tracks line content and positions in a way that distinguishes explicit newlines from ordinary word wrapping.

The Rust version must preserve that distinction:

- explicit newline characters terminate the current line,
- line state is reset after an explicit newline,
- text after a newline begins a new logical line rather than continuing wrap calculations from the previous one.

### 10. Next-line left margin control

The module exposes an operation for setting the left margin for the next line. Source evidence indicates this affects subsequent wrapped or newly started lines rather than retroactively reformatting already buffered output.

The Rust version must preserve the semantics that:

- a caller can request a left margin value to apply to the next line,
- the change affects subsequent line starts,
- already emitted output is unchanged,
- the operation returns status indicating success or failure.

### 11. Error reporting

The module exposes an error-query function and stores error state internally.

The Rust rewrite must preserve:

- persistent recording of sink/write-related failure state,
- ability for callers to query whether an error has occurred,
- failure signaling from operations that cannot complete because of write or internal processing errors.

---

## User Scenarios & Testing

### Scenario 1: Wrap text to a file descriptor-backed destination

A caller opens a wrapping handle for an output file descriptor, writes a long line of text, and flushes or closes the handle.

Expected behavior:

- output is emitted to the file descriptor,
- lines are wrapped according to the effective right margin,
- no text is lost between write and close,
- close finalizes any pending buffered content.

### Scenario 2: Use a custom writer callback

A caller opens the handle with a custom writer and opaque context, then submits text in multiple chunks.

Expected behavior:

- the custom writer receives output in the correct byte order,
- wrapping behavior is the same as with a file descriptor destination,
- segmented writes behave as one continuous text stream unless explicit newlines occur,
- sink failures propagate to return values and error state.

### Scenario 3: Write text that already contains explicit newlines

A caller writes text containing one or more newline characters.

Expected behavior:

- each explicit newline ends the current logical line,
- wrapping state restarts after each newline,
- subsequent text begins on a new line with current margin rules.

### Scenario 4: Wrap at word boundaries

A caller writes text long enough to exceed the right margin.

Expected behavior:

- the module does not simply split arbitrarily when a prior break opportunity exists,
- wrapping occurs at a whitespace-derived boundary when possible,
- the continuation line begins with the applicable left margin state.

### Scenario 5: Preserve behavior across multiple incremental writes

A caller sends part of a word or sentence in one write call and the remainder in later write calls.

Expected behavior:

- the module maintains sufficient internal state to wrap correctly across call boundaries,
- output matches the same logical result as if the same bytes had been written in a single call.

### Scenario 6: Apply a next-line left margin

A caller writes some text, requests a new left margin for the next line, then continues writing until wrapping or newline causes another line to begin.

Expected behavior:

- the requested left margin affects the next started line,
- the current already-started line is not reformatted retroactively,
- subsequent wrapped/new lines use the updated margin as defined by source behavior.

### Scenario 7: Flush pending content without closing

A caller writes text that remains buffered, then calls flush.

Expected behavior:

- all pending buffered content is emitted,
- the handle remains usable after flush unless an error has occurred,
- error state is set if flush cannot fully emit buffered content.

### Scenario 8: Observe write failure and query error state

A caller uses a destination that fails during output.

Expected behavior:

- the write/flush/close-related operation reports failure,
- the handle records an error state,
- a later error query reports that failure.

### Testing focus

The Rust version must be tested for:

- open/write/flush/close lifecycle behavior,
- file-descriptor-backed and callback-backed destinations,
- wrapping at and below the right margin,
- explicit newline handling,
- whitespace-prefix-sensitive wrapping cases,
- incremental writes across chunk boundaries,
- next-line left-margin application,
- failure propagation and error querying.

---

## Requirements

### Functional Requirements

#### FR-1: Open a word-wrap output handle
The module shall create a stateful wrapping handle bound to either a caller-supplied writer callback and opaque data or to a file descriptor-backed writer, as evidenced by `wordwrap_open` and `wordwrap_fdopen`.

#### FR-2: Initialize line state on creation
The module shall initialize internal line-tracking state when a handle is created so that the first write begins from a clean logical line state, as evidenced by `wordwrap_line_init` usage from open-time setup.

#### FR-3: Determine an effective right margin
The module shall assign an effective right margin during handle creation and use that margin for wrapping decisions, as evidenced by `detect_right_margin` and its association with `wordwrap_open`.

#### FR-4: Accept incremental byte-oriented writes
The module shall accept caller-provided text with explicit byte length and incorporate it into the current wrapped output stream, as evidenced by `wordwrap_write`.

#### FR-5: Accept whole-string convenience writes
The module shall accept a NUL-terminated string convenience input and process it as wrapped output text, as evidenced by `wordwrap_puts`.

#### FR-6: Preserve logical text order across segmented writes
The module shall maintain internal buffered state so that multiple sequential write calls are processed as one continuous stream unless explicit line breaks occur, as evidenced by `wordwrap_write`, `wordwrap_rescan`, and the stored line-position state in `wordwrap_file`.

#### FR-7: Wrap output according to right margin
The module shall prevent continued output beyond the effective right margin by wrapping buffered content into a new line when needed, as evidenced by `wordwrap_write`, `wordwrap_rescan`, and tracked positions in `wordwrap_file`.

#### FR-8: Prefer wrap points derived from whitespace scanning
The module shall use whitespace-aware scanning to identify suitable wrap boundaries, as evidenced by `wsprefix`, `wordwrap_rescan`, and wrap-position tracking within `wordwrap_file`.

#### FR-9: Distinguish explicit newlines from automatic wrapping
The module shall treat explicit newline characters as terminating the current logical line and resetting line state for following text, as evidenced by line reinitialization logic and `wordwrap_write` processing.

#### FR-10: Apply left margin changes to the next line
The module shall support setting a left margin value that takes effect when the next line begins, as evidenced by `wordwrap_next_left_margin`.

#### FR-11: Flush buffered output on request
The module shall provide an operation that emits any pending buffered content to the output destination without closing the handle, as evidenced by `wordwrap_flush`.

#### FR-12: Flush pending buffered output during close
The module shall attempt to emit pending buffered content before releasing the handle during close, as evidenced by `wordwrap_close` and `full_write`.

#### FR-13: Attempt complete output to the destination
The module shall repeatedly invoke the configured output sink as needed to attempt full emission of buffered data, as evidenced by `full_write` and `_ww_fd_writer`.

#### FR-14: Record and expose error state
The module shall preserve an error status when output or processing fails and provide a way for callers to query that status, as evidenced by `wordwrap_error`, `full_write`, and close/flush/write status behavior.

#### FR-15: Support multibyte-aware text scanning used for wrapping
The module shall perform text scanning for wrapping using multibyte-aware conversion behavior sufficient to preserve source-equivalent wrap decisions, as evidenced by `safe_mbrtowc` and its use in scanning helpers.

### Key Entities

#### `wordwrap_file`
The central module state object representing an open wrapping stream.

Responsibilities evidenced in the source:

- stores the output destination configuration,
- stores buffered line content,
- stores current and pending margin-related state,
- stores scanning and position-tracking state used to decide wraps,
- stores multibyte conversion state,
- stores persistent error status.

Relationship:
- created by open operations,
- consumed by write/flush/error/margin/close operations,
- owns the state required by helper behaviors such as rescanning and full writes.

#### Position records
The module uses position-tracking structures (`struct position`) to mark significant locations within buffered text.

Functional role evidenced in the source:

- track current line progress,
- identify candidate wrap boundaries,
- support rescanning after additional text is buffered.

Relationship:
- embedded within `wordwrap_file`,
- updated during scanning and writing,
- used to decide where output can be wrapped.

#### Output sink configuration
The module binds each handle to either:

- a generic writer callback plus opaque data, or
- a file descriptor through the module’s default adapter.

Functional role:
- abstracts where emitted wrapped bytes are sent,
- allows the same wrapping behavior with different destinations.

Relationship:
- configured at open time,
- used by full-write and flush/close paths.

#### Margin state
The module maintains a right margin and left-margin-related state for line formatting.

Functional role:
- right margin constrains line width,
- next-line left margin affects where subsequent lines begin.

Relationship:
- initialized at open time,
- consulted during wrapping,
- updated by the next-left-margin operation.

---

## Success Criteria

1. A Rust implementation can open a wrapping handle for both supported destination modes and successfully emit text through either mode, matching the functional scope of `wordwrap_open` and `wordwrap_fdopen`.

2. Given input shorter than the effective right margin, the Rust implementation emits the text without introducing additional wrap breaks, except for explicit newline handling evidenced by `wordwrap_write`.

3. Given input that exceeds the effective right margin and contains whitespace break opportunities, the Rust implementation produces wrapped output at suitable word boundaries consistent with the source behavior evidenced by `wsprefix`, `wordwrap_rescan`, and `wordwrap_write`.

4. Given explicit newline characters in input, the Rust implementation terminates the current logical line and restarts line state for following text, consistent with source line-reset behavior.

5. Given the same byte stream delivered in one write or in multiple sequential writes, the Rust implementation produces the same final wrapped output, consistent with the buffered incremental design evidenced by `wordwrap_write` and `wordwrap_rescan`.

6. After calling the next-line left-margin operation before a new line begins, the Rust implementation applies the requested left margin to the next started line and not retroactively to already emitted output, consistent with `wordwrap_next_left_margin`.

7. Calling flush on a handle with pending buffered content causes that content to be emitted without closing the handle, consistent with `wordwrap_flush`.

8. Calling close on a handle with pending buffered content attempts final emission before releasing the handle, consistent with `wordwrap_close`.

9. When the underlying sink performs partial writes, the Rust implementation continues output attempts until the buffered chunk is fully written or a failure is reached, consistent with `full_write`.

10. When the underlying sink fails, the Rust implementation returns failure from the relevant operation and preserves an error state observable through the error-query operation, consistent with `wordwrap_error` and write/flush/close behavior.

11. Wrapping decisions for multibyte text are made using multibyte-aware scanning behavior sufficient to match the source’s observable output decisions, consistent with `safe_mbrtowc` and scanning helpers.