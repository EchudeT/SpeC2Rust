# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_wordwrap.c_13`

## Metadata

- Project: `cflow-new`
- Source module: `src/parseopt/wordwrap.c`
- Module category: `module_cluster`
- Target Rust branch: `110-module_src_parseopt_wordwrap.c_13-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides buffered word-wrapping output for text streams. It accepts text through a stateful writer object, tracks line layout, applies left and right margin rules, preserves and interprets whitespace relevant to wrapping, and emits formatted output through either a caller-supplied write callback or a file-descriptor-backed default writer.

The Rust rewrite must preserve the observable behavior evidenced by the C module:

- opening a wrapping writer over either a file descriptor or caller-provided sink,
- detecting or assigning a right margin used for wrapping,
- buffering input until a complete writeable segment can be emitted,
- wrapping long text across lines while maintaining line state and margins,
- supporting explicit flush, close, error query, and string-oriented write operations,
- handling multibyte text during width and whitespace-related scanning in a non-crashing manner.

This specification covers only behavior evidenced by `src/parseopt/wordwrap.c`.

## Feature Specification

### 1. Stateful word-wrapping output

The module shall expose a stateful output handle representing an active wrapping session. Callers write text incrementally, and the module formats that text according to current line state rather than simply forwarding bytes unchanged.

Supported session lifecycle behavior is evidenced by:

- opening a handle with `wordwrap_open`,
- opening a descriptor-backed handle with `wordwrap_fdopen`,
- flushing buffered formatted output with `wordwrap_flush`,
- closing the handle with `wordwrap_close`,
- querying the error state with `wordwrap_error`.

### 2. Configurable output sink with descriptor-backed default

The module shall support two output modes:

- a caller-supplied write callback plus opaque user data,
- a default writer bound to a file descriptor.

`wordwrap_fdopen` shall create a wrapping session that writes to a file descriptor using the module’s descriptor-backed writer behavior. `wordwrap_open` shall create a wrapping session around the supplied callback and callback data.

The Rust version must preserve the distinction between a generic sink-backed session and a file-descriptor-backed session, while matching externally visible behavior.

### 3. Right-margin based line wrapping

The module shall maintain a right margin used to decide when buffered text must wrap to a new line. The right margin is part of active formatting behavior and is established during session setup, including terminal-width-based detection when applicable as evidenced by `detect_right_margin`.

The Rust version must implement right-margin-driven wrapping behavior so that long output is emitted as wrapped lines instead of unrestricted single-line output.

### 4. Left-margin continuation control

The module shall support changing the left margin for the next output line using `wordwrap_next_left_margin`. This affects subsequent wrapped output rather than retroactively changing already emitted text.

The Rust rewrite must preserve this deferred line-margin behavior.

### 5. Buffered writing with complete-output semantics

The module buffers text and writes formatted output through the sink in complete segments. When emitting buffered data, it must attempt to write the entire segment rather than silently accepting partial output, as evidenced by `full_write`.

The Rust implementation must preserve the observable contract that successful formatted emission fully transfers the intended output segment to the underlying sink, or records/report an error.

### 6. Explicit flush behavior

The module shall provide an explicit flush operation that forces any pending formatted output to be emitted to the underlying sink, as evidenced by `wordwrap_flush`.

The Rust version must ensure that data accepted by prior write calls is not left indefinitely buffered when flush is requested.

### 7. Write operations for byte strings and C-style strings

The module shall support:

- writing a buffer with explicit length via `wordwrap_write`,
- writing a NUL-terminated string via `wordwrap_puts`.

The Rust rewrite must support equivalent usage patterns for caller-provided text input.

### 8. Whitespace-aware wrapping and line-state maintenance

The module tracks positions within a line and rescans buffered content to update line state, whitespace prefixes, and wrap opportunities, as evidenced by `wsprefix`, `wordwrap_rescan`, and internal position tracking. The resulting behavior is that wrapping decisions are sensitive to whitespace and current line layout, not only raw byte count.

The Rust version must preserve whitespace-aware line formatting behavior.

### 9. Multibyte-safe text scanning

The module performs multibyte-to-wide-character scanning through `safe_mbrtowc` when processing text for wrapping and whitespace analysis. The evidenced requirement is robust text scanning in the presence of multibyte input, including invalid or incomplete sequences being handled safely enough for formatting logic to continue without crashing.

The Rust rewrite must preserve safe processing of multibyte text during wrapping-related scans.

### 10. Error state reporting

The module shall record and expose output error state via `wordwrap_error`. Errors from the underlying sink or from failed emission shall be observable to the caller after attempted operations.

The Rust version must provide equivalent error observability for the wrapping session.

## User Scenarios & Testing

### Scenario 1: Open a wrapper around a file descriptor and write short text

A caller opens a wrapping session bound to a file descriptor and writes text shorter than the current right margin.

Expected behavior:

- the text is accepted,
- wrapping does not introduce unnecessary line breaks,
- `flush` or `close` causes pending output to appear on the descriptor,
- no error is reported on a successful sink.

Test coverage:

- create descriptor-backed wrapper,
- write one short line,
- flush,
- verify output bytes match the input text formatting expected for a single line.

### Scenario 2: Write a long paragraph that exceeds the right margin

A caller writes a paragraph whose length exceeds the active right margin.

Expected behavior:

- output is split across multiple lines,
- the split occurs according to the module’s wrapping rules rather than arbitrary truncation,
- the entire accepted input is represented in output except for formatting changes inherent to wrapping.

Test coverage:

- use a known right margin in a controlled environment,
- write a long text with spaces,
- flush,
- verify that emitted lines do not exceed the expected margin behavior except where evidenced formatting rules require otherwise.

### Scenario 3: Incremental writes across multiple calls

A caller feeds a paragraph through several successive `write` calls rather than one call.

Expected behavior:

- wrapping behavior is based on the combined stream of input,
- line state carries across calls,
- output is equivalent to writing the same byte sequence in one call, subject to flush boundaries.

Test coverage:

- compare output of one large write vs several partial writes containing the same combined text,
- verify matching formatted output.

### Scenario 4: Apply a new left margin for the next line

A caller writes text, requests a new left margin for the next line, and continues writing.

Expected behavior:

- already emitted or current-line content is not reformatted retroactively,
- subsequent line starts use the requested left margin,
- wrapped continuation after the change reflects the new margin.

Test coverage:

- write content that spans at least two lines,
- request next left margin before the next line begins,
- verify the next produced line begins with the expected indentation.

### Scenario 5: Use a custom sink callback

A caller opens a wrapping session with a custom writer callback and opaque data.

Expected behavior:

- formatted output is delivered through the callback,
- callback data is passed through to the sink,
- flush and close complete output emission through the callback path.

Test coverage:

- use an in-memory sink callback,
- write and flush text,
- verify captured output matches expected formatted output.

### Scenario 6: Underlying sink performs partial writes

A caller uses a sink callback that accepts only part of a requested buffer per invocation.

Expected behavior:

- the wrapping layer continues attempting output until the entire formatted segment is written or an error occurs,
- successful overall operation produces complete output without truncation.

Test coverage:

- implement a custom sink that intentionally writes small chunks,
- write text and flush,
- verify the sink receives the complete formatted content.

### Scenario 7: Error propagation from the sink

A caller uses a sink callback that fails during output.

Expected behavior:

- the write/flush/close operation reports failure according to the module contract,
- the wrapper enters or exposes an error state,
- `wordwrap_error` reflects that an error occurred.

Test coverage:

- use a sink that fails predictably,
- perform write and flush attempts,
- verify failure result and subsequent error query.

### Scenario 8: Multibyte text input

A caller writes text containing multibyte characters.

Expected behavior:

- the module processes the text without crashing,
- wrapping and whitespace scanning remain well-defined,
- invalid or incomplete multibyte sequences do not cause memory unsafety or uncontrolled failure.

Test coverage:

- write valid UTF-8 or locale-relevant multibyte input,
- write malformed byte sequences if the Rust port models raw byte input,
- verify safe completion and consistent error/reporting behavior as defined by the port.

## Requirements

### Functional Requirements

#### FR-1: Wrapping session creation
The module shall create a wrapping output session from either:

- a file descriptor, or
- a caller-supplied write callback with opaque user data.

Traceability: `wordwrap_open`, `wordwrap_fdopen`, `_ww_fd_writer`.

#### FR-2: Session initialization
On session creation, the module shall initialize line-tracking state and establish initial formatting state needed for wrapping, including right-margin setup and initial line state.

Traceability: `wordwrap_line_init`, `detect_right_margin`, `wordwrap_open`, `struct wordwrap_file`.

#### FR-3: Buffered text acceptance
The module shall accept caller-provided text input as either a byte buffer with explicit length or a NUL-terminated string.

Traceability: `wordwrap_write`, `wordwrap_puts`.

#### FR-4: Margin-based line wrapping
The module shall wrap output according to an active right margin and current line state, producing line breaks when required by accumulated content and formatting rules.

Traceability: `detect_right_margin`, `wordwrap_write`, `wordwrap_rescan`, `struct position`, `struct wordwrap_file`.

#### FR-5: Whitespace-sensitive formatting
The module shall inspect leading or relevant whitespace within buffered text and use that information in line-state maintenance and wrap decisions.

Traceability: `wsprefix`, `wordwrap_rescan`, `wordwrap_write`.

#### FR-6: Deferred left-margin update
The module shall allow the caller to set the left margin to be used beginning with the next line.

Traceability: `wordwrap_next_left_margin`, `struct wordwrap_file`.

#### FR-7: Safe multibyte scanning
The module shall scan text for character and whitespace analysis in a way that safely handles multibyte input and malformed sequences without unsafe failure.

Traceability: `safe_mbrtowc`, `wsprefix`, `wordwrap_rescan`.

#### FR-8: Complete sink emission
When emitting formatted output to the underlying sink, the module shall continue writing until the full intended segment has been transferred or an error prevents completion.

Traceability: `full_write`, `_ww_fd_writer`.

#### FR-9: Explicit flush
The module shall provide a flush operation that emits pending buffered output to the underlying sink.

Traceability: `wordwrap_flush`.

#### FR-10: Close finalization
The module shall provide a close operation that finalizes the wrapping session and releases session resources after attempting required final output behavior.

Traceability: `wordwrap_close`.

#### FR-11: Error reporting
The module shall expose whether an output error has occurred during session use.

Traceability: `wordwrap_error`, `full_write`, `wordwrap_flush`, `wordwrap_close`.

### Key Entities

#### `WordWrapFile` session object
A stateful wrapping-output handle corresponding to `struct wordwrap_file`. It owns:

- the underlying output target reference,
- the write callback or descriptor-backed writer configuration,
- current formatting and line-tracking state,
- buffering for not-yet-emitted formatted text,
- error status.

This is the central entity through which all operations occur.

Traceability: `struct wordwrap_file`, `wordwrap_open`, `wordwrap_close`, `wordwrap_write`, `wordwrap_flush`, `wordwrap_error`.

#### Line position state
A small position-tracking structure corresponding to `struct position` is used to represent locations and/or layout state relevant to the current buffered line and wrap points.

Relationships:

- maintained inside the wrapping session,
- updated during rescanning and writing,
- used to decide where line breaks and whitespace handling apply.

Traceability: `struct position`, `wordwrap_rescan`, `wordwrap_write`, `wordwrap_line_init`.

#### Underlying writer
A sink abstraction consisting of either:

- a custom callback plus caller data, or
- the file-descriptor-backed writer path.

Relationships:

- owned or referenced by the wrapping session,
- used by flush/write-close emission paths,
- subject to complete-write semantics and error propagation.

Traceability: `_ww_fd_writer`, `wordwrap_open`, `wordwrap_fdopen`, `full_write`.

#### Margin configuration
Formatting state within the wrapping session that includes at least:

- an active right margin used for wrapping,
- a current or pending left margin used for line starts.

Relationships:

- initialized at open time,
- consulted during write/rescan,
- modified by next-left-margin requests.

Traceability: `detect_right_margin`, `wordwrap_next_left_margin`, `wordwrap_write`, `struct wordwrap_file`.

## Success Criteria

1. A Rust wrapping session can be created over both a descriptor-backed sink and a custom sink, and both forms accept text input successfully.
   - Traceability: `wordwrap_open`, `wordwrap_fdopen`.

2. Writing text shorter than the active wrap width produces output without unnecessary inserted line breaks.
   - Traceability: `wordwrap_write`, `wordwrap_flush`.

3. Writing text longer than the active wrap width produces wrapped output across multiple lines.
   - Traceability: `detect_right_margin`, `wordwrap_write`, `wordwrap_rescan`.

4. Writing the same logical text in one call or many sequential calls yields equivalent formatted output, aside from differences directly caused by explicit flush boundaries.
   - Traceability: `wordwrap_write`, `wordwrap_rescan`.

5. Setting the next left margin changes formatting beginning with the next produced line and does not retroactively alter already emitted content.
   - Traceability: `wordwrap_next_left_margin`.

6. Flush emits all pending buffered output to the sink on successful operation.
   - Traceability: `wordwrap_flush`, `full_write`.

7. Close finalizes output handling and returns a status consistent with whether output succeeded.
   - Traceability: `wordwrap_close`.

8. When the sink performs partial writes, the module still emits complete formatted segments unless a sink error occurs.
   - Traceability: `full_write`, `_ww_fd_writer`.

9. When the sink fails, the module reports failure and exposes an error state observable through the error-query operation.
   - Traceability: `wordwrap_error`, `full_write`, `wordwrap_flush`, `wordwrap_close`.

10. Multibyte input, including malformed byte sequences where applicable to the Rust API, is processed without memory-unsafety or uncontrolled termination during wrapping-related scanning.
    - Traceability: `safe_mbrtowc`, `wsprefix`, `wordwrap_rescan`.