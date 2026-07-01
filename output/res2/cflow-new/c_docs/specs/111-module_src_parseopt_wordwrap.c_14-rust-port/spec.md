# spec.md

## Title

Functional Specification for `module_src_parseopt_wordwrap.c_14` Rust Port

## Document Control

- **Project:** `cflow-new`
- **Module:** `module_src_parseopt_wordwrap.c_14`
- **Category:** `module_cluster`
- **Source file:** `src/parseopt/wordwrap.c`
- **Rust branch:** `111-module_src_parseopt_wordwrap.c_14-rust-port`
- **Generation date:** `2026-06-17`

## Overview

This module provides formatted text output through a word-wrapping writer abstraction. Its exposed behavior is centered on sending characters and formatted text to a `WORDWRAP_FILE` target while respecting paragraph and wrapping behavior maintained by the module’s internal output state.

The Rust rewrite must preserve the observable behavior evidenced by the source module’s public entry points:

- `wordwrap_putc`
- `wordwrap_para`
- `wordwrap_vprintf`
- `wordwrap_printf`

The rewrite scope is limited to the functionality represented by these operations and the output-state entities they rely on.

## Feature Specification

### Summary

The module acts as a text-emission layer for help-style or prose-style output where individual characters, paragraph breaks, and formatted strings are written through a word-wrap-aware output handle. The handle stores the current output state, including position-related state used to decide how emitted text should affect line and paragraph layout.

### In-Scope Functionality

The Rust version must implement:

1. **Character-oriented wrapped output**
   - Accept a single character for emission through a word-wrap-aware writer handle.
   - Update output state consistently with the emitted character.
   - Return an integer status compatible with success/failure signaling.

2. **Paragraph separation**
   - Emit a paragraph break through the writer handle.
   - Cause subsequent text to start after the paragraph separator according to the module’s maintained layout rules.

3. **Formatted output with varargs-backed formatting**
   - Accept a format string and argument list and emit the resulting text through the word-wrap-aware writer.
   - Apply the same wrapping and layout behavior as other text written through the module.
   - Preserve error signaling when formatting or output fails.

4. **Convenience formatted output**
   - Provide a variadic formatted-print operation equivalent in behavior to the varargs-list form.
   - Forward formatting semantics through the same writer behavior and status reporting.

5. **Position-aware layout behavior**
   - Maintain internal position state sufficient to support word wrapping and paragraph formatting across multiple write calls.
   - Preserve continuity of layout across mixed use of character output, paragraph emission, and formatted output.

### Out of Scope

The Rust rewrite must not introduce new externally visible capabilities not evidenced by the source input, including:

- new public formatting features,
- new persistence or serialization behavior,
- thread-safety guarantees,
- recovery or retry APIs,
- FFI-facing compatibility layers beyond what is needed internally for the port.

## User Scenarios & Testing

### Scenario 1: Incremental character emission

A caller has a word-wrap writer handle and emits text one character at a time. The module must treat these calls as a continuous stream, preserving internal position tracking so that wrapping behavior remains consistent across the sequence.

**Test focus:**
- Multiple `wordwrap_putc` calls produce continuous output behavior.
- State after earlier characters affects later layout decisions.
- Return status is propagated per call.

### Scenario 2: Formatted help text output

A caller writes a formatted message containing plain text and substituted values through the formatted-print interface. The module must emit the formatted text through the same wrapped output path used by direct character emission.

**Test focus:**
- `wordwrap_printf` formats arguments and writes the resulting text.
- Wrapped output behavior matches the behavior expected from equivalent character-stream emission.
- Errors from formatting/output are reflected in the return value.

### Scenario 3: Varargs-list formatted emission

A caller already holds a `va_list`-style argument bundle and writes formatted output using the varargs-list entry point. The module must produce output behavior equivalent to the convenience variadic form.

**Test focus:**
- `wordwrap_vprintf` accepts a format string plus argument list and emits identical output to the variadic wrapper for the same content.
- Return values are consistent between the two paths.

### Scenario 4: Paragraph break between text blocks

A caller writes one block of text, requests a paragraph break, then writes another block. The module must separate the two blocks as a paragraph boundary rather than as uninterrupted flowing text.

**Test focus:**
- `wordwrap_para` creates a visible paragraph separation in output behavior.
- Subsequent formatted or character output begins after the separator according to maintained layout state.
- Repeated paragraph operations remain well-defined and return status codes.

### Scenario 5: Mixed-mode output session

A caller mixes formatted writes, direct character writes, and paragraph breaks on the same writer handle. The module must preserve a single coherent wrapping state across all operations.

