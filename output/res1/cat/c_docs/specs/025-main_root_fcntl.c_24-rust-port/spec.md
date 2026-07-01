# spec.md

## Title

Functional Specification: `main_root_fcntl.c_24`

## Summary

This module provides the file-descriptor duplication behavior represented by `dupfd` in `fcntl.c`. Its responsibility is to create a duplicate of an existing file descriptor at or above a caller-specified minimum descriptor value, while applying the requested duplication flags and preserving error behavior expected by the original module.

The Rust rewrite must preserve the observable behavior of this descriptor-duplication logic, including validation of inputs, descriptor selection rules, flag handling, and failure outcomes traceable to the source module.

## Scope

In scope:

- Duplicating an existing file descriptor.
- Selecting a duplicate descriptor number that is at least a requested minimum.
- Applying duplication-time flags passed to the operation.
- Returning either the new descriptor or an error result consistent with the source behavior.
- Using file status metadata as needed to validate or support duplication behavior, as evidenced by the module’s use of `struct stat`.

Out of scope:

- Defining new public APIs beyond what is needed to represent this module’s behavior in Rust.
- Managing unrelated `fcntl` operations not evidenced by the provided module analysis.
- Adding concurrency, recovery, persistence, serialization, or cross-platform abstractions not evidenced by the source input.

## Feature Specification

### Feature: File descriptor duplication with minimum target descriptor

The module implements duplication of an open file descriptor into a new descriptor whose numeric value is greater than or equal to a caller-provided lower bound.

The Rust version must implement:

- Acceptance of:
  - an existing source descriptor,
  - a requested minimum new descriptor value,
  - duplication flags.
- Validation that the source descriptor is duplicable.
- Rejection of invalid source descriptors or invalid requested target ranges.
- Selection of a new descriptor number that satisfies the minimum bound.
- Return of the newly created descriptor on success.
- Failure signaling when duplication cannot be completed.

### Feature: Duplication flag handling

The module accepts flags as part of duplication behavior. The Rust version must preserve the functional effect of those flags as supported by the source behavior of `dupfd`.

At minimum, the Rust rewrite must:

- Distinguish calls with different flag values.
- Apply supported duplication-related flags to the new descriptor.
- Reject unsupported or invalid flag combinations if the source behavior does so.

### Feature: Source descriptor validation

The module must ensure the original descriptor refers to a valid open file description before duplication succeeds.

The Rust version must preserve behavior where:

- duplication from an invalid descriptor fails,
- duplication from a valid open descriptor can succeed if other constraints are satisfied.

### Feature: Result and error behavior

The module returns an integer result representing success or failure of the duplication operation.

The Rust rewrite must preserve the same externally visible outcome categories:

- success with a usable new descriptor,
- failure for invalid source descriptor,
- failure for invalid requested new descriptor value,
- failure for invalid flags,
- failure when the duplication operation cannot allocate or assign a new descriptor.

## User Scenarios & Testing

### Scenario 1: Duplicate a valid descriptor to the next available slot at or above a minimum

A caller has an open descriptor and requests duplication with a minimum new descriptor value.

Expected behavior:

- The operation succeeds if a descriptor is available.
- The returned descriptor value is greater than or equal to the requested minimum.
- The returned descriptor is distinct from the source descriptor.
- The returned descriptor refers to the same underlying open file description as the source.

### Scenario 2: Duplicate a valid descriptor with duplication flags

A caller duplicates an open descriptor while supplying supported flags.

Expected behavior:

- The operation succeeds when the flag set is valid.
- The created descriptor reflects the requested duplication-time flag effect.

### Scenario 3: Reject an invalid source descriptor

A caller supplies a descriptor that is not open or otherwise invalid.

Expected behavior:

- The operation fails.
- No new descriptor is created.

### Scenario 4: Reject an invalid minimum target descriptor

A caller supplies an invalid minimum descriptor value.

Expected behavior:

