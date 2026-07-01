# spec.md

## Title

Rust Functional Specification for `module_gnu_fcntl.c_27`

## Overview

This module provides file-descriptor duplication behavior associated with GNU `fcntl` support, centered on duplicating an existing open file descriptor onto or above a requested descriptor number while honoring descriptor flag behavior requested by the caller.

The Rust rewrite must preserve the observable behavior of the C module’s duplication logic as evidenced by `dupfd` in `gnu/fcntl.c`. The scope of this specification is limited to that behavior and the directly related file-status inspection needed to determine whether descriptors are valid and open.

## Scope

In scope:

- Duplicating an existing file descriptor.
- Selecting a duplicated descriptor at or above a caller-supplied minimum descriptor number.
- Applying requested duplication-time descriptor flag behavior.
- Detecting and reporting failure when the source descriptor is invalid, the requested target range is invalid, or duplication cannot be completed.
- Using file-descriptor status inspection as needed to distinguish valid versus invalid descriptors.

Out of scope:

- Any unrelated `fcntl` operations not evidenced by this module input.
- New public APIs beyond what is needed to preserve this module’s existing functional role.
- Additional platform features or guarantees not evidenced by the source module.

## Feature Specification

### Feature: File descriptor duplication with minimum target selection

The module must provide the behavior of duplicating an already open file descriptor (`oldfd`) to a new descriptor whose numeric value is at least `newfd`.

The Rust version must implement:

- Validation of the source descriptor as an open descriptor.
- Validation of the requested minimum target descriptor.
- Creation of a new descriptor referring to the same open file description as the source.
- Selection of a returned descriptor number that is greater than or equal to the requested minimum.
- Failure signaling when duplication cannot be performed.

### Feature: Duplication-time descriptor flag handling

The module must support duplication behavior that is sensitive to caller-provided flags.

The Rust version must implement the observable effect of requested duplication flags on the duplicated descriptor, specifically preserving the distinction between ordinary duplication and duplication that requests close-on-exec behavior when such behavior is requested by the caller.

### Feature: Error-preserving failure behavior

When duplication cannot be completed, the Rust version must fail in the same functional cases covered by the C module:

- invalid source descriptor
- invalid requested minimum descriptor
- unsupported or invalid duplication flag combination
- system-level failure to allocate or create the duplicate descriptor

The Rust version must preserve failure as an explicit outcome and must not silently substitute success, fallback behavior, or unrelated descriptor operations.

## User Scenarios & Testing

### Scenario 1: Duplicate an open descriptor above a minimum value

A caller has an open descriptor and needs another descriptor that refers to the same open file description, but the resulting descriptor number must not be below a chosen minimum.

Expected behavior:

- The duplication succeeds.
- The returned descriptor is greater than or equal to the requested minimum.
- The returned descriptor is distinct from the source descriptor.
- The duplicate refers to the same underlying open file description.

Testing guidance:

- Open a file or pipe endpoint.
- Request duplication with a minimum below the next available descriptor and with a minimum above several currently used descriptors.
- Verify the returned descriptor satisfies the minimum and is usable for I/O equivalently to the source.

### Scenario 2: Duplicate with close-on-exec requested

A caller needs a duplicated descriptor that is marked close-on-exec as part of the duplication request.

Expected behavior:

- The duplication succeeds if the environment supports the requested operation.
- The returned descriptor has close-on-exec enabled.
- The source descriptor’s state is not altered solely by duplicating it.

Testing guidance:

- Duplicate an open descriptor with the flag combination corresponding to close-on-exec duplication.
- Inspect descriptor flags on the result.
- Confirm the source descriptor remains open and unchanged in descriptor flag state unless separately modified.

### Scenario 3: Reject an invalid source descriptor

A caller passes a descriptor that is not open or otherwise invalid.

Expected behavior:

- The duplication fails.
- No duplicate descriptor is created.

Testing guidance:

- Use a negative descriptor or a descriptor known to be closed.
- Verify failure is returned.
- Verify no new usable descriptor appears.

### Scenario 4: Reject an invalid minimum target descriptor

A caller requests duplication with an invalid minimum descriptor value.

Expected behavior:

- The duplication fails.
- The failure is reported without creating a duplicate.

Testing guidance:

- Pass a negative requested minimum.
- Verify failure and absence of a created duplicate descriptor.

### Scenario 5: Handle descriptor allocation failure

A caller requests duplication when the process cannot obtain another descriptor.

Expected behavior:

- The duplication fails explicitly.
- The source descriptor remains usable after failure.

Testing guidance:

- Exhaust available descriptors in a controlled test environment, then request duplication.
- Verify failure.
- Confirm the original descriptor still works.

## Requirements

### Functional Requirements

- **FR-1**: The module shall duplicate an existing open file descriptor and produce a new descriptor referring to the same open file description.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **FR-2**: The module shall ensure the duplicated descriptor number is greater than or equal to the caller-supplied minimum descriptor number.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **FR-3**: The module shall reject duplication requests whose source descriptor is invalid or not open.
  **Traceability:** `gnu/fcntl.c`, `dupfd`; related status inspection via `struct stat`

- **FR-4**: The module shall reject duplication requests whose requested minimum descriptor number is invalid.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **FR-5**: The module shall honor caller-requested duplication flags that affect the duplicated descriptor’s close-on-exec state.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **FR-6**: The module shall fail explicitly when duplication cannot be completed due to invalid flags or operating-system duplication failure.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **FR-7**: The module shall not close, replace, or otherwise invalidate the source descriptor as part of successful or failed duplication attempts.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

### Key Entities

- **Source file descriptor (`oldfd`)**: The existing descriptor that must already designate an open file description before duplication can succeed.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **Minimum target descriptor (`newfd`)**: The lowest acceptable numeric value for the duplicate descriptor. It constrains descriptor selection but is not itself required to be open.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **Duplication flags (`flags`)**: Caller-provided control values that determine permitted duplication mode and whether close-on-exec behavior is requested for the duplicate.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **File status record (`struct stat`)**: Status information used in descriptor validity/open-state checks associated with duplication behavior.
  **Traceability:** `gnu/fcntl.c`, `struct stat`

- **Duplicated file descriptor**: The result of a successful operation; it is a distinct descriptor number that references the same underlying open file description as the source descriptor and may carry requested descriptor flags.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

## Success Criteria

- **SC-1**: For a valid open source descriptor and a valid minimum target, duplication succeeds and returns a descriptor number greater than or equal to the requested minimum.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **SC-2**: The descriptor returned by a successful duplication is distinct from the source descriptor and supports equivalent access to the same underlying open file description.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **SC-3**: When close-on-exec duplication is requested through supported flags, the duplicated descriptor is created with close-on-exec enabled.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **SC-4**: When the source descriptor is invalid or closed, the operation fails and does not create a usable duplicate descriptor.
  **Traceability:** `gnu/fcntl.c`, `dupfd`; related status inspection via `struct stat`

- **SC-5**: When the requested minimum descriptor is invalid, the operation fails without creating a duplicate descriptor.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **SC-6**: When duplication fails due to system resource limits or OS-reported duplication failure, the failure is surfaced explicitly and the original descriptor remains usable.
  **Traceability:** `gnu/fcntl.c`, `dupfd`

- **SC-7**: No supported scenario in this specification requires behavior beyond descriptor duplication, minimum-target selection, validity checking, and duplication-time flag handling.
  **Traceability:** `gnu/fcntl.c`, `dupfd`