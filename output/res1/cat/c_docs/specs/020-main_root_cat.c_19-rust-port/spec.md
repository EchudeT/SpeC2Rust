# spec.md

## Title

Functional Specification — `main_root_cat.c_19` Rust Port

## Metadata

- Project: `cat`
- Module: `main_root_cat.c_19`
- Category: `main_cluster`
- Source files analyzed: `cat.c`
- Rust branch: `020-main_root_cat.c_19-rust-port`
- Generation date: `2026-06-06`

## Overview

This module provides the main text-stream copying behavior of `cat`, including:

- printing usage/help text and terminating with the requested status,
- copying input directly when no content transformation is requested,
- copying input while applying line-oriented and character-oriented display options,
- maintaining line numbering state across output,
- flushing buffered transformed output,
- running the default copy path used by the program entry flow.

The Rust rewrite must preserve the observable behavior evidenced by this module: it must support both a fast plain-copy mode and a formatted copy mode with the same option-controlled transformations.

## Scope

This specification covers only the functionality evidenced by the analyzed module:

- help/usage output behavior,
- plain input-to-output copying,
- transformed copying with display and numbering options,
- line number state progression,
- buffered pending-output flush behavior,
- the top-level copy path used by the module.

This specification does not define unrelated command dispatch, option parsing policy beyond what is reflected in behavior flags passed to this module, or features not evidenced by the analyzed functions.

## Feature Specification

### F1. Usage output

The module must provide behavior equivalent to a usage/help routine that:

- emits program usage/help text,
- terminates according to a provided status value,
- supports both successful help display and failure/diagnostic usage exit paths.

This behavior is evidenced by `usage`.

### F2. Sequential line number generation

The module must maintain line numbering state for formatted output.

Required behavior:

- line numbers advance sequentially,
- numbering can be applied to all lines or only nonblank lines depending on active mode,
- numbering state persists correctly as processing continues through the input stream.

This behavior is evidenced by `next_line_num` and by numbering-related control flow in `cat`.

### F3. Plain copy mode

The module must support a direct copy mode for input data when formatting is not required.

Required behavior:

- read input into a caller-provided buffer,
- write the read data to output without text transformation,
- report success or failure of the copy operation.

This behavior is evidenced by `simple_cat`.

### F4. Buffered output flush

The module must support flushing pending transformed output accumulated in an output buffer.

Required behavior:

- output all bytes currently staged as pending,
- update buffered-output state so subsequent writes continue correctly.

This behavior is evidenced by `write_pending`.

### F5. Formatted copy mode

The module must support a formatting copy path controlled by the following behavior flags:

- `show_nonprinting`
- `show_tabs`
- `number`
- `number_nonblank`
- `show_ends`
- `squeeze_blank`

Required behavior:

- copy input to output while applying the active display transformations,
- preserve normal pass-through behavior for characters not affected by selected options,
- perform line-oriented processing needed for numbering, blank-line handling, and line-end display,
- return success or failure for the overall operation.

This behavior is evidenced by `cat`.

#### F5.1 Nonprinting-character display

When enabled, the module must render nonprinting characters in a visible representation instead of emitting them unchanged, except where another active option governs the display form.

Evidenced by `cat` through `show_nonprinting`.

#### F5.2 Tab display

When enabled, tab characters must be displayed visibly rather than passed through as literal tabs.

Evidenced by `cat` through `show_tabs`.

#### F5.3 Line numbering of all lines

When `number` mode is enabled, the module must prefix lines with line numbers.

Evidenced by `cat` and `next_line_num`.

#### F5.4 Line numbering of nonblank lines only

When `number_nonblank` mode is enabled, the module must number nonblank lines and must not number blank lines.

Evidenced by `cat` and `next_line_num`.

#### F5.5 End-of-line display

When enabled, the module must visibly mark line ends in output.

Evidenced by `cat` through `show_ends`.

#### F5.6 Squeezing repeated blank lines

