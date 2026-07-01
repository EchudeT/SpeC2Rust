# spec.md

## Title
Rust Functional Specification for `module_gnu_dup2.c_25`

## Document Metadata
- Project: `cflow-new`
- Module: `module_gnu_dup2.c_25`
- Category: `module_cluster`
- Source scope: `gnu/dup2.c`
- Rust branch: `031-module_gnu_dup2.c_25-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides a replacement `dup2`-style operation that duplicates one open file descriptor onto a requested descriptor number while preserving the source module’s observable compatibility behavior across supported environments.

The Rust rewrite must implement the module’s effective behavior as defined by the source functions in `gnu/dup2.c`, including:
- a replacement public duplication operation,
- handling of the case where source and destination descriptors are equal,
- preservation of non-throwing behavior for the core duplication path,
- environment-specific compatibility handling for Windows-style descriptor duplication,
- compatibility handling for directory-descriptor cases identified through file status inspection.

This module’s purpose is functional compatibility for descriptor duplication, not expansion of the descriptor API surface.

## Scope
In scope:
- Replacement behavior corresponding to `rpl_dup2`.
- Correct duplication semantics for valid and invalid descriptor combinations.
- Compatibility behavior evidenced by helper paths in the source module.
- Use of file status inspection where required to preserve compatibility behavior.

Out of scope:
- Any API beyond the replacement `dup2` behavior evidenced in this module.
- New descriptor-management features.
- Guarantees not evidenced by the source module, including thread-safety contracts or recovery systems.

## Feature Specification

### Feature Summary
The Rust module must provide a `dup2`-equivalent replacement operation that:
1. Duplicates an open descriptor `fd` onto `desired_fd`.
2. Returns the destination descriptor on success.
3. Fails with error signaling compatible with the source module’s behavior when duplication is not permitted or inputs are invalid.
4. Correctly handles the special case where `fd == desired_fd`.
5. Preserves compatibility behavior for platform-specific cases addressed by the source module, including Windows-specific duplication handling and directory-descriptor handling.

### Functional Behavior
The replacement operation must satisfy the following behavior boundaries evidenced by `gnu/dup2.c`:

- When asked to duplicate a descriptor onto a different descriptor number, the module must perform a duplication operation with `dup2` semantics.
- When the source and destination descriptor numbers are the same, the module must not treat that case as a normal cross-descriptor duplication; it must instead preserve the compatibility behavior expected by the replacement implementation, including validation of descriptor usability.
- The module must support a non-throwing internal duplication path corresponding to the source module’s helper behavior.
- On Windows-targeted behavior paths, the module must preserve the source module’s replacement semantics for descriptor duplication rather than relying solely on a naïve direct system call mapping.
- For compatibility cases involving directory descriptors, the module must preserve the source module’s behavior that depends on file status classification.

### Platform/Environment Compatibility
The source module contains distinct compatibility paths. The Rust rewrite must preserve the functional effect of these paths where applicable to the target build/platform configuration:
- A generic non-throwing duplication behavior.
- A Windows-specific compatibility duplication behavior.
- A directory-descriptor compatibility path that uses file status information.

No additional platform abstractions are required beyond these evidenced behaviors.

## User Scenarios & Testing

### Scenario 1: Duplicate one valid descriptor onto another descriptor number
A caller has an open descriptor `fd` and requests duplication onto `desired_fd`, where `fd != desired_fd`.

Expected behavior:
- The operation succeeds when the source descriptor is valid and the destination can be used as a duplication target.
- The returned value is `desired_fd`.
- After success, `desired_fd` refers to the same open file description semantics expected from `dup2`.

Test coverage:
- Open a file, duplicate its descriptor to an unused target descriptor, and verify success return value.
- Confirm that I/O through the duplicated descriptor is valid.

### Scenario 2: Source and destination descriptors are the same
A caller invokes the replacement operation with `fd == desired_fd`.

Expected behavior:
- The operation does not create a second descriptor.
- If the descriptor is valid, the operation succeeds and returns that descriptor number.
- If the descriptor is invalid, the operation fails consistently with replacement `dup2` semantics.

Test coverage:
- Call the operation with a valid open descriptor as both arguments and verify success with the same returned value.
- Call the operation with an invalid descriptor number as both arguments and verify failure.

### Scenario 3: Invalid source descriptor
A caller requests duplication from a descriptor that is not open.

Expected behavior:
- The operation fails.
- No successful destination duplication is reported.

Test coverage:
- Invoke the operation using a known invalid source descriptor and verify failure.
- Verify that a destination descriptor number is not left as a successful duplicate by the module.

### Scenario 4: Replace an existing destination descriptor
A caller duplicates a valid source descriptor onto a destination descriptor that is already open.

Expected behavior:
- The destination becomes the duplicate target according to `dup2` semantics.
- The operation returns the destination descriptor on success.

Test coverage:
- Open two descriptors, duplicate the first onto the second’s descriptor number, and verify success.
- Confirm that the resulting destination behaves as the duplicated source.

### Scenario 5: Windows compatibility path
A caller runs on a build/target configuration requiring the Windows-specific compatibility behavior evidenced by the module.

Expected behavior:
- Descriptor duplication follows the replacement module’s compatibility semantics for that environment.
- The same same-descriptor and invalid-descriptor rules continue to hold.

Test coverage:
- Platform-targeted tests on the relevant configuration covering successful duplication, same-descriptor handling, and invalid-source failure.

### Scenario 6: Directory-descriptor compatibility case
A caller duplicates a descriptor associated with a directory in an environment where the source module’s directory-specific compatibility path applies.

Expected behavior:
- The duplication behavior matches the source module’s compatibility handling for directory descriptors.
- File status classification is used to distinguish this case where required by the source behavior.

Test coverage:
- On a configuration where this path is relevant, open a directory descriptor, invoke the replacement operation, and verify behavior matches module expectations for success or failure as defined by the source behavior.

## Requirements

### Functional Requirements

#### FR-1: Replacement duplication operation
The Rust module shall provide the functional equivalent of the source module’s replacement `dup2` operation, accepting a source descriptor and a desired destination descriptor and producing `dup2`-compatible success or failure behavior.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`

