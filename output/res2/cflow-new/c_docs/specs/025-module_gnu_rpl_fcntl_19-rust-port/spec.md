# spec.md

## Title

Rust Functional Specification for `module_gnu_rpl_fcntl_19`

## Status

Draft

## Context

This module covers the `gnu/fcntl.c` functionality centered on replacement handling for file descriptor duplication operations. The analyzed behavior is limited to two internal operations:

- duplication to a descriptor at or above a requested minimum (`rpl_fcntl_DUPFD`)
- duplication to a descriptor at or above a requested minimum with close-on-exec set (`rpl_fcntl_DUPFD_CLOEXEC`)

The Rust rewrite for branch `025-module_gnu_rpl_fcntl_19-rust-port` must preserve the observable behavior of these operations, including their result and error behavior as used by the surrounding project.

## Scope

In scope:

- Functional behavior for duplicating an existing file descriptor to a new descriptor number meeting a caller-provided lower bound
- Functional behavior for requesting duplication with close-on-exec semantics
- Validation and propagation of success and failure outcomes tied to descriptor duplication
- Any file-status inspection needed to determine descriptor validity or resulting descriptor state, as evidenced by use of `struct stat`

Out of scope:

- General-purpose `fcntl` replacement beyond the two identified duplication behaviors
- New public APIs not required to support the current module behavior
- Any guarantees not evidenced by the source analysis

## Feature Specification

The Rust module must implement the functional role of this C module: provide replacement logic for two descriptor-duplication cases that mirror the C module’s behavior.

### Feature 1: Duplicate to a descriptor at or above a target

The module must support duplication of an already-open file descriptor such that:

- the source descriptor is an existing open descriptor
- the caller provides a minimum target descriptor number
- on success, the operation returns a new open descriptor number that is greater than or equal to the requested target
- the returned descriptor refers to the same underlying open file description as the source descriptor in the normal POSIX sense of descriptor duplication
- on failure, the operation reports failure in a way compatible with surrounding module expectations, including preserving appropriate system error behavior

This feature is traceable to `rpl_fcntl_DUPFD`.

### Feature 2: Duplicate with close-on-exec semantics

The module must support duplication of an already-open file descriptor with a request that the new descriptor be marked close-on-exec, such that:

- the source descriptor is an existing open descriptor
- the caller provides a minimum target descriptor number
- on success, the operation returns a new open descriptor number that is greater than or equal to the requested target
- the resulting descriptor has close-on-exec enabled when the operation succeeds
- if the platform path used to obtain such a descriptor cannot directly provide the requested semantics, the module must still preserve the observable behavior expected by the original replacement logic
- on failure, the operation reports failure compatibly with surrounding expectations

This feature is traceable to `rpl_fcntl_DUPFD_CLOEXEC`.

## User Scenarios & Testing

### Scenario 1: Duplicate a valid descriptor above a minimum

A caller has an open file descriptor and requests a duplicate whose numeric value is at least a specific lower bound.

Expected behavior:

- the call succeeds
- the returned descriptor is open
- the returned descriptor number is greater than or equal to the requested minimum
- operations valid on the original descriptor are also valid on the duplicate in the normal duplicated-descriptor sense

Suggested tests:

- open a file or pipe endpoint
- request duplication with a target above standard descriptors
- verify returned descriptor number meets the lower bound
- verify I/O or status access through the new descriptor succeeds

Traceability: `rpl_fcntl_DUPFD`.

### Scenario 2: Reject duplication from an invalid source descriptor

A caller passes a descriptor that is not open.

Expected behavior:

- the call fails
- failure is reported consistently with system error behavior expected by the original module

Suggested tests:

- pass `-1` or a closed descriptor
- verify the operation fails
- verify no new descriptor is created

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

### Scenario 3: Enforce the requested minimum descriptor number

A caller requests duplication with a minimum target larger than currently low-numbered available descriptors.

Expected behavior:

- the call succeeds only if duplication is possible
- when it succeeds, the returned descriptor number is not below the requested minimum

Suggested tests:

- open a descriptor
- request duplication with several minimum values
- verify each successful result is `>= target`

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

