# spec.md

## Title
Rust Port Functional Specification: `main_root_stat_04`

## Metadata
- Project: `pwd`
- Module: `main_root_stat_04`
- Category: `main_cluster`
- Rust branch: `004-main_root_stat_04-rust-port`
- Source files analyzed: `pwd.c`, `root-dev-ino.c`
- Generation date: 2026-06-09

## Overview
This module provides path-resolution behavior needed by the `pwd` program for determining and reporting the current working directory, with specific support for:

- obtaining the logical current working directory from environment-visible state when valid (`logical_getcwd` in `pwd.c`)
- obtaining device/inode identity for the filesystem root (`get_root_dev_ino` in `root-dev-ino.c`)

The Rust rewrite must preserve the observable behavior of these responsibilities as used by the `pwd` main flow. The module specification is limited to functionality evidenced by the analyzed files and named functions.

## Scope
In scope:

- validation and retrieval of a logical current directory path
- retrieval of root directory identity as a `(device, inode)` pair
- use of filesystem metadata needed to compare directory identities

Out of scope:

- introduction of new command-line behavior
- new public APIs beyond what is needed for the module role
- non-evidenced caching, concurrency, serialization, FFI, or recovery features

## Feature Specification

### Feature 1: Logical current-directory retrieval
The module must support obtaining the current working directory as a logical path string through behavior corresponding to `logical_getcwd`.

The logical path result is derived from the process-visible working-directory path state and must only be accepted when it matches the actual current directory according to filesystem metadata comparison. The validation is evidenced by the use of `struct stat` within `logical_getcwd`.

Behavioral expectations:

- The module evaluates a candidate logical path for the current directory.
- The candidate logical path is accepted only if metadata for the candidate path and metadata for the actual current directory identify the same directory.
- When accepted, the returned value represents the logical directory path rather than a reconstructed physical path.
- When the logical path is absent or does not identify the same directory as the actual current directory, the logical retrieval operation does not produce a valid logical path result.

This feature exists to let the surrounding `pwd` logic prefer a logical path when it is valid.

### Feature 2: Root directory identity retrieval
The module must support obtaining the filesystem identity of the root directory through behavior corresponding to `get_root_dev_ino`.

Behavioral expectations:

- The module queries metadata for the root path `/`.
- From that metadata, it fills or returns a `dev_ino` value containing the root directory device and inode identity.
- The result is suitable for directory identity comparison elsewhere in the program.

This feature exists to allow the main path-resolution flow to recognize or compare against the root directory during current-directory determination.

## User Scenarios & Testing

### Scenario 1: Valid logical working directory is used
A user runs `pwd` in a directory where the logical path tracked by the process corresponds to the actual current directory.

Expected module behavior:

- logical current-directory retrieval succeeds
- the returned string is the logical path
- metadata for the logical path and the actual current directory compare as the same directory

Test guidance:

- set up a process environment with a valid logical working directory path
- invoke the Rust equivalent of logical retrieval
- verify that a non-empty path is returned
- verify by filesystem metadata that the returned path and `.` refer to the same directory

Traceability: `logical_getcwd` in `pwd.c`, `struct stat`

### Scenario 2: Invalid logical working directory is rejected
A user runs `pwd` after conditions that make the logical path stale or inconsistent with the actual current directory.

Expected module behavior:

- logical current-directory retrieval does not accept the stale logical path
- the operation reports absence/failure rather than returning an incorrect path

Test guidance:

- arrange a mismatch between the process-visible logical path and the actual current directory
- invoke the Rust equivalent of logical retrieval
- verify that the module does not return the mismatched logical path

Traceability: `logical_getcwd` in `pwd.c`, `struct stat`

### Scenario 3: Root identity can be retrieved for comparisons
The surrounding `pwd` logic needs root device/inode information while resolving a directory path.

Expected module behavior:

- root directory metadata for `/` is read successfully when the filesystem permits it
- a `dev_ino` result is produced containing the root identity

Test guidance:

- invoke the Rust equivalent of root identity retrieval
- verify that the result contains the same device and inode values as direct metadata lookup on `/`

Traceability: `get_root_dev_ino` in `root-dev-ino.c`, `dev_ino`, `struct stat`

