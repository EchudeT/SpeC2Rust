# spec.md

## Title

Rust Functional Specification for `module_gnu_fcntl.c_27`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_fcntl.c_27`
- **Category**: `module_cluster`
- **Source file**: `gnu/fcntl.c`
- **Rust branch**: `033-module_gnu_fcntl.c_27-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides file-descriptor duplication behavior centered on duplicating an existing file descriptor onto a descriptor number at or above a caller-specified minimum, with control over descriptor flags at duplication time.

The Rust rewrite must preserve the observable behavior of this module’s descriptor-duplication logic, including successful duplication, validation of inputs, and propagation of failure conditions from the underlying operating system interfaces.

## Scope

### In Scope

- Duplicating an existing open file descriptor.
- Ensuring the returned descriptor number is greater than or equal to a requested minimum descriptor value.
- Applying duplication-time descriptor flag behavior represented by the module’s duplication flags input.
- Reporting success and failure through integer return values and OS-style error outcomes.
- Behavior that depends on file-descriptor state observable through system facilities associated with descriptor metadata.

### Out of Scope

- General file opening, closing, reading, or writing.
- File status flag management unrelated to descriptor duplication.
- New public APIs beyond the functionality evidenced by this module.
- Concurrency guarantees, async behavior, serialization, recovery layers, or performance extensions.

## Feature Specification

### Feature: File Descriptor Duplication With Minimum Target and Flags

The module implements duplication of an existing file descriptor to produce another valid descriptor referring to the same open file description, while respecting a requested lower bound for the new descriptor number and a flags parameter that influences duplication behavior.

The Rust version must implement equivalent functional behavior for:

- Accepting an existing descriptor to duplicate.
- Accepting a requested minimum descriptor number for the duplicate.
- Accepting duplication flags supported by the source module behavior.
- Returning a valid duplicated descriptor on success.
- Failing when the source descriptor is invalid, when the requested descriptor constraints are invalid, or when the operating system cannot satisfy duplication.
- Preserving OS-visible semantics of duplicated descriptors, including the fact that the duplicate refers to the same underlying open file description as the original.
- Preserving descriptor-metadata behavior relevant to duplication outcomes and flag application.

## User Scenarios & Testing

### Scenario 1: Duplicate a valid descriptor above a requested minimum

A caller has an open file descriptor and requests a duplicate with a minimum target value such as `newfd = 10`.

**Expected behavior**
- The operation succeeds if the source descriptor is valid and the system can allocate a duplicate.
- The returned descriptor is greater than or equal to `10`.
- The returned descriptor refers to the same open file description as the original descriptor.

**Test guidance**
- Open a file or pipe.
- Duplicate its descriptor with a minimum value above standard descriptors.
- Verify the returned value is at least the requested minimum.
- Verify shared open-file-description behavior through observable state such as shared file offset where applicable.

### Scenario 2: Duplicate a valid descriptor when the minimum target is already available

A caller requests duplication with a minimum target value that can be used immediately.

**Expected behavior**
- The operation returns a valid descriptor satisfying the minimum bound.
- The result remains a duplicate of the original descriptor rather than an unrelated open handle.

**Test guidance**
- Ensure a candidate descriptor range is free.
- Perform duplication with that lower bound.
- Confirm successful return and duplicate relationship.

### Scenario 3: Reject an invalid source descriptor

A caller passes a descriptor that is not open.

**Expected behavior**
- The operation fails.
- The failure is reported consistently with OS error behavior for invalid descriptors.

**Test guidance**
- Use a clearly invalid descriptor value or a descriptor that has been closed.
- Verify failure and OS-style invalid-descriptor error reporting.

### Scenario 4: Reject an invalid requested minimum descriptor

A caller passes an invalid minimum descriptor value.

**Expected behavior**
- The operation fails.
- The failure is reported consistently with OS error behavior for invalid descriptor arguments.

**Test guidance**
- Call the duplication function with an invalid minimum such as a negative value not accepted by the source behavior.
- Verify failure and corresponding argument-related error behavior.

### Scenario 5: Apply supported duplication-time flags

A caller requests duplication with flags that affect descriptor state at creation time.

**Expected behavior**
- The duplication succeeds when the flags are valid and supported in the same circumstances as the source module.
- The resulting descriptor reflects the requested duplication-time flag effect.

**Test guidance**
- Duplicate with the relevant supported flag setting.
- Inspect descriptor state using OS facilities that expose descriptor flags.
- Verify the requested flag effect is present on the duplicate.

### Scenario 6: Propagate system allocation failure

A caller requests duplication but the system cannot allocate another descriptor.

**Expected behavior**
- The operation fails without inventing replacement behavior.
- The reported error corresponds to the underlying system failure.

