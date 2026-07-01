# spec.md

## Title
Rust Functional Specification: `main_root_rpl_fcntl_14`

## Document Metadata
- Project: `cat`
- Module: `main_root_rpl_fcntl_14`
- Category: `main_cluster`
- Source basis: `fcntl.c`
- Rust branch: `015-main_root_rpl_fcntl_14-rust-port`
- Generation date: `2026-06-06`

## Overview
This module provides replacement behavior for file-descriptor duplication operations associated with `fcntl` handling. Its scope is limited to two duplication paths:

- duplicating a file descriptor to a descriptor number at or above a requested minimum (`DUPFD`-style behavior),
- duplicating a file descriptor with close-on-exec semantics (`DUPFD_CLOEXEC`-style behavior).

The Rust rewrite must preserve the observable behavior of these operations as replacement logic within the program’s main execution cluster. The specification is limited to the functionality evidenced by the module analysis and does not define broader `fcntl` coverage beyond these duplication cases.

## Feature Specification

### In-Scope Functionality
The Rust version must implement replacement logic equivalent to the module’s two descriptor-duplication behaviors:

1. **Descriptor duplication with minimum target**
   - Accept an existing file descriptor and a requested minimum descriptor value.
   - Return a duplicated descriptor that satisfies the requested lower bound when the operation succeeds.
   - Preserve failure signaling when duplication cannot be completed.

2. **Descriptor duplication with close-on-exec intent**
   - Ensure the duplicated descriptor is produced with close-on-exec behavior consistent with the module’s purpose.

### Behavioral Boundaries
- The module’s functional responsibility is restricted to replacement behavior for these duplication cases.
- The Rust rewrite must not introduce unrelated descriptor operations or general-purpose file-status manipulation not evidenced by the source module.
- Any checks or behavior involving file-descriptor validity, duplication outcome, and close-on-exec state must remain within the behavioral envelope implied by the source functions.

## User Scenarios & Testing

### Scenario 1: Duplicate an open descriptor to a descriptor number at or above a minimum
A caller has a valid open file descriptor and needs another descriptor referring to the same open file entry, with the duplicate assigned to a descriptor number not lower than a requested minimum.

**Expected support in Rust**
- The operation succeeds when the source descriptor is valid and duplication is permitted.
- The returned descriptor is greater than or equal to the requested minimum.
- The returned descriptor is usable as a duplicate of the original descriptor.

**Testing focus**
- Open a file or pipe, request duplication with a minimum target value, and verify:
  - a non-negative descriptor is returned,
  - the returned descriptor is at least the requested minimum,
  - I/O behavior confirms the duplicate refers to the same underlying open file description as expected for descriptor duplication.

### Scenario 2: Duplicate an open descriptor with close-on-exec behavior
A caller has a valid open file descriptor and needs a duplicate that is also marked so it will be closed on an `exec`-style transition.

**Expected support in Rust**
- The operation succeeds when the source descriptor is valid and duplication is permitted.
- The returned descriptor is greater than or equal to the requested minimum.
- The duplicate has close-on-exec behavior applied.

**Testing focus**
- Open a file or pipe, request duplication with close-on-exec behavior, and verify:
  - a non-negative descriptor is returned,
  - the returned descriptor is at least the requested minimum,
  - the duplicate reports the close-on-exec flag as set through the platform’s descriptor-flag query path.

### Scenario 3: Reject duplication for an invalid source descriptor
A caller attempts to duplicate a descriptor that is not valid.

**Expected support in Rust**
- The operation fails.
- The failure is reported consistently as an unsuccessful duplication result.

**Testing focus**
- Pass an invalid descriptor such as `-1` or a closed descriptor and verify that no duplicate is returned.

### Scenario 4: Reject impossible or invalid target constraints
A caller provides a target lower bound that cannot be honored or is invalid for the operating environment.

**Expected support in Rust**
- The operation fails rather than producing a descriptor that violates the requested bound.

