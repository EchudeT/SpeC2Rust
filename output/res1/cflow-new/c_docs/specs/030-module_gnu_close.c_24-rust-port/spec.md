# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_gnu_close.c_24`
- **Category**: `module_cluster`
- **Source evidence**: `gnu/close.c`
- **Primary behaviors evidenced**:
  - `close_nothrow(int fd)` closes a file descriptor while preserving the caller-visible `errno` side effects expected by the wrapper logic.
  - `rpl_close(int fd)` provides the module’s exported close behavior and handles the documented wrapper semantics around descriptor closure.

## Feature Specification

### Summary

This module provides a close-operation wrapper for file descriptors. The Rust rewrite must preserve the module’s observable behavior when asked to close a file descriptor, including its handling of success and failure results and the wrapper-specific treatment of error reporting.

### Functional Scope

The Rust version must implement:

- A module-level close operation corresponding to `rpl_close(int fd)`.
- Internal handling equivalent to the helper behavior of `close_nothrow(int fd)` where needed to preserve the exported function’s observable semantics.
- Close behavior for integer file descriptors only; no additional resource-management capabilities are evidenced.

### In-Scope Behavior

The Rust rewrite must:

- Accept an integer file descriptor as input to the exported close operation.
- Attempt to close that descriptor.
- Return a success/failure result consistent with the C module’s close wrapper behavior.
- Preserve the wrapper’s intended error-handling semantics when the underlying close attempt fails.
- Avoid introducing additional public behavior beyond the module’s close wrapper responsibility.

### Out of Scope

The Rust rewrite must not add or require:

- New public APIs beyond the close-wrapper functionality evidenced here.
- Buffered I/O abstractions, stream management, or ownership frameworks not present in the source evidence.
- Thread-safety guarantees, async behavior, retries, or recovery behavior not evidenced by `gnu/close.c`.

## User Scenarios & Testing

### Scenario 1: Successful descriptor close

A caller has a valid open file descriptor and invokes the module’s exported close function.

**Expected outcome**:
- The module attempts to close the descriptor.
- The operation reports success in the Rust-facing form chosen for the port.
- No extra behavior is performed beyond the close operation and its required wrapper semantics.

**Test coverage**:
- Open a file or pipe to obtain a valid descriptor.
- Call the Rust port’s exported close wrapper.
- Verify the wrapper reports success.
- Verify the descriptor is no longer usable according to OS behavior.

### Scenario 2: Close failure is surfaced

A caller invokes the module’s exported close function with a descriptor value that cannot be successfully closed, such as an invalid or already-closed descriptor.

**Expected outcome**:
- The module reports failure.
- The failure reflects the underlying close failure in a way consistent with the C module’s wrapper behavior.

**Test coverage**:
- Call the Rust port with an invalid descriptor.
- Verify failure is returned.
- Verify the error state exposed by the Rust port matches the wrapper’s intended observable semantics.

### Scenario 3: Wrapper preserves its documented error-handling behavior

A caller depends on the wrapper’s specific treatment of close-related errors rather than invoking the raw system close directly.

**Expected outcome**:
- The exported Rust behavior matches the C wrapper’s observable result contract, including the semantics supported through the helper behavior evidenced by `close_nothrow`.

**Test coverage**:
- Exercise failure paths that trigger the wrapper logic.
- Compare success/failure and exposed error behavior against the original C module on the same platform assumptions used by the port.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide an exported close operation corresponding to `rpl_close` from `gnu/close.c`.
  - **Traceability**: `gnu/close.c`, `rpl_close`

- **FR-2**: The exported close operation shall accept a file descriptor identifier as input and attempt to close that descriptor.
  - **Traceability**: `gnu/close.c`, `rpl_close(int fd)`

- **FR-3**: The exported close operation shall report whether the close attempt succeeded or failed, preserving the original module’s observable result semantics.

- **FR-4**: The Rust implementation shall preserve the wrapper-specific error-handling behavior supported by the internal helper corresponding to `close_nothrow`.
  - **Traceability**: `gnu/close.c`, `close_nothrow`, `rpl_close`

- **FR-5**: On close failure, the module shall expose failure information consistent with the C module’s wrapper behavior rather than inventing alternate behavior.
  - **Traceability**: `gnu/close.c`, `rpl_close`, `close_nothrow`

- **FR-6**: The module shall remain limited to file-descriptor close behavior and shall not introduce unrelated resource lifecycle features.
  - **Traceability**: scope evidenced only by `gnu/close.c`, `rpl_close`, `close_nothrow`

### Key Entities

- **File descriptor (`int fd`)**
  - The sole input entity used by the module.
  - Represents the OS-level descriptor targeted by the close wrapper.
  - **Traceability**: `close_nothrow(int fd)`, `rpl_close(int fd)`

- **Close result**
  - The success/failure outcome of the close attempt as observed through the module’s exported behavior.
  - In the Rust rewrite, this may be represented idiomatically, but it must preserve the C module’s externally observable semantics.
  - **Traceability**: `rpl_close`

- **Error state**
  - The failure information associated with an unsuccessful close attempt.
  - Its externally observable behavior must match the wrapper semantics enforced by the original module.
  - **Traceability**: `close_nothrow`, `rpl_close`

## Success Criteria

- **SC-1**: For a valid open descriptor, the Rust exported close wrapper completes with a success result in tests that mirror `rpl_close` usage.
  - **Traceability**: `gnu/close.c`, `rpl_close`

- **SC-2**: For an invalid or already-closed descriptor, the Rust exported close wrapper returns failure rather than reporting success.

- **SC-3**: Failure-path tests confirm that the Rust port preserves the same observable wrapper error semantics as the original behavior supported by `close_nothrow` and used by `rpl_close`.
  - **Traceability**: `gnu/close.c`, `close_nothrow`, `rpl_close`

- **SC-4**: The Rust module exposes no extra public functionality beyond the close wrapper behavior evidenced by this source module.
  - **Traceability**: `gnu/close.c`

- **SC-5**: Scenario-based tests for successful close, failed close, and wrapper-specific error handling all pass on the target branch `030-module_gnu_close.c_24-rust-port`.