# spec.md

## Title
Functional Specification for `main_root_fcntl.c_24` Rust Port

## Metadata
- Project: `cat`
- Module: `main_root_fcntl.c_24`
- Category: `main_cluster`
- Source file: `fcntl.c`
- Primary function in scope: `dupfd`
- Referenced data structure: `struct stat`
- Target Rust branch: `025-main_root_fcntl.c_24-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides the file-descriptor duplication behavior required by the project’s `fcntl.c` logic. Its in-scope functionality is the ability to duplicate an existing file descriptor into a valid descriptor number at or above a caller-specified lower bound, while honoring duplication mode flags and reporting failure through standard error-style outcomes.

The Rust rewrite must preserve the observable behavior of this descriptor-duplication logic as defined by the source module boundary. The rewrite must not introduce unrelated capabilities beyond this duplication behavior.

## Feature Specification

### Summary
The Rust version must implement the module behavior represented by `dupfd`, which performs descriptor duplication from an existing open file descriptor to another descriptor slot selected according to caller constraints.

### Functional Behavior
The module must:
- Accept an existing source file descriptor.
- Accept a minimum target descriptor number.
- Accept duplication-control flags.
- Attempt to create a duplicate descriptor derived from the source descriptor.
- Ensure the returned descriptor, on success, is valid and not less than the requested minimum descriptor number.
- Preserve failure behavior when duplication is not possible due to invalid inputs or operating-system-level refusal.
- Return an integer result indicating either the duplicated descriptor or failure.

### Behavioral Boundaries
The Rust version is in scope only for:
- Duplicating file descriptors with a caller-provided lower-bound target number.
- Respecting duplication flags as part of duplication behavior.
- Reporting success or failure in a way equivalent to the C module’s externally visible behavior.

The Rust version is not required by this specification to provide:
- A new public abstraction layer over file descriptors.
- Extended descriptor lifecycle management beyond duplication.
- Unrelated file metadata processing, even though `struct stat` appears in the source file.

## User Scenarios & Testing

### Scenario 1: Duplicate a valid descriptor above a minimum slot
A caller has an already open file descriptor and needs another descriptor that refers to the same underlying open file, with the requirement that the new descriptor number be at least a specified minimum.

Expected behavior:
- The operation succeeds.
- The returned descriptor number is greater than or equal to the requested minimum.
- The returned descriptor is usable as a duplicate of the source descriptor.

Test coverage:
- Open a file or pipe endpoint.
- Request duplication with a minimum descriptor lower than currently available free descriptors.
- Verify the call succeeds and the returned descriptor meets the lower-bound rule.

### Scenario 2: Duplicate a valid descriptor when the requested minimum is already occupied
A caller requests duplication at or above a descriptor threshold where some descriptor numbers are already in use.

Expected behavior:
- The operation finds or obtains a valid duplicate descriptor at or above the requested minimum.
- It does not return a descriptor below the requested minimum.

Test coverage:
- Open multiple descriptors to occupy lower slots.
- Request duplication from a valid descriptor using a minimum inside the occupied range.
- Verify the result is still at or above the requested minimum.

### Scenario 3: Reject an invalid source descriptor
A caller supplies a source descriptor that is not open or otherwise invalid.

Expected behavior:
- The operation fails.
- It does not produce a usable duplicate descriptor.

Test coverage:
- Call the function with a negative descriptor or a closed descriptor.
- Verify failure is reported.

### Scenario 4: Reject an invalid target lower bound
A caller supplies an invalid lower-bound descriptor number.

Expected behavior:
- The operation fails when the lower bound is not acceptable to the underlying duplication rules.
- No duplicate descriptor is created.

Test coverage:
- Call the function with an invalid lower-bound value.
- Verify failure is reported.

### Scenario 5: Preserve flag-governed duplication behavior
A caller requests duplication with specific flags supported by the source module behavior.

Expected behavior:
- The operation accepts the flags parameter and applies duplication behavior consistently with the source module.
- Supported success and failure outcomes match the source-visible behavior for the same inputs.

Test coverage:
- Exercise duplication with flag values used by the source module.
- Compare success/failure and returned descriptor constraints against expected behavior.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide descriptor duplication behavior equivalent to `dupfd` from `fcntl.c`.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **FR-2**: The module shall accept three inputs: a source file descriptor, a minimum target descriptor number, and duplication flags.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **FR-3**: When duplication succeeds, the module shall return a valid duplicated descriptor number that is not less than the requested minimum target descriptor number.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **FR-4**: The duplicated descriptor shall refer to the same underlying open file description as the supplied source descriptor, as expected of file-descriptor duplication behavior.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **FR-5**: When the source descriptor is invalid, the module shall fail rather than returning a successful duplicate descriptor.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **FR-6**: When the requested minimum target descriptor number is invalid for duplication, the module shall fail rather than returning a successful duplicate descriptor.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **FR-7**: The module shall incorporate the provided flags into duplication behavior and preserve the source module’s observable outcomes for supported flag use.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **FR-8**: The module shall expose success and failure through integer-style result behavior compatible with the source module’s observable contract.
  **Traceability**: `fcntl.c`, function `dupfd`.

### Key Entities
- **File descriptor**
  - Integer identifier for an open file, pipe, or similar kernel-managed resource.
  - Used as both the source descriptor and the returned duplicated descriptor.
  - Relationship: `dupfd` consumes one existing descriptor and, on success, produces another descriptor tied to the same open resource.
  - **Traceability**: `fcntl.c`, function `dupfd`.

- **Duplication lower bound**
  - Integer constraint specifying the minimum acceptable numeric value for the duplicated descriptor.
  - Relationship: constrains descriptor selection during duplication.

- **Duplication flags**
  - Integer flag set affecting duplication behavior.
  - Relationship: modifies how duplication is requested or validated.

- **`struct stat`**
  - Source-file data structure reference present in the module file.
  - Relationship: referenced in the source file but not required as a primary behavior carrier for the in-scope `dupfd` functionality.
  - **Traceability**: `fcntl.c`, referenced type `struct stat`.

## Success Criteria
- **SC-1**: For a valid open source descriptor and valid lower bound, the Rust module returns success with a descriptor number greater than or equal to the requested minimum in all covered tests.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **SC-2**: In tests using a valid source descriptor, operations performed through the original and duplicated descriptors demonstrate duplicate-descriptor semantics consistent with the operating system’s descriptor duplication behavior.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **SC-3**: For invalid source descriptors, the Rust module reports failure in all covered tests and does not yield a usable descriptor.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **SC-4**: For invalid lower-bound descriptor inputs, the Rust module reports failure in all covered tests and does not yield a usable descriptor.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **SC-5**: For the set of duplication flags exercised by source-equivalent tests, the Rust module’s success/failure outcomes match the source module’s observable behavior.
  **Traceability**: `fcntl.c`, function `dupfd`.

- **SC-6**: The Rust rewrite remains limited to the evidenced module responsibility of descriptor duplication and does not require unrelated functionality to satisfy its tests.
  **Traceability**: `fcntl.c`, function `dupfd`; source-file referenced type `struct stat`.

## Out of Scope
- Defining new public APIs beyond what is necessary to preserve the source module behavior.
- Adding non-evidenced file metadata features based on `struct stat`.
- Adding concurrency guarantees, persistence mechanisms, serialization, FFI layers, or performance targets not evidenced by the source module.