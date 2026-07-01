# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_wordwrap.c_13`

## Overview

This module provides a stateful word-wrapping output writer. It accepts text incrementally, tracks margins and current line state, wraps output at a right margin, preserves and reapplies indentation behavior across wrapped lines, and writes the resulting bytes through a caller-supplied output target or directly to a file descriptor.

The Rust rewrite must preserve the observable behavior evidenced by `src/parseopt/wordwrap.c`, specifically the functionality exposed through:

- opening a word-wrap writer over a file descriptor or caller-provided write callback,
- incrementally writing text,
- flushing buffered wrapped output,
- reporting write/error state,
- closing the writer,
- controlling the left margin for the next line.

The module is not a general text formatting engine. The required scope is limited to the word-wrapping and output behaviors evidenced by the listed functions and the internal state tracked by `struct wordwrap_file` and `struct position`.

## Feature Specification

### Summary

The module implements a buffered writer that reformats text into wrapped lines before sending it to an output sink. Wrapping behavior depends on tracked line position, left margin state, whitespace scanning, and a detected or configured right margin. The writer supports both direct file-descriptor-backed output and custom output callbacks.

### Required Functional Behavior

1. **Open and initialize a word-wrap writer**
   - The module must create a writer state from either:
     - a file descriptor plus optional custom write callback/data (`wordwrap_open`), or
     - a file descriptor using the module’s default file-descriptor writer (`wordwrap_fdopen`).
   - Initialization must establish a valid empty line state suitable for incremental writes, including margin-related state and error state.
   - When created for a terminal-like file descriptor, the writer must determine the right margin using the same source of information evidenced by `detect_right_margin`; when unavailable, it must fall back to the module’s default right-margin behavior.

2. **Write through an abstract output sink**
   - The module must support writing wrapped output through:
     - a supplied callback of the form equivalent to `(data, bytes, len) -> ssize_t`, or
     - a built-in file-descriptor writer.
   - Output operations must attempt to emit the full requested buffered content before reporting success, as evidenced by `full_write`.

3. **Maintain incremental wrapping state across calls**
   - `wordwrap_write` must accept arbitrary text fragments and continue wrapping correctly even when a logical line spans multiple write calls.
   - The module must preserve enough state to continue processing after partial input, including current column/position tracking and pending buffered text.

4. **Wrap lines at the active right margin**
   - The writer must identify wrap opportunities based on whitespace boundaries and current line width state.
   - When text exceeds the active right margin, the writer must break output into lines according to the module’s wrap logic and continue the remaining text on a subsequent line.
   - Wrapped continuation lines must honor the active left-margin behavior tracked by the writer state.

5. **Track and apply left margin behavior**
   - The module must support setting the left margin to be used for the next line through `wordwrap_next_left_margin`.
   - The next-line margin setting must affect subsequent output as evidenced by the module’s line reinitialization behavior.
   - Margin handling must be part of line-state initialization and wrapping behavior, not a separate formatting pass.

6. **Recognize and process whitespace prefixes**
   - The module must detect leading whitespace in text segments for indentation and wrap calculations, as evidenced by `wsprefix`.
   - The amount of leading whitespace relevant to line state must be derived using the same class of character-aware scanning evidenced by the original module.

7. **Support multibyte-aware text scanning for wrapping decisions**
   - The module must interpret text for whitespace/prefix/rescan purposes with multibyte character handling equivalent in intent to `safe_mbrtowc` and the rescan logic.
   - Invalid or incomplete multibyte sequences must not cause undefined behavior in wrapping logic; behavior must remain safe and deterministic.

8. **Rescan buffered line content when needed**
   - The module must be able to recompute line-position-related state from buffered content after changes that require it, as evidenced by `wordwrap_rescan`.
   - This includes maintaining correct wrapping decisions after internal buffer updates.

9. **Flush pending wrapped output**
   - `wordwrap_flush` must force any buffered content pending emission to be written to the output sink.
   - After a successful flush, no previously buffered bytes remain unwritten within the module.

10. **Expose error status**
    - `wordwrap_error` must report whether the writer is in an error state due to prior failed output or related failure during processing.
    - Once an error is recorded, subsequent calls must reflect that state consistently.

11. **Close the writer**
    - `wordwrap_close` must flush/finalize module state and release resources associated with the writer object.
    - Closing must report success or failure consistent with final write/error outcome.

12. **Provide string convenience write**
    - `wordwrap_puts` must write a NUL-terminated string equivalent to passing its byte contents and length to the main write path.

## User Scenarios & Testing

