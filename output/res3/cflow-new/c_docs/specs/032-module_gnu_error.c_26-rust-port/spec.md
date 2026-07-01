# spec.md

## Title

Functional Specification: `module_gnu_error.c_26` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_error.c_26`
- Category: `module_cluster`
- Source file: `gnu/error.c`
- Rust branch: `032-module_gnu_error.c_26-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides formatted error-reporting functions for emitting diagnostic messages to the standard error stream, optionally including system error text derived from an error number, and optionally terminating the process with a caller-specified status.

The Rust rewrite must preserve the observable behavior evidenced by the C module:

- emit user-facing diagnostic messages to standard error,
- support plain message reporting and file/line-qualified message reporting,
- append an error description when an error number is provided,
- flush standard output before writing diagnostics,
- avoid duplicate line-based output for repeated reports of the same source location when the corresponding mode is enabled,
- optionally terminate execution after reporting.

## Scope

### In Scope

The Rust module must implement the behavior represented by these functions in `gnu/error.c`:

- `error`
- `error_at_line`
- internal support behavior for:
  - checking whether a file descriptor is open,
  - flushing standard output before diagnostics,
  - printing an error-text suffix for a supplied error number.

### Out of Scope

The Rust port specification does not require any capabilities not evidenced by the module analysis, including:

- new public APIs beyond the module’s reporting behavior,
- structured logging,
- thread-safety guarantees,
- persistence, serialization, or recovery features,
- FFI design requirements,
- performance or benchmark targets beyond functional equivalence.

## Feature Specification

### Summary

The module is a diagnostic reporting facility centered on two public reporting operations:

1. report an error message with optional system error text,
2. report an error message associated with a file name and line number, with optional suppression of repeated reports for the same location.

Both operations must be able to terminate the process when called with a nonzero status.

### Functional Behavior

#### 1. Standard diagnostic emission

The module must format a caller-supplied message with variadic arguments and write the resulting diagnostic to the standard error stream.

The Rust version must preserve these externally visible behaviors:

- the message is produced from a format string and arguments,
- output is intended for end users as a diagnostic,
- reporting occurs even when no system error number is supplied.

Traceability: `error`, `error_at_line`.

#### 2. Optional system error text

When the caller supplies a nonzero error number, the module must append a textual description corresponding to that error number to the diagnostic output.

This behavior is part of both public reporting functions.

Traceability: `print_errno_message`, `error`, `error_at_line`.

#### 3. File and line qualified reporting

The module must support diagnostics that identify a source location using:

- a file name,
- a line number.

The Rust version must emit diagnostics that include this location context before or alongside the formatted message, as required by the module’s location-reporting function.

Traceability: `error_at_line`.

#### 4. Standard output flushing before diagnostics

Before emitting diagnostics, the module must attempt to flush standard output so that previously buffered normal output does not appear after an error message.

This flushing behavior is conditional on standard output being open.

Traceability: `is_open`, `flush_stdout`, `error`, `error_at_line`.

#### 5. Open-descriptor check for stdout flush handling

The module contains support behavior for determining whether a file descriptor is open before trying to flush the corresponding stream.

The Rust version must preserve the observable intent of this behavior: avoid relying on a flush of standard output when the underlying descriptor is not open.

Traceability: `is_open`, `flush_stdout`.

#### 6. Duplicate suppression for repeated line diagnostics

For location-based diagnostics, the module must support suppression of repeated output for the same file name and line number when the module’s duplicate-suppression mode is enabled.

The Rust rewrite must preserve the location-based duplicate suppression behavior evidenced by `error_at_line`; it must not emit a repeated diagnostic for the same `(file_name, line_number)` pair when suppression is active.

Traceability: `error_at_line`.

#### 7. Optional process termination

After emitting a diagnostic, the module must terminate the process when the caller supplies a nonzero status value.

When the status value is zero, the module must return after reporting.

Traceability: `error`, `error_at_line`.

## User Scenarios & Testing

### Scenario 1: Report a formatted non-fatal diagnostic

A caller detects an application-level error that does not require immediate termination and invokes the plain reporting function with:

- `status = 0`,
- `errnum = 0`,
- a format string and arguments.

Expected behavior:

- standard output is flushed if applicable,
- the formatted message is written to standard error,
- no system error text is appended,
- the function returns to the caller.