### Scenario 4: Root identity supports root comparison
The surrounding main flow compares some directory metadata against root metadata to detect arrival at root.

Expected module behavior:

- the root identity result is usable for equality comparison with other directory metadata-derived identities
- when metadata from `/` is compared against the retrieved root identity, the comparison succeeds

Test guidance:

- retrieve root identity through the module
- obtain metadata for `/`
- verify equality of device and inode components

Traceability: `get_root_dev_ino` in `root-dev-ino.c`, `dev_ino`

## Requirements

### Functional Requirements

#### FR-1: Validate logical path against actual current directory
The Rust module shall provide behavior equivalent to `logical_getcwd` that validates a candidate logical working-directory path by comparing filesystem identity metadata of that path with metadata of the actual current directory.

Traceability: `pwd.c:299-323`, `logical_getcwd`, `struct stat`

#### FR-2: Return logical path only when validated
The Rust module shall return a logical current-directory path only when the candidate logical path is validated as referring to the same directory as the actual current directory.

Traceability: `pwd.c:299-323`, `logical_getcwd`

#### FR-3: Reject absent or mismatched logical path
The Rust module shall not produce a logical current-directory result when the candidate logical path is unavailable or when metadata comparison shows it does not match the actual current directory.

Traceability: `pwd.c:299-323`, `logical_getcwd`, `struct stat`

#### FR-4: Retrieve root filesystem identity
The Rust module shall provide behavior equivalent to `get_root_dev_ino` that reads metadata for `/` and produces the corresponding root directory device/inode identity.

Traceability: `root-dev-ino.c:28-37`, `get_root_dev_ino`, `dev_ino`, `struct stat`

#### FR-5: Populate reusable root identity structure
The Rust module shall represent the root identity in a `dev_ino`-equivalent structure suitable for later directory identity comparisons by the surrounding main logic.

Traceability: `root-dev-ino.c:28-37`, `get_root_dev_ino`, `dev_ino`

### Key Entities

#### `dev_ino`
A filesystem identity entity containing the device and inode values used to identify a directory. In this module, it is used specifically for root directory identity retrieval and comparison.

Relationships:

- populated from `stat` metadata for `/`
- compared against directory metadata-derived identities in surrounding `pwd` logic

Traceability: `root-dev-ino.c`, `pwd.c`

#### Filesystem metadata (`stat`)
Filesystem status information for a path or directory, used in this module to determine whether two paths identify the same directory and to extract root device/inode information.

Relationships:

- used by logical current-directory validation
- used to populate `dev_ino`

Traceability: `pwd.c`, `root-dev-ino.c`

#### Logical current-directory path
A string path representing the process-visible logical working directory candidate. This is accepted only if validated against actual current-directory metadata.

Relationships:

- validated using `stat`
- returned by the logical retrieval behavior when valid

Traceability: `logical_getcwd` in `pwd.c`

## Success Criteria

### SC-1: Valid logical path acceptance
When given a valid logical current-directory candidate that refers to the same directory as `.`, the Rust module returns that logical path.

Traceability: FR-1, FR-2

### SC-2: Invalid logical path rejection
When the logical current-directory candidate does not refer to the same directory as `.`, the Rust module does not return that candidate as a valid logical result.

Traceability: FR-1, FR-3

### SC-3: Root identity correctness
When retrieving root identity, the Rust module returns device and inode values that match direct filesystem metadata for `/`.

Traceability: FR-4, FR-5

### SC-4: Comparison suitability
The root identity produced by the Rust module can be compared for equality with directory metadata-derived device/inode values, and it compares equal for `/`.

Traceability: FR-5

### SC-5: Scope fidelity
The Rust rewrite implements the above logical-path validation and root-identity retrieval behaviors without requiring additional non-evidenced module capabilities.

Traceability: module scope derived from `logical_getcwd` and `get_root_dev_ino`

## Acceptance Notes
- The Rust implementation may differ internally from the C code, but it must preserve the specified functional behavior.
- Any API shape used in the Rust port must still support the validated logical-path outcome and root `dev_ino` retrieval described above.
- Error signaling details may be adapted to Rust conventions, provided success and failure behavior remain consistent with the requirements.