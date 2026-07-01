# spec.md

## Overview

This module provides process-exit handling for standard output closure in the `cat` project. It allows the caller to optionally record a file name associated with stdout error reporting, configure whether broken-pipe (`EPIPE`) write/close failures should be ignored, and perform the final stdout close/error check.

The Rust rewrite must preserve the same observable behavior boundaries:

- accept a file name to be used in diagnostics related to stdout closure failure,
- accept a configuration flag controlling whether `EPIPE` is ignored,
- perform stdout finalization/checking,
- suppress fatal handling for `EPIPE` only when configured to do so,
- otherwise treat stdout close/finalization failure as an error path.

## Feature Specification

### Summary

The module centralizes the final stdout shutdown behavior used by the program's main flow. Its role is not general stream management; it exists specifically to support end-of-program handling for standard output.

### Functional Scope

The Rust version must implement these capabilities:

1. **Associate a file name with stdout error reporting**
   - The module must allow the caller to set a file name value before stdout is finalized.
   - That value is used as contextual information for any failure reported during stdout close/finalization.

2. **Configure broken-pipe handling**
   - The module must allow the caller to enable or disable ignoring `EPIPE` conditions.
   - This setting affects stdout finalization behavior only.

3. **Finalize and validate stdout**
   - The module must provide a finalization operation for stdout.
   - If finalization succeeds, it completes without error handling.
   - If finalization fails with `EPIPE` and ignore-`EPIPE` is enabled, the failure is not treated as fatal by this module.
   - If finalization fails for any other reason, or for `EPIPE` when ignore-`EPIPE` is disabled, the module must follow the module's error path for stdout closure failure.

### Out of Scope

The Rust version must not introduce unevidenced behavior, including:

- management of arbitrary file descriptors or streams beyond stdout,
- new public configuration beyond file name and ignore-`EPIPE`,
- retry/recovery logic,
- concurrency guarantees,
- persistence or serialization of module state.

## User Scenarios & Testing

### Scenario 1: Normal program exit after writing to stdout

A caller writes output successfully and invokes the module's stdout finalization at shutdown.

Expected behavior:

- stdout is finalized/checked,
- no error path is triggered,
- the operation completes cleanly.

### Scenario 2: Program wants contextual diagnostics for stdout closure failure

A caller sets a file name before shutdown, then stdout finalization encounters a non-ignored error.

Expected behavior:

- the configured file name is the module state used for diagnostics about the stdout failure,
- the module follows its failure path.

### Scenario 3: Broken pipe should be ignored

A caller enables ignore-`EPIPE`, and stdout finalization encounters an `EPIPE` condition.

Expected behavior:

- the module suppresses fatal handling for that condition,
- shutdown can proceed without treating the `EPIPE` close/finalization failure as an error.

### Scenario 4: Broken pipe should not be ignored

A caller leaves ignore-`EPIPE` disabled or explicitly disables it, and stdout finalization encounters `EPIPE`.

Expected behavior:

- the module does not suppress the error,
- the module follows its stdout failure path.

### Scenario 5: Setter order before finalization

A caller sets the file name and/or ignore-`EPIPE` policy before invoking stdout finalization.

Expected behavior:

- the most recently configured values in module state are the ones applied when finalization runs.

### Testing Guidance

The Rust rewrite should be validated with tests covering:

- successful stdout finalization,
- finalization failure with a non-`EPIPE` error,
- `EPIPE` failure while ignore-`EPIPE` is enabled,
- `EPIPE` failure while ignore-`EPIPE` is disabled,
- file name configuration being reflected in the module's failure context,
- repeated setter calls where the last configured value governs behavior.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a way to set a file name value associated with stdout close/finalization error reporting.
  **Traceability:** `close_stdout_set_file_name` in `closeout.c`

- **FR-2**: The module shall provide a way to configure whether `EPIPE` is ignored during stdout close/finalization handling.
  **Traceability:** `close_stdout_set_ignore_EPIPE` in `closeout.c`

- **FR-3**: The module shall provide a stdout finalization operation intended for program shutdown.
  **Traceability:** `close_stdout` in `closeout.c`

- **FR-4**: During stdout finalization, if no error occurs, the module shall complete without triggering its failure path.
  **Traceability:** `close_stdout` in `closeout.c`

- **FR-5**: During stdout finalization, if an `EPIPE` condition occurs and ignore-`EPIPE` is enabled, the module shall suppress fatal handling of that condition.
  **Traceability:** `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`

- **FR-6**: During stdout finalization, if an error other than ignored `EPIPE` occurs, the module shall treat the condition as a stdout close/finalization failure.
  **Traceability:** `close_stdout` in `closeout.c`

- **FR-7**: If a file name has been configured before stdout finalization fails, the module shall use that stored file name as failure context for stdout-related diagnostics.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout` in `closeout.c`

- **FR-8**: Configuration set through the module's setters shall remain in effect until changed by a later setter call or consumed by process execution flow.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `closeout.c`

### Key Entities

- **Configured stdout file name**
  - A module-level stored value representing the file name associated with stdout failure reporting.
  - Set by the file-name setter.
  - Consulted by stdout finalization on error.

- **Ignore-`EPIPE` setting**
  - A module-level stored boolean-like setting controlling whether `EPIPE` is treated as ignorable during stdout finalization.
  - Set by the ignore-`EPIPE` setter.
  - Consulted by stdout finalization when an error is encountered.

- **Stdout finalization action**
  - The module operation that closes or validates stdout at shutdown and decides whether to suppress or trigger failure handling based on the stored configuration and encountered error.

## Success Criteria

- **SC-1**: A Rust implementation exposes module behavior equivalent to the three evidenced operations: set file-name context, set ignore-`EPIPE`, and finalize/check stdout.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`

- **SC-2**: When stdout finalization succeeds, tests confirm that no failure path is triggered.
  **Traceability:** `close_stdout`

- **SC-3**: When stdout finalization yields `EPIPE` and ignore-`EPIPE` is enabled, tests confirm that the condition is not treated as a fatal stdout close failure.
  **Traceability:** `close_stdout_set_ignore_EPIPE`, `close_stdout`

- **SC-4**: When stdout finalization yields `EPIPE` and ignore-`EPIPE` is disabled, tests confirm that the condition is treated as a stdout close failure.
  **Traceability:** `close_stdout_set_ignore_EPIPE`, `close_stdout`

- **SC-5**: When stdout finalization yields a non-`EPIPE` error, tests confirm that the condition is treated as a stdout close failure regardless of ignore-`EPIPE` setting.
  **Traceability:** `close_stdout`

- **SC-6**: When a file name is configured before a failing stdout finalization, tests confirm that the configured value is the failure context used by the module.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout`

- **SC-7**: When setters are called multiple times before finalization, tests confirm that finalization uses the last configured file name and last configured ignore-`EPIPE` value.
  **Traceability:** `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`