Traceability: `flush_stdout`, `error`.

#### Test expectations

- Captured standard error contains the formatted message.
- No extra error-text suffix derived from an error number is present.
- Execution continues after the call.

### Scenario 2: Report a system-related non-fatal diagnostic

A caller encounters an OS or library failure and invokes the plain reporting function with:

- `status = 0`,
- `errnum` set to a known error value,
- a format string and arguments.

Expected behavior:

- standard output is flushed if applicable,
- the formatted message is written to standard error,
- a textual description for the supplied error number is appended,
- the function returns.

Traceability: `print_errno_message`, `error`.

#### Test expectations

- Captured standard error contains both the caller’s message and an error description corresponding to the supplied error number.
- Execution continues after the call.

### Scenario 3: Report a source-location diagnostic

A caller wants to identify where an issue occurred and invokes the line-reporting function with:

- `status = 0`,
- `errnum = 0` or a known error number,
- `file_name`,
- `line_number`,
- a format string and arguments.

Expected behavior:

- the diagnostic includes the provided file and line context,
- the message is formatted from the supplied arguments,
- system error text is included only when a nonzero error number is supplied,
- the function returns.

Traceability: `error_at_line`.

#### Test expectations

- Captured standard error includes the given file name.
- Captured standard error includes the given line number.
- Captured standard error includes the formatted message.
- Error-text suffix behavior matches the `errnum` input.

### Scenario 4: Suppress repeated line diagnostics for the same location

A caller issues two line-based diagnostics with the same file name and line number while duplicate-suppression mode is enabled.

Expected behavior:

- the first diagnostic is emitted,
- the second diagnostic for the same location is suppressed,
- a diagnostic for a different file name or line number is still emitted.

Traceability: `error_at_line`.

#### Test expectations

- First call produces output.
- Second call with identical `(file_name, line_number)` produces no additional output.
- A call changing either file name or line number produces output.

### Scenario 5: Fatal reporting

A caller encounters an unrecoverable condition and invokes either reporting function with a nonzero `status`.

Expected behavior:

- the diagnostic is emitted before termination,
- the process exits with the supplied status.

Traceability: `error`, `error_at_line`.

#### Test expectations

- The process terminates.
- Exit status equals the supplied nonzero status.
- Captured standard error contains the expected diagnostic content produced before exit.

### Scenario 6: Preserve output ordering relative to stdout

A caller writes buffered normal output to standard output and then invokes a reporting function.

Expected behavior:

- the module flushes standard output before writing the diagnostic, when stdout is open,
- user-visible output ordering does not place buffered normal output after the diagnostic due to missing flush.

Traceability: `is_open`, `flush_stdout`, `error`, `error_at_line`.

#### Test expectations

- In a harness that buffers stdout, visible stdout content appears before the diagnostic once both streams are observed after the call.
- No failure occurs solely because stdout’s descriptor is not open.

## Requirements

### Functional Requirements

#### FR-1: Formatted error reporting

The module shall provide a reporting operation that accepts:

- a termination status,
- an error number,
- a format string,
- variadic formatting arguments,

and emits a formatted diagnostic to standard error.

Traceability: `error`.

#### FR-2: Source-location error reporting

The module shall provide a reporting operation that accepts:

- a termination status,
- an error number,
- a file name,
- a line number,
- a format string,
- variadic formatting arguments,

and emits a formatted diagnostic to standard error that includes the provided location.

Traceability: `error_at_line`.

#### FR-3: Error-number text inclusion

When a reporting operation receives a nonzero error number, it shall include a textual description corresponding to that error number in the diagnostic output.

Traceability: `print_errno_message`, `error`, `error_at_line`.

#### FR-4: No error-number suffix when absent

When a reporting operation receives `errnum = 0`, it shall emit the caller’s diagnostic without adding system error text.

Traceability: `error`, `error_at_line`.

#### FR-5: Flush-before-report behavior

Before emitting a diagnostic, the module shall attempt to flush standard output when stdout is open.

Traceability: `is_open`, `flush_stdout`, `error`, `error_at_line`.

#### FR-6: Safe handling of closed stdout for flush behavior

The module shall check whether the stdout file descriptor is open before depending on flush behavior, so that diagnostic reporting still proceeds even if stdout is closed.

