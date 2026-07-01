# spec.md

## Title

Functional Specification: Rust Port of `src/parseopt/wordwrap.c`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_wordwrap.c_14`
- Category: `module_cluster`
- Source file: `src/parseopt/wordwrap.c`
- Rust branch: `111-module_src_parseopt_wordwrap.c_14-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides formatted text output with word-wrapping behavior for option/help-style text generation. Its exposed behavior is centered on writing characters, emitting paragraph breaks, and printing formatted text through a wrapping-aware output object.

The Rust rewrite must preserve the observable behavior of the C module’s word-wrapping output flow as evidenced by:

- `wordwrap_putc`
- `wordwrap_para`
- `wordwrap_vprintf`
- `wordwrap_printf`

The rewrite must preserve the module’s role as a stateful text formatter that tracks output position and applies wrapping decisions while writing to an associated output target.

## Feature Specification

### Supported functionality

The Rust module shall implement a stateful word-wrapping writer abstraction equivalent in behavior to the C module’s `wordwrap_file` state object.

The module shall support:

1. Writing individual characters through a wrapping-aware output path.
2. Emitting paragraph separation through a dedicated paragraph operation.
3. Writing formatted text using a `printf`-style interface, including a variadic-facing convenience entry point and a lower-level formatted-output entry point.
4. Maintaining output-position state sufficient to make line-wrapping and paragraph-layout decisions.
5. Preserving text layout behavior across sequences of writes, rather than treating each call as an isolated string emission.

### Behavioral boundaries

The Rust port shall remain limited to the functionality evidenced by the source module:

- word-wrapped text emission,
- paragraph handling,
- formatted output integration,
- internal tracking of output position.

The Rust port shall not require or imply capabilities not evidenced here, such as unrelated parsing behavior, persistence, concurrency guarantees, serialization, or expanded formatting features beyond those needed to preserve the module’s observed output behavior.

## User Scenarios & Testing

### Scenario 1: Writing help text one character at a time

A caller has a word-wrapping output object and emits text incrementally by sending characters through the character-output function.

Expected behavior:

- Characters are accepted through the wrapping-aware interface.
- The writer updates its internal position as output progresses.
- Wrapping behavior remains consistent with the current line state rather than resetting on each character.

Test guidance:

- Feed a sequence of ordinary printable characters.
- Include whitespace and newline-sensitive content.
- Verify that output and line positioning remain consistent across repeated `putc` calls.

### Scenario 2: Printing a formatted sentence that may span lines

A caller prints a formatted string containing substituted values through the formatted output function.

Expected behavior:

- The formatted text is emitted through the same wrapping-aware logic as other output.
- Wrapping decisions are based on the resulting rendered text.
- The function reports success/failure in a manner consistent with the original API family.

Test guidance:

- Print a sentence long enough to require wrapping.
- Use at least one formatting substitution.
- Compare resulting line breaks and spacing behavior with the C module output for the same state and width conditions.

### Scenario 3: Starting a new paragraph

A caller emits text, then requests a paragraph break, then emits more text.

Expected behavior:

- The paragraph operation creates a visible paragraph separation in output behavior.
- Output position state is reset or advanced as needed for the next paragraph according to the original module behavior.
- The next text begins in paragraph-initial layout state.

Test guidance:

- Emit one block of text, call the paragraph function, then emit another block.
- Verify that the separation matches the C module’s paragraph behavior.
- Confirm that wrapping of the second block is based on paragraph-start state.

### Scenario 4: Mixed incremental and formatted output

A caller combines direct character output, paragraph separation, and formatted text on the same wrapping object.

Expected behavior:

- All operations share one continuous formatting state.
- Layout remains coherent when switching between APIs.
- No operation bypasses the wrapping rules maintained by the module state.

Test guidance:

- Build one output document using all three public behaviors.
- Verify that the final output matches the C implementation for the same sequence of operations.

## Requirements

### Functional Requirements

#### FR-1: Stateful wrapping writer

The module shall provide a stateful output object corresponding to `struct wordwrap_file` that retains formatting state across calls.

Traceability:
- Type: `struct wordwrap_file` (`src/parseopt/wordwrap.c:72-95`)
- Evidence of stateful API use: `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`

#### FR-2: Character output through wrapping logic

The module shall support writing a single character to the wrapping writer through functionality equivalent to `wordwrap_putc`.

Expected outcome:
- The character is processed as part of the current wrapped output stream.
- Writer state is updated to reflect the emitted character’s effect on output position.

Traceability:
- Function: `wordwrap_putc` (`src/parseopt/wordwrap.c:658-663`)

#### FR-3: Paragraph emission

The module shall support a paragraph operation equivalent to `wordwrap_para`.

Expected outcome:
- The operation produces paragraph separation behavior consistent with the C module.
- Writer state after the operation is suitable for continued wrapped output in a new paragraph.

Traceability:
- Function: `wordwrap_para` (`src/parseopt/wordwrap.c:668-675`)

#### FR-4: Formatted output from a prebuilt argument list