When enabled, the module must suppress repeated adjacent blank output lines so that runs of blank lines are reduced.

Evidenced by `cat` through `squeeze_blank`.

### F6. Top-level copy path

The module must provide a higher-level copy routine used by program flow to execute copying with the currently selected behavior.

Required behavior:

- select the appropriate copy strategy consistent with module behavior,
- propagate success/failure as an integer status suitable for caller use.

This behavior is evidenced by `copy_cat`.

## User Scenarios & Testing

### Scenario 1: User requests help text

A user invokes the program in a way that requires usage/help output.

Expected support:

- help/usage text is printed,
- exit status matches the requested status path.

Traceability: `usage`.

### Scenario 2: Plain concatenation with no formatting options

A user concatenates file or stream content without enabling display transforms.

Expected support:

- bytes are copied from input to output unchanged,
- operation succeeds for valid readable input and writable output,
- failures are reported through the module’s status result.

Traceability: `simple_cat`, `copy_cat`.

### Scenario 3: Display nonprinting bytes visibly

A user enables visible rendering for nonprinting characters.

Expected support:

- affected characters are rendered in a visible form,
- unaffected printable content still appears in order.

Traceability: `cat` with `show_nonprinting`.

### Scenario 4: Display tabs visibly

A user enables visible tab display.

Expected support:

- tab characters are not emitted as literal tabs,
- output shows a visible tab representation.

Traceability: `cat` with `show_tabs`.

### Scenario 5: Show line ends

A user enables line-end marking.

Expected support:

- each displayed line end is visibly marked,
- normal line content remains in correct sequence.

Traceability: `cat` with `show_ends`.

### Scenario 6: Number every line

A user enables numbering for all lines.

Expected support:

- each output line receives a sequential number prefix,
- numbering advances correctly across successive lines.

Traceability: `cat`, `next_line_num`.

### Scenario 7: Number only nonblank lines

A user enables numbering only for nonblank lines.

Expected support:

- nonblank lines receive sequential number prefixes,
- blank lines remain unnumbered.

Traceability: `cat`, `next_line_num`.

### Scenario 8: Squeeze repeated blank lines

A user enables blank-line squeezing on input containing multiple adjacent blank lines.

Expected support:

- repeated adjacent blank lines are suppressed,
- a reduced blank-line run is output instead of the original full run.

Traceability: `cat` with `squeeze_blank`.

### Scenario 9: Combined formatting options

A user enables multiple display options at once, such as numbering, showing line ends, and visible tab/nonprinting display.

Expected support:

- combined output reflects all selected transformations together,
- line numbering and character display rules remain consistent,
- output ordering remains faithful to input ordering.

Traceability: `cat`, `write_pending`, `next_line_num`.

### Scenario 10: Buffered transformed output reaches a flush point

A formatted-copy operation accumulates pending output that must be emitted.

Expected support:

- all pending transformed bytes are written,
- no already-prepared output is lost across flush boundaries.

Traceability: `write_pending`, `cat`.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a usage/help behavior that prints usage information and terminates with a caller-selected status.
  Traceability: `usage`.

- **FR-2**: The module shall support a plain copy mode that transfers input bytes to output without content transformation.
  Traceability: `simple_cat`.

- **FR-3**: The module shall report success or failure for plain copy operations.
  Traceability: `simple_cat`.

- **FR-4**: The module shall support a formatted copy mode that processes input using option-controlled display behavior.
  Traceability: `cat`.

- **FR-5**: The formatted copy mode shall support visible rendering of nonprinting characters when enabled.
  Traceability: `cat` (`show_nonprinting`).

- **FR-6**: The formatted copy mode shall support visible rendering of tab characters when enabled.
  Traceability: `cat` (`show_tabs`).

- **FR-7**: The formatted copy mode shall support numbering of all output lines when enabled.
  Traceability: `cat` (`number`), `next_line_num`.

