# spec.md

## Title

Functional Specification for `main_root_cat.c_19` Rust Port

## Metadata

- Project: `cat`
- Module: `main_root_cat.c_19`
- Category: `main_cluster`
- Source file: `cat.c`
- Rust branch: `020-main_root_cat.c_19-rust-port`
- Generation date: 2026-06-07

## Overview

This module provides the core data-stream output behavior for the `cat` program’s main execution path. It covers:

- printing usage/help text and exiting with a requested status,
- copying input to output in a fast path when no content transformation is requested,
- copying input to output in a formatted path when line numbering or character-display options are active,
- maintaining line-number state across processed input,
- flushing buffered output that has been prepared but not yet written.

The Rust rewrite must preserve the observable behavior of these responsibilities as evidenced by the source module functions `usage`, `next_line_num`, `simple_cat`, `write_pending`, `cat`, and `copy_cat`.

## Feature Specification

### Feature 1: Usage output and termination

The module emits program usage information and terminates with a caller-selected status code.

#### Behavior
- When invoked for help or invalid invocation handling, the module must produce usage text appropriate for the `cat` program.
- The module must terminate processing after usage handling with the provided exit status.
- The Rust version must preserve the distinction between successful and unsuccessful usage exits when the caller requests different status values.

#### Traceability
- `usage` in `cat.c:83-127`

### Feature 2: Fast-path byte copying

The module supports unformatted transfer of input bytes to output when no display-transforming options are active.

#### Behavior
- Input is read in blocks into a provided buffer and written to standard output without content transformation.
- The operation reports success or failure as a boolean result.
- The Rust version must preserve raw byte copying semantics for this path, including support for arbitrary byte content rather than only text lines.

#### Traceability
- `simple_cat` in `cat.c:155-181`

### Feature 3: Buffered output flush of prepared data

The module can flush a partially filled output buffer that already contains pending bytes.

#### Behavior
- When output bytes have been accumulated into an output buffer, the module writes the pending region and advances or resets buffered state so processing can continue correctly.
- The Rust version must preserve the guarantee that pending prepared output is emitted before the buffer is reused or before processing completes.

#### Traceability
- `write_pending` in `cat.c:187-197`

### Feature 4: Formatted copy with content-display options

The module supports a transformation path that copies input while applying display-oriented formatting options.

#### Supported behaviors
- Show nonprinting characters in a visible representation when enabled.
- Show tab characters in a visible representation when enabled.
- Prefix lines with line numbers when line numbering is enabled.
- Prefix only nonblank lines with line numbers when nonblank-only numbering is enabled.
- Mark line ends visibly when enabled.
- Squeeze multiple adjacent blank lines when enabled.

#### Behavioral rules
- The module must read from input buffers and write transformed output into an output buffer, preserving input order.
- The module must combine options correctly when multiple display options are enabled together.
- The module must preserve line-oriented behavior for numbering and blank-line handling.
- The module must return success or failure for the copy operation.
- The Rust version must preserve the option-controlled formatting behaviors evidenced by the `cat` function signature and processing role.

#### Traceability
- `cat` in `cat.c:211-497`

### Feature 5: Line-number progression

The module maintains and advances internal line-number state used by formatted output.

#### Behavior
- A next-line-number operation updates the current printable line number for subsequent numbered output lines.
- Number progression must remain consistent across continued processing of the same execution flow.
- The Rust version must preserve monotonic line-number advancement during numbered output.

#### Traceability
- `next_line_num` in `cat.c:131-149`
- Used by `cat` in `cat.c:211-497`

### Feature 6: Main copy-path selection

The module provides a higher-level copy routine that performs the effective copying mode for the current execution path.

#### Behavior
- The module selects or invokes the relevant copy behavior for current conditions and reports completion status as an integer result.
- The Rust version must preserve the module’s role as the main copy executor for this part of the program.

#### Traceability
- `copy_cat` in `cat.c:503-532`

## User Scenarios & Testing

