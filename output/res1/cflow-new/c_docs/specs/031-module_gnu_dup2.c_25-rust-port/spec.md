# spec.md

## Title

Functional Specification: `module_gnu_dup2.c_25` Rust Port

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_gnu_dup2.c_25`
- **Category**: `module_cluster`
- **Source file**: `gnu/dup2.c`
- **Rust branch**: `031-module_gnu_dup2.c_25-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module provides replacement `dup2` behavior that preserves expected file-descriptor duplication semantics across differing platform conditions handled in the C source. The module’s functional boundary is narrow: it accepts an existing file descriptor and a target descriptor number, performs duplication or no-op behavior as appropriate, and reports success or failure using `dup2`-compatible outcomes.

The Rust rewrite must preserve the observable behavior of the source module, including:
- successful duplication of one descriptor onto another descriptor number,
- correct handling of the case where source and target descriptors are the same,
- propagation of failure for invalid or unsupported duplication cases,
- platform-conditioned handling evidenced by the source functions for Windows-specific behavior and directory-descriptor handling.

## Feature Specification

### Feature Summary

Implement a Rust module that provides the same functional behavior as the source replacement `dup2` logic embodied by `rpl_dup2`, with support paths corresponding to:
- a non-throwing duplication attempt,
- Windows-specific duplication handling,
- special handling for a directory file descriptor case,
- top-level replacement `dup2` behavior that selects and exposes the module’s effective semantics.

### In-Scope Behavior

The Rust version must:
1. Accept a source file descriptor and a desired target file descriptor number.
2. Duplicate the source descriptor onto the target descriptor number when the operation is valid.
3. Leave behavior compatible with `dup2` expectations when source and target descriptors are identical.
4. Return an error outcome when duplication cannot be performed.
5. Preserve source-module special cases evidenced by dedicated helper functions:
   - Windows-specific duplication behavior,
   - directory-descriptor-related handling,
   - a duplication path intended not to introduce throwing behavior at the helper level.

### Out-of-Scope Behavior

The Rust version must not claim or introduce:
- APIs beyond the evidenced replacement `dup2` functionality,
- descriptor management features unrelated to duplication,
- concurrency guarantees,
- serialization, persistence, recovery, or monitoring features,
- performance or benchmark commitments.

## User Scenarios & Testing

### Scenario 1: Duplicate an open descriptor to a different descriptor number

**Description**: A caller has a valid open file descriptor and needs it duplicated onto a specified descriptor number.

**Expected behavior**:
- The operation succeeds.
- The resulting target descriptor refers to the same open file description as the source, consistent with `dup2` behavior.
- The returned result indicates success in a `dup2`-compatible way.

**Test coverage**:
- Open a file or pipe endpoint.
- Duplicate it to a different descriptor number.
- Verify the operation reports success and the target descriptor is usable.

### Scenario 2: Source and target descriptors are the same

**Description**: A caller requests duplication where `fd == desired_fd`.

**Expected behavior**:
- The module preserves `dup2`-compatible semantics for this no-op case.
- Success or failure must match the source module’s intended replacement behavior for this condition.

**Test coverage**:
- Call the Rust replacement with the same valid descriptor for both arguments.
- Verify the outcome matches the source behavior.
- Repeat with an invalid descriptor to verify error behavior remains correct.

### Scenario 3: Invalid source descriptor

**Description**: A caller provides a descriptor that is not open or otherwise invalid.

**Expected behavior**:
- The module reports failure.
- It must not silently succeed.

**Test coverage**:
- Invoke the function with a clearly invalid source descriptor.
- Verify that an error result is returned.

### Scenario 4: Target descriptor cannot be used

**Description**: A caller requests duplication onto a target descriptor number that the underlying platform rejects.

**Expected behavior**:
- The module reports failure consistent with source behavior.

**Test coverage**:
- Exercise a failing target-descriptor case supported by the host test environment.
- Verify the function returns failure rather than partial success.

### Scenario 5: Platform-specific duplication path on Windows

**Description**: On Windows builds, duplication must respect the source module’s dedicated handling path.

**Expected behavior**:
- The Rust port uses behavior equivalent to the Windows-specific functional path in the source.
- Observable success and failure results remain compatible with the source module.

**Test coverage**:
- Build and run on Windows.
- Verify valid duplication succeeds and invalid duplication fails.
- Include the identical-descriptor case if applicable.

### Scenario 6: Directory-descriptor-related handling

**Description**: The source includes a dedicated path associated with directory descriptor handling.

**Expected behavior**:
- The Rust version preserves the source module’s observable behavior for this class of descriptors where the platform and test environment make such descriptors available.

