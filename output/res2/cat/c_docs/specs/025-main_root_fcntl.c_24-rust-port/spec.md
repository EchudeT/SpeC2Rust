# spec.md

## Title
Rust Functional Specification for `main_root_fcntl.c_24`

## Metadata
- **Project**: `cat`
- **Module**: `main_root_fcntl.c_24`
- **Category**: `main_cluster`
- **Source file**: `fcntl.c`
- **Primary analyzed function**: `dupfd`
- **Rust branch**: `025-main_root_fcntl.c_24-rust-port`
- **Generation date**: `2026-06-07`

## Overview
This module provides file-descriptor duplication behavior within the project’s `fcntl`-related functionality. Its analyzed responsibility is to duplicate an existing file descriptor onto a descriptor number at or above a requested minimum, while honoring duplication mode flags and reporting failure when duplication cannot be performed.

The Rust rewrite must preserve this functional boundary: it must implement the module behavior needed to create a duplicate descriptor from an existing descriptor, using caller-provided source descriptor, minimum target descriptor, and duplication flags, with observable behavior compatible with the original module role.

## Feature Specification

### Summary
The module implements descriptor-duplication support centered on `dupfd`. This behavior accepts:
- an existing file descriptor,
- a requested lower bound for the duplicated descriptor number,
- duplication-related flags.

It returns a file descriptor representing the duplicated open file description when successful, or an error result when duplication is invalid or cannot be completed.

### Required Rust Module Functionality
The Rust version must implement the following functionality evidenced by the analyzed module:

1. **Duplicate an existing descriptor**
   - Given a valid open source descriptor, create a duplicate descriptor referring to the same underlying open file description.

2. **Honor requested minimum descriptor number**
   - The returned descriptor must satisfy the requested lower bound semantics represented by the `newfd` argument.

3. **Apply duplication flags as part of duplication behavior**
   - The duplication operation must account for the provided `flags` argument in the same functional role as the source module.

4. **Reject invalid duplication requests**
   - If the source descriptor is invalid, the requested target bound is invalid, or duplication otherwise cannot proceed, the operation must fail rather than produce a usable duplicated descriptor.

5. **Expose success/failure as an integer-like descriptor result**
   - The observable outcome must distinguish successful duplication from failure in a way compatible with the source module’s role.

## User Scenarios & Testing

### Scenario 1: Duplicate a valid descriptor
A caller has an already-open file descriptor and requests a duplicate with a minimum descriptor value.

**Expected behavior**
- The operation succeeds.
- The returned descriptor is valid and usable.
- The returned descriptor is greater than or equal to the requested minimum.
- The duplicate refers to the same underlying open file description as the original.

**Test guidance**
- Open a file.
- Request duplication with a minimum descriptor value.
- Verify the returned descriptor is valid and `>=` the requested minimum.
- Verify operations through either descriptor affect the same open file description semantics expected for duplicated descriptors.

### Scenario 2: Duplicate with duplication flags
A caller requests duplication and supplies flags that influence duplication behavior.

**Expected behavior**
- The duplication request succeeds or fails according to the validity of the flags and the underlying operating conditions.
- When successful, the resulting descriptor reflects the semantics associated with the provided flags.

**Test guidance**
- Invoke duplication with supported flag values used by the original module context.
- Verify resulting descriptor state matches expected flag-driven behavior.

### Scenario 3: Invalid source descriptor
A caller passes a source descriptor that is not open or otherwise invalid.

**Expected behavior**
- The operation fails.
- No valid duplicate descriptor is returned.

**Test guidance**
- Call duplication with a clearly invalid descriptor value.
- Verify failure is reported.

### Scenario 4: Invalid requested descriptor bound
A caller passes an invalid minimum target descriptor value.

**Expected behavior**
- The operation fails.
- No duplicate descriptor is created.

**Test guidance**
- Call duplication with an invalid lower-bound descriptor number.
- Verify failure is reported.

### Scenario 5: Resource or system limitation during duplication
A caller makes a valid request, but the environment cannot allocate or return a duplicated descriptor.

**Expected behavior**
- The operation fails cleanly.
- No success result is produced.

**Test guidance**
- Exercise the module under constrained descriptor availability or simulated system-call failure.
- Verify failure is surfaced without reporting a valid duplicated descriptor.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide descriptor duplication behavior corresponding to `dupfd` in `fcntl.c`.
- **FR-2**: The module shall accept a source file descriptor as input and treat it as the descriptor to duplicate.
- **FR-3**: The module shall accept a minimum target descriptor value and ensure that any successful duplicated descriptor number is not less than that value.
- **FR-4**: The module shall accept duplication flags and apply them to the duplication operation according to the source module’s behavior.
- **FR-5**: The module shall return a descriptor result representing a successful duplication when duplication can be completed.
- **FR-6**: The module shall fail the operation when the source descriptor is invalid.
- **FR-7**: The module shall fail the operation when the requested minimum target descriptor value is invalid for duplication.
- **FR-8**: The module shall fail the operation when the operating environment cannot complete the duplication request.
- **FR-9**: The module shall not report success unless a valid duplicated descriptor has been obtained.

### Key Entities
- **File descriptor**
  - Integer-valued process resource handle used as both source input and duplication result.
  - Relationship: `dupfd` consumes one source descriptor and produces one duplicated descriptor on success.

- **Minimum target descriptor value**
  - Integer-valued lower bound constraining the descriptor number chosen for the duplicate.
  - Relationship: applied to the duplication request to restrict acceptable result descriptor numbers.

- **Duplication flags**
  - Integer-valued duplication options associated with the duplication request.
  - Relationship: modify duplication behavior for the produced descriptor when the request succeeds.

- **File status information (`struct stat`)**
  - Source-analyzed data structure present in the module.
  - Relationship: module-associated file metadata structure available within the source file context; no broader functionality beyond this presence is asserted from the provided evidence.

## Success Criteria
- **SC-1**: For a valid open source descriptor and valid minimum target value, the Rust module returns success with a valid descriptor number `>=` the requested minimum.
- **SC-2**: For a successful duplication, the returned descriptor refers to the same underlying open file description as the source descriptor.
- **SC-3**: For an invalid source descriptor input, the Rust module reports failure and does not yield a valid duplicated descriptor.
- **SC-4**: For an invalid minimum target descriptor input, the Rust module reports failure and does not yield a valid duplicated descriptor.
- **SC-5**: When duplication flags are provided in a valid request, the Rust module’s successful result reflects the flag-controlled duplication semantics required by the source module behavior.
- **SC-6**: When the operating environment cannot complete duplication, the Rust module reports failure rather than a spurious success value.
- **SC-7**: All behaviors above are traceable to the analyzed source file `fcntl.c` and its duplication function `dupfd`.