### Scenario 1: User requests usage information
A user invokes the program in a way that requires usage/help output. The module prints usage text and exits with the requested status.

#### Test expectations
- Usage text is emitted.
- Exit status matches the requested status path.
- No copy processing continues after usage handling.

#### Traceability
- `usage`

### Scenario 2: User concatenates input without formatting options
A user runs `cat` in plain mode so input should be copied byte-for-byte to output.

#### Test expectations
- Output bytes exactly match input bytes.
- Binary and non-text byte values are preserved.
- The operation reports success on successful reads/writes.

#### Traceability
- `simple_cat`

### Scenario 3: User enables visible nonprinting display
A user requests visible rendering of nonprinting bytes.

#### Test expectations
- Nonprinting bytes are not emitted as raw bytes where the option requires visible representation.
- Input ordering is preserved.
- Other bytes continue to appear in correct sequence.

#### Traceability
- `cat` with `show_nonprinting=true`

### Scenario 4: User enables visible tab display
A user requests that tab characters be shown visibly.

#### Test expectations
- Tab input is represented visibly rather than as a literal tab when the option is enabled.
- Non-tab characters retain correct relative order.

#### Traceability
- `cat` with `show_tabs=true`

### Scenario 5: User enables line numbering for all lines
A user requests line numbering across the copied content.

#### Test expectations
- Each output line receives a line number prefix.
- Line numbers advance in order from one numbered line to the next.
- Numbering remains consistent across buffer boundaries.

#### Traceability
- `cat` with `number=true`
- `next_line_num`

### Scenario 6: User enables numbering of only nonblank lines
A user requests numbering only for nonblank lines.

#### Test expectations
- Nonblank lines receive number prefixes.
- Blank lines do not receive number prefixes.
- Number progression advances only when a nonblank line is numbered.

#### Traceability
- `cat` with `number_nonblank=true`
- `next_line_num`

### Scenario 7: User requests visible line ends
A user requests explicit end-of-line markers.

#### Test expectations
- Line endings are marked visibly in output when enabled.
- Line content remains in order before the end marker.

#### Traceability
- `cat` with `show_ends=true`

### Scenario 8: User requests blank-line squeezing
A user requests reduction of repeated blank lines.

#### Test expectations
- Adjacent blank lines beyond the permitted first blank line are suppressed.
- Nonblank lines remain present and ordered.
- Numbering and end markers, if also enabled, continue to behave consistently with squeezed output.

#### Traceability
- `cat` with `squeeze_blank=true`

### Scenario 9: User combines multiple formatting options
A user enables several display options at once, such as numbering, visible ends, and visible tabs.

#### Test expectations
- All enabled transformations appear together in the output.
- Line numbering remains aligned with the transformed line structure.
- Pending buffered data is fully emitted.

#### Traceability
- `cat`
- `write_pending`
- `next_line_num`

### Scenario 10: Program completes main copy processing
The main execution path invokes the module’s top-level copy routine.

#### Test expectations
- The routine returns an integer completion result.
- The selected copy path completes without dropping pending output.
- Failure conditions are surfaced through the routine’s result.

#### Traceability
- `copy_cat`

## Requirements

### Functional Requirements

#### FR-1: Usage handling
The module shall provide a usage/help operation that emits `cat` usage text and terminates with a caller-specified status.

- Traceability: `usage`

#### FR-2: Unformatted copy path
The module shall provide a copy path that transfers input to output without content transformation and reports success/failure.

- Traceability: `simple_cat`

#### FR-3: Prepared-output flushing
The module shall flush pending bytes from an output buffer before buffer reuse or completion of formatted processing.

- Traceability: `write_pending`, `cat`

#### FR-4: Nonprinting-character display option
The module shall support a mode in which nonprinting input characters are rendered in a visible output form.

- Traceability: `cat`

#### FR-5: Tab-display option
The module shall support a mode in which tab characters are rendered in a visible output form.

- Traceability: `cat`