### Scenario 1: Open a writer on a file descriptor and write a short line
A caller opens the module with `wordwrap_fdopen`, writes a line shorter than the right margin, flushes, and closes.

**Expected behavior**
- Output bytes match the input text exactly, except for any line-state behavior directly evidenced by the module.
- No unintended wrapping occurs.
- Flush and close succeed.
- Error status remains clear.

### Scenario 2: Incrementally write a long paragraph that must wrap
A caller opens the writer, sends a paragraph in several `wordwrap_write` calls, and the combined text exceeds the right margin.

**Expected behavior**
- Wrapping is determined from the full logical stream, not separately per call.
- Line breaks are inserted according to the module’s whitespace-aware wrap behavior.
- Continuation lines use the module’s current left-margin rules.
- Flush emits the final buffered remainder.

### Scenario 3: Use a custom writer callback
A caller opens the writer with a custom callback and user data, then writes wrapped text.

**Expected behavior**
- The callback receives the transformed output bytes in one or more calls.
- The module retries/continues as needed to complete full emission of buffered data unless an error occurs.
- On callback failure, the module records an error and reports it via write/flush/close status and `wordwrap_error`.

### Scenario 4: Set the next line’s left margin
A caller writes some text, calls `wordwrap_next_left_margin`, then writes more text that starts a subsequent line or wraps onto one.

**Expected behavior**
- The next applicable line begins with the requested left margin behavior.
- Margin state affects line initialization for subsequent output, not already-emitted text.
- The margin change is observable in wrapped output.

### Scenario 5: Preserve indentation-sensitive wrapping
A caller writes text that begins with whitespace or contains indentation before content.

**Expected behavior**
- Leading whitespace is recognized for line-state calculations as evidenced by `wsprefix`.
- Wrapped output preserves the module’s indentation semantics for continuation lines.
- Whitespace handling does not collapse unrelated content beyond what is evidenced by wrap processing.

### Scenario 6: Handle multibyte input safely
A caller writes UTF-8 or other multibyte text, including boundary cases where a multibyte sequence may be invalid or split in ways relevant to scanning.

**Expected behavior**
- The module does not crash or enter undefined state.
- Wrapping and whitespace scanning remain deterministic and safe.
- Error behavior, if any, follows the module’s normal write/error reporting path.

### Scenario 7: Query error state after sink failure
A custom writer fails partway through output.

**Expected behavior**
- The active write/flush/close operation reports failure.
- `wordwrap_error` reports the recorded error state afterward.
- The module does not falsely report success once the failure has occurred.

## Requirements

### Functional Requirements

#### FR-1: Writer creation
The Rust module shall provide a way to create a word-wrap writer over a file descriptor with either a caller-supplied writer callback or the module’s default file-descriptor writer, corresponding to `wordwrap_open` and `wordwrap_fdopen`.

**Traceability:** `wordwrap_open`, `_ww_fd_writer`, `wordwrap_fdopen`, `struct wordwrap_file`

#### FR-2: Initial line state setup
On creation and when starting a new line, the module shall initialize line-related state, including whitespace-clearing behavior and margin-dependent state, corresponding to `wordwrap_line_init`.

**Traceability:** `wordwrap_line_init`, `struct wordwrap_file`, `struct position`

#### FR-3: Right margin determination
The module shall determine an active right margin using terminal/window information when available for the associated file descriptor and otherwise use fallback behavior consistent with `detect_right_margin`.

**Traceability:** `detect_right_margin`, `wordwrap_open`

#### FR-4: Full buffered output emission
When flushing buffered content to the sink, the module shall continue writing until all requested bytes are emitted or an error occurs, corresponding to `full_write`.

**Traceability:** `full_write`, `_ww_fd_writer`

#### FR-5: Incremental text writing
The module shall accept byte string input with explicit length and process it incrementally through the word-wrapping state machine, corresponding to `wordwrap_write`.

**Traceability:** `wordwrap_write`, `struct wordwrap_file`

#### FR-6: String convenience writing
The module shall provide a convenience operation for writing NUL-terminated strings by forwarding their contents to the main write path, corresponding to `wordwrap_puts`.

**Traceability:** `wordwrap_puts`, `wordwrap_write`

#### FR-7: Whitespace-prefix detection
The module shall detect leading whitespace in buffered or incoming text for indentation and wrap logic, corresponding to `wsprefix`.

**Traceability:** `wsprefix`, `wordwrap_rescan`, `wordwrap_write`

#### FR-8: Multibyte-safe character scanning
The module shall perform character scanning used by wrapping logic in a multibyte-safe manner that remains well-defined for invalid input, corresponding to `safe_mbrtowc`.

