# Functional Specification: main_root_stat_04

## Document Control

- Project: `pwd`
- Module: `main_root_stat_04`
- Category: `main_cluster`
- Rust branch: `004-main_root_stat_04-rust-port`
- Source basis: `pwd.c`, `root-dev-ino.c`
- Generation date: `2026-06-07`

## 1. Feature Specification

### 1.1 Purpose

This module provides the root-related path resolution behavior used by the `pwd` program for determining and validating the current working directory. Its functional scope is limited to:

- obtaining the device/inode identity of the filesystem root, and
- producing a logical current working directory path from environment and filesystem state checks.

The Rust rewrite must preserve this behavior.

### 1.2 Functional Scope

The module includes two directly evidenced behaviors:

1. **Root identity acquisition**
   - Retrieve the device and inode pair for the root directory `/`.
   - Return that identity through a `dev_ino` record supplied by the caller.

2. **Logical current-directory resolution**
   - Determine a logical current working directory string.
   - Use the current environment path candidate and filesystem metadata checks to decide whether that logical path is acceptable.
   - Only return a logical path when it is validated against current directory state; otherwise the logical result is not accepted.

### 1.3 Included Behavior

The Rust version must implement the following behavior evidenced by the source module:

- Access filesystem metadata for `/` to identify root.
- Access filesystem metadata for the current directory and for the candidate logical path.
- Compare metadata needed to determine whether the candidate logical path refers to the same directory as the current process working directory.
- Produce a path string result for the logical current directory only when validation succeeds.

### 1.4 Excluded Behavior

This specification does not require any behavior not evidenced by the provided module analysis, including:

- defining new public APIs beyond the functionality already represented,
- introducing additional path canonicalization rules,
- adding non-existent recovery or retry mechanisms,
- adding serialization, concurrency, or FFI features.

## 2. User Scenarios & Testing

### 2.1 Scenario: Read root identity for later directory comparison

A caller needs the filesystem identity of `/` so that higher-level path resolution logic can determine when traversal has reached the root directory.

**Expected behavior**
- The module reads metadata for `/`.
- The module fills the provided `dev_ino` record with root device and inode values.
- The returned record matches the metadata of `/`.

**Testing approach**
- Call the Rust equivalent of root identity acquisition with a writable `dev_ino` value.
- Verify that the returned device and inode equal the values from `stat("/")`.

### 2.2 Scenario: Accept a logical working directory when it matches the current directory

A process has an environment-provided logical path candidate for the current working directory, and that path refers to the same directory as the actual current directory.

**Expected behavior**
- The module checks metadata for the current directory and for the candidate logical path.
- If the metadata identify the same directory, the module returns the logical path string.

**Testing approach**
- Set up a working directory where the logical path candidate names the actual current directory.
- Invoke the logical current-directory function.
- Verify that a non-null/non-empty logical path is returned and corresponds to the candidate path.

### 2.3 Scenario: Reject a logical working directory when it does not match current directory state

A process has an environment-provided logical path candidate, but that path no longer identifies the actual current working directory.

**Expected behavior**
- The module compares current-directory metadata with candidate-path metadata.
- If they do not identify the same directory, the module does not accept the logical path.

**Testing approach**
- Provide a candidate path that differs from the actual current directory.
- Invoke the logical current-directory function.
- Verify that the logical path result is rejected according to the Rust API design for failure in this case.

### 2.4 Scenario: Reject a logical path candidate that cannot be validated from filesystem metadata

A logical path candidate exists, but the module cannot retrieve metadata needed to validate it.

**Expected behavior**
- The module does not treat the candidate as a valid logical current directory.
- Failure to validate prevents returning that logical path as a successful result.

**Testing approach**
- Provide a candidate path that cannot be `stat`-checked from the current process state.
- Invoke the logical current-directory function.
- Verify that no successful logical path is produced.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1 Root device/inode retrieval
The module shall obtain filesystem metadata for the root directory `/` and expose its device and inode values through a caller-supplied `dev_ino` entity.

**Traceability:** `root-dev-ino.c`, `get_root_dev_ino`, `dev_ino`, `stat`

#### FR-2 Root identity result propagation
The module shall return the populated root `dev_ino` record as the function result so callers can use the same object for subsequent directory identity comparisons.

**Traceability:** `root-dev-ino.c`, `get_root_dev_ino`, `dev_ino`

#### FR-3 Logical current-directory validation
The module shall validate a logical current-directory path candidate by comparing filesystem identity information for:
- the actual current directory, and
- the candidate logical path.

The logical path shall only be accepted if those identities match.

**Traceability:** `pwd.c`, `logical_getcwd`, `stat`

#### FR-4 Logical path result production
When logical current-directory validation succeeds, the module shall produce the logical path string as the function result.

**Traceability:** `pwd.c`, `logical_getcwd`

#### FR-5 Validation-gated failure behavior
When the candidate logical path cannot be validated or does not match the actual current directory, the module shall not report that candidate as a successful logical current-directory result.

**Traceability:** `pwd.c`, `logical_getcwd`, `stat`

### 3.2 Key Entities

#### `dev_ino`
A filesystem identity record containing the device/inode pair used to compare directories by identity rather than by string path form.

**Relationships**
- Populated from root directory metadata by `get_root_dev_ino`.
- Used by directory-resolution logic to recognize or compare directory identities.

**Traceability:** `root-dev-ino.c`, `pwd.c`, `dev_ino`

#### `stat`
Filesystem metadata used to derive directory identity and validate whether two path references designate the same directory.

**Relationships**
- Read for `/` when populating `dev_ino`.
- Read for the current directory and for a logical path candidate during logical current-directory validation.

**Traceability:** `root-dev-ino.c`, `pwd.c`, `stat`

#### `file_name`
A path string holder used within `pwd.c` path-handling logic.

**Relationships**
- Represents path data involved in current-directory construction and return values.
- Participates in the logical path workflow that leads to `logical_getcwd` results.

**Traceability:** `pwd.c`, `file_name`

## 4. Success Criteria

### 4.1 Behavioral Equivalence

- The Rust module correctly retrieves the device and inode of `/` and returns them through a `dev_ino` structure.
- The Rust module returns a logical current-directory path only when filesystem metadata confirms that the candidate path and actual current directory are the same directory.

### 4.2 Negative Case Handling

- If metadata lookup for `/` fails, root identity retrieval does not report a false root identity.
- If metadata lookup for the logical path candidate or current directory fails, the logical path is not accepted as a successful validated result.
- If candidate-path identity differs from current-directory identity, the logical path is rejected.

### 4.3 Testable Acceptance Conditions

The Rust rewrite is accepted when all of the following are true:

1. A test comparing the Rust root identity result against `stat("/")` passes.
2. A test where the logical path candidate names the actual current directory returns that logical path successfully.
3. A test where the logical path candidate identifies a different directory does not return a successful logical path.
4. A test where candidate-path metadata cannot be obtained does not return a successful logical path.