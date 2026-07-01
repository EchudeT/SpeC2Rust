# spec.md

## Title
Rust Port Functional Specification: `main_root_rpl_fcntl_14`

## Status
Draft

## Summary
This module provides replacement behavior for selected file-descriptor duplication operations based on `fcntl`. Its functional scope is limited to obtaining a duplicated file descriptor at or above a requested minimum descriptor number, including a variant that requests close-on-exec behavior.

The Rust rewrite must preserve the observable behavior of the analyzed C module for the two covered operations:
- duplicate file descriptor with `F_DUPFD`-style semantics
- duplicate file descriptor with `F_DUPFD_CLOEXEC`-style semantics

The module’s purpose is behavioral compatibility for these duplication requests, including correct success and failure outcomes for valid and invalid inputs.

## Scope
In scope:
- behavior corresponding to `rpl_fcntl_DUPFD`
- behavior corresponding to `rpl_fcntl_DUPFD_CLOEXEC`
- interaction with file descriptor state sufficient to validate duplication outcomes
- use of file status information as needed to determine descriptor validity

Out of scope:
- unrelated `fcntl` commands
- broader file I/O behavior beyond duplication-related effects
- new public APIs not evidenced by the source analysis

## Source Basis
Traceability for this specification is based on:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`
- `struct stat`

## Feature Specification

### Feature: Replacement file-descriptor duplication
The module implements replacement logic for two descriptor-duplication operations that mirror established `fcntl` duplication requests.

#### Supported behavior
1. Accept an existing file descriptor and a target minimum descriptor number.
2. Attempt to create a new descriptor referring to the same open file description as the source descriptor.
3. Ensure the returned descriptor number is greater than or equal to the requested target minimum.
4. Preserve normal failure behavior when duplication cannot be performed because inputs are invalid or the operation is unsupported in the current environment.
5. Support a close-on-exec duplication mode corresponding to the cloexec variant.

### Feature: Close-on-exec duplication mode
For the cloexec variant, the module must perform duplication such that the resulting descriptor is created with close-on-exec semantics when the operation succeeds.

### Feature: Descriptor validity-sensitive behavior
The module distinguishes successful duplication from invalid-descriptor or invalid-target cases. Descriptor state inspection may be used to determine whether a source descriptor is valid and suitable for duplication.

## User Scenarios & Testing

### Scenario 1: Duplicate a valid descriptor to the next available descriptor at or above a minimum
A caller has an open file descriptor and requests duplication with a nonnegative target minimum.

Expected result:
- a new descriptor is returned
- the new descriptor number is at least the requested minimum
- the new descriptor refers to the same underlying open file description as the source

Test guidance:
- open a file or pipe
- call the Rust port’s `DUPFD` behavior with a minimum such as `0` or a higher unused number
- verify success and descriptor-number constraint
- verify shared file offset or equivalent shared-open-description behavior

### Scenario 2: Duplicate a valid descriptor with close-on-exec requested
A caller has an open descriptor and requests the cloexec duplication form.

Expected result:
- a new descriptor is returned
- the descriptor number is at least the requested minimum
- the duplicated descriptor has close-on-exec enabled

Test guidance:
- open a file
- call the Rust port’s `DUPFD_CLOEXEC` behavior
- verify success
- inspect descriptor flags and confirm close-on-exec is set

### Scenario 3: Reject an invalid source descriptor
A caller passes a descriptor that is not open.

Expected result:
- the operation fails
- no new descriptor is created

Test guidance:
- use a clearly invalid descriptor such as `-1` or a closed descriptor
- verify failure outcome matches expected platform-visible error behavior for invalid source descriptors

### Scenario 4: Reject an invalid target minimum
A caller passes a target minimum that is outside accepted bounds for the duplication request.

Expected result:
- the operation fails
- no descriptor is returned

Test guidance:
- invoke with an invalid negative target minimum
- verify failure outcome and absence of a newly open descriptor

### Scenario 5: Preserve independent descriptor identity while sharing the open file description
A caller duplicates a valid source descriptor and then uses both source and duplicate.

Expected result:
- closing one descriptor does not by itself invalidate the other
- operations reflecting shared open-file-description state remain consistent with duplication semantics

Test guidance:
- duplicate a descriptor
- close the source and verify the duplicate remains usable, or vice versa
- where applicable, confirm shared file offset before closing one side

## Requirements

### Functional Requirements

#### FR-1: Duplicate descriptor with minimum-number constraint
The Rust module shall provide behavior equivalent to the C module’s `rpl_fcntl_DUPFD`, duplicating a valid source file descriptor and returning a descriptor number greater than or equal to the requested target minimum.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`