**Traceability:** `safe_mbrtowc`, `wsprefix`, `wordwrap_rescan`

#### FR-9: Rescanning of line state
The module shall be able to rescan buffered content to recompute position-related wrapping state when needed, corresponding to `wordwrap_rescan`.

**Traceability:** `wordwrap_rescan`, `struct position`, `struct wordwrap_file`

#### FR-10: Flush support
The module shall provide an operation that forces pending buffered output to be emitted to the sink, corresponding to `wordwrap_flush`.

**Traceability:** `wordwrap_flush`, `full_write`

#### FR-11: Error reporting
The module shall expose whether the writer is in an error state after failed output or related processing failure, corresponding to `wordwrap_error`.

**Traceability:** `wordwrap_error`, `struct wordwrap_file`

#### FR-12: Next-line left margin control
The module shall allow the caller to set the left margin to be applied to the next line, corresponding to `wordwrap_next_left_margin`.

**Traceability:** `wordwrap_next_left_margin`, `wordwrap_line_init`, `struct wordwrap_file`

#### FR-13: Close/finalization
The module shall provide a close/finalization operation that releases writer resources and reports final success or failure, corresponding to `wordwrap_close`.

**Traceability:** `wordwrap_close`, `wordwrap_flush`, `struct wordwrap_file`

### Key Entities

#### `WordWrapFile` writer state
Rust shall represent the state carried by `struct wordwrap_file` as the module’s central writer object.

This entity holds:
- the output target configuration,
- current and next-line margin state,
- buffered text awaiting emission or rescan,
- current line-processing state,
- and recorded error state.

It is the owner of all state needed for incremental wrapping across writes.

**Traceability:** `struct wordwrap_file`, `wordwrap_open`, `wordwrap_write`, `wordwrap_flush`, `wordwrap_close`

#### `Position` line-position state
Rust shall represent the role of `struct position` as line/scan position state used during wrapping and rescanning.

This entity captures the position information required to:
- identify current line width/progress,
- mark wrap-relevant positions,
- and recompute state while rescanning buffered text.

**Traceability:** `struct position`, `wordwrap_line_init`, `wordwrap_rescan`, `wordwrap_write`

#### Output sink abstraction
Rust shall represent the output destination as either:
- a file-descriptor-backed sink using the module’s default writer behavior, or
- a caller-provided write callback plus associated user data.

This abstraction exists only to support the emission behavior evidenced by the C module.

**Traceability:** `_ww_fd_writer`, `wordwrap_open`, `wordwrap_fdopen`, `full_write`

## Success Criteria

1. **Creation parity**
   - The Rust module can create a writer using both supported creation modes evidenced by the C module.
   - Verified by tests covering callback-backed and file-descriptor-backed writers.

2. **No-wrap passthrough correctness**
   - For input shorter than the active right margin, emitted output matches expected unwrapped content.
   - Verified by deterministic unit tests using `wordwrap_write`, `wordwrap_flush`, and `wordwrap_puts`.

3. **Incremental wrapping correctness**
   - For a paragraph provided across multiple writes, output is identical to output produced when the same bytes are provided in a single write.
   - Verified by comparative tests over the same content split at different boundaries.

4. **Margin behavior correctness**
   - After invoking next-line left-margin control, the subsequent applicable line reflects the requested margin behavior.
   - Verified by tests that inspect wrapped output before and after `wordwrap_next_left_margin`.

5. **Whitespace-prefix handling correctness**
   - Inputs with leading whitespace produce output consistent with the module’s indentation-sensitive wrap logic.
   - Verified by tests using indented text and wrap-triggering line lengths.

6. **Flush correctness**
   - After a successful flush, all bytes accepted for output before the call have been delivered to the sink.
   - Verified by sink-capture tests that compare expected and actual emitted bytes.

7. **Error-state correctness**
   - If the output sink fails, the active operation reports failure and `wordwrap_error` subsequently indicates error state.
   - Verified by tests using a callback that fails after a controlled number of bytes or calls.

8. **Close correctness**
   - Closing a successful writer returns success; closing after a sink failure reports failure consistently with module error state.
   - Verified by tests covering both normal and failing output sinks.

9. **Multibyte safety**
   - Tests with valid multibyte text and invalid byte sequences complete without panic, memory unsafety, or undefined behavior in Rust, while preserving deterministic write/error reporting.
   - Verified by unit tests exercising wrap/scanning paths.

10. **Resource cleanup**
    - Repeated open/write/flush/close cycles complete without leaked module-owned state as observable in Rust test execution.
    - Verified by standard Rust test runs over repeated lifecycle scenarios.