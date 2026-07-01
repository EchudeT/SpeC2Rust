# spec.md

## Title

Functional Specification: `module_gnu_rpl_fcntl_19` Rust Port

## Overview

This module provides replacement behavior for specific `fcntl`-based file descriptor duplication operations. Its scope is limited to duplicating an existing file descriptor to a descriptor number at or above a caller-supplied minimum, with one variant preserving normal inheritance behavior and another requiring close-on-exec behavior on the duplicated descriptor.

The Rust rewrite must preserve the functional behavior of the analyzed C module for these duplication cases, including success and failure behavior that is observable through the returned file descriptor result and error reporting from the underlying operating system interfaces.

## Scope

In scope:

- Replacement handling for duplication equivalent to `F_DUPFD`
- Replacement handling for duplication equivalent to `F_DUPFD_CLOEXEC`
- Validation and propagation of failure conditions relevant to these operations
- Behavior that depends on descriptor state as observable through file descriptor and file status inspection

Out of scope:

- General `fcntl` command dispatch beyond the analyzed duplication behavior
- New APIs not evidenced by the source module
- Additional file descriptor management features not present in the analyzed module

## Source Basis

This specification is derived from the following analyzed module elements:

- File: `gnu/fcntl.c`
- Functions:
  - `rpl_fcntl_DUPFD`
  - `rpl_fcntl_DUPFD_CLOEXEC`
- Data structure usage:
  - `struct stat`

## Feature Specification

### Feature

Provide replacement duplication operations for file descriptors that behave like the analyzed C module’s `F_DUPFD` and `F_DUPFD_CLOEXEC` support logic.

### Functional Behavior

1. The module must accept:
   - an existing open file descriptor
   - a minimum target descriptor number

2. For the duplication operation corresponding to `F_DUPFD`:
   - the module must create a duplicate of the supplied file descriptor
   - the returned descriptor must be greater than or equal to the requested minimum target
   - the returned descriptor must refer to the same underlying open file description as the original descriptor in the normal OS sense of descriptor duplication

3. For the duplication operation corresponding to `F_DUPFD_CLOEXEC`:
   - the returned descriptor must have close-on-exec behavior enabled when the operation succeeds

4. The module must reject invalid duplication attempts in the same functional situations covered by the C module, including:
   - invalid source file descriptor
   - invalid requested target minimum
   - inability of the operating system to allocate a new descriptor satisfying the request
   - inability to ensure required close-on-exec state for the cloexec variant

5. The module must preserve failure observability:
   - on failure, no successful descriptor result is produced
   - the operation must report an error consistent with the failing system interaction

6. The module must distinguish the two duplication modes only by the required descriptor inheritance behavior:
   - normal duplication mode does not require close-on-exec to be set
   - cloexec duplication mode requires close-on-exec to be set on the resulting descriptor

## User Scenarios & Testing

### Scenario 1: Duplicate an open descriptor with a minimum descriptor floor

A caller has an open file descriptor and needs a duplicate whose numeric value is at least a specified minimum.

Expected support:

- the operation succeeds when the source descriptor is valid and resources allow duplication
- the returned descriptor number is greater than or equal to the requested minimum
- the returned descriptor is distinct from the original descriptor

Suggested tests:

- open a file or pipe endpoint
- request duplication with minimum target `0`
- verify returned descriptor is valid and not equal to the original
- request duplication with a higher minimum target
- verify returned descriptor is at least that minimum

### Scenario 2: Duplicate with close-on-exec required

A caller needs a duplicate descriptor that will be marked close-on-exec.

Expected support:

- the cloexec duplication operation succeeds when the source descriptor is valid and the required descriptor state can be established
- the returned descriptor is at least the requested minimum
- the resulting descriptor has close-on-exec enabled

Suggested tests:

- open a file descriptor
- invoke the cloexec duplication operation
- inspect descriptor flags
- verify the close-on-exec flag is set on the returned descriptor

### Scenario 3: Invalid source descriptor

A caller passes a descriptor that is not open.

Expected support:

- the operation fails
- no new descriptor is returned

Suggested tests:

- call each duplication variant with `-1` or another invalid descriptor
- verify failure is reported

### Scenario 4: Invalid target minimum

A caller requests duplication with an invalid target lower bound.

Expected support:

- the operation fails in invalid-input cases covered by the original module behavior
- no new descriptor is returned

Suggested tests:

- call each duplication variant with a negative minimum target
- verify failure is reported

### Scenario 5: Source and duplicate refer to the same opened object

A caller relies on duplication semantics rather than reopening a path.

