# spec.md

## Overview

This module defines process-exit handling for standard output closure. Its purpose is to finalize writes to `stdout`, detect output errors that become visible only at close time, and terminate the program with an error when such failures occur. It also allows the caller to configure two aspects of that behavior:

- the file name to mention in diagnostics associated with stdout write/close failure
- whether broken-pipe (`EPIPE`) close failures should be ignored

The Rust rewrite must preserve this functional role: provide module-level configuration for stdout-close diagnostics and perform a final stdout close/check step that either returns silently on success or causes process termination on detected output failure, subject to the configured `EPIPE` policy.

## Scope

In scope for this module:

- storing an optional output file name for diagnostic use
- storing a module-wide policy for whether `EPIPE` is ignored
- performing the stdout finalization check/close action
- reporting failure through program termination when stdout cannot be cleanly closed, except where configured `EPIPE` is ignored

Out of scope:

- general output formatting
- opening or managing arbitrary files beyond identifying stdout in diagnostics
- recovery after stdout close failure
- defining a broader process-exit framework beyond this stdout-specific responsibility

## Public Functional Surface

This specification covers the behavior evidenced by these functions:

- `close_stdout_set_file_name`
- `close_stdout_set_ignore_EPIPE`
- `close_stdout`

## Feature Specification

### 1. Configurable diagnostic target name

The module must allow the program to set a file name string representing the logical destination associated with standard output. When a stdout close/write-finalization error is reported, this configured name is used as the diagnostic target identity.

If no name has been configured, the module must still perform stdout finalization and error handling, but diagnostics may use the module’s default stdout-oriented identification.

Traceability: `close_stdout_set_file_name`

### 2. Configurable broken-pipe handling

The module must allow the program to configure whether `EPIPE` errors detected during stdout finalization are ignored.

- When ignore is enabled, an `EPIPE` condition during stdout finalization must not trigger fatal termination by this module.
- When ignore is disabled, `EPIPE` is treated like other stdout finalization failures and must trigger fatal termination.

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout`

### 3. Final stdout close/check behavior

The module must provide a finalization operation for standard output.

Its behavior must be:

- attempt the module’s stdout close/check action
- if no failure is detected, return without terminating
- if failure is detected:
  - suppress fatal handling only for `EPIPE` when configured to ignore it
  - otherwise emit an error diagnostic associated with the configured/default output name and terminate the process with failure

This operation is intended for use at program shutdown, after stdout output is complete.

Traceability: `close_stdout`

## User Scenarios & Testing

### Scenario 1: Normal program exit after successful stdout output

A command writes data to standard output and invokes the module’s finalization function before exiting. No stdout error exists.

Expected behavior:

- finalization completes without visible error
- the module does not terminate the process
- no diagnostic is emitted

Traceability: `close_stdout`

### Scenario 2: stdout failure with configured output name

A command directs stdout to a named file or otherwise wants diagnostics to mention a specific target. It sets that file name and later invokes stdout finalization. A write/close error is present.

Expected behavior:

- finalization detects failure
- diagnostic identifies the configured target name
- the process terminates with failure

Traceability: `close_stdout_set_file_name`, `close_stdout`

### Scenario 3: broken pipe ignored

A command writes to a pipeline whose reader exits early. Before shutdown, the program configures the module to ignore `EPIPE` and invokes stdout finalization.

Expected behavior:

- if the finalization failure is `EPIPE`, the module does not terminate the process on that basis
- no fatal diagnostic is emitted for the ignored `EPIPE` case

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout`

### Scenario 4: broken pipe not ignored

A command writes to a pipeline whose reader exits early and does not enable `EPIPE` ignore mode. It invokes stdout finalization.

Expected behavior:

- `EPIPE` is treated as a failure
- the module emits an error diagnostic
- the process terminates with failure

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout`

### Scenario 5: configuration applied before finalization

A command updates module configuration before invoking finalization.

Expected behavior:

- the file-name setting used by diagnostics is the most recently configured one
- the `EPIPE` policy used by finalization is the currently configured one

Traceability: `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`

## Requirements

### Functional Requirements

#### FR-1: File-name configuration
The module shall provide a way to set the diagnostic file name associated with stdout finalization errors.

Traceability: `close_stdout_set_file_name`

#### FR-2: File-name usage
When stdout finalization fails and the module emits a diagnostic, the diagnostic shall identify the output target using the configured file name if one has been set.

Traceability: `close_stdout_set_file_name`, `close_stdout`

#### FR-3: EPIPE ignore configuration
The module shall provide a way to configure whether `EPIPE` detected during stdout finalization is ignored.

Traceability: `close_stdout_set_ignore_EPIPE`

#### FR-4: Successful finalization path
If stdout finalization detects no error, the module shall return without terminating the process.

Traceability: `close_stdout`

#### FR-5: Fatal handling for non-ignored failure
If stdout finalization detects an error other than an ignored `EPIPE`, the module shall emit an error diagnostic and terminate the process with failure.

Traceability: `close_stdout`

#### FR-6: Ignored EPIPE behavior
If stdout finalization detects `EPIPE` and `EPIPE` ignore mode is enabled, the module shall suppress fatal termination for that condition.

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout`

#### FR-7: Configuration affects later finalization
Configuration set through the module’s setters shall affect subsequent invocation of stdout finalization.

Traceability: `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`

### Key Entities

#### 1. Diagnostic target name
A module-level string value representing the logical output destination name for stdout-related diagnostics.

Relationship:
- consulted by stdout finalization error reporting

Traceability: `close_stdout_set_file_name`, `close_stdout`

#### 2. EPIPE ignore flag
A module-level boolean policy value indicating whether `EPIPE` during stdout finalization should be treated as ignorable.

Relationship:
- consulted by stdout finalization when evaluating detected close/check errors

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout`

#### 3. Stdout finalization operation
The module action that performs the final close/check of standard output and applies the configured diagnostic name and `EPIPE` policy.

Relationship:
- consumes both module-level configuration values during failure handling

Traceability: `close_stdout`

## Success Criteria

### Behavioral correctness

1. When stdout finalization succeeds, invoking the Rust module’s finalization operation completes without process termination.
   - Traceability: `close_stdout`

2. When stdout finalization fails with a non-`EPIPE` error, invoking the Rust module’s finalization operation results in an error diagnostic and process-failure termination.

3. When stdout finalization fails with `EPIPE` and ignore mode is disabled, invoking the Rust module’s finalization operation results in an error diagnostic and process-failure termination.
   - Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout`

4. When stdout finalization fails with `EPIPE` and ignore mode is enabled, invoking the Rust module’s finalization operation does not terminate the process for that condition.

5. After setting a diagnostic file name, a later stdout-finalization failure diagnostic refers to that configured name.
   - Traceability: `close_stdout_set_file_name`, `close_stdout`

### Testability expectations

6. Tests shall cover at least:
   - successful stdout finalization
   - failing stdout finalization with configured file name
   - `EPIPE` failure with ignore disabled
   - `EPIPE` failure with ignore enabled
   - configuration changes applied before finalization
   - Traceability: `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`

7. The Rust rewrite shall expose no required functionality beyond the behaviors specified in this document for this module.
   - Traceability: all listed functions