Traceability: `is_open`, `flush_stdout`.

#### FR-7: Duplicate suppression for repeated file/line reports

The source-location reporting operation shall suppress repeated diagnostics for the same file name and line number when the module’s duplicate-suppression mode is enabled.

Traceability: `error_at_line`.

#### FR-8: No suppression across different locations

The source-location reporting operation shall continue to emit diagnostics when either the file name or line number differs from the most recently suppressed or recorded location pair.

Traceability: `error_at_line`.

#### FR-9: Non-fatal return behavior

When invoked with `status = 0`, each reporting operation shall return to the caller after completing diagnostic emission.

Traceability: `error`, `error_at_line`.

#### FR-10: Fatal exit behavior

When invoked with a nonzero status, each reporting operation shall terminate the process with that status after completing diagnostic emission.

Traceability: `error`, `error_at_line`.

### Key Entities

This module does not define complex domain data structures in the analyzed file. Its key entities are behavioral inputs and retained reporting state.

#### Entity: Plain diagnostic request

A plain diagnostic request consists of:

- `status`,
- `errnum`,
- `message` format string,
- formatting arguments.

Relationship:
- consumed by the plain reporting operation to produce a standard error diagnostic and optional exit behavior.

Traceability: `error`.

#### Entity: Location diagnostic request

A location diagnostic request consists of:

- `status`,
- `errnum`,
- `file_name`,
- `line_number`,
- `message` format string,
- formatting arguments.

Relationship:
- consumed by the location reporting operation to produce a source-qualified diagnostic and optional exit behavior.

Traceability: `error_at_line`.

#### Entity: Error number

An error number is an integer identifying a system or library error condition.

Relationship:
- interpreted by the module to obtain printable error text appended to diagnostics when nonzero.

Traceability: `print_errno_message`, `error`, `error_at_line`.

#### Entity: Duplicate-suppression state

The module maintains state relevant to repeated location-based diagnostics, specifically whether repeated reports for the same `(file_name, line_number)` should be emitted or suppressed under the module’s enabled mode.

Relationship:
- consulted by `error_at_line` before emission,
- updated based on the current location report.

Traceability: `error_at_line`.

## Success Criteria

### SC-1: Plain reporting correctness

Given a plain reporting call with `status = 0` and `errnum = 0`, the Rust module emits the formatted message to standard error and returns without process termination.

Traceability: `error`.

### SC-2: Error-text correctness

Given a reporting call with a known nonzero error number, the Rust module emits diagnostic text that includes a textual description for that error number.

Traceability: `print_errno_message`, `error`, `error_at_line`.

### SC-3: Location formatting correctness

Given a location-reporting call with a file name and line number, the Rust module emits a diagnostic containing both the file name and line number in addition to the formatted message.

Traceability: `error_at_line`.

### SC-4: Duplicate suppression correctness

With duplicate-suppression mode enabled, two consecutive location-reporting calls using the same file name and line number result in only one emitted diagnostic; changing either field results in a new diagnostic.

Traceability: `error_at_line`.

### SC-5: Fatal exit correctness

Given a reporting call with a nonzero status, the Rust module terminates the process with exactly that exit status after emitting the diagnostic.

Traceability: `error`, `error_at_line`.

### SC-6: Flush-before-diagnostic behavior

In a test environment with buffered stdout still open, output written to stdout before a reporting call is flushed before the diagnostic is emitted to stderr.

Traceability: `is_open`, `flush_stdout`, `error`, `error_at_line`.

### SC-7: Closed-stdout robustness

If stdout is closed before a reporting call, the Rust module still emits the diagnostic to standard error without requiring stdout flush success.

Traceability: `is_open`, `flush_stdout`.

## Constraints

- The Rust rewrite must preserve the functional boundaries evidenced by `gnu/error.c`.
- The specification permits internal redesign but not removal of observable reporting behavior.
- No additional public capability is required unless already evidenced by this module’s behavior.

## Acceptance Notes

Acceptance should be based on black-box observation of:

- stderr diagnostic content,
- inclusion or absence of error-text suffixes,
- inclusion of file/line context,
- suppression of duplicate line diagnostics when enabled,
- return vs. exit behavior,
- stdout flush interaction visible at process I/O boundaries.