- **FR-8**: The formatted copy mode shall support numbering of nonblank output lines only when enabled.
  Traceability: `cat` (`number_nonblank`), `next_line_num`.

- **FR-9**: The formatted copy mode shall support visible marking of line ends when enabled.
  Traceability: `cat` (`show_ends`).

- **FR-10**: The formatted copy mode shall support suppression of repeated adjacent blank lines when enabled.
  Traceability: `cat` (`squeeze_blank`).

- **FR-11**: The module shall maintain sequential line-number state across processed lines.
  Traceability: `next_line_num`, `cat`.

- **FR-12**: The module shall support flushing pending transformed output from an output buffer to the destination stream.
  Traceability: `write_pending`.

- **FR-13**: The module shall provide a top-level copy routine that returns an integer status for caller use.
  Traceability: `copy_cat`.

- **FR-14**: The top-level copy routine shall use behavior consistent with the selected copy path and propagate failure through its status result.
  Traceability: `copy_cat`, `simple_cat`, `cat`.

### Key Entities

- **Input buffer**: A caller-supplied byte buffer used to hold source data for copying.
  Relationship: consumed by plain copy and formatted copy paths.
  Traceability: `simple_cat`, `cat`.

- **Output buffer**: A caller-supplied byte buffer used to accumulate transformed output before writing.
  Relationship: populated during formatted copy and flushed by pending-write behavior.
  Traceability: `write_pending`, `cat`.

- **Line number state**: Internal mutable state representing the current output line number.
  Relationship: advanced by sequential numbering logic and used by formatted copy when numbering is active.
  Traceability: `next_line_num`, `cat`.

- **Formatting option set**: The boolean behavior controls for visible nonprinting display, visible tab display, numbering, numbering nonblank lines, end-of-line marking, and blank-line squeezing.
  Relationship: governs the transformation rules applied by formatted copy.
  Traceability: `cat`.

- **File status metadata (`struct stat`)**: File metadata referenced by the module.
  Relationship: supports top-level copy behavior where file properties may influence execution flow.
  Traceability: `cat.c` anonymous `struct stat`, `copy_cat`.

- **Long-option descriptor (`struct option`)**: Option description metadata referenced by the module.
  Relationship: associated with usage/help and command option presentation context.
  Traceability: `cat.c` anonymous `struct option`, `usage`.

## Success Criteria

- **SC-1**: The Rust module prints usage/help output and exits through the same success/failure path distinctions represented by the source module.
  Traceability: `usage`.

- **SC-2**: With no formatting features active, the Rust module copies input to output without altering byte content.
  Traceability: `simple_cat`.

- **SC-3**: The Rust module returns explicit success/failure results for both plain and formatted copy paths.
  Traceability: `simple_cat`, `cat`, `copy_cat`.

- **SC-4**: When nonprinting display is enabled, the Rust module visibly represents nonprinting characters in output.
  Traceability: `cat`.

- **SC-5**: When tab display is enabled, the Rust module visibly represents tab characters in output.
  Traceability: `cat`.

- **SC-6**: When all-line numbering is enabled, the Rust module prefixes each output line with sequential line numbers.
  Traceability: `cat`, `next_line_num`.

- **SC-7**: When nonblank-line numbering is enabled, the Rust module numbers nonblank lines and leaves blank lines unnumbered.
  Traceability: `cat`, `next_line_num`.

- **SC-8**: When line-end display is enabled, the Rust module visibly marks line ends in output.
  Traceability: `cat`.

- **SC-9**: When blank-line squeezing is enabled, the Rust module suppresses repeated adjacent blank lines.
  Traceability: `cat`.

- **SC-10**: During formatted output, pending buffered output is fully flushed without losing already-prepared bytes.
  Traceability: `write_pending`, `cat`.

- **SC-11**: The Rust top-level copy routine returns an integer-compatible status outcome suitable for integration with the surrounding program flow.
  Traceability: `copy_cat`.