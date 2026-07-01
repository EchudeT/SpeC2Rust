# spec.md

## Title

Functional Specification for `module_src_parseopt_wordwrap.c_14` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_wordwrap.c_14`
- Category: `module_cluster`
- Source file: `src/parseopt/wordwrap.c`
- Rust branch: `111-module_src_parseopt_wordwrap.c_14-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides formatted text output through a word-wrapping writer abstraction. Its public behavior is evidenced by four callable functions:

- `wordwrap_putc`
- `wordwrap_para`
- `wordwrap_vprintf`
- `wordwrap_printf`

The Rust rewrite must preserve the module’s observable role as a text-emission component that accepts characters and formatted strings, applies word-wrapping behavior through a maintained writer state, and supports paragraph separation.

The specification covers only behavior evidenced by the source module and its exposed state model. It does not require new APIs or extended capabilities beyond the C module’s role.

## Feature Specification

### Summary

The Rust version must implement a word-wrapping output facility centered on a writer object corresponding to `struct wordwrap_file`. The facility must support:

- emitting individual characters to the wrapped output stream,
- emitting paragraph breaks,
- emitting formatted text using `printf`-style formatting inputs,
- maintaining output position state needed for wrapping behavior.

### Supported behavior

1. **Character-based wrapped output**
   - The module must accept a single character for output through the writer handle.
   - The character must be processed under the same wrapping rules used by formatted output so that direct character emission and formatted emission remain behaviorally consistent.

2. **Paragraph separation**
   - The module must provide a paragraph operation that emits a paragraph break through the writer abstraction.
   - Paragraph output must interact correctly with the maintained output position state so that subsequent text begins in paragraph-separated output rather than continuing the previous line as uninterrupted text.

3. **Formatted wrapped output**
   - The module must support formatted text emission from a format string plus variable arguments.
   - The module must also support the `va_list`-based variant of the same operation.
   - The `printf` wrapper must be behaviorally equivalent to invoking the `va_list` variant with the same format and arguments.

4. **Stateful wrapping context**
   - The writer object must carry enough state to track output positioning and support wrapping decisions across multiple calls.
   - This includes position-related state evidenced by the module’s `struct position` usage within `struct wordwrap_file`.

5. **Return-value-based operation status**
   - The public operations return `int` in the C module.
   - The Rust port must preserve a clear success/failure result model for each public operation so callers can observe whether output succeeded.

## User Scenarios & Testing

### Scenario 1: Emit help or usage text incrementally
A caller owns a word-wrapping writer and emits text one piece at a time. Some content is written with formatted output, and some punctuation or spacing is written with single-character output.

**Expected support**
- Mixed use of formatted text and single-character output must produce a coherent wrapped stream.
- Writer state must persist across calls so later output continues from the current position.

**Test focus**
- Call formatted output, then character output, then formatted output again.
- Verify that output order and wrapping behavior are consistent across call boundaries.

### Scenario 2: Separate paragraphs in wrapped text
A caller prints one block of prose, invokes the paragraph operation, and then prints another block.

**Expected support**
- The second block must begin after a paragraph break rather than being appended directly to the previous line.
- Paragraph handling must update internal position state appropriately for continued wrapped output.

**Test focus**
- Emit text, invoke paragraph separation, emit more text.
- Verify that paragraph separation is visible in output and that subsequent wrapping starts from the correct paragraph context.

### Scenario 3: Use `printf`-style formatting for wrapped output
A caller prints wrapped content using a format string and arguments, including the convenience wrapper and the `va_list` form.

**Expected support**
- Both entry points must accept equivalent formatting content.
- The convenience wrapper must behave as a thin wrapper over the variadic-list form.

**Test focus**
- Compare output from `wordwrap_printf` and `wordwrap_vprintf` for the same format and values.
- Verify identical emitted text and equivalent success/failure reporting.

### Scenario 4: Stream output over multiple calls
A caller writes a long logical message across many invocations rather than one large formatted call.

**Expected support**
- Wrapping decisions must be based on cumulative writer state, not reset on each call.
- The module must remain usable for continued output after earlier successful calls.

**Test focus**
- Emit a long sentence in several calls.
- Verify that line wrapping reflects the continuing position rather than restarting at column zero for each call.

### Scenario 5: Observe operation failure
A caller depends on return values to detect whether output succeeded.

**Expected support**
- Each public operation must expose success/failure to the caller.
- Failure in underlying output or formatting must be reportable through the public result.

