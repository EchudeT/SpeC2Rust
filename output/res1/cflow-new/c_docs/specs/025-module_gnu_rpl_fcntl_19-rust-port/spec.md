# spec.md

## Title

Rust Port Functional Specification: `module_gnu_rpl_fcntl_19`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_rpl_fcntl_19`
- **Category**: `module_cluster`
- **Source files analyzed**: `gnu/fcntl.c`
- **Rust branch**: `025-module_gnu_rpl_fcntl_19-rust-port`
- **Generation date**: 2026-06-11

## Overview

This module provides replacement behavior for specific `fcntl`-style file descriptor duplication operations. Its functional scope is limited to duplicating an existing file descriptor into a new descriptor at or above a requested minimum value, with distinct handling for:

- duplication equivalent to `F_DUPFD`
- duplication equivalent to `F_DUPFD_CLOEXEC`

The Rust rewrite must preserve the observable behavior of these replacement operations, including validation of inputs, duplication semantics, and close-on-exec behavior for the `CLOEXEC` variant.

## Feature Specification

### Functional Scope

The Rust version must implement the module behavior corresponding to two replacement duplication operations:

1. **Duplicate a file descriptor at or above a requested target descriptor number**
   - Accept an input file descriptor and a minimum target descriptor number.
   - Return a new valid descriptor referring to the same open file description as the source descriptor.
   - Ensure the returned descriptor number is greater than or equal to the requested target.

2. **Duplicate a file descriptor at or above a requested target descriptor number with close-on-exec set**
   - Ensure the returned descriptor has the close-on-exec flag set on success.

### Behavioral Boundaries

The Rust version must remain within the behavior evidenced by the analyzed module:

- It is a descriptor-duplication compatibility layer, not a general-purpose `fcntl` implementation.
- It operates on existing file descriptors and descriptor flags relevant to duplication.
- It may rely on file status inspection as part of determining valid descriptor behavior, as evidenced by use of `struct stat`, but must not expose additional filesystem features beyond what is necessary for these duplication operations.

## User Scenarios & Testing

### Scenario 1: Duplicate an open descriptor above a minimum number

A caller has an open file descriptor and needs a duplicate descriptor whose numeric value is at least a requested minimum.

**Expected behavior**
- The operation succeeds when the source descriptor is valid and duplication is possible.
- The returned descriptor is not lower than the requested target.
- The returned descriptor refers to the same underlying open file description as the source.

**Test coverage**
- Open a file or pipe endpoint.
- Request duplication with a target larger than standard descriptors.
- Verify returned descriptor is `>= target`.
- Verify I/O or file offset behavior is shared in the way expected for duplicated descriptors.

### Scenario 2: Duplicate an open descriptor with close-on-exec enabled

A caller needs a duplicate descriptor that should not remain open across an exec boundary.

**Expected behavior**
- The operation succeeds when the source descriptor is valid and duplication is possible.
- The returned descriptor is not lower than the requested target.
- The returned descriptor has close-on-exec set.

**Test coverage**
- Duplicate a valid descriptor using the `CLOEXEC` variant.
- Inspect descriptor flags.
- Verify `FD_CLOEXEC` is set on the returned descriptor.

### Scenario 3: Reject an invalid source descriptor

A caller accidentally passes a descriptor that is not open.

**Expected behavior**
- The operation fails rather than creating a new descriptor.
- The failure is reported consistently with system duplication failure behavior.

**Test coverage**
- Pass a negative descriptor and a closed descriptor.
- Verify the operation returns failure.
- Verify no new descriptor is created.

### Scenario 4: Reject an invalid target descriptor floor

A caller provides an invalid minimum target value.

**Expected behavior**
- The operation fails when the requested target descriptor number is invalid for duplication.

**Test coverage**
- Pass a negative target value.
- Verify failure is returned.

### Scenario 5: Preserve distinction between plain duplication and cloexec duplication

A caller depends on whether the duplicate inherits or explicitly sets close-on-exec.

**Expected behavior**
- Plain duplication does not claim `CLOEXEC` behavior.
- `CLOEXEC` duplication does set close-on-exec on the returned descriptor.

**Test coverage**
- Duplicate the same source descriptor with both operations.
- Compare the returned descriptor flags.
- Verify the distinction is observable.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide behavior equivalent to duplicating a valid file descriptor into a new descriptor number greater than or equal to a caller-supplied minimum target.
  **Traceability**: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`