- The operation fails.
- No descriptor duplication occurs.

### Scenario 5: Reject invalid or unsupported flags

A caller supplies flags that the source behavior does not accept for descriptor duplication.

Expected behavior:

- The operation fails.
- No new descriptor is created.

### Scenario 6: Validate duplicated-descriptor equivalence

A caller duplicates an open file descriptor and then inspects file identity or state through metadata/stat-style checks.

Expected behavior:

- Source and duplicate descriptors identify the same underlying open file target.
- Both descriptors remain independently closable as distinct descriptor numbers.

### Testing Guidance

The Rust version should be tested with:

- valid open file descriptors from regular files,
- invalid descriptor values,
- boundary values for the minimum target descriptor,
- accepted and rejected flag values,
- verification that the duplicate descriptor number is at least the requested minimum,
- verification that source and duplicate refer to the same file object using file metadata comparisons where appropriate.

## Requirements

### Functional Requirements

- **FR-1**: The module shall duplicate an existing file descriptor and return a new descriptor number on success.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-2**: The module shall ensure the returned descriptor number is greater than or equal to the caller-provided minimum descriptor value.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-3**: The module shall fail when the source descriptor is invalid or not open.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-4**: The module shall evaluate the provided duplication flags and apply supported flag behavior to the new descriptor.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-5**: The module shall fail when the provided flags are invalid for the duplication operation.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-6**: The module shall fail when the requested minimum descriptor value is invalid for duplication.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-7**: The module shall not report success unless a distinct new descriptor has been created.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-8**: The module shall preserve the source module’s externally observable success/failure behavior for descriptor duplication requests.
  **Traceability**: `fcntl.c`, `dupfd`

- **FR-9**: The module shall support verification of descriptor identity equivalence through file status metadata consistent with the source module’s use of file status information.
  **Traceability**: `fcntl.c`, `dupfd`; `struct stat`

### Key Entities

- **File descriptor**
  - Integer handle identifying an open file description.
  - Serves as both the source input and the returned duplicated result.
  - Relationship: one valid source descriptor may produce one distinct duplicated descriptor.

- **Minimum new descriptor value**
  - Integer lower bound constraining the acceptable numeric value of the duplicate.
  - Relationship: the returned descriptor must satisfy this bound.

- **Duplication flags**
  - Integer option set controlling permitted duplication-time behavior.
  - Relationship: influences whether duplication succeeds and what properties the new descriptor receives.

- **File status metadata (`struct stat`)**
  - Metadata representation used for file identity/state checks relevant to validating duplication outcomes.
  - Relationship: can be used to confirm that source and duplicate refer to the same underlying file target.

## Success Criteria

- **SC-1**: For a valid open source descriptor and a valid minimum target value, the Rust module returns success with a descriptor number greater than or equal to the requested minimum.
  **Traceability**: `dupfd`

- **SC-2**: For every successful duplication, the returned descriptor number is different from the source descriptor number.
  **Traceability**: `dupfd`

- **SC-3**: For an invalid source descriptor, the Rust module returns failure and does not create a duplicate descriptor.
  **Traceability**: `dupfd`

- **SC-4**: For an invalid minimum target descriptor value, the Rust module returns failure.
  **Traceability**: `dupfd`

- **SC-5**: For invalid or unsupported duplication flags, the Rust module returns failure.
  **Traceability**: `dupfd`

- **SC-6**: For accepted flags, the Rust module produces a duplicate descriptor whose flag-related behavior matches the source module’s observable duplication semantics.
  **Traceability**: `dupfd`

- **SC-7**: Metadata-based validation using file status information confirms that a successfully duplicated descriptor refers to the same underlying file target as the source descriptor.
  **Traceability**: `dupfd`; `struct stat`

- **SC-8**: The Rust rewrite reproduces the source module’s observable duplication outcomes for the supported input classes: valid duplication, invalid source descriptor, invalid minimum target value, and invalid flags.
  **Traceability**: `dupfd`