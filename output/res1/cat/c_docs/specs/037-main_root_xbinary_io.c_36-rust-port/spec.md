# spec.md

## Overview

This module provides a single failure-handling behavior related to switching standard I/O streams into binary mode. The analyzed C module exposes one function, `xset_binary_mode_error`, whose role is to terminate execution after reporting the failure condition.

The Rust rewrite must preserve this functional boundary:

- represent the module-level binary-mode setup failure path,
- emit the corresponding fatal error behavior,
- not return to the caller.

This specification is limited to the behavior evidenced by `xbinary-io.c` and does not introduce additional binary-I/O management features beyond the observed fatal error path.

## Feature Specification

### Feature: fatal handling for binary-mode setup failure

The module defines the program behavior used when an attempt to set binary mode has failed and execution must not continue.

The Rust version must implement a module-local or crate-appropriate function with equivalent behavior to:

- signal that setting binary mode failed,
- terminate program flow immediately,
- never return control to the caller.

### Behavioral Notes

- The module is concerned with the error path only, not with performing binary-mode changes itself.
- The behavior is unconditional once invoked: there is no recovery path in this module.
- The failure is process-fatal from the perspective of the caller.

## User Scenarios & Testing

### Scenario 1: binary mode cannot be enabled

A higher-level startup or I/O preparation path attempts to place a stream or file descriptor into binary mode. That operation fails. The program invokes this module’s failure handler.

Expected result:

- the failure handler reports the error in the program’s standard fatal-error manner,
- execution stops,
- no caller-visible return occurs.

### Scenario 2: caller relies on non-returning fatal path

A caller uses this function in a branch that should be impossible to continue from after a binary-mode setup failure.

Expected result:

- control flow is terminated at the call site,
- any code after the call is unreachable in normal execution.

### Scenario 3: Rust port integration in command startup path

The Rust port integrates this module into the same logical place where the C program would abort on binary-mode setup failure.

Expected result:

- when the binary-mode setup failure path is exercised, the Rust program exhibits the same fatal outcome,
- no substitute recovery behavior is introduced by the port.

### Suggested Tests

1. **Fatal termination test**
   - Invoke the Rust equivalent of the failure handler in an isolated subprocess.
   - Verify that the subprocess terminates abnormally or with the project’s defined fatal-exit behavior.
   - Verify that no normal return value is observed.

2. **Unreachable continuation test**
   - Place an observable action after the call in a test harness subprocess.
   - Verify that the observable action does not occur.

3. **Failure-message-path test**
   - If the surrounding project defines a standard fatal diagnostic channel, verify that invoking the handler emits output through that channel before termination.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide the binary-mode failure handling behavior represented by `xset_binary_mode_error`.
  **Traceability**: `xbinary-io.c`, `xset_binary_mode_error`.

- **FR-2**: Invoking the module’s failure handler shall terminate control flow and shall not return to its caller.
  **Traceability**: `xset_binary_mode_error` is declared `_Noreturn`.

- **FR-3**: The module shall treat the condition as a fatal error related to failure to set binary mode.
  **Traceability**: `xset_binary_mode_error` name and module file `xbinary-io.c`.

- **FR-4**: The Rust rewrite shall preserve the module’s limited scope as an error-reporting/termination path and shall not require this module to perform binary-mode configuration itself.
  **Traceability**: only the fatal helper function is evidenced in `xbinary-io.c`.

### Key Entities

- **Fatal binary-mode error handler**
  - The module’s only evidenced entity is a non-returning function that represents the program’s response to binary-mode setup failure.
  - Relationship: it is invoked by higher-level code that attempted binary-mode setup; this module itself does not define the setup operation in the provided analysis.

## Success Criteria

- **SC-1**: The Rust module exposes behavior equivalent to the C module’s single evidenced responsibility: handling binary-mode setup failure fatally.
  **Traceability**: `xbinary-io.c`, `xset_binary_mode_error`.

- **SC-2**: When the Rust failure handler is invoked in a subprocess test, execution does not return past the call site.
  **Traceability**: `_Noreturn` contract of `xset_binary_mode_error`.

- **SC-3**: No tested integration path depends on this module to configure binary mode; it is used only for the fatal error path evidenced by the source analysis.
  **Traceability**: absence of any other evidenced function in `xbinary-io.c`.

- **SC-4**: The Rust port does not add alternate recovery, retry, or continuation behavior to this module.
  **Traceability**: unconditional fatal role of `xset_binary_mode_error`.