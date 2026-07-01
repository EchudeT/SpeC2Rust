# spec.md

## Title

Rust Functional Specification for `module_gnu_error.c_26`

## Summary

This module provides formatted error-reporting functions for command-line style programs. It emits diagnostic messages to the standard error stream, optionally appends a textual description of an error number, can include source location context, flushes standard output before writing diagnostics, and may terminate the process when requested by the caller.

The Rust rewrite must preserve the observable behavior defined by the C module in `gnu/error.c` for the analyzed functionality only.

## Scope

Included in scope:

- Reporting a formatted diagnostic message to standard error.
- Optionally appending the message associated with a supplied error number.
- Reporting diagnostics with optional file and line context.
- Flushing standard output before emitting diagnostics.
- Avoiding duplicate consecutive location-based diagnostics for the same file and line when the module’s existing behavior requires it.
- Exiting the process when a nonzero status is supplied.

Out of scope:

- Any new public API beyond the behavior evidenced by `error` and `error_at_line`.
- Any persistence, structured logging, localization expansion beyond existing message behavior, thread-safety guarantees, or recovery features not evidenced by this module.

## Source Evidence

Primary source file:

- `gnu/error.c`

Primary functions covered by this specification:

- `is_open`
- `flush_stdout`
- `print_errno_message`
- `error`
- `error_at_line`

## Feature Specification

### Feature: Process-style diagnostic reporting

The module supplies a diagnostic reporting facility intended for use by other parts of the program when an operation fails or a warning must be shown.

The Rust version must implement the following externally observable behavior:

1. A caller can report a formatted message with printf-style arguments.
2. The module writes that message as a diagnostic to the standard error stream.
3. Before writing the diagnostic, the module flushes standard output when appropriate so that prior normal output is not left buffered ahead of the error text.
4. If the caller supplies a nonzero error number, the module appends the textual system error description corresponding to that number.
5. If the caller supplies file and line information, the emitted diagnostic includes that location context.
6. If the caller requests termination by passing a nonzero status, the module terminates the process with that status after emitting the diagnostic.
7. For location-based reporting, repeated reporting for the same file and line must follow the module’s existing suppression behavior rather than emitting the same location-based diagnostic again.

### Feature: Standard output flush coordination

The module contains behavior that checks whether standard output is open and flushes it before diagnostics are emitted. The Rust rewrite must preserve the observable purpose of this behavior:

- When diagnostics are about to be written, pending standard output should be flushed if flushing is applicable.
- This behavior must not require callers to perform a separate flush step.

### Feature: Error text rendering

When an error number is provided, the module renders a human-readable form of that error and appends it to the diagnostic message. The Rust rewrite must preserve this user-visible outcome:

- No error-number text is appended when the supplied error number is zero.
- A human-readable error description is appended when the supplied error number is nonzero.

## User Scenarios & Testing

### Scenario 1: Report a warning message without terminating

A caller detects a non-fatal issue and invokes the module with:

- status = 0
- errnum = 0
- a formatted message

Expected result:

- The message is emitted to standard error.
- No system error text is appended.
- The process continues running.

Test coverage:

- Verify standard error contains the formatted message.
- Verify the process does not exit.
- Verify no extra error-description suffix is present.

### Scenario 2: Report a system-call failure with errno text

A caller detects a failure associated with a system error number and invokes the module with:

- status = 0
- errnum = a nonzero error code
- a formatted message

Expected result:

- The message is emitted to standard error.
- A separator and the textual description for the error number are appended.
- The process continues running.

Test coverage:

- Verify output contains the original formatted message.
- Verify output also contains the expected error description for the chosen error number.
- Verify no termination occurs.

### Scenario 3: Report an error and terminate

A caller encounters a fatal condition and invokes the module with:

- status = nonzero
- errnum = 0 or nonzero
- a formatted message

Expected result:

- The diagnostic is emitted before termination.
- The process exits with the supplied status code.

Test coverage:

- Verify the child process exits with the requested code.
- Verify the diagnostic was written before exit.

### Scenario 4: Report a diagnostic with file and line context

A caller wants to identify the source location of a problem and invokes the location-based entry point with:

- file name
- line number
- status = 0
- errnum = 0 or nonzero
- a formatted message

Expected result:

- The diagnostic includes the file name and line number context.
- The message text is included.
- If errnum is nonzero, the corresponding error description is appended.

Test coverage:

- Verify output contains file name.
- Verify output contains line number.
- Verify output contains message text.
- When errnum is nonzero, verify output contains the error description.

### Scenario 5: Suppress repeated identical location reports

A caller invokes the location-based reporting function multiple times for the same file and line under the module’s duplicate-suppression conditions.

Expected result:

- The first diagnostic is emitted.
- A later diagnostic for the same file and line is suppressed if the C module would suppress it.

Test coverage:

- Invoke with identical file and line inputs in sequence.
- Verify emitted output count matches the C module’s observed behavior.

