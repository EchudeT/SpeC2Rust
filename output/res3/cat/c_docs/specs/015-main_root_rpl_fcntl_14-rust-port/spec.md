# spec.md

## Title
Rust Functional Specification for `main_root_rpl_fcntl_14`

## Overview
This module provides replacement behavior for selected file-descriptor duplication operations based on `fcntl`-style semantics. The analyzed C module contains logic for duplicating an existing file descriptor into a new descriptor at or above a requested minimum, and for duplicating a descriptor with close-on-exec requested.

The Rust rewrite must preserve the observable behavior of this module’s two functional responsibilities:

- duplicate a file descriptor with `F_DUPFD` semantics
- duplicate a file descriptor with `F_DUPFD_CLOEXEC` semantics

The scope of this specification is limited to the behavior evidenced by `fcntl.c`, specifically the functions:

- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

A `struct stat` use is also evidenced and may participate in behavior needed to validate or classify descriptors, but no broader file metadata feature is specified beyond what is required to preserve duplication behavior.

## Feature Specification

### Summary
The module acts as a behavioral wrapper or replacement for descriptor duplication operations when standard `fcntl` behavior is not used directly or must be normalized. The Rust version must implement equivalent functional outcomes for the supported duplication commands.

### In-Scope Functionality
The Rust module must support:

1. Duplicating an existing open file descriptor into a descriptor number that is greater than or equal to a caller-supplied minimum target.
2. Duplicating an existing open file descriptor while requesting close-on-exec on the resulting descriptor.
3. Returning success with the duplicated descriptor number when duplication succeeds.
4. Reporting failure when duplication cannot be performed for the provided descriptor or target conditions.
5. Preserving distinction between plain duplication behavior and close-on-exec duplication behavior.

### Out of Scope
The Rust rewrite must not claim or introduce:
- new public APIs beyond what is required to express the two evidenced behaviors
- descriptor management features unrelated to duplication
- generalized file status inspection as a standalone feature
- thread-safety guarantees
- serialization, persistence, recovery, benchmarking, or FFI contracts

## User Scenarios & Testing

### Scenario 1: Duplicate to the next available descriptor at or above a minimum
A caller has a valid open file descriptor and needs another descriptor referencing the same underlying open file description, with the new descriptor number not less than a requested minimum.

Expected behavior:
- the operation succeeds when a suitable descriptor can be allocated
- the returned descriptor number is greater than or equal to the requested target
- the returned descriptor is distinct from the input descriptor

Test coverage:
- valid source descriptor, low minimum target
- valid source descriptor, minimum target already occupied
- valid source descriptor, minimum target equal to source descriptor number
- invalid source descriptor

### Scenario 2: Duplicate with close-on-exec requested
A caller needs a duplicate descriptor that is intended to be marked close-on-exec at creation time or with equivalent observable result.

Expected behavior:
- the operation succeeds when duplication is possible
- the returned descriptor number is greater than or equal to the requested target
- the resulting descriptor behaves as the close-on-exec variant of duplication

Test coverage:
- valid source descriptor and valid target
- invalid source descriptor
- target values that are not acceptable to the underlying operation
- comparison against plain duplication behavior to ensure the close-on-exec case remains distinct

### Scenario 3: Error propagation on unsupported or invalid duplication attempts
A caller passes inputs for which duplication cannot succeed, such as an invalid descriptor or disallowed target range.

Expected behavior:
- the operation reports failure
- no successful duplicate descriptor is returned
- the Rust behavior matches the failure behavior expected from the C replacement logic for the same conditions

Test coverage:
- closed or never-opened descriptor
- invalid target lower bound
- environmental failure where allocation of a new descriptor is not possible

### Scenario 4: Result remains a duplicate of the same underlying open file description
A caller uses the duplicated descriptor as a substitute for the original for standard descriptor-level operations.

Expected behavior:
- the new descriptor refers to the same underlying open file description as the original, consistent with descriptor duplication semantics
- plain duplication and close-on-exec duplication differ only in the requested descriptor flag behavior, not in the underlying opened resource they reference

Test coverage:
- duplicate a readable or writable descriptor and verify shared file-offset or equivalent duplication semantics where applicable
- verify that closing the duplicate does not invalidate the original descriptor, and vice versa, except for normal underlying resource lifetime rules

## Requirements

### Functional Requirements

#### FR-1: Plain descriptor duplication
The module shall provide behavior equivalent to `rpl_fcntl_DUPFD`, duplicating a valid file descriptor and returning a new descriptor number that is not less than the requested target minimum.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`

#### FR-2: Close-on-exec descriptor duplication
The module shall provide behavior equivalent to `rpl_fcntl_DUPFD_CLOEXEC`, duplicating a valid file descriptor and producing a new descriptor that reflects the close-on-exec duplication mode.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-3: Input validation through operation outcome
The module shall reject duplication attempts that are invalid for the underlying descriptor state or requested target range, by reporting failure rather than returning a usable duplicate descriptor.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-4: Distinct semantics between the two duplication modes
The module shall preserve the behavioral difference between plain duplication and close-on-exec duplication. The plain duplication path shall not be treated as identical to the close-on-exec path.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-5: Successful result refers to the same underlying open file description
On success, the returned descriptor shall be a duplicate of the input descriptor in the standard file-descriptor sense, not an independently reopened resource.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-6: Any metadata inspection used by the module must remain subordinate to duplication behavior
If descriptor or file status inspection is used, including any use corresponding to `struct stat`, it shall only support correct duplication behavior and shall not become an independent feature of the Rust module.

Traceability:
- `fcntl.c`
- `struct stat`

### Key Entities

#### File Descriptor
An integer handle representing an open file or similar kernel-managed resource. It is the primary input entity for both duplication behaviors.

Relationships:
- one source file descriptor may be duplicated into one newly allocated file descriptor
- duplication success creates a second descriptor referring to the same underlying open file description

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### Target Minimum Descriptor Number
An integer lower bound that constrains the number assigned to the duplicated descriptor.

Relationships:
- associated with each duplication request
- evaluated together with the source descriptor to determine whether duplication succeeds and which descriptor number may be returned

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### Descriptor Duplication Mode
The operation variant indicating either plain duplication or close-on-exec duplication.

Relationships:
- selects which duplication behavior is applied to the same basic source descriptor and target minimum inputs

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### File Status Metadata
Status information represented in the C module by `struct stat`, used only insofar as required to support correct duplication behavior.

Relationships:
- auxiliary to descriptor handling
- not a standalone module output

Traceability:
- `struct stat`

## Success Criteria

### SC-1: Correct plain duplication result
For valid inputs corresponding to the plain duplication path, the Rust module returns a descriptor number greater than or equal to the requested minimum and distinct from the source descriptor.

Traceability:
- `rpl_fcntl_DUPFD`

### SC-2: Correct close-on-exec duplication result
For valid inputs corresponding to the close-on-exec duplication path, the Rust module returns a descriptor number greater than or equal to the requested minimum and preserves the close-on-exec duplication semantics.

Traceability:
- `rpl_fcntl_DUPFD_CLOEXEC`

### SC-3: Correct failure behavior
For invalid source descriptors or invalid duplication requests, the Rust module reports failure and does not expose a successful duplicate descriptor result.

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

### SC-4: Behavioral equivalence of duplication semantics
In tests that compare original and duplicated descriptors, the Rust module demonstrates standard duplication semantics by operating on the same underlying open file description rather than a separately opened resource.

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

### SC-5: No unsupported feature expansion
The Rust rewrite remains limited to the evidenced responsibilities of the analyzed module and does not introduce unrelated descriptor-management features.

Traceability:
- `fcntl.c`