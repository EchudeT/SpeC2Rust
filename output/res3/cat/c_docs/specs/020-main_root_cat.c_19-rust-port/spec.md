# spec.md

## Title

Functional Specification: `main_root_cat.c_19` Rust Port

## Metadata

- Project: `cat`
- Module: `main_root_cat.c_19`
- Category: `main_cluster`
- Source file: `cat.c`
- Rust branch: `020-main_root_cat.c_19-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides the main content-copying behavior for `cat`, including:

- printing usage/help text with status-sensitive termination,
- copying input to output in a simple fast path,
- copying input to output in a formatted path that applies line numbering and display transformations,
- flushing pending output content,
- selecting and executing a final copy path for standard operation.

The Rust rewrite must preserve the observable behavior of these responsibilities as evidenced by the source module functions `usage`, `next_line_num`, `simple_cat`, `write_pending`, `cat`, and `copy_cat`.

## Feature Specification

### 1. Usage and termination behavior

The module must provide behavior equivalent to `usage(int status)`:

- When invoked for successful help/usage display, it must emit usage information appropriate for the program and terminate with the provided success status.
- When invoked for an error path, it must emit the usage guidance appropriate for failure and terminate with the provided non-success status.

This requirement is limited to usage-output and exit-status behavior evidenced by the module.

### 2. Simple byte-stream copy

The module must support a simple copy mode equivalent to `simple_cat`:

- Read input data into a supplied buffer.
- Write the data to standard output unchanged.
- Continue until end of input.
- Report success or failure based on I/O outcome.

This mode is the unformatted path and must preserve byte content exactly for transferred data.

### 3. Pending output flush

The module must support explicit flushing of pending generated output equivalent to `write_pending`:

- Given an output buffer and a current write position, write the buffered pending bytes to output.
- After the write, the pending region is considered consumed.

The Rust version must preserve the functional effect that buffered transformed output is emitted before further processing depends on that state.

### 4. Formatted copy with display options

The module must support a formatted copy mode equivalent to `cat(...)` that transforms input presentation according to supplied option flags.

The Rust version must implement the following behaviors, individually and in combination as supported by the source module parameters:

- **Show nonprinting characters**
  Render nonprinting bytes visibly instead of passing them through unchanged.

- **Show tabs**
  Render tab characters visibly instead of outputting raw tab bytes.

- **Number lines**
  Prefix lines with a line number.

- **Number nonblank lines only**
  Prefix only nonblank lines with a line number.

- **Show line ends**
  Render end-of-line visibly.

- **Squeeze blank lines**
  Collapse repeated adjacent blank lines in output presentation.

The formatted copy mode must read input, apply the selected presentation rules, buffer generated output as needed, and emit the resulting output stream. It must report success or failure based on I/O outcome.

### 5. Line number progression

The module must maintain line-number progression equivalent to `next_line_num` for numbering behavior:

- Line numbers must advance in sequence as lines are numbered.
- The numbering state must support repeated use while processing a stream.

The Rust version must preserve observable numbering progression in output.

### 6. Final copy-path execution

The module must provide top-level copy execution behavior equivalent to `copy_cat`:

- Execute the module’s selected copy operation for standard use.
- Return an integer status indicating success or failure.

The Rust port must preserve the role of this function as the module’s copy-operation result producer.

## User Scenarios & Testing

### Scenario 1: Show usage information

A user requests help or otherwise triggers usage output.

Expected behavior:

- Usage text is printed.
- The process terminates with the requested status code.
- Success and failure invocations remain distinguishable by exit status.

### Scenario 2: Copy input unchanged

A user runs `cat` without formatting options on input that does not require transformation.

Expected behavior:

- Input bytes are copied to standard output unchanged.
- All content is emitted in order.
- The operation reports success when no read/write error occurs.

Recommended tests:

- Copy a short text input and compare output byte-for-byte.
- Copy binary-like content containing tabs, control bytes, and high-bit bytes in simple mode and verify exact preservation.
- Inject read or write failure and verify failure result.

### Scenario 3: Number all lines

A user requests line numbering for all lines.

Expected behavior:

- Every line in the output is prefixed with a line number.
- Line numbers advance monotonically in sequence.
- Blank lines are also numbered in this mode.

Recommended tests:

- Input with multiple nonblank lines.
- Input with interspersed blank lines.
- Verify numbering starts consistently and increments once per numbered line.

### Scenario 4: Number only nonblank lines

A user requests numbering of nonblank lines only.

Expected behavior:

- Nonblank lines receive line numbers.
- Blank lines are emitted without numbers.
- Number progression skips blank lines rather than assigning them a number.

Recommended tests:

- Input with runs of blank and nonblank lines.
- Verify only nonblank lines have prefixes.

### Scenario 5: Show tabs and line ends

A user requests visible display of tabs and line endings.

Expected behavior:

- Tabs are rendered visibly.
- End-of-line is rendered visibly.
- Other content remains in the correct order around these markers.

Recommended tests:

- Input containing tab-separated text and newline-terminated lines.
- Verify visible tab and end markers appear at the correct positions.

### Scenario 6: Show nonprinting characters

A user requests visible rendering of nonprinting bytes.

Expected behavior:

- Nonprinting bytes are represented visibly in output instead of being emitted raw.
- Printable characters remain readable in sequence.

Recommended tests:

- Input containing control characters.
- Input containing printable text mixed with nonprinting bytes.
- Verify transformed output distinguishes nonprinting bytes.

### Scenario 7: Squeeze repeated blank lines

A user requests collapsing of repeated blank lines.

Expected behavior:

- Multiple adjacent blank lines are reduced in presentation.
- Nonblank lines remain in order.
- A single blank separator is preserved where applicable.

Recommended tests:

- Input with long blank-line runs between text lines.
- Input beginning or ending with repeated blank lines.
- Verify repeated blank runs are collapsed.

### Scenario 8: Combined formatting options

A user requests a combination such as numbering, visible line ends, visible tabs, and squeeze-blank behavior.

Expected behavior:

- The output reflects all requested transformations together.
- Numbering behavior remains consistent with the chosen numbering mode.
- No option suppresses another except where implied by source behavior, such as `number_nonblank` changing which lines are numbered.

Recommended tests:

- Mixed-content input including blank lines, tabs, and nonprinting bytes.
- Compare output against expected combined formatting.

## Requirements

### Functional Requirements

- **FR-1**: The module shall emit usage/help text and terminate with the caller-supplied status, matching the role of `usage` in `cat.c`.
- **FR-2**: The module shall support an unformatted copy path that transfers input to standard output byte-for-byte until EOF, matching `simple_cat`.
- **FR-3**: The module shall detect and report failure when input reading or output writing fails during copy operations, matching the boolean success/failure role of `simple_cat` and `cat`, and the integer result role of `copy_cat`.
- **FR-4**: The module shall support formatted copy processing controlled by flags for nonprinting display, tab display, line numbering, nonblank-only numbering, line-end display, and blank-line squeezing, matching the parameters and purpose of `cat`.
- **FR-5**: The module shall increment line numbering state as numbered lines are emitted, matching `next_line_num`.
- **FR-6**: The module shall flush pending generated output before continuing when buffered transformed output needs emission, matching `write_pending`.
- **FR-7**: The module shall preserve input order in output, subject only to the explicit display and line-selection transformations requested by the formatting flags in `cat`.
- **FR-8**: The module shall provide a top-level copy routine that executes the selected copy behavior and returns a process-style success/failure status, matching `copy_cat`.

### Key Entities

- **Input stream content**
  The byte sequence read from the current input source and processed either by the simple copy path or the formatted copy path.

- **Output buffer state**
  The pending generated output region managed by formatted copying and explicit flush behavior. This is the state consumed by the functionality corresponding to `write_pending`.

- **Line numbering state**
  The mutable numbering state advanced by functionality corresponding to `next_line_num` and consulted by formatted copying when numbering is enabled.

- **Formatting option set**
  The boolean option combination passed to the formatted copy routine:
  - show nonprinting,
  - show tabs,
  - number lines,
  - number nonblank lines,
  - show line ends,
  - squeeze blank lines.

- **File/status metadata**
  Source-level evidence includes use of `struct stat` and `struct option` in `cat.c`. In this module specification, they are relevant only insofar as they support module decisions and option-driven behavior already evidenced above; no additional externally visible capability is specified from these types alone.

## Success Criteria

- **SC-1**: A help/usage invocation produces usage output and exits with the requested status code.
- **SC-2**: In unformatted mode, test inputs are reproduced byte-for-byte on standard output.
- **SC-3**: In formatted mode, each enabled display option changes output as expected for representative inputs containing tabs, line endings, blank lines, and nonprinting bytes.
- **SC-4**: When line numbering is enabled, numbered lines appear with monotonically advancing numbers; when nonblank-only numbering is enabled, blank lines remain unnumbered.
- **SC-5**: When blank-line squeezing is enabled, repeated adjacent blank lines are collapsed in output for representative blank-line runs.
- **SC-6**: Combined-option tests produce output consistent with the simultaneous application of the selected transformations supported by `cat`.
- **SC-7**: Simulated or real read/write failures cause the copy routine to report failure rather than success.
- **SC-8**: The top-level copy execution path returns a status value that distinguishes success from failure.

## Traceability

| Spec item | Source evidence |
|---|---|
| FR-1, SC-1 | `usage` in `cat.c:83-127` |
| FR-2, FR-3, SC-2, SC-7 | `simple_cat` in `cat.c:155-181` |
| FR-4, FR-6, FR-7, SC-3, SC-4, SC-5, SC-6, SC-7 | `cat` in `cat.c:211-497` |
| FR-5, SC-4 | `next_line_num` in `cat.c:131-149` |
| FR-6 | `write_pending` in `cat.c:187-197` |
| FR-3, FR-8, SC-8 | `copy_cat` in `cat.c:503-532` |
| Key entity: file/status metadata | `struct stat` at `cat.c:541`; `struct option` at `cat.c:552` |