#### FR-6: Line-numbering option
The module shall support numbering output lines and advancing the current line number as numbered lines are emitted.

- Traceability: `cat`, `next_line_num`

#### FR-7: Nonblank-only numbering option
The module shall support numbering only nonblank lines, without assigning numbers to blank lines.

- Traceability: `cat`, `next_line_num`

#### FR-8: End-of-line marking option
The module shall support visible marking of line ends in output.

- Traceability: `cat`

#### FR-9: Blank-line squeezing option
The module shall support suppression of repeated adjacent blank lines when the squeeze option is enabled.

- Traceability: `cat`

#### FR-10: Combined-option processing
The module shall apply supported display options in combination within a single formatted copy pass.

- Traceability: `cat`

#### FR-11: Line-number state progression
The module shall maintain line-number state across processing so that numbering continues correctly over successive lines.

- Traceability: `next_line_num`, `cat`

#### FR-12: Main copy execution
The module shall provide a top-level copy routine that executes the effective copy behavior for this main execution path and returns an integer result.

- Traceability: `copy_cat`

### Key Entities

#### Entity 1: Input buffer
A contiguous memory region used to hold bytes read from input for either raw copying or formatted copying.

#### Entity 2: Output buffer
A contiguous memory region used to accumulate bytes that will be written to output, including transformed display output and pending buffered data.

#### Entity 3: Line-number state
Internal state representing the current printable line number used when numbering output lines. This state is advanced by the line-number progression operation and consumed by formatted output processing.

#### Entity 4: Copy-option set
The set of boolean formatting controls that determine formatted behavior:
- show nonprinting characters,
- show tabs,
- number lines,
- number nonblank lines,
- show line ends,
- squeeze blank lines.

#### Entity 5: File status information
A `struct stat` instance used in the module context as source-side file metadata relevant to copy behavior selection or validation.

- Traceability: anonymous `struct stat` reference at `cat.c:541`

#### Entity 6: Command-line option descriptors
A `struct option` array or entries used in the module context to define accepted options for invocation behavior.

- Traceability: anonymous `struct option` reference at `cat.c:552`

#### Relationships
- The top-level copy routine uses input and output buffers to perform copying.
- The formatted copy path interprets the copy-option set to decide how bytes and lines are rendered.
- The formatted copy path uses line-number state when either numbering mode is enabled.
- The output buffer may hold pending bytes that must be flushed before reuse or completion.

## Success Criteria

1. The Rust module provides a usage operation whose observable result includes usage text emission and termination with the requested status.
   - Traceability: `usage`

2. In plain-copy mode, the Rust module reproduces input bytes exactly at output for representative text and binary inputs.
   - Traceability: `simple_cat`

3. In formatted mode, the Rust module supports each evidenced option flag: visible nonprinting, visible tabs, line numbering, nonblank-only numbering, visible line ends, and blank-line squeezing.
   - Traceability: `cat`

4. When line numbering is enabled, output line numbers advance in order and remain correct across multi-buffer processing.
   - Traceability: `next_line_num`, `cat`

5. When nonblank-only numbering is enabled, blank lines are not numbered and nonblank lines are numbered in increasing order.
   - Traceability: `cat`, `next_line_num`

6. When blank-line squeezing is enabled, repeated adjacent blank lines are suppressed while preserving the first blank line in a run.

7. When multiple formatting options are enabled together, the Rust module produces output reflecting all enabled options in a single pass without dropping content.
   - Traceability: `cat`, `write_pending`

8. Pending buffered output is emitted before formatted processing completion, so no prepared output bytes are lost.
   - Traceability: `write_pending`, `cat`

9. The top-level copy routine returns an integer completion result and surfaces success/failure of the executed copy path.
   - Traceability: `copy_cat`

10. The Rust implementation remains within the evidenced functional scope of this module and does not require unsupported new capabilities beyond the source behavior described above.
   - Traceability: `usage`, `simple_cat`, `write_pending`, `cat`, `next_line_num`, `copy_cat`