### Scenario 4: Duplicate with close-on-exec requested

A caller needs a duplicate descriptor that will be closed across `exec`.

Expected behavior:

- the call succeeds when duplication is possible
- the returned descriptor is open
- the returned descriptor has the close-on-exec flag set

Suggested tests:

- open a descriptor
- request duplication through the close-on-exec path
- inspect the resulting descriptor flags
- verify `FD_CLOEXEC` is set

Traceability: `rpl_fcntl_DUPFD_CLOEXEC`.

### Scenario 5: Preserve failure behavior when duplication cannot be completed

A caller requests duplication under conditions where the system rejects the request, such as invalid arguments or descriptor exhaustion.

Expected behavior:

- the operation fails rather than returning a partially valid result
- no success result is reported
- error behavior remains system-compatible for the failure case encountered

Suggested tests:

- use invalid minimum targets where applicable
- create conditions approaching descriptor limits if the test environment permits
- verify failure is surfaced correctly

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

## Requirements

### Functional Requirements

#### FR-1: Source descriptor validation

The module shall accept a source file descriptor as input and shall succeed only when that descriptor is valid and open.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

#### FR-2: Minimum target enforcement

The module shall accept a requested minimum target descriptor number and, on success, shall return a new descriptor number greater than or equal to that requested minimum.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

#### FR-3: Duplicate descriptor creation

The module shall create a new descriptor that refers to the same underlying open file description as the source descriptor, consistent with descriptor duplication behavior.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

#### FR-4: Failure signaling

When duplication cannot be completed, the module shall report failure and shall not report a successful descriptor result.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

#### FR-5: Close-on-exec duplication behavior

For the close-on-exec variant, the module shall ensure that a successful result has the close-on-exec property enabled on the returned descriptor.

Traceability: `rpl_fcntl_DUPFD_CLOEXEC`.

#### FR-6: Descriptor-state compatibility checks

The module shall support any descriptor-state inspection necessary to distinguish valid descriptor outcomes from invalid ones, as evidenced by the use of file status data.

Traceability: `gnu/fcntl.c`, `struct stat`.

### Key Entities

#### Entity 1: Source file descriptor

An integer file descriptor identifying an already-open file, pipe, or similar kernel-managed resource. It is the input resource from which duplication is requested.

Relationships:

- used by both duplication operations
- must be valid for duplication to succeed

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

#### Entity 2: Minimum target descriptor number

An integer lower bound constraining the numeric value of the duplicate descriptor to be returned on success.

Relationships:

- paired with a source descriptor in both operations
- constrains successful result selection

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

#### Entity 3: Duplicated file descriptor

The returned descriptor produced on successful duplication.

Relationships:

- derived from the source descriptor
- must satisfy the minimum target constraint
- in the close-on-exec variant, must also have `FD_CLOEXEC` semantics

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

#### Entity 4: File status record

A file status structure used for descriptor-related state inspection where needed by the module behavior.

Relationships:

- supports validation or behavioral checks tied to descriptor handling

Traceability: `struct stat`, `gnu/fcntl.c`.

## Success Criteria

### SC-1: Successful DUPFD behavior

For a valid open source descriptor and a valid target minimum, the Rust module returns a new open descriptor number `>= target` in all tested cases corresponding to the original DUPFD behavior.

Traceability: `rpl_fcntl_DUPFD`.

### SC-2: Successful DUPFD_CLOEXEC behavior

For a valid open source descriptor and a valid target minimum, the Rust module returns a new open descriptor number `>= target` and the returned descriptor has close-on-exec enabled in all tested success cases.

Traceability: `rpl_fcntl_DUPFD_CLOEXEC`.

### SC-3: Invalid source rejection

For invalid or closed source descriptors, both duplication variants fail in all tested cases and do not produce a usable new descriptor.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

### SC-4: Lower-bound compliance

For a range of tested target minimum values, every successful result from both variants satisfies the numeric lower-bound requirement.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.

### SC-5: No false success on failure paths

Under tested failure conditions, the Rust module never reports success without producing a valid duplicated descriptor satisfying the requested semantics.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`.