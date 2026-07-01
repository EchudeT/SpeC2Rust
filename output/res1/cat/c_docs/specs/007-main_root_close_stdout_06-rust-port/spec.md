# Specification: main_root_close_stdout_06

- **Project**: `cat`
- **Module**: `main_root_close_stdout_06`
- **Category**: `main_cluster`
- **Source basis**: `closeout.c`
- **Rust target branch**: `007-main_root_close_stdout_06-rust-port`
- **Generation date**: `2026-06-06`

## 1. Overview

This module provides process-exit handling for standard output. Its role is to finalize stdout reliably, report write or close failures, and optionally tailor that reporting based on the current output target and broken-pipe handling mode.

The Rust rewrite must preserve this behavior boundary:

- allow the caller to register the current output file name for diagnostics,
- allow the caller to enable or disable special handling for `EPIPE`,
- provide a finalization routine that closes or flushes stdout and reacts to failure in the same functional situations as the C module.

This module is intended for use by the program’s main execution path near termination, after output operations have completed.

## 2. Feature Specification

### 2.1 Configurable output-target context

The module supports storing the name of the output file associated with stdout-related failure reporting.

The Rust version must support:

- setting an optional file name string used as diagnostic context for stdout finalization errors,
- using the most recently configured file name when an error is reported by stdout finalization.

If no file name has been set, behavior must still remain valid; the module must still perform stdout finalization and error handling.

### 2.2 Configurable broken-pipe handling

The module supports configuring whether `EPIPE` should be ignored during stdout finalization.

The Rust version must support:

- setting a boolean mode controlling whether broken-pipe errors on stdout finalization are treated as ignorable,
- applying that mode when stdout finalization encounters an `EPIPE` condition.

When ignore mode is enabled, `EPIPE` during finalization must not be treated as a normal fatal output-close failure. When ignore mode is disabled, `EPIPE` must be handled like other stdout finalization errors.

### 2.3 Stdout finalization and failure handling

The module provides a final routine for stdout shutdown.

The Rust version must support a `close_stdout`-equivalent behavior that:

- finalizes stdout at program end,
- detects failure from that finalization,
- distinguishes the ignorable `EPIPE` case from non-ignorable failure according to configured mode,
- causes process-level error reporting/termination behavior when stdout finalization fails in a non-ignored case.

This module’s functional scope is limited to stdout finalization and related failure policy. It does not define general output writing behavior.

## 3. User Scenarios & Testing

### Scenario 1: Normal program completion with successful stdout finalization

A command writes its output successfully and, before exiting, invokes this module’s stdout finalization routine.

Expected behavior:

- stdout finalization succeeds,
- no error is reported,
- execution can terminate normally.

**Test evidence target**: calling the Rust finalization routine after successful stdout use completes without error signaling.

### Scenario 2: Output file name registered for diagnostics

A command redirects stdout to a named output file and registers that file name with this module before finalization.

Expected behavior:

- if stdout finalization later fails in a non-ignored way, error handling uses the registered file name as the diagnostic target context.

**Test evidence target**: after setting a file name, a simulated non-ignored finalization failure produces failure reporting associated with that configured name.

### Scenario 3: Broken pipe ignored

A command writes to a pipe whose reader closes early. Before finalization, the program enables ignore-`EPIPE` mode and then finalizes stdout.

Expected behavior:

- if stdout finalization encounters `EPIPE`,
- the module treats that condition as ignorable,
- no normal fatal closeout failure path is triggered for that condition.

**Test evidence target**: with ignore mode enabled, a simulated `EPIPE` during stdout finalization is suppressed from fatal handling.

### Scenario 4: Broken pipe not ignored

A command writes to a pipe whose reader closes early, but ignore-`EPIPE` mode is not enabled.

Expected behavior:

- if stdout finalization encounters `EPIPE`,
- the module treats it as a stdout finalization failure,
- error handling follows the non-ignored failure path.

**Test evidence target**: with ignore mode disabled, a simulated `EPIPE` during stdout finalization triggers failure handling.

### Scenario 5: Last configuration wins

A command changes output context or broken-pipe mode before finalizing stdout.

Expected behavior:

- the finalization routine uses the most recent configured file name,
- the finalization routine uses the most recent configured ignore-`EPIPE` setting.

