# spec.md

## Overview

This module provides a single failure-handling routine used when memory allocation cannot be satisfied. The Rust rewrite must preserve the observable behavior of that routine: emit the standard allocation-failure diagnostic through the existing error-reporting path and then terminate execution without returning to the caller.

## Scope

In scope for this module:

- Handling the fatal out-of-memory condition represented by the module function `xalloc_die`.
- Producing the same class of user-visible failure message as the C module.
- Ending control flow immediately after the failure is reported.

Out of scope:

- Performing allocation itself.
- Retrying allocations.
- Recovery from allocation failure.
- Defining broader application error-policy beyond this module’s fatal allocation-failure behavior.

## Feature Specification

### Feature: Fatal handling for allocation failure

The module defines the program behavior for an unrecoverable allocation failure.

The Rust version must implement a functionally equivalent operation that:

- Can be invoked when an allocation helper determines that memory allocation has failed and the program cannot continue.
- Reports an allocation failure using the project’s existing fatal error reporting behavior.
- Does not return to its caller.

### Observable behavior

When triggered, the module must:

1. Emit a diagnostic corresponding to “memory exhausted”.
2. Use the fatal-program-exit path associated with that diagnostic.
3. Terminate execution in a non-returning manner.

No additional functional behavior is evidenced for this module.

## User Scenarios & Testing

### Scenario 1: Allocation helper encounters unrecoverable failure

A caller in the project detects that a requested allocation cannot be completed and delegates failure handling to this module.

Expected result:

- The module reports the standard allocation exhaustion message.
- Program control does not return to the caller.

Test approach:

- Invoke the Rust replacement from a test harness that can observe process termination.
- Verify that execution terminates and that no code after the call runs.

### Scenario 2: User-visible fatal diagnostic on memory exhaustion

The application reaches an out-of-memory path that uses this module.

Expected result:

- The emitted diagnostic indicates memory exhaustion through the same error-reporting channel expected by the project.
- The process exits through the fatal path.

Test approach:

- Capture stderr or the project’s fatal diagnostic output path in an integration-style test.
- Verify presence of the allocation exhaustion message and fatal termination.

### Scenario 3: Non-returning contract

Code using this module relies on the failure handler never returning.

Expected result:

- The Rust implementation has non-returning behavior in practice and in its callable contract.
- Any cleanup or logic placed after a direct call is unreachable during execution.

Test approach:

- Use a controlled subprocess test that calls the function and places a sentinel action immediately after it.
- Verify that the sentinel action is never observed.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide the fatal allocation-failure behavior represented by `xalloc_die`.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.

- **FR-2**: When invoked, the module shall emit a diagnostic indicating memory exhaustion.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.

- **FR-3**: After reporting the allocation failure, the module shall terminate execution and shall not return to its caller.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.

- **FR-4**: The Rust rewrite shall preserve the module’s role as a dedicated handler for unrecoverable allocation failure, without adding recovery behavior or alternate outcomes.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.

### Key Entities

This module has no module-defined persistent data structures or complex entities.

Key functional entity:

- **Fatal allocation-failure handler**: the operation represented by `xalloc_die`, which serves as the module’s only functional boundary and is invoked by other code when memory exhaustion must be treated as fatal.

Relationship:

- Callers detect or conclude allocation failure elsewhere, then transfer control to this module’s handler, which reports the failure and ends execution.

## Success Criteria

- **SC-1**: A call to the Rust implementation of the module’s failure handler results in process termination every time and never returns normally.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.

- **SC-2**: In an integration test, invoking the handler produces a diagnostic corresponding to memory exhaustion on the expected error-reporting path.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.

- **SC-3**: In subprocess-based tests, no statement placed immediately after a direct call to the handler is executed.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.

- **SC-4**: The Rust module exposes no additional evidenced functionality beyond fatal handling of allocation exhaustion.
  **Traceability**: `gnu/xalloc-die.c`, function `xalloc_die`.