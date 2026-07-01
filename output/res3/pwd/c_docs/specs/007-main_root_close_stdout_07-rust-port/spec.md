# spec.md

## Overview

This module provides process-exit stdout closing behavior for the `pwd` project. It lets the program optionally register a file name associated with stdout output, optionally ignore `EPIPE` on stdout close, and perform a final stdout close operation that reports write/close failures as fatal program errors.

The Rust rewrite must preserve the same functional boundary:

- accept an optional output file name used in diagnostics,
- accept a policy switch for whether `EPIPE` during stdout close is ignored,
- close stdout on demand,
- treat close failures according to the configured policy.

This specification is derived from `closeout.c` and is limited to the behavior evidenced by:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`

## Scope

In scope:

- configuration of module-level stdout close diagnostics context,
- configuration of `EPIPE` handling for stdout close,
- execution of final stdout close with error handling.

Out of scope:

- opening or managing arbitrary files,
- writing output data,
- stderr closing behavior,
- recovery after close failure,
- new public APIs beyond the evidenced module surface.

## Feature Specification

### Summary

The module centralizes finalization of stdout. Before program termination, callers may set contextual information for diagnostics and may select whether broken-pipe errors are ignored. When finalization runs, the module closes stdout and enforces fatal handling for relevant failures.

### Required behavior

1. The module shall allow the caller to associate a file name string with stdout output for later diagnostic use.
2. The module shall allow the caller to configure whether `EPIPE` is ignored during stdout close failure handling.
3. The module shall provide a finalization operation that closes stdout.
4. If stdout close succeeds, the finalization operation shall complete without reporting an error.
5. If stdout close fails with `EPIPE` and ignore-`EPIPE` mode is enabled, the finalization operation shall suppress fatal reporting for that condition.
6. If stdout close fails for any other reason, or if ignore-`EPIPE` mode is disabled, the finalization operation shall report the failure as fatal.
7. Fatal reporting shall use the configured file name context when such context was previously set; otherwise it shall operate without file-name context.

## User Scenarios & Testing

### Scenario 1: Normal successful program output finalization

A program writes its output to stdout, then invokes this module’s finalization operation during shutdown. Stdout closes successfully, so shutdown continues without an error report.

Test expectations:

- configure no special options,
- call finalization after writing to stdout,
- verify no fatal error is reported when stdout close succeeds.

### Scenario 2: Output directed to a named file with close failure

A program conceptually associates stdout with a destination file name and stores that name through this module before shutdown. When stdout close fails, the module reports a fatal error that includes file-context information.

Test expectations:

- set a file name before finalization,
- induce a stdout close failure other than ignored `EPIPE`,
- verify fatal failure is reported,
- verify the configured file name is the context used for the diagnostic path.

### Scenario 3: Broken pipe should be ignored

A program writes to a pipeline whose reader terminates early. The caller enables ignore-`EPIPE` mode before finalization. If stdout close encounters `EPIPE`, the module suppresses fatal reporting.

Test expectations:

- enable ignore-`EPIPE`,
- induce `EPIPE` on stdout close,
- call finalization,
- verify no fatal error is reported for `EPIPE`.

### Scenario 4: Broken pipe should not be ignored

A program writes to a pipeline whose reader terminates early, but the caller does not enable ignore-`EPIPE`. Stdout close failure with `EPIPE` remains fatal.

Test expectations:

- leave ignore-`EPIPE` disabled,
- induce `EPIPE` on stdout close,
- call finalization,
- verify fatal error is reported.

### Scenario 5: Configuration before finalization

A caller configures file-name context and `EPIPE` policy before invoking finalization. The finalization behavior reflects the latest configured values.

Test expectations:

- set file name,
- set ignore-`EPIPE` policy,
- call finalization,
- verify close behavior follows the active configuration.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a way to store a file name associated with stdout output for use by later close-error reporting.
  **Traceability:** `close_stdout_set_file_name` in `closeout.c`

- **FR-2**: The module shall provide a way to store whether `EPIPE` should be ignored during stdout close failure handling.
  **Traceability:** `close_stdout_set_ignore_EPIPE` in `closeout.c`

- **FR-3**: The module shall provide an operation that closes stdout.
  **Traceability:** `close_stdout` in `closeout.c`

- **FR-4**: When the stdout close operation detects no error, it shall return normally without fatal reporting.
  **Traceability:** `close_stdout` in `closeout.c`

- **FR-5**: When the stdout close operation detects `EPIPE` and ignore-`EPIPE` has been enabled, it shall suppress fatal reporting for that error.
  **Traceability:** `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`

- **FR-6**: When the stdout close operation detects an error that is not an ignored `EPIPE`, it shall perform fatal error reporting.
  **Traceability:** `close_stdout` in `closeout.c`

- **FR-7**: Fatal reporting behavior shall use previously configured file-name context if one was set through the module.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout` in `closeout.c`

### Key Entities

- **Stdout close context**: Module-held state consisting of:
  - optional file name associated with stdout output,
  - boolean policy indicating whether `EPIPE` is ignored during stdout close handling.
  **Traceability:** implied by `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, and consumed by `close_stdout` in `closeout.c`

- **Stdout finalization action**: The close operation that reads the current context and applies it to stdout close outcome handling.
  **Traceability:** `close_stdout` in `closeout.c`

## Success Criteria

- **SC-1**: A Rust implementation exposes equivalent module functionality for setting file-name context, setting ignore-`EPIPE` policy, and finalizing stdout close.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`

- **SC-2**: In tests where stdout close succeeds, finalization completes without fatal error reporting.
  **Traceability:** `close_stdout`

- **SC-3**: In tests where stdout close fails with `EPIPE` and ignore-`EPIPE` is enabled, finalization does not trigger fatal error reporting.
  **Traceability:** `close_stdout_set_ignore_EPIPE`, `close_stdout`

- **SC-4**: In tests where stdout close fails with `EPIPE` and ignore-`EPIPE` is disabled, finalization triggers fatal error reporting.
  **Traceability:** `close_stdout_set_ignore_EPIPE`, `close_stdout`

- **SC-5**: In tests where stdout close fails with a non-`EPIPE` error, finalization triggers fatal error reporting regardless of ignore-`EPIPE` configuration.
  **Traceability:** `close_stdout`

- **SC-6**: In tests where file-name context was configured before a fatal stdout close failure, the fatal reporting path uses that context.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout`

## Constraints

- The Rust rewrite must preserve only the evidenced behavior of this module.
- The specification does not require new APIs, extended configuration, concurrency guarantees, or recovery flows.
- The module is specified as a stdout finalization helper, not as a general stream-management subsystem.