#### FR-2: Duplicate descriptor with close-on-exec semantics
The Rust module shall provide behavior equivalent to `rpl_fcntl_DUPFD_CLOEXEC`, duplicating a valid source file descriptor and returning a descriptor number greater than or equal to the requested target minimum, with close-on-exec enabled on the new descriptor.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-3: Fail on invalid source descriptor
The Rust module shall fail the duplication request when the source descriptor is invalid or not open.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`
- `struct stat`

#### FR-4: Fail on invalid target minimum
The Rust module shall fail the duplication request when the requested target minimum is invalid for the operation.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-5: Return a distinct descriptor on success
On successful duplication, the Rust module shall return a descriptor distinct from the source descriptor and referring to the same underlying open file description.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-6: Do not create a descriptor on failure
When a duplication request fails, the Rust module shall not leave behind a newly usable duplicated descriptor.

Traceability:
- `fcntl.c`
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

### Key Entities

#### File descriptor
An integer handle naming an open file, pipe, or similar kernel-managed resource. It is the primary input and output entity of both supported operations.

Relationships:
- a source file descriptor is provided by the caller
- a newly duplicated file descriptor may be returned
- both descriptors refer to the same open file description after successful duplication

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### Target minimum descriptor number
An integer lower bound that constrains the descriptor number selected for the duplicate.

Relationships:
- consumed by both duplication operations
- compared against the returned descriptor number on success
- validated as part of request acceptance

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

#### Descriptor status information
File status metadata represented through `struct stat`, used by the module’s analyzed source as part of descriptor-related validation.

Relationships:
- associated with an existing file descriptor
- informs validity-sensitive behavior in duplication processing

Traceability:
- `fcntl.c`
- `struct stat`

## Success Criteria

### SC-1: Correct DUPFD success behavior
For a valid open source descriptor and a valid target minimum, the Rust port successfully returns a descriptor number greater than or equal to the requested minimum.

Traceability:
- `rpl_fcntl_DUPFD`

### SC-2: Correct DUPFD_CLOEXEC success behavior
For a valid open source descriptor and a valid target minimum, the Rust port successfully returns a descriptor number greater than or equal to the requested minimum, and the returned descriptor has close-on-exec set.

Traceability:
- `rpl_fcntl_DUPFD_CLOEXEC`

### SC-3: Correct invalid-source failure behavior
For invalid or closed source descriptors, both supported operations fail and do not produce a usable duplicate descriptor.

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`
- `struct stat`

### SC-4: Correct invalid-target failure behavior
For invalid target minimum values, both supported operations fail.

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

### SC-5: Shared open-file-description semantics preserved
After a successful duplication, tests demonstrate that source and duplicate behave as duplicated descriptors to the same open file description, such as sharing file offset where applicable while remaining independently closable.

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

### SC-6: No stray duplicate on failure
Failure-path tests confirm that unsuccessful requests do not leave an extra open descriptor created by the operation.

Traceability:
- `rpl_fcntl_DUPFD`
- `rpl_fcntl_DUPFD_CLOEXEC`

## Acceptance Notes
The Rust rewrite is acceptable when all listed scenarios are supported and all success criteria are demonstrated by automated tests on the target platform branch `015-main_root_rpl_fcntl_14-rust-port`, without introducing behavior beyond the documented module scope.