**Test evidence target**: repeated setter calls affect subsequent finalization according to the final values provided.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Output file name configuration
The module shall provide a way to set the file name associated with stdout diagnostics before stdout finalization.
**Traceability**: `close_stdout_set_file_name` in `closeout.c`.

#### FR-2: Broken-pipe ignore configuration
The module shall provide a way to enable or disable ignoring `EPIPE` during stdout finalization.
**Traceability**: `close_stdout_set_ignore_EPIPE` in `closeout.c`.

#### FR-3: Stdout finalization entry point
The module shall provide a finalization routine for stdout intended to be called near program exit.
**Traceability**: `close_stdout` in `closeout.c`.

#### FR-4: Non-ignored finalization failure handling
When stdout finalization fails for a reason that is not configured to be ignored, the module shall perform failure handling rather than silently succeeding.
**Traceability**: `close_stdout` in `closeout.c`.

#### FR-5: Conditional `EPIPE` suppression
When stdout finalization fails specifically with `EPIPE` and ignore mode is enabled, the module shall suppress normal fatal failure handling for that condition.
**Traceability**: `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`.

#### FR-6: Diagnostic context usage
When a file name has been configured and stdout finalization fails in a non-ignored way, the module shall use that configured file name as diagnostic context for the failure.
**Traceability**: `close_stdout_set_file_name`, `close_stdout` in `closeout.c`.

#### FR-7: Most-recent configuration application
The stdout finalization routine shall apply the most recently configured file name and the most recently configured ignore-`EPIPE` mode.
**Traceability**: `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout` in `closeout.c`.

### 4.2 Key Entities

#### Entity: configured output file name
A module-level diagnostic context value representing the file associated with stdout. It is optional and is set before finalization. The stdout finalization routine reads this value if it needs to report a non-ignored failure.
**Traceability**: `close_stdout_set_file_name`, `close_stdout`.

#### Entity: ignore-`EPIPE` mode
A module-level boolean policy value controlling whether broken-pipe errors from stdout finalization are treated as ignorable. It is set before finalization and read by the stdout finalization routine.
**Traceability**: `close_stdout_set_ignore_EPIPE`, `close_stdout`.

#### Entity: stdout finalization operation
The end-of-program action that closes or flushes stdout, checks for failure, and applies diagnostic and `EPIPE` policy.
**Traceability**: `close_stdout`.

## 5. Success Criteria

### SC-1: Supported configuration operations
The Rust module exposes behavior-equivalent configuration operations for output file name and ignore-`EPIPE` mode, and those settings affect subsequent stdout finalization.
**Measured by**: tests covering setter-then-finalize flows.
**Traceability**: `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`.

### SC-2: Correct success-path behavior
When stdout finalization succeeds, the Rust module completes without invoking failure handling.
**Measured by**: success-path test with no reported or propagated finalization failure.
**Traceability**: `close_stdout`.

### SC-3: Correct non-ignored failure behavior
When stdout finalization fails with a non-ignored error, the Rust module follows the module’s failure path and does not silently accept the failure.
**Measured by**: injected finalization-failure test excluding ignored `EPIPE`.
**Traceability**: `close_stdout`.

### SC-4: Correct ignored-`EPIPE` behavior
When stdout finalization fails with `EPIPE` and ignore mode is enabled, the Rust module suppresses normal fatal failure handling for that condition.
**Measured by**: injected `EPIPE` test with ignore mode enabled.
**Traceability**: `close_stdout_set_ignore_EPIPE`, `close_stdout`.

### SC-5: Correct diagnostic-context behavior
When a file name has been configured and stdout finalization fails in a non-ignored way, the Rust module’s failure reporting path uses that file name as diagnostic context.
**Measured by**: failure-path test verifying configured-name usage.
**Traceability**: `close_stdout_set_file_name`, `close_stdout`.

### SC-6: Last-write-wins configuration semantics
If configuration setters are called multiple times before stdout finalization, the Rust module uses the most recent values during the finalization decision path.
**Measured by**: tests that override earlier file name and `EPIPE` settings before finalization.
**Traceability**: `close_stdout_set_file_name`, `close_stdout_set_ignore_EPIPE`, `close_stdout`.