The module shall support formatted text output through functionality equivalent to `wordwrap_vprintf`.

Expected outcome:
- A format string and argument list are rendered and emitted through the wrapping-aware writer.
- Wrapping behavior applies to the rendered output, not to raw format tokens.

Traceability:
- Function: `wordwrap_vprintf` (`src/parseopt/wordwrap.c:680-731`)

#### FR-5: Formatted output from a variadic call

The module shall support a convenience formatted output operation equivalent to `wordwrap_printf`.

Expected outcome:
- It accepts the same logical inputs as the C API’s variadic formatted output entry point.
- Its observable output behavior matches that of passing the same rendered text through the wrapping writer.

Traceability:
- Function: `wordwrap_printf` (`src/parseopt/wordwrap.c:737-747`)

#### FR-6: Position-aware layout decisions

The module shall maintain sufficient positional state to drive wrapping and paragraph behavior.

Expected outcome:
- Output behavior depends on tracked position within the current line/document state.
- State transitions are preserved across character, paragraph, and formatted output operations.

Traceability:
- Types: `struct position` occurrences in `src/parseopt/wordwrap.c`
- Type: `struct wordwrap_file` fields containing position state (`src/parseopt/wordwrap.c:72-95`)

#### FR-7: Shared behavior across all public output paths

The module shall ensure that the public output operations act on the same writer state and produce coherent combined output.

Expected outcome:
- Mixing character output, paragraph emission, and formatted output on one writer yields one continuous wrapped document stream.

Traceability:
- Functions: `wordwrap_putc`, `wordwrap_para`, `wordwrap_vprintf`, `wordwrap_printf`
- Type: `struct wordwrap_file`

### Key Entities

#### `wordwrap_file`

Primary state holder for the module’s output behavior.

Role:
- Represents an active wrapping-aware output stream.
- Stores the state needed to track current output position and layout progress.
- Serves as the shared object used by all public output operations.

Traceability:
- Type: `struct wordwrap_file` (`src/parseopt/wordwrap.c:72-95`)

#### `position`

Internal positional value used to describe output location relevant to wrapping/layout behavior.

Role:
- Captures position-like state used by the writer.
- Supports decisions about line progression and paragraph transitions.

Traceability:
- Type: `struct position` (multiple occurrences in `src/parseopt/wordwrap.c`)

#### `winsize`

Auxiliary size-related structure present in the module and related to output width handling.

Role:
- Represents window or width dimensions used by the module where applicable to layout decisions.

Traceability:
- Type: `struct winsize` (`src/parseopt/wordwrap.c:118`)

#### Entity relationships

- A `wordwrap_file` contains and updates position-related state.
- Public output functions operate on a `wordwrap_file`.
- Position state influences how text is laid out as characters and formatted strings are emitted.
- Size-related information may constrain layout behavior where the source module does so.

## Success Criteria

### SC-1: Public behavior coverage

The Rust module exposes behaviorally equivalent support for:
- single-character wrapped output,
- paragraph emission,
- formatted wrapped output from prebuilt arguments,
- formatted wrapped output from convenience-call input.

Measured by:
- Existence of Rust-side functionality covering each of the four C entry points.
- Output comparison tests against the C module for representative inputs.

Traceability:
- `wordwrap_putc`
- `wordwrap_para`
- `wordwrap_vprintf`
- `wordwrap_printf`

### SC-2: Output equivalence for mixed-operation documents

For test cases that combine character output, paragraph breaks, and formatted output on one writer, the Rust port produces the same final text layout as the C module for the same sequence of operations.

Measured by:
- Golden-output comparison for at least one mixed-operation document.

Traceability:
- `wordwrap_putc`
- `wordwrap_para`
- `wordwrap_vprintf`
- `wordwrap_printf`
- `struct wordwrap_file`

### SC-3: Position-sensitive wrapping preservation

For inputs that exceed a line’s available width under the source module’s conditions, the Rust port preserves the same wrap placement behavior as the C implementation.

Measured by:
- Side-by-side output tests using long formatted and incremental text sequences.
- Verification that line breaks occur at the same points as the C module output.

Traceability:
- `struct wordwrap_file`
- `struct position`
- `wordwrap_putc`
- `wordwrap_vprintf`

### SC-4: Paragraph-state preservation

After a paragraph operation, subsequent output in the Rust port starts with the same layout state and visible separation as in the C module.

Measured by:
- Before/after paragraph output comparison tests.
- Confirmation that the second paragraph wraps like a fresh paragraph in the C implementation.

Traceability:
- `wordwrap_para`
- `struct wordwrap_file`
- `struct position`

### SC-5: Cross-call state continuity

Successive calls on the same writer in the Rust port preserve cumulative state exactly enough to match the C module’s final output.

Measured by:
- Tests that split the same logical text across multiple calls in different groupings and compare against C behavior for the same grouping.

Traceability:
- `struct wordwrap_file`
- `wordwrap_putc`
- `wordwrap_vprintf`
- `wordwrap_printf`