- **FR-2**: The module shall fail the duplication operation when the source file descriptor is invalid or not open.
  **Traceability**: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

- **FR-3**: The module shall fail the duplication operation when the minimum target descriptor value is invalid for descriptor duplication.
  **Traceability**: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

- **FR-4**: On successful plain duplication, the module shall return a new descriptor that refers to the same open file description as the source descriptor.
  **Traceability**: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`

- **FR-5**: The module shall provide behavior equivalent to duplicating a valid file descriptor into a new descriptor number greater than or equal to a caller-supplied minimum target, with close-on-exec enabled on the returned descriptor.
  **Traceability**: `gnu/fcntl.c`, `rpl_fcntl_DUPFD_CLOEXEC`

- **FR-6**: On successful `CLOEXEC` duplication, the returned descriptor shall have the close-on-exec flag set.
  **Traceability**: `gnu/fcntl.c`, `rpl_fcntl_DUPFD_CLOEXEC`

- **FR-7**: The module shall preserve the behavioral distinction between plain duplication and `CLOEXEC` duplication.
  **Traceability**: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

- **FR-8**: Where descriptor validity or file object state must be checked as part of performing the replacement behavior, the module shall use file status information only to support these duplication semantics.
  **Traceability**: `gnu/fcntl.c`, `struct stat`

### Key Entities

- **File descriptor**
  - Integer handle representing an open file, pipe endpoint, or similar kernel-managed open object.
  - Acts as the source input and as the returned duplicated handle.
  - Central relationship: one source descriptor may produce one new duplicated descriptor that references the same open file description.

- **Minimum target descriptor number**
  - Integer lower bound that constrains the numeric value of the duplicated descriptor.
  - Relationship: applied to both duplication variants as a floor on the returned descriptor number.

- **Duplicated descriptor**
  - Newly created file descriptor returned by the operation on success.
  - Relationship to source descriptor: shares the same underlying open file description.
  - Additional relationship for the `CLOEXEC` variant: must carry the close-on-exec descriptor flag.

- **File status record (`struct stat`)**
  - File status structure evidenced by the module analysis.
  - Relationship: supports validation or classification needed by the replacement duplication behavior, without creating any separate public data model for the module.

## Success Criteria

- **SC-1**: For a valid open source descriptor and valid target floor, plain duplication returns success and a descriptor number `>=` the requested target.
  **Traceability**: `rpl_fcntl_DUPFD`

- **SC-2**: For a valid open source descriptor and valid target floor, `CLOEXEC` duplication returns success and a descriptor number `>=` the requested target.
  **Traceability**: `rpl_fcntl_DUPFD_CLOEXEC`

- **SC-3**: In both duplication variants, the returned descriptor refers to the same open file description as the source, demonstrable through shared offset or equivalent duplicated-descriptor behavior.
  **Traceability**: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

- **SC-4**: The `CLOEXEC` duplication variant produces a returned descriptor with close-on-exec set, and this is observable through descriptor flag inspection.
  **Traceability**: `rpl_fcntl_DUPFD_CLOEXEC`

- **SC-5**: Passing an invalid or closed source descriptor causes the operation to fail in both variants.
  **Traceability**: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

- **SC-6**: Passing an invalid minimum target descriptor value causes the operation to fail in both variants.
  **Traceability**: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

- **SC-7**: Tests demonstrate an observable difference between plain duplication and `CLOEXEC` duplication with respect to the close-on-exec flag.
  **Traceability**: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

## Out of Scope

The Rust port specification does not require any behavior beyond the evidenced module scope, including:

- a full replacement for all `fcntl` commands
- new public APIs unrelated to descriptor duplication
- guarantees about thread-safety, async behavior, serialization, or recovery
- benchmark targets or performance claims
- filesystem metadata APIs beyond what is needed to support the replacement duplication behavior