### Scenario 6: Flush pending standard output before diagnostic emission

A program writes normal output to standard output without fully flushing it, then invokes the diagnostic module.

Expected result:

- Standard output is flushed before the diagnostic is emitted.
- User-visible ordering does not leave earlier standard output buffered behind the error text.

Test coverage:

- Arrange buffered standard output content.
- Invoke diagnostic reporting.
- Verify the observable ordering matches the C module behavior.

## Requirements

### Functional Requirements

#### FR-1: Formatted diagnostic emission

The module shall accept a caller-supplied format string and arguments and emit the resulting message as a diagnostic to the standard error stream.

Traceability:

- `error`
- `error_at_line`

#### FR-2: Optional error-number description

The module shall append a human-readable description of the supplied error number when `errnum` is nonzero.

Traceability:

- `print_errno_message`
- `error`
- `error_at_line`

#### FR-3: No error-number description when absent

The module shall not append a system error description when `errnum` is zero.

Traceability:

- `print_errno_message`
- `error`
- `error_at_line`

#### FR-4: Standard output flush before diagnostics

The module shall flush standard output before emitting a diagnostic when flushing is applicable to the current standard output stream state.

Traceability:

- `is_open`
- `flush_stdout`
- `error`
- `error_at_line`

#### FR-5: File and line contextual diagnostics

The module shall support a diagnostic form that includes a file name and line number in the emitted output.

Traceability:

- `error_at_line`

#### FR-6: Process termination on nonzero status

The module shall terminate the process after emitting the diagnostic when the supplied status is nonzero, using that status as the exit status.

Traceability:

- `error`
- `error_at_line`

#### FR-7: Non-terminating behavior on zero status

The module shall return control to the caller after emitting the diagnostic when the supplied status is zero.

Traceability:

- `error`
- `error_at_line`

#### FR-8: Duplicate location suppression behavior

The module shall preserve the existing duplicate-suppression behavior for repeated location-based diagnostics referring to the same file name and line number.

Traceability:

- `error_at_line`

### Key Entities

This module does not define complex public data structures in the analyzed input. Its key entities are behavioral inputs and output targets:

#### Entity: Diagnostic message request

A request consisting of:

- termination status
- optional error number
- format string with arguments
- optionally, file name and line number

Relationship:

- Consumed by `error` or `error_at_line` to produce diagnostic output and possible process termination.

#### Entity: Standard output stream state

The current state of standard output relevant to whether flushing should be attempted before emitting diagnostics.

Relationship:

- Consulted by `is_open` and `flush_stdout`.
- Affects ordering of normal output relative to diagnostics.

#### Entity: Standard error diagnostic output

The textual diagnostic emitted by the module.

Relationship:

- Produced by `error` and `error_at_line`.
- May include the formatted message, location context, and error-number description.

#### Entity: Previous location state for duplicate suppression

The remembered file-and-line identity relevant to suppressing repeated location-based diagnostics.

Relationship:

- Consulted and updated by `error_at_line` to match the module’s existing repeated-location behavior.

## Success Criteria

### SC-1: Correct message emission

For calls equivalent to the C module’s `error` with `status = 0` and `errnum = 0`, the Rust version emits the formatted diagnostic message to standard error and returns to the caller.

Traceability:

- `error`

### SC-2: Correct error text inclusion

For calls equivalent to the C module’s `error` or `error_at_line` with nonzero `errnum`, the Rust version includes a human-readable description of that error number in the emitted diagnostic.

Traceability:

- `print_errno_message`
- `error`
- `error_at_line`

### SC-3: Correct omission of error text

For calls equivalent to the C module’s `error` or `error_at_line` with `errnum = 0`, the Rust version does not add an error-number description.

Traceability:

- `print_errno_message`
- `error`
- `error_at_line`

### SC-4: Correct location rendering

For calls equivalent to the C module’s `error_at_line`, the Rust version includes file name and line number context in the emitted diagnostic.

Traceability:

- `error_at_line`

### SC-5: Correct termination behavior

For calls equivalent to the C module’s `error` or `error_at_line` with nonzero `status`, the Rust version exits with the same status after writing the diagnostic.

Traceability:

- `error`
- `error_at_line`

### SC-6: Correct non-terminating behavior

For calls equivalent to the C module’s `error` or `error_at_line` with `status = 0`, the Rust version does not terminate the process.

Traceability:

- `error`
- `error_at_line`

### SC-7: Output ordering preserved by flush behavior

In tests where standard output contains buffered content before a diagnostic call, the Rust version preserves the C module’s observable ordering by flushing standard output before diagnostic emission when applicable.

Traceability:

- `is_open`
- `flush_stdout`
- `error`
- `error_at_line`

### SC-8: Duplicate location handling matches C behavior

In repeated `error_at_line` calls using the same file and line, the Rust version emits or suppresses subsequent diagnostics exactly as the C module does for the same sequence.

Traceability:

- `error_at_line`