#### FR-2: Success return behavior
On successful duplication, the module shall report success using the destination descriptor value.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`
- helper duplication paths in `gnu/dup2.c`

#### FR-3: Same-descriptor handling
When the source descriptor and desired destination descriptor are equal, the module shall preserve the source module’s special-case behavior, including success for a valid descriptor and failure for an invalid descriptor rather than attempting a normal duplicate-to-different-target operation.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`
- `dup2_nothrow` in `gnu/dup2.c:50-58`

#### FR-4: Invalid descriptor failure
When the source descriptor is invalid, the module shall fail rather than reporting a successful duplication.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`
- `dup2_nothrow` in `gnu/dup2.c:50-58`

#### FR-5: Non-throwing core duplication path
The module shall preserve the source module’s non-throwing helper behavior for the underlying duplication attempt used by the replacement logic.

Traceability:
- `dup2_nothrow` in `gnu/dup2.c:50-58`
- `rpl_dup2` in `gnu/dup2.c:161-191`

#### FR-6: Windows compatibility behavior
For target environments where the source module uses a Windows-specific compatibility path, the Rust module shall preserve that path’s effective descriptor-duplication behavior.

Traceability:
- `ms_windows_dup2` in `gnu/dup2.c:72-104`
- `rpl_dup2` in `gnu/dup2.c:161-191`

#### FR-7: Directory-descriptor compatibility behavior
For target environments where the source module uses a directory-descriptor compatibility path, the Rust module shall preserve that path’s effective behavior when duplicating descriptors associated with directories.

Traceability:
- `klibc_dup2dirfd` in `gnu/dup2.c:112-138`
- `struct stat` use at `gnu/dup2.c:144`
- `rpl_dup2` in `gnu/dup2.c:161-191`

#### FR-8: File-status-based classification for directory cases
Where required by the source behavior, the module shall inspect file status information to determine whether the source descriptor is associated with a directory for compatibility handling.

Traceability:
- `klibc_dup2dirfd` in `gnu/dup2.c:112-138`
- `struct stat` at `gnu/dup2.c:144`

### Key Entities

#### Entity: File Descriptor
An integer handle representing an open file-like resource and serving as both input and output identity for duplication behavior.

Relationships:
- Provided as `fd` and `desired_fd` to the replacement duplication operation.
- Subject to validity checks.
- May refer to regular files or directories for compatibility purposes.

Traceability:
- All functions in `gnu/dup2.c`

#### Entity: File Status Record
A status structure used to classify the resource referenced by a descriptor for compatibility behavior, specifically directory-related handling.

Relationships:
- Derived from a source descriptor.
- Used only to determine the applicable duplication behavior path for directory cases.

Traceability:
- `struct stat` at `gnu/dup2.c:144`
- `klibc_dup2dirfd` in `gnu/dup2.c:112-138`

#### Entity: Replacement Duplication Result
The observable outcome of the operation: either success with the destination descriptor value or failure according to replacement `dup2` semantics.

Relationships:
- Produced by the replacement duplication operation.
- Depends on descriptor validity, same-descriptor conditions, and applicable compatibility path.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`

## Success Criteria

### SC-1: Correct success value
For successful duplication where `fd != desired_fd`, the Rust module returns the destination descriptor value in all tested supported environments.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`

### SC-2: Correct same-descriptor behavior
For calls where `fd == desired_fd`, tests show:
- success and same-value return for a valid open descriptor, and
- failure for an invalid descriptor.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`
- `dup2_nothrow` in `gnu/dup2.c:50-58`

### SC-3: Invalid source failure
For invalid source descriptors, tests show that the Rust module does not report successful duplication.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`

### SC-4: Existing destination replacement
Tests demonstrate that duplicating onto an already open destination descriptor succeeds according to `dup2` semantics and returns the destination descriptor.

Traceability:
- `rpl_dup2` in `gnu/dup2.c:161-191`

### SC-5: Windows-path compatibility
On target configurations requiring the Windows compatibility path, targeted tests confirm behavior consistent with the source module for success, same-descriptor handling, and invalid-source failure.

Traceability:
- `ms_windows_dup2` in `gnu/dup2.c:72-104`
- `rpl_dup2` in `gnu/dup2.c:161-191`

### SC-6: Directory-case compatibility
On target configurations requiring the directory-descriptor compatibility path, targeted tests confirm behavior consistent with the source module when the source descriptor refers to a directory.

Traceability:
- `klibc_dup2dirfd` in `gnu/dup2.c:112-138`
- `struct stat` at `gnu/dup2.c:144`

## Acceptance Notes
- Conformance is measured against the observable behavior evidenced by `gnu/dup2.c`.
- The Rust rewrite may differ internally, but it must not omit the compatibility cases represented by the source helper paths.
- No undocumented functionality is required beyond the replacement `dup2` behavior described in this specification.