Expected support:

- the duplicate refers to the same opened object as the original descriptor
- observable file status for the duplicated handle remains consistent with descriptor duplication semantics

Suggested tests:

- duplicate an open descriptor
- compare behavior through operations that confirm descriptor duplication semantics
- where supported by test environment, inspect metadata/state as done through descriptor-based system inspection

## Requirements

### Functional Requirements

#### FR-1: Duplicate descriptor with minimum target
The Rust module shall provide behavior equivalent to the analyzed replacement duplication operation for the non-cloexec case: given a valid source file descriptor and a target minimum, it returns a new descriptor whose numeric value is greater than or equal to that minimum.

Traceability: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`

#### FR-2: Duplicate descriptor with close-on-exec
The Rust module shall provide behavior equivalent to the analyzed replacement duplication operation for the cloexec case: given a valid source file descriptor and a target minimum, it returns a new descriptor whose numeric value is greater than or equal to that minimum and whose close-on-exec flag is enabled.

Traceability: `gnu/fcntl.c`, `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-3: Preserve duplication semantics
For both duplication variants, the Rust module shall duplicate an existing open descriptor rather than reopening a file by path, so that the result refers to the same underlying opened object in the sense of OS descriptor duplication.

Traceability: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-4: Reject invalid source descriptors
For both duplication variants, the Rust module shall fail when the supplied source descriptor is not valid for duplication.

Traceability: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-5: Reject invalid target minimum values
For both duplication variants, the Rust module shall fail when the requested minimum target value is invalid for the underlying duplication operation.

Traceability: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-6: Propagate allocation or OS-level duplication failure
For both duplication variants, the Rust module shall fail without returning a successful descriptor if the operating system cannot create a duplicate descriptor satisfying the request.

Traceability: `gnu/fcntl.c`, `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

#### FR-7: Enforce close-on-exec for cloexec variant
If the cloexec duplication variant cannot ensure that the returned descriptor has close-on-exec enabled, the Rust module shall report failure rather than returning a descriptor that does not meet the requested mode.

Traceability: `gnu/fcntl.c`, `rpl_fcntl_DUPFD_CLOEXEC`

### Key Entities

#### File Descriptor
An integer OS handle identifying an open file, pipe endpoint, or similar open resource. It is the primary input and output entity of this module.

Relationship to requirements:

- source operand for both duplication variants
- result of successful duplication operations

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

#### Minimum Target Descriptor
An integer lower bound supplied by the caller that constrains the numeric value of the duplicated descriptor.

Relationship to requirements:

- used to determine whether a returned descriptor is acceptable
- invalid values produce failure

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

#### Close-on-exec Descriptor State
A descriptor flag state required specifically for the cloexec duplication variant.

Relationship to requirements:

- mandatory postcondition for cloexec duplication success
- distinguishes the cloexec operation from normal duplication

Traceability: `rpl_fcntl_DUPFD_CLOEXEC`

#### `struct stat`
Filesystem/object status information used by the analyzed C module as supporting descriptor state inspection.

Relationship to requirements:

- supports validation or comparison of descriptor-observable object state where needed by module behavior

Traceability: `gnu/fcntl.c`, `struct stat`

## Success Criteria

### SC-1: Correct descriptor floor behavior
For each duplication variant, when invoked with a valid open source descriptor and a valid minimum target, the module returns a valid descriptor number greater than or equal to the requested minimum.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

### SC-2: Distinct returned descriptor
For each successful duplication variant call, the returned descriptor is distinct from the source descriptor.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

### SC-3: Cloexec state correctness
For each successful cloexec duplication call, inspection of the resulting descriptor confirms that close-on-exec is enabled.

Traceability: `rpl_fcntl_DUPFD_CLOEXEC`

### SC-4: Invalid source failure
For each duplication variant, calls using an invalid source descriptor fail and do not produce a usable duplicated descriptor.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

### SC-5: Invalid minimum failure
For each duplication variant, calls using an invalid minimum target fail and do not produce a usable duplicated descriptor.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`

### SC-6: Duplication semantics preserved
For successful calls, observable behavior confirms the returned descriptor is a duplicate of the original open descriptor rather than an unrelated reopened handle.

Traceability: `rpl_fcntl_DUPFD`, `rpl_fcntl_DUPFD_CLOEXEC`, `struct stat`

### SC-7: Failure does not downgrade cloexec guarantees
The cloexec duplication variant never reports success with a returned descriptor lacking the required close-on-exec state.

Traceability: `rpl_fcntl_DUPFD_CLOEXEC`