**Test focus**
- Use a test writer or sink capable of inducing output failure.
- Verify that all public operations propagate failure through their return/result contract.

## Requirements

### Functional Requirements

#### FR-1: Wrapped writer interface
The module shall provide a writer-oriented interface corresponding to the `WORDWRAP_FILE`-based API evidenced by `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, and `wordwrap_printf` in `src/parseopt/wordwrap.c`.

**Traceability**
- Functions: `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`
- Type: `struct wordwrap_file`

#### FR-2: Single-character emission
The module shall support emitting one character at a time through the wrapping writer and shall apply the writer’s active text-layout behavior to that emitted character.

**Traceability**
- Function: `wordwrap_putc`
- Type: `struct wordwrap_file`

#### FR-3: Paragraph emission
The module shall support a paragraph operation that inserts a paragraph break in the wrapped output stream and updates writer state so subsequent output is emitted in the new paragraph context.

**Traceability**
- Function: `wordwrap_para`
- Type: `struct wordwrap_file`
- Related state: `struct position`

#### FR-4: Formatted text emission from variadic argument list
The module shall support formatted output from a format string and a variadic argument list, with wrapping behavior applied as text is emitted.

**Traceability**
- Function: `wordwrap_vprintf`
- Type: `struct wordwrap_file`

#### FR-5: Formatted text emission from variadic arguments
The module shall support formatted output from a format string and direct variadic arguments, equivalent in behavior to the variadic-list form.

**Traceability**
- Function: `wordwrap_printf`
- Function relationship: wrapper over `wordwrap_vprintf`

#### FR-6: Persistent output position tracking
The module shall maintain output position state within the writer object across calls so that wrapping and paragraph behavior operate on the cumulative output context.

**Traceability**
- Types: `struct wordwrap_file`, `struct position`
- Functions: all four public functions

#### FR-7: Observable operation status
The module shall return a caller-observable status for each public operation, preserving the ability to distinguish successful output from failure.

**Traceability**
- Functions: `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`

### Key Entities

#### `wordwrap_file`
Primary writer-state entity for this module. It represents the active wrapped-output context used by all public functions. It owns or references the state required to continue output over time, including position-tracking fields evidenced by embedded `struct position` members.

**Relationship**
- Consumed by all public API functions.
- Contains position-related state used to determine current output context.

#### `position`
Position-tracking entity used by the writer state. Multiple occurrences in the source indicate that the module tracks more than one position aspect while producing wrapped text.

**Relationship**
- Stored within `wordwrap_file`.
- Updated as characters, formatted text, and paragraph breaks are emitted.

#### `winsize`
Auxiliary size-related entity present in the source module. It evidences that wrapping behavior is related to output width or terminal size constraints.

**Relationship**
- Supports the writer’s wrapping context.
- Relevant to layout decisions but not itself a public operation target.

## Success Criteria

1. **Public operation coverage**
   - The Rust port exposes equivalents for character emission, paragraph emission, variadic formatted emission, and convenience formatted emission.
   - Traceability: `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`

2. **Consistent mixed-output behavior**
   - In tests combining formatted text and single-character output on one writer instance, emitted output remains ordered and uses one continuous wrapping state.
   - Traceability: `wordwrap_putc`, `wordwrap_vprintf`, `wordwrap_printf`, `struct wordwrap_file`

3. **Paragraph handling correctness**
   - In tests inserting a paragraph break between two text blocks, the second block is emitted in a paragraph-separated context rather than directly continuing the first block.
   - Traceability: `wordwrap_para`, `struct wordwrap_file`, `struct position`

4. **Formatted API equivalence**
   - For identical format strings and values, the Rust equivalents of `wordwrap_printf` and `wordwrap_vprintf` produce identical output and equivalent success/failure results.
   - Traceability: `wordwrap_vprintf`, `wordwrap_printf`

5. **State persistence across calls**
   - In multi-call output tests, wrapping behavior reflects cumulative writer position rather than resetting between calls unless explicitly reinitialized by the caller.
   - Traceability: `struct wordwrap_file`, `struct position`, all public functions

6. **Failure propagation**
   - When the underlying sink or formatting path fails in controlled tests, each public operation reports failure through its return/result contract.
   - Traceability: all public functions

7. **No unsupported feature expansion**
   - The Rust port does not require new public capabilities beyond the evidenced wrapped-output, paragraph, formatting, and state-tracking behavior of `src/parseopt/wordwrap.c`.
   - Traceability: source file and public function set