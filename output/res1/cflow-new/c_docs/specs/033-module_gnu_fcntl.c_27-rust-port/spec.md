# spec.md

## Title

Rust Port Functional Specification: `module_gnu_fcntl.c_27`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_fcntl.c_27`
- **Category**: `module_cluster`
- **Source file**: `gnu/fcntl.c`
- **Primary function in scope**: `dupfd`
- **Relevant source type**: `struct stat`
- **Rust branch**: `033-module_gnu_fcntl.c_27-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module provides file-descriptor duplication behavior centered on duplicating an existing descriptor to a descriptor number at or above a caller-specified minimum, with requested descriptor flags applied to the duplicate. The Rust rewrite must preserve the observable behavior of this duplication operation, including success and failure outcomes that are attributable to the source module scope.

The specification is limited to functionality evidenced by the analyzed module scope. No additional APIs, persistence behavior, concurrency guarantees, or extended descriptor-management features are included.

## Feature Specification

### Summary

The Rust module shall implement the behavior represented by the source module’s descriptor-duplication helper:

- accept an existing open file descriptor,
- accept a minimum target descriptor number,
- accept descriptor-related flags for the duplicated descriptor,
- create and return a duplicate descriptor satisfying the minimum-number constraint,
- preserve failure behavior when duplication cannot be completed.

### In-Scope Behavior

The Rust version must implement the following functional behavior evidenced by the source module:

1. **Duplicate an existing descriptor**
   - The module duplicates a valid existing file descriptor.
   - The result refers to the same underlying open file description as the original descriptor, as expected of descriptor duplication behavior.

2. **Honor a minimum descriptor number**
   - The duplicate must be assigned to a descriptor number that is greater than or equal to the requested minimum.
   - The original descriptor is not required to equal the returned descriptor number.

3. **Apply requested duplication flags**
   - The duplication operation accepts flags associated with the newly created descriptor.
   - The Rust implementation must preserve the source module’s externally visible handling of those flags for the duplicate.

4. **Report errors through operation failure**
   - If the source descriptor is invalid, the requested target range is invalid, or the duplication operation otherwise cannot be performed, the operation must fail rather than silently producing an unusable result.
   - The Rust port must preserve failure as an observable outcome for invalid or unsupported requests within this module’s scope.

### Out of Scope

The Rust rewrite must not claim or introduce functionality not evidenced by the module analysis, including:

- new public APIs beyond the module’s duplication behavior,
- thread-safety guarantees,
- serialization or persistence behavior,
- recovery or retry policies,
- cross-process coordination features,
- performance or benchmark commitments.

## User Scenarios & Testing

### Scenario 1: Duplicate a valid descriptor above a minimum

A caller has an already open file descriptor and needs another descriptor referring to the same open file description, but with a descriptor number not lower than a requested minimum.

**Expected behavior**
- The operation succeeds.
- The returned descriptor number is greater than or equal to the requested minimum.
- The returned descriptor is usable as a duplicate of the original descriptor.

**Testing approach**
- Open a file or pipe endpoint.
- Request duplication with a minimum descriptor number above standard descriptors.
- Verify success and returned descriptor number constraint.
- Verify the returned descriptor can be used for ordinary file-descriptor operations appropriate to the underlying object.

### Scenario 2: Duplicate with descriptor flags requested for the new descriptor

A caller needs the duplicate to reflect requested descriptor flag state at creation time.

**Expected behavior**
- The operation succeeds when the request is valid.
- The duplicate reflects the requested flag behavior supported by the source module’s duplication contract.

**Testing approach**
- Duplicate a valid descriptor with a flag request supported by the module contract.
- Inspect the duplicate’s descriptor flags using standard descriptor flag inspection.
- Verify requested flag state is present on the duplicate.

### Scenario 3: Reject an invalid source descriptor

A caller passes a descriptor that is not open or otherwise invalid.

**Expected behavior**
- The operation fails.
- No new valid duplicate descriptor is returned.

**Testing approach**
- Pass a clearly invalid descriptor value.
- Verify the operation reports failure.
- Verify no usable duplicate descriptor is produced.

### Scenario 4: Reject an invalid minimum descriptor request

A caller requests duplication using an invalid minimum descriptor value.

**Expected behavior**
- The operation fails.
- The module does not produce a duplicate descriptor outside the requested contract.

**Testing approach**
- Pass an invalid minimum descriptor value.
- Verify failure is reported.

### Scenario 5: Surface duplication failure from the operating environment

A caller requests duplication under conditions where the operating system refuses the operation, such as resource exhaustion or descriptor limits.

**Expected behavior**
- The operation fails in a way observable to the caller.
- The Rust port does not mask the failure as success.

**Testing approach**
- Exercise the module in a constrained environment or with induced descriptor exhaustion.
- Verify the duplication request fails rather than returning a valid descriptor.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide descriptor duplication behavior for an existing file descriptor.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **FR-2**: The module shall require the duplicate descriptor number to be greater than or equal to a caller-specified minimum descriptor value.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **FR-3**: The module shall accept descriptor flags as part of the duplication request and apply the externally observable flag behavior required for the newly created descriptor.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **FR-4**: The module shall fail the operation when the source descriptor is invalid.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **FR-5**: The module shall fail the operation when the requested minimum descriptor value is invalid for the duplication contract.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **FR-6**: The module shall return the newly created descriptor on success and a failure result on error.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **FR-7**: The module shall preserve the original descriptor as the source of duplication rather than replacing its role with unrelated descriptor creation behavior.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

### Key Entities

- **File descriptor**
  - Integer handle representing an open file, pipe, socket, or similar operating-system-managed resource.
  - Serves as both the source input and the returned duplicate result.
  - Relationship: the duplicate refers to the same underlying open file description as the source descriptor.

- **Minimum descriptor value**
  - Integer lower bound requested by the caller for the duplicate descriptor number.
  - Relationship: constrains the acceptable numeric value of the returned duplicate descriptor.

- **Descriptor flags**
  - Integer flag set supplied with the duplication request.
  - Relationship: affects the properties of the duplicate descriptor at creation time.

- **`struct stat`**
  - Source-level filesystem metadata structure present in the analyzed module scope.
  - Relationship to this module scope is evidentiary only; no independent behavior beyond analyzed scope is specified here.

## Success Criteria

- **SC-1**: For a valid open source descriptor and valid minimum descriptor value, the Rust module returns success with a duplicate descriptor number greater than or equal to the requested minimum.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **SC-2**: For successful duplication requests that include supported descriptor flags, inspection of the returned descriptor confirms the requested externally visible flag state.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **SC-3**: For an invalid source descriptor, the Rust module returns failure and does not yield a usable duplicate descriptor.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **SC-4**: For an invalid minimum descriptor request, the Rust module returns failure.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **SC-5**: When the underlying environment refuses duplication, the Rust module surfaces failure rather than returning a fabricated success result.
  **Traceability**: `gnu/fcntl.c`, `dupfd`

- **SC-6**: The Rust rewrite does not require functionality beyond the source module’s evidenced duplication behavior to satisfy its tests.
  **Traceability**: `gnu/fcntl.c`, `dupfd`, analyzed module scope