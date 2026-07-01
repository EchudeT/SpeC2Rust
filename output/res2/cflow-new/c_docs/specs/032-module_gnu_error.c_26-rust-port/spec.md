# spec.md

## Title

Functional Specification: `module_gnu_error.c_26` Rust Port

## Document Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_error.c_26`
- **Category**: `module_cluster`
- **Source file**: `gnu/error.c`
- **Rust branch**: `032-module_gnu_error.c_26-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides formatted diagnostic reporting functions for emitting error messages to the standard error stream, with optional inclusion of system error text and source location context. It also manages related output behavior needed when diagnostics are emitted, including coordination with standard output flushing and conditional process termination.

The Rust rewrite must preserve the observable behavior of the C module’s diagnostic functions and their supporting formatting rules as evidenced by the analyzed source.

## Scope

### In Scope

- Emitting formatted diagnostic messages to standard error.
- Supporting a variant that includes file name and line number context.
- Optionally appending an error description corresponding to a supplied error number.
- Flushing standard output before writing diagnostics when appropriate.
- Optionally terminating the process when a nonzero status is supplied.
- Supporting repeated calls while preserving message emission behavior.

### Out of Scope

- Defining new logging levels, sinks, or configuration interfaces.
- Adding structured diagnostics, localization controls, or formatting extensions not evidenced by the source.
- Introducing new public APIs beyond the behavior represented by the analyzed functions.
- Guaranteeing thread-safety or async behavior.

## Feature Specification

### Feature 1: Formatted diagnostic emission

The module shall provide a function equivalent in behavior to `error(status, errnum, message, ...)` that emits a single diagnostic message to standard error.

Behavior to preserve:

- Accept a printf-style message format and variadic arguments.
- Emit the formatted message as a diagnostic line.
- If `errnum` is nonzero, append a separator and the corresponding system error message.
- End the diagnostic with a newline.
- If `status` is nonzero, terminate the process after emitting the message.

This feature is evidenced by:
- `error` in `gnu/error.c`
- `print_errno_message` in `gnu/error.c`
- `flush_stdout` in `gnu/error.c`

### Feature 2: Source-location-aware diagnostic emission

The module shall provide a function equivalent in behavior to `error_at_line(status, errnum, file_name, line_number, message, ...)` that emits a diagnostic including source location context.

Behavior to preserve:

- Include the supplied file name and line number in the diagnostic output.
- Emit the caller-supplied formatted message.
- If `errnum` is nonzero, append the corresponding system error message.
- End the diagnostic with a newline.
- If `status` is nonzero, terminate the process after emitting the message.

This feature is evidenced by:
- `error_at_line` in `gnu/error.c`
- `print_errno_message` in `gnu/error.c`
- `flush_stdout` in `gnu/error.c`

### Feature 3: Standard output coordination before diagnostics

Before emitting diagnostics, the module shall perform the same observable stdout-handling behavior as the C module.

Behavior to preserve:

- Attempt to flush standard output before writing the diagnostic.
- Avoid unsafe or inappropriate flush behavior when standard output is not considered open for this purpose.

This feature is evidenced by:
- `flush_stdout` in `gnu/error.c`
- `is_open` in `gnu/error.c`

### Feature 4: Error-number text rendering

The module shall support rendering of error text associated with a provided error number.

Behavior to preserve:

- When an error number is supplied for a diagnostic, render a textual description for that error number in the emitted message.
- When no error number is supplied, omit that appended system error text.

This feature is evidenced by:
- `print_errno_message` in `gnu/error.c`
- call paths from `error` and `error_at_line`

## User Scenarios & Testing

### Scenario 1: Emit a plain diagnostic without a system error

A caller needs to report a user-facing error condition using a formatted message only.

Expected behavior:

- The module writes one diagnostic to standard error.
- The output contains the formatted message text.
- The output ends with a newline.
- No system error text is appended when `errnum` is zero.
- The process continues when `status` is zero.

Suggested tests:

- Call the plain diagnostic function with `status = 0`, `errnum = 0`, and a format string with arguments.
- Verify stderr contains the expected formatted message and newline only.
- Verify no process termination occurs.

Traceability:
- `error`

### Scenario 2: Emit a diagnostic with errno-derived detail

A caller needs to report an operation failure and include the associated system error description.

Expected behavior:

- The module writes the caller’s formatted message to standard error.
- The module appends the textual description corresponding to the supplied error number.
- The output ends with a newline.

Suggested tests:

- Call the plain diagnostic function with a known nonzero error number.
- Verify stderr contains both the formatted message and the expected error description form.
- Verify exactly one newline terminates the diagnostic.

Traceability:
- `error`
- `print_errno_message`

### Scenario 3: Emit a source-location-aware diagnostic

A caller reports an issue tied to an input file or source location.

Expected behavior:

- The diagnostic includes the supplied file name.
- The diagnostic includes the supplied line number.
- The diagnostic includes the formatted message text.
- If `errnum` is nonzero, the diagnostic also includes the corresponding system error text.
- The output ends with a newline.

Suggested tests:

- Call the location-aware diagnostic function with a sample file name and line number.
- Verify stderr output includes file name, line number, message, and optional errno text in the same emitted diagnostic.

Traceability:
- `error_at_line`
- `print_errno_message`

### Scenario 4: Flush stdout before reporting a diagnostic

A caller has pending output on standard output and then emits a diagnostic.

Expected behavior:

- Any eligible buffered standard output content is flushed before the diagnostic is written to standard error.
- The Rust port must preserve the observable ordering intent of the C module.

Suggested tests:

- Arrange buffered stdout output without a trailing flush.
- Emit a diagnostic.
- Verify the stdout content becomes visible before or consistently with the diagnostic according to the module’s flush behavior.

Traceability:
- `flush_stdout`
- `is_open`
- callers: `error`, `error_at_line`

### Scenario 5: Emit a diagnostic and then terminate

A caller uses the module for fatal errors.

Expected behavior:

- The full diagnostic is emitted before termination.
- Process termination occurs when `status` is nonzero.
- The termination status corresponds to the supplied status value.

Suggested tests:

- Invoke each public diagnostic form in a subprocess with a nonzero status.
- Verify the subprocess exits with the supplied status.
- Verify the expected diagnostic is present on standard error before exit.

Traceability:
- `error`
- `error_at_line`

## Requirements

### Functional Requirements

#### FR-1: Plain diagnostic output

The Rust module shall provide behavior equivalent to the C module’s plain diagnostic function that accepts:

- an exit status,
- an error number,
- a printf-style message format,
- and format arguments,

and emits the resulting diagnostic to standard error.

Traceability:
- `error`

#### FR-2: Location-aware diagnostic output

The Rust module shall provide behavior equivalent to the C module’s location-aware diagnostic function that accepts:

- an exit status,
- an error number,
- a file name,
- a line number,
- a printf-style message format,
- and format arguments,

and emits the resulting diagnostic to standard error including the supplied location context.

Traceability:
- `error_at_line`

#### FR-3: Conditional inclusion of system error text

For both diagnostic entry points, when the supplied error number is nonzero, the emitted diagnostic shall include the textual description associated with that error number; when the supplied error number is zero, that appended system error text shall be omitted.

Traceability:
- `print_errno_message`
- `error`
- `error_at_line`

#### FR-4: Diagnostic termination format

Each emitted diagnostic shall be completed as a single newline-terminated message.

Traceability:
- `error`
- `error_at_line`
- `print_errno_message`

#### FR-5: Stdout flush coordination

Before writing a diagnostic, the Rust module shall perform stdout flushing behavior equivalent to the C module, including the conditional handling associated with whether stdout is open for this purpose.

Traceability:
- `flush_stdout`
- `is_open`
- `error`
- `error_at_line`

#### FR-6: Fatal-exit behavior

When the supplied status is zero, the diagnostic functions shall return after emission. When the supplied status is nonzero, they shall terminate the process after emitting the diagnostic.

Traceability:
- `error`
- `error_at_line`

#### FR-7: Repeated-call usability

The module shall support repeated invocation of its diagnostic functions without requiring caller-managed module initialization.

Traceability:
- `error`
- `error_at_line`
- supporting helpers in `gnu/error.c`

### Key Entities

This module does not define core application data structures in the analyzed file. Its functional entities are procedural inputs and output targets:

#### Entity 1: Diagnostic request

A diagnostic request consists of the inputs provided to one of the diagnostic functions:

- `status`
- `errnum`
- message format and arguments
- optional `file_name`
- optional `line_number`

Relationship:
- A diagnostic request is transformed into one emitted stderr message, and may also trigger process termination.

Traceability:
- `error`
- `error_at_line`

#### Entity 2: Standard output state

The module consults the state of standard output to determine whether stdout should be flushed before diagnostic emission.

Relationship:
- Standard output state influences pre-diagnostic flushing behavior but is not itself emitted as part of the diagnostic.

Traceability:
- `is_open`
- `flush_stdout`

#### Entity 3: System error description

A system error description is the text corresponding to a nonzero error number.

Relationship:
- It is appended to the caller-supplied diagnostic message when `errnum` is nonzero.

Traceability:
- `print_errno_message`

## Success Criteria

### SC-1: Correct plain-message emission

Given `status = 0` and `errnum = 0`, the Rust port emits the formatted message to standard error as one newline-terminated diagnostic and does not terminate the process.

Traceability:
- `error`

### SC-2: Correct errno text inclusion

Given a known nonzero error number, the Rust port emits a diagnostic containing both the formatted message and the matching system error text.

Traceability:
- `print_errno_message`
- `error`
- `error_at_line`

### SC-3: Correct location rendering

Given `file_name` and `line_number`, the Rust port emits a diagnostic that includes both values along with the formatted message.

Traceability:
- `error_at_line`

### SC-4: Correct fatal-exit behavior

When either diagnostic function is invoked with a nonzero status in a subprocess test, the subprocess exits with that status after writing the diagnostic.

Traceability:
- `error`
- `error_at_line`

### SC-5: Stdout coordination preserved

In a test with buffered stdout output followed by a diagnostic call, the Rust port preserves the C module’s observable intent that stdout is flushed before the diagnostic is emitted when stdout is eligible for flushing.

Traceability:
- `flush_stdout`
- `is_open`

### SC-6: Repeated invocation remains correct

Across multiple consecutive calls mixing plain and location-aware diagnostics, each call emits exactly one complete diagnostic with correct inclusion or omission of errno text and no required reinitialization step.

Traceability:
- `error`
- `error_at_line`
- `print_errno_message`

## Acceptance Notes

- Conformance is based on externally observable behavior of message emission, optional errno text inclusion, location context inclusion, stdout flush coordination, newline termination, and fatal exit behavior.
- The Rust implementation may choose different internal mechanisms, but it must not alter the specified behavior or add unsupported capabilities.