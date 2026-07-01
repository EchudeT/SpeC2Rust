# spec.md

## Title

Functional Specification: `main_root_close_stdout_07`

## Metadata

- Project: `pwd`
- Module: `main_root_close_stdout_07`
- Category: `main_cluster`
- Source file: `closeout.c`
- Rust target branch: `007-main_root_close_stdout_07-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides process-exit handling for standard output. It allows the caller to:

- associate a file name with standard output for diagnostics,
- configure whether broken-pipe (`EPIPE`) write-close failures are ignored,
- perform final standard output closure checking and terminate with an error when that close operation fails under the module's rules.

The Rust rewrite must preserve this behavior as a small exit-time utility module for stdout finalization and related error reporting policy.

## Feature Specification

### Summary

The module manages final shutdown of standard output. Its behavior is controlled by module-held configuration set before closure:

1. a stored file name used to describe the output destination in diagnostics,
2. a stored policy flag controlling whether `EPIPE` close failures are ignored.

When invoked, the close operation must attempt to finalize standard output and evaluate the result. If closure fails and the failure is not covered by the ignore-`EPIPE` policy, the module must report a write/close error associated with the configured output target and terminate with failure behavior equivalent to the C module's role.

### Supported behavior

#### Output target naming

The module accepts a file name string to represent the output destination. This name is used only as diagnostic context for stdout close failure handling.

If no explicit name has been set, the Rust version must preserve the module's effective default output-target naming behavior used for diagnostics.

#### Broken pipe policy

The module accepts a boolean setting that controls whether an `EPIPE` condition during stdout finalization is treated as ignorable.

- When ignore is enabled, an `EPIPE` close failure on stdout must not trigger fatal error handling.
- When ignore is disabled, `EPIPE` must be treated like other close failures.

#### Final stdout closure check

The module provides a finalization entry point that closes or finalizes standard output and checks for failure.

On failure, behavior depends on the error condition and current configuration:

- non-`EPIPE` close failures must trigger fatal output error handling,
- `EPIPE` close failures must trigger fatal output error handling unless the ignore policy is enabled.

Fatal handling must remain tied to stdout write/close failure reporting semantics evidenced by this module's purpose.

## User Scenarios & Testing

### Scenario 1: Normal successful stdout finalization

A command writes output to stdout and, before process exit, invokes the module's close routine.

Expected behavior:

- stdout finalization succeeds,
- no error is reported,
- the process continues its normal exit path.

### Scenario 2: Close failure with named output target

A caller configures a specific output file name and later stdout finalization fails.

Expected behavior:

- the configured file name is used as the output target identity for diagnostics,
- the failure is treated as fatal unless it is an ignored `EPIPE`.

### Scenario 3: Broken pipe ignored

A command writes to a pipeline consumer that exits early, causing stdout finalization to encounter `EPIPE`. The caller has enabled ignore-`EPIPE`.

Expected behavior:

- the module suppresses fatal handling for that `EPIPE`,
- the close routine returns after handling the condition according to ignore policy.

### Scenario 4: Broken pipe not ignored

A command writes to a pipeline consumer that exits early, causing stdout finalization to encounter `EPIPE`. The caller has not enabled ignore-`EPIPE`.

Expected behavior:

- the module treats the failure as fatal,
- diagnostic handling uses the configured or default output target name.

### Scenario 5: Default diagnostic target naming

A caller does not set any file name and invokes final stdout closure, which fails.

Expected behavior:

- the module uses its built-in default output target name for diagnostics,
- fatal handling follows the same rules as any other close failure.

### Test coverage expectations

The Rust version must be testable for at least:

- setting output target name before finalization,
- toggling ignore-`EPIPE` behavior,
- successful close path,
- close failure with non-`EPIPE` error,
- close failure with `EPIPE` when ignored,
- close failure with `EPIPE` when not ignored,
- default-name diagnostic path when no file name was set.

## Requirements

### Functional Requirements

#### FR-1: Configurable diagnostic output target
Traceability: `close_stdout_set_file_name` in `closeout.c`

The module shall allow the caller to set the file name associated with stdout for later close-error diagnostics.

#### FR-2: Configurable `EPIPE` ignore policy
Traceability: `close_stdout_set_ignore_EPIPE` in `closeout.c`

The module shall allow the caller to configure whether an `EPIPE` condition encountered during stdout closure is ignored.

#### FR-3: Stdout finalization entry point
Traceability: `close_stdout` in `closeout.c`

The module shall provide an operation that finalizes stdout and checks whether that finalization failed.

#### FR-4: Fatal handling for non-ignored close failures
Traceability: `close_stdout` in `closeout.c`

If stdout finalization fails with an error other than an ignored `EPIPE`, the module shall invoke failure handling corresponding to stdout write/close error reporting and abnormal command termination behavior.

#### FR-5: Suppression of fatal handling for ignored `EPIPE`
Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`

If stdout finalization fails with `EPIPE` and ignore-`EPIPE` is enabled, the module shall not invoke fatal close-error handling for that condition.

#### FR-6: Diagnostic context uses configured or default target name
Traceability: `close_stdout_set_file_name`, `close_stdout` in `closeout.c`

When close-error handling is triggered, the module shall associate the failure with the configured output file name, or with the module's default stdout target name if no name was configured.

### Key Entities

#### Entity: Output target name
Traceability: `close_stdout_set_file_name` in `closeout.c`

A module-retained diagnostic identifier representing the destination associated with stdout. It is set by the caller and consumed by close-error handling.

#### Entity: Ignore-`EPIPE` flag
Traceability: `close_stdout_set_ignore_EPIPE` in `closeout.c`

A module-retained boolean policy value controlling whether `EPIPE` during stdout finalization is fatal.

#### Entity: Stdout close outcome
Traceability: `close_stdout` in `closeout.c`

The result of finalizing stdout, including success, `EPIPE` failure, or other failure. This outcome is evaluated against the stored policy and diagnostic target name to determine behavior.

#### Relationships

- The output target name and ignore-`EPIPE` flag are configuration inputs retained by the module before finalization.
- The stdout close outcome is interpreted using the ignore-`EPIPE` flag.
- If the close outcome requires fatal handling, the output target name provides diagnostic context.

## Success Criteria

### SC-1: File name configuration affects diagnostics
Traceability: `close_stdout_set_file_name`, `close_stdout` in `closeout.c`

In tests that simulate stdout close failure after setting a file name, the Rust module uses that configured name in the failure path rather than the default target name.

### SC-2: Default name remains available
Traceability: `close_stdout_set_file_name`, `close_stdout` in `closeout.c`

In tests that simulate stdout close failure without prior file-name configuration, the Rust module follows the module's default diagnostic naming behavior.

### SC-3: `EPIPE` ignore policy is honored
Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`

In tests that simulate `EPIPE` during stdout finalization:
- with ignore enabled, fatal failure handling is not triggered;
- with ignore disabled, fatal failure handling is triggered.

### SC-4: Non-`EPIPE` close failures remain fatal
Traceability: `close_stdout` in `closeout.c`

In tests that simulate a stdout close failure with an error other than `EPIPE`, fatal failure handling is triggered regardless of the ignore-`EPIPE` setting.

### SC-5: Successful close is non-fatal
Traceability: `close_stdout` in `closeout.c`

In tests where stdout finalization succeeds, the close operation completes without triggering failure handling.

### SC-6: Module scope remains limited to stdout close handling
Traceability: all listed functions in `closeout.c`

The Rust rewrite exposes and implements only the functionality evidenced here: setting diagnostic file name, setting ignore-`EPIPE` policy, and performing stdout finalization/error handling, without adding unrelated public capabilities.