**Test focus:**
- State continuity is preserved when alternating among `wordwrap_printf`, `wordwrap_vprintf`, `wordwrap_putc`, and `wordwrap_para`.
- No operation resets layout state unexpectedly unless implied by paragraph semantics.

## Requirements

### Functional Requirements

#### FR-1: Wrapped writer handle support
The module shall operate on a word-wrap writer handle representing an output destination plus the state needed for layout decisions.

**Traceability:** `struct wordwrap_file`, `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`

#### FR-2: Single-character output
The module shall provide an operation that accepts one character and writes it through the wrapped writer, returning an integer status.

**Traceability:** `wordwrap_putc`

#### FR-3: Paragraph emission
The module shall provide an operation that emits a paragraph break for the wrapped writer and returns an integer status.

**Traceability:** `wordwrap_para`

#### FR-4: Formatted output from argument list
The module shall provide an operation that accepts a format string plus argument list and emits the formatted result through the wrapped writer, returning an integer status.

**Traceability:** `wordwrap_vprintf`

#### FR-5: Variadic formatted output
The module shall provide a variadic formatted-print operation whose observable output behavior is equivalent to the argument-list form for the same format and values.

**Traceability:** `wordwrap_printf`, `wordwrap_vprintf`

#### FR-6: Shared layout state across operations
The module shall preserve output position and wrapping state across successive operations on the same writer handle, including mixed use of character output, paragraph output, and formatted output.

**Traceability:** `struct wordwrap_file`, `struct position`, `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`

#### FR-7: Position-based text layout
The module shall maintain internal position-related state sufficient to support line/column-aware text layout behavior used for wrapping and paragraph handling.

**Traceability:** `struct position`, `struct wordwrap_file`

#### FR-8: Error status propagation
The module shall report success or failure via the integer return value of each exposed operation, preserving failure information from formatting or output processing.

**Traceability:** `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`

### Key Entities

#### `wordwrap_file`
The central module entity is a word-wrap output context. It represents the active output target together with mutable layout state used while writing wrapped text, formatted strings, and paragraph breaks.

**Relationship:** Owns or aggregates position-tracking state and is consumed by all exposed operations.

**Traceability:** `struct wordwrap_file`

#### `position`
This entity represents output position state used by the module to track where text is within the current layout. Multiple position values appear in the module, indicating that the writer context maintains more than one position-related marker for formatting and wrapping decisions.

**Relationship:** Stored within the word-wrap output context and updated as text is emitted.

**Traceability:** `struct position`

#### `winsize`
This entity represents terminal or display width information referenced by the module and supports width-sensitive text layout behavior.

**Relationship:** Used by the module’s layout logic as an external sizing concept associated with wrapping decisions.

**Traceability:** `struct winsize`

## Success Criteria

### Functional Acceptance Criteria

1. **Character output parity**
   - A Rust implementation of the character-output operation accepts a writer handle and character input and returns an integer-like success/failure result equivalent in role to the C module.
   - **Traceability:** `wordwrap_putc`

2. **Paragraph handling parity**
   - A Rust implementation of paragraph emission creates paragraph separation behavior on the same writer state and returns success/failure status.
   - **Traceability:** `wordwrap_para`

3. **Formatted output parity**
   - A Rust implementation of formatted output from an argument-list pathway emits formatted text through the wrapped writer and returns success/failure status.
   - **Traceability:** `wordwrap_vprintf`

4. **Variadic wrapper parity**
   - The Rust rewrite provides a convenience formatted-output entry point equivalent in observable behavior to the argument-list form for identical formatted content.
   - **Traceability:** `wordwrap_printf`, `wordwrap_vprintf`

5. **State continuity across mixed calls**
   - In tests that mix character writes, paragraph breaks, and formatted writes on one writer handle, output layout behavior remains continuous and coherent across calls.
   - **Traceability:** `struct wordwrap_file`, `struct position`, all four exposed functions

6. **Position-aware wrapping preserved**
   - The Rust rewrite preserves stateful, position-based layout behavior rather than treating each call as an isolated write.
   - **Traceability:** `struct position`, `struct wordwrap_file`

7. **No scope expansion**
   - The Rust module exposes only the behavior needed to support the evidenced wrapped-output and paragraph/formatting operations, without adding unrelated public capabilities.
   - **Traceability:** source scope limited to `src/parseopt/wordwrap.c` public functions and associated state types

## Notes for Port Validation

- Validation should compare observable output behavior and return-status behavior, not line-by-line implementation strategy.
- Tests should exercise repeated and mixed calls on the same writer state because the module’s functionality depends on persistent layout state.
- The port should preserve the module’s role as a formatting and wrapped-output component, not broaden it into a general text framework.