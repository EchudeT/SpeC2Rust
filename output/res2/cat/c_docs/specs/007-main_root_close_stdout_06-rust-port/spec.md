# spec.md

## Overview

This module centralizes program-exit handling for standard output closure. It provides a small interface for:

- recording the current output file name for diagnostics,
- configuring whether broken-pipe (`EPIPE`) errors should be ignored during stdout close/flush handling,
- performing the final stdout close check and reporting failure as a fatal program error when applicable.

The Rust rewrite must preserve this behavior boundary: it is an application-level shutdown helper for stdout, not a general stream management subsystem.

## Scope

In scope:

- storing an optional file name used in stdout-related diagnostics,
- storing a process-level setting that controls whether `EPIPE` is ignored,
- checking the result of final stdout shutdown and exiting with an error on failure unless the failure is configured to be ignored.

Out of scope:

- opening output files,
- performing ordinary writes,
- managing arbitrary file descriptors beyond standard output,
- introducing additional public configuration or error-recovery features.

## Feature Specification

### Functional Summary

The module supports programs that write to standard output and need a uniform finalization step at program exit. Before shutdown, the caller may register the relevant output file name and may choose whether broken-pipe failures should be ignored. When the final close operation is invoked, the module evaluates whether stdout completed successfully. If not, it handles the failure as a fatal condition, except for the configured broken-pipe ignore case.

### Required Rust Behavior

The Rust version must implement equivalent observable behavior for the three documented operations:

- set the file name associated with stdout diagnostics,
- set whether `EPIPE` is ignored during stdout finalization,
- finalize stdout and terminate with an error if finalization fails and the failure is not ignorable.

The Rust rewrite must keep these operations within one coherent module boundary serving program-exit stdout handling.

## User Scenarios & Testing

### Scenario 1: Normal successful stdout finalization

A command writes output successfully and, before exiting, invokes the module’s stdout finalization routine.

Expected behavior:

- no error is reported,
- program shutdown proceeds normally.

Testable outcome:

- finalization returns/continues without fatal termination when stdout has no pending error.

### Scenario 2: Associate an output file name for diagnostics

A command is writing to a named destination and records that file name before final stdout finalization.

Expected behavior:

- if stdout finalization later fails, diagnostics are associated with the configured file name.

Testable outcome:

- after setting a file name, a forced stdout close failure produces an error path that references the configured name rather than behaving as if no name were set.

### Scenario 3: Broken pipe is not ignored

A command writes into a pipeline whose reader exits early, and the module is left in its default or explicitly non-ignoring mode for `EPIPE`.

Expected behavior:

- stdout finalization treats broken pipe as an error,
- program exits fatally on finalization failure.

Testable outcome:

- under a simulated `EPIPE` close/finalization failure with ignore disabled, the module follows the fatal error path.

### Scenario 4: Broken pipe is ignored

A command writes into a pipeline whose reader exits early, and the module is configured to ignore `EPIPE`.

Expected behavior:

- stdout finalization does not treat `EPIPE` as fatal,
- program shutdown proceeds without fatal termination for that specific error class.

Testable outcome:

- under a simulated `EPIPE` close/finalization failure with ignore enabled, the module suppresses fatal handling.

### Scenario 5: Non-`EPIPE` stdout failure remains fatal

A command encounters a stdout finalization failure caused by an error other than broken pipe, even when broken pipe ignoring is enabled.

Expected behavior:

- the failure is still treated as fatal.

Testable outcome:

- under a simulated non-`EPIPE` close/finalization failure, the module follows the fatal error path regardless of the `EPIPE` ignore setting.

## Requirements

### Functional Requirements

#### FR-1: File name registration
The module shall allow the caller to register a file name string used for stdout-related diagnostics during finalization failure handling.

Traceability: `close_stdout_set_file_name` in `closeout.c`.

#### FR-2: Broken-pipe ignore configuration
The module shall allow the caller to configure whether stdout finalization should ignore `EPIPE` failures.

Traceability: `close_stdout_set_ignore_EPIPE` in `closeout.c`.

#### FR-3: Stdout finalization check
The module shall provide an operation that performs the final stdout close/check step at program shutdown.

Traceability: `close_stdout` in `closeout.c`.

#### FR-4: Fatal handling of stdout finalization failure
When the stdout finalization step detects failure, the module shall treat the failure as fatal and terminate through the program’s error path, except where explicitly suppressed by the `EPIPE` ignore rule.

Traceability: `close_stdout` in `closeout.c`.

#### FR-5: Conditional suppression for `EPIPE`
If stdout finalization fails with `EPIPE` and the ignore setting is enabled, the module shall suppress fatal handling for that case.

Traceability: interaction of `close_stdout_set_ignore_EPIPE` and `close_stdout` in `closeout.c`.

#### FR-6: Diagnostic context uses configured file name
If a file name has been registered before stdout finalization failure occurs, the module shall use that configured name as diagnostic context for the failure path.

Traceability: interaction of `close_stdout_set_file_name` and `close_stdout` in `closeout.c`.

### Key Entities

#### 1. Output file name state
A module-level optional file name value used as context when reporting stdout finalization failures.

Relationship:
- set by the file name registration operation,
- consumed by the stdout finalization operation when reporting failure.

Traceability: `close_stdout_set_file_name`, `close_stdout` in `closeout.c`.

#### 2. Ignore-`EPIPE` state
A module-level boolean setting controlling whether broken-pipe finalization errors are fatal.

Relationship:
- set by the ignore configuration operation,
- consulted by the stdout finalization operation during failure classification.

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`.

#### 3. Stdout finalization result
The success or failure outcome of the final stdout close/check step.

Relationship:
- evaluated by the finalization operation,
- interpreted using the ignore-`EPIPE` state,
- reported using the output file name state when fatal.

Traceability: `close_stdout` in `closeout.c`.

## Success Criteria

### SC-1: File name state affects diagnostics
Given a configured file name and a forced stdout finalization failure, the Rust module’s fatal error path includes or uses that configured file context.

Traceability: `close_stdout_set_file_name`, `close_stdout` in `closeout.c`.

### SC-2: Ignore flag is configurable
The Rust module accepts both enabled and disabled `EPIPE` ignore settings, and later stdout finalization behavior changes accordingly.

Traceability: `close_stdout_set_ignore_EPIPE` in `closeout.c`.

### SC-3: Successful finalization is non-fatal
When stdout finalization succeeds, invoking the Rust module’s finalization operation does not trigger fatal termination.

Traceability: `close_stdout` in `closeout.c`.

### SC-4: `EPIPE` can be suppressed
When stdout finalization fails specifically with `EPIPE` and ignore is enabled, the Rust module does not take the fatal error path.

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`.

### SC-5: Non-`EPIPE` failures remain fatal
When stdout finalization fails with an error other than `EPIPE`, the Rust module takes the fatal error path regardless of the ignore setting.

Traceability: `close_stdout` in `closeout.c`.

### SC-6: Disabled ignore causes fatal `EPIPE`
When stdout finalization fails with `EPIPE` and ignore is disabled, the Rust module takes the fatal error path.

Traceability: `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`.