**Test coverage**:
- On a platform where directory descriptors can be opened and duplicated in the same way as in the source environment, open a directory descriptor and invoke the replacement function.
- Verify success or failure matches the source behavior for that case.

## Requirements

### Functional Requirements

#### FR-1: Replacement duplication entry point
The Rust module shall provide functionality equivalent to the source replacement `dup2` entry point (`rpl_dup2`) that accepts a source descriptor and desired target descriptor and returns a `dup2`-compatible success or failure result.

**Traceability**: `gnu/dup2.c`, `rpl_dup2`

#### FR-2: Valid descriptor duplication
When given a valid source descriptor and a valid distinct target descriptor number, the module shall duplicate the source onto the target and report success.

**Traceability**: `gnu/dup2.c`, `rpl_dup2`

#### FR-3: Same-descriptor semantics
When the source descriptor and desired target descriptor are equal, the module shall preserve the source module’s `dup2`-compatible behavior for that condition rather than treating it as a normal cross-descriptor duplication.

**Traceability**: `gnu/dup2.c`, `rpl_dup2`

#### FR-4: Invalid-input failure propagation
When duplication cannot be completed because the source descriptor is invalid or the target descriptor request is rejected, the module shall report failure.

**Traceability**: `gnu/dup2.c`, `rpl_dup2`, `dup2_nothrow`

#### FR-5: Windows-specific behavior preservation
On Windows targets, the module shall preserve the source module’s platform-specific duplication behavior represented by the dedicated Windows helper path.

**Traceability**: `gnu/dup2.c`, `ms_windows_dup2`, `rpl_dup2`

#### FR-6: Directory-descriptor case preservation
Where the source module uses a dedicated directory-descriptor handling path, the Rust port shall preserve the same observable behavior for that class of input.

**Traceability**: `gnu/dup2.c`, `klibc_dup2dirfd`, `rpl_dup2`

#### FR-7: Non-throwing helper-equivalent behavior
The Rust implementation shall preserve the helper-level behavior represented by the non-throwing duplication path, in the sense that duplication attempts through this path produce explicit operation results rather than introducing additional externally visible behavior beyond success or failure.

**Traceability**: `gnu/dup2.c`, `dup2_nothrow`

### Key Entities

#### File descriptor
An integer-like process or runtime handle identifying an open file, pipe, directory, or similar resource. It is the primary input entity for all module behavior.

**Traceability**: `gnu/dup2.c`, all listed functions

#### Desired file descriptor
The requested numeric descriptor slot onto which the source descriptor is to be duplicated. Its relationship to the source descriptor determines whether the operation is a no-op case or a duplication case.

**Traceability**: `gnu/dup2.c`, all listed functions

#### File status metadata
The module references `struct stat`, evidencing that file status information may be consulted as part of determining behavior for certain descriptor classes. The Rust port must preserve only the resulting observable behavior, not the C representation.

**Traceability**: `gnu/dup2.c`, `struct stat`, `klibc_dup2dirfd`, `rpl_dup2`

## Success Criteria

### SC-1: Behavioral compatibility for normal duplication
For valid, distinct descriptors, tests demonstrate that the Rust implementation succeeds in duplicating the source descriptor onto the requested target descriptor in all cases covered by the source module’s main entry behavior.

**Traceability**: `gnu/dup2.c`, `rpl_dup2`

### SC-2: Correct same-descriptor outcome
Tests demonstrate that calls where source and target descriptors are identical produce outcomes matching the source module’s `dup2`-compatible behavior.

**Traceability**: `gnu/dup2.c`, `rpl_dup2`

### SC-3: Correct failure signaling
Tests demonstrate that invalid source descriptors and rejected target-descriptor requests result in failure outcomes rather than silent success.

**Traceability**: `gnu/dup2.c`, `dup2_nothrow`, `rpl_dup2`

### SC-4: Windows path compatibility
On Windows, tests demonstrate that the Rust implementation preserves the source module’s observable duplication behavior for successful and failing cases handled by the Windows-specific path.

**Traceability**: `gnu/dup2.c`, `ms_windows_dup2`, `rpl_dup2`

### SC-5: Directory-descriptor case compatibility
Where the test platform supports the relevant descriptor kind, tests demonstrate that the Rust implementation matches the source module’s observable behavior for the directory-descriptor-related case.

**Traceability**: `gnu/dup2.c`, `klibc_dup2dirfd`, `struct stat`, `rpl_dup2`

### SC-6: No unsupported capability expansion
Review of the Rust module confirms that it exposes only the duplication-related functionality evidenced by the source module and does not add unrelated public behavior.

**Traceability**: `gnu/dup2.c`, module scope