**Test guidance**
- Exercise under a constrained descriptor limit or controlled failure environment.
- Verify failure and underlying OS-style error propagation.

## Requirements

### Functional Requirements

#### FR-1: Duplicate from an existing descriptor
The module shall accept an already open file descriptor as the source of duplication and attempt to create another descriptor referring to the same open file description.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### FR-2: Enforce minimum descriptor number
The module shall ensure that any successful duplication result is a descriptor number greater than or equal to the caller-provided minimum target descriptor value.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### FR-3: Support duplication flags input
The module shall accept a flags input that affects duplication behavior and shall apply only the duplication-time behavior evidenced by the source module.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### FR-4: Preserve duplicate-descriptor semantics
On success, the module shall return a descriptor that is a duplicate of the source descriptor, meaning both descriptors refer to the same underlying open file description as observable through normal OS file-descriptor semantics.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### FR-5: Reject invalid source descriptors
The module shall fail when the source descriptor is not valid for duplication.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### FR-6: Reject invalid requested descriptor arguments
The module shall fail when the requested minimum descriptor argument is outside the valid range accepted by the source behavior.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### FR-7: Propagate operating-system failure outcomes
The module shall report failure when underlying descriptor-duplication or related descriptor-state operations fail, without substituting unrelated success behavior.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### FR-8: Use descriptor metadata when required by duplication behavior
The module shall preserve behavior that depends on file-descriptor or file-object metadata observable through system status inspection associated with `struct stat`.

**Traceability**: `gnu/fcntl.c`, `dupfd`, `struct stat`

### Key Entities

#### Entity: Source File Descriptor
An integer descriptor that must already identify an open file description and serves as the object being duplicated.

**Relationships**
- Input to the duplication operation.
- Determines whether duplication can succeed.
- Shares underlying open-file-description state with the returned descriptor on success.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### Entity: Minimum Target Descriptor Value
An integer lower bound constraining the acceptable numeric value of the returned duplicate descriptor.

**Relationships**
- Input to the duplication operation.
- Validated before or during duplication.
- Constrains the success result.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### Entity: Duplication Flags
An integer flags value that controls supported duplication-time descriptor behavior.

**Relationships**
- Input to the duplication operation.
- Affects descriptor state or duplication path when valid.
- May cause failure when invalid or unsupported under source behavior.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### Entity: Duplicated File Descriptor
The integer descriptor returned on success.

**Relationships**
- Produced from the source descriptor.
- Must satisfy the minimum target constraint.
- Refers to the same open file description as the source descriptor.

**Traceability**: `gnu/fcntl.c`, `dupfd`

#### Entity: File Status Metadata
System-provided metadata represented through `struct stat`, used where duplication behavior depends on file-object characteristics.

**Relationships**
- Consulted in support of duplication behavior.
- Provides observable file/object classification data relevant to module decisions.

**Traceability**: `gnu/fcntl.c`, `struct stat`

## Success Criteria

### SC-1: Correct successful duplication
For valid inputs in supported conditions, the Rust module returns a valid file descriptor and that descriptor is greater than or equal to the requested minimum.

**Traceability**: `gnu/fcntl.c`, `dupfd`

### SC-2: Correct duplicate relationship
For successful duplication, observable OS behavior confirms that source and returned descriptors refer to the same open file description.

**Measurable check**
- Shared file offset or equivalent duplicate-descriptor semantics are observable in tests.

**Traceability**: `gnu/fcntl.c`, `dupfd`

### SC-3: Correct invalid-source failure
When called with an invalid source descriptor, the Rust module fails and exposes an OS-consistent invalid-descriptor outcome.

**Traceability**: `gnu/fcntl.c`, `dupfd`

### SC-4: Correct invalid-target failure
When called with an invalid minimum descriptor argument, the Rust module fails and exposes an OS-consistent invalid-argument outcome.

**Traceability**: `gnu/fcntl.c`, `dupfd`

### SC-5: Correct flag application
When called with valid supported duplication flags, the Rust module produces a duplicate whose descriptor state reflects the requested flag effect.

**Traceability**: `gnu/fcntl.c`, `dupfd`

### SC-6: Correct system-error propagation
When the operating system refuses duplication due to resource exhaustion or similar system-level failure, the Rust module fails and preserves the failure outcome rather than masking it.

**Traceability**: `gnu/fcntl.c`, `dupfd`

### SC-7: Metadata-dependent behavior preserved
Any duplication behavior in the source module that depends on status inspection associated with `struct stat` remains behaviorally equivalent in the Rust rewrite for the tested descriptor/object classes covered by the source behavior.

**Traceability**: `gnu/fcntl.c`, `dupfd`, `struct stat`