**Testing focus**
- Use invalid or unsupported target values and verify failure behavior.

## Requirements

### Functional Requirements

#### FR-1: Support replacement duplication with minimum descriptor bound
Traceable to: `fcntl.c`, `rpl_fcntl_DUPFD`

The module shall provide behavior equivalent to duplicating an existing file descriptor such that the resulting descriptor number is not less than the caller-provided target minimum.

#### FR-2: Report failure for unsuccessful minimum-bound duplication
Traceable to: `fcntl.c`, `rpl_fcntl_DUPFD`

When the source descriptor is invalid or the duplication request cannot be satisfied, the module shall report failure rather than returning a usable duplicate.

#### FR-3: Support replacement duplication with close-on-exec behavior
Traceable to: `fcntl.c`, `rpl_fcntl_DUPFD_CLOEXEC`

The module shall provide behavior equivalent to duplicating an existing file descriptor such that:
- the resulting descriptor number is not less than the caller-provided target minimum, and
- the resulting duplicate has close-on-exec behavior applied.

#### FR-4: Report failure for unsuccessful close-on-exec duplication
Traceable to: `fcntl.c`, `rpl_fcntl_DUPFD_CLOEXEC`

When the source descriptor is invalid or the close-on-exec duplication request cannot be satisfied, the module shall report failure rather than returning a usable duplicate.

#### FR-5: Preserve distinction between ordinary duplication and close-on-exec duplication
Traceable to: `fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

The module shall preserve the behavioral distinction between:
- duplication that creates another descriptor without the close-on-exec requirement, and
- duplication that creates another descriptor with the close-on-exec requirement.

### Key Entities

#### File descriptor
Traceable to: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

An integer handle representing an open file-related resource. It is the source object being validated and duplicated by the module.

#### Target minimum descriptor value
Traceable to: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

An integer lower bound supplied by the caller. The module uses this value to constrain the minimum numeric value of any returned duplicate descriptor.

#### Duplicated file descriptor
Traceable to: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

The result of a successful duplication operation. It refers to the same underlying open file description as the original descriptor for standard descriptor-duplication semantics and must satisfy the requested lower-bound constraint.

#### Close-on-exec attribute
Traceable to: `rpl_fcntl_DUPFD_CLOEXEC`

A descriptor-level property required for the close-on-exec duplication path. This attribute distinguishes the `DUPFD_CLOEXEC` replacement behavior from ordinary duplication.

#### File status metadata
Traceable to: `fcntl.c`, `struct stat`

File-related metadata structure present in the module context. The Rust rewrite may rely on equivalent file metadata interaction only to the extent needed to preserve the evidenced behavior of the duplication logic; no broader metadata API is required by this specification.

## Success Criteria

### SC-1: Minimum-bound duplication succeeds correctly
Traceable to: `rpl_fcntl_DUPFD`

For a valid open descriptor and a valid target minimum, the Rust implementation returns a successful duplicate whose descriptor number is greater than or equal to the requested minimum.

### SC-2: Close-on-exec duplication succeeds correctly
Traceable to: `rpl_fcntl_DUPFD_CLOEXEC`

For a valid open descriptor and a valid target minimum, the Rust implementation returns a successful duplicate whose descriptor number is greater than or equal to the requested minimum and whose close-on-exec flag is set.

### SC-3: Invalid source descriptors fail
Traceable to: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

For each supported duplication path, providing an invalid or closed source descriptor results in failure and does not produce a usable duplicate descriptor.

### SC-4: Target constraint is not violated
Traceable to: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

The Rust implementation never reports success with a returned descriptor number below the caller-provided minimum target.

### SC-5: Behavioral distinction is preserved
Traceable to: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

Tests that compare ordinary duplication and close-on-exec duplication show that the close-on-exec property is present only for the close-on-exec path, while both paths preserve successful duplication semantics when valid inputs are provided.