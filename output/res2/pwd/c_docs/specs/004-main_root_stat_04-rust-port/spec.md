# spec.md

## Overview

This module covers the root-stat-dependent path resolution behavior used by `pwd`, based on `pwd.c` and `root-dev-ino.c`.

It has two evidenced responsibilities:

- obtain the device/inode identity of the filesystem root via `get_root_dev_ino`
- derive a logical current working directory string via `logical_getcwd`

The Rust rewrite for branch `004-main_root_stat_04-rust-port` must preserve the same functional boundaries and observable behavior relevant to these responsibilities.

## Feature Specification

### Feature: Root device/inode identity lookup

The module must provide behavior equivalent to obtaining the `(dev, ino)` identity for the filesystem root directory.

This behavior is evidenced by:

- `get_root_dev_ino` in `root-dev-ino.c`
- use of `struct dev_ino`
- use of `struct stat`

Expected behavior:

- inspect the root directory path `/`
- collect its device and inode identity from filesystem metadata
- store the result into a caller-provided `dev_ino` record
- return the populated record on success
- report failure if root metadata cannot be obtained

This feature exists to support root detection during path-related logic.

### Feature: Logical current working directory resolution

The module must provide behavior equivalent to computing the logical current working directory string.

This behavior is evidenced by:

- `logical_getcwd` in `pwd.c`
- use of `struct stat`
- use of `struct dev_ino`
- repeated use of `struct file_name`

Expected behavior:

- read the `PWD` environment variable as a candidate logical path
- validate that the candidate corresponds to the current directory rather than trusting it blindly
- compare metadata for the candidate path and the current directory to confirm identity
- accept the candidate only when validation succeeds
- return a path string representing the logical current working directory
- report failure when no valid logical path can be confirmed

The Rust version must preserve the distinction between a logical path candidate and validated filesystem identity.

## User Scenarios & Testing

### Scenario 1: Root identity is needed for path logic

A caller needs the filesystem identity of `/` in order to determine whether directory traversal or path assembly has reached root.

The Rust module must support:

- requesting root identity using a caller-supplied record
- receiving the root device/inode values when `/` can be stated successfully
- receiving failure when `/` cannot be stated

Testing should verify:

- successful population of root `dev` and `ino` on normal systems
- failure propagation when root metadata lookup is forced to fail in a controlled test seam

### Scenario 2: `PWD` names the actual current directory

The process environment contains a `PWD` value, and that path refers to the same directory as the current working directory.

The Rust module must support:

- reading the candidate logical path from the environment
- validating candidate identity against the current directory
- returning the logical path when validation succeeds

Testing should verify:

- a valid `PWD` path is returned unchanged as the logical cwd result
- validation is based on filesystem identity, not only string form

### Scenario 3: `PWD` is absent or invalid

The environment does not provide `PWD`, or it provides a path that does not match the current directory.

The Rust module must support:

- rejecting an absent candidate
- rejecting a mismatched candidate after metadata comparison
- signaling failure to obtain a valid logical cwd from this mechanism

Testing should verify:

- absence of `PWD` does not produce a false logical path
- a stale or forged `PWD` is rejected when its metadata differs from the current directory
- failure is observable to the caller

### Scenario 4: Root-aware validation participates in directory logic

Directory logic that depends on distinguishing root from non-root directories uses root device/inode information consistently.

The Rust module must support:

- obtaining current-directory metadata
- comparing directory identity values against root identity values where needed by path logic

Testing should verify:

- root identity values are suitable for equality comparison with other directory metadata
- the module does not confuse string path equality with device/inode equality

## Requirements

### Functional Requirements

#### FR-1: Root metadata retrieval

The module shall retrieve filesystem metadata for `/` and extract device/inode identity into a `dev_ino` record.

Traceability:

- `root-dev-ino.c`
- `get_root_dev_ino`
- `dev_ino`
- `stat`

#### FR-2: Caller-supplied root record population

The module shall populate a caller-provided root identity record rather than requiring internally owned persistent state.

Traceability:

- `get_root_dev_ino (struct dev_ino *root_d_i)`

#### FR-3: Root lookup failure reporting

If metadata for `/` cannot be obtained, the module shall report failure instead of returning synthesized identity values.

Traceability:

- `get_root_dev_ino`
- `stat`

#### FR-4: Logical path candidate intake

The module shall obtain a logical current-directory candidate from the process environment via the `PWD` variable.

Traceability:

- `logical_getcwd`
- `pwd.c`

#### FR-5: Candidate validation against current directory

The module shall validate the `PWD` candidate by comparing filesystem identity information for the candidate path and the actual current directory.

Traceability:

- `logical_getcwd`
- `stat`
- `dev_ino`

#### FR-6: Valid logical cwd return

When the `PWD` candidate matches the actual current directory, the module shall return that logical path as the logical cwd result.

Traceability:

- `logical_getcwd`

#### FR-7: Invalid logical cwd rejection

When `PWD` is absent or does not identify the current directory, the module shall reject it and report failure from the logical cwd routine.

Traceability:

- `logical_getcwd`
- `stat`

### Key Entities

#### `dev_ino`

A filesystem identity record containing the directory device and inode values used for equality checks.

Relationship to module behavior:

- populated for the root directory by `get_root_dev_ino`
- used by path logic to compare directory identity, including root-related checks

Traceability:

- `root-dev-ino.c`
- `pwd.c`

#### `stat`

Filesystem metadata used to derive device/inode identity for directory comparison.

Relationship to module behavior:

- provides metadata for `/`
- provides metadata for the current directory and logical-path candidate during validation

Traceability:

- `pwd.c`
- `root-dev-ino.c`

#### `file_name`

A path/string holder used within path-related logic in `pwd.c`.

Relationship to module behavior:

- represents path values manipulated during cwd-related processing
- supports handling of logical path text distinct from metadata identity

Traceability:

- `pwd.c`

## Success Criteria

### SC-1: Correct root identity retrieval

Given a normal accessible filesystem root, calling the Rust equivalent of `get_root_dev_ino` returns success and yields `dev` and `ino` values equal to those obtained by statting `/`.

Traceability:

- `root-dev-ino.c`
- `get_root_dev_ino`

### SC-2: Correct failure on root stat error

When root metadata retrieval is made to fail in test conditions, the Rust equivalent of `get_root_dev_ino` returns failure and does not claim valid root identity.

Traceability:

- `root-dev-ino.c`
- `get_root_dev_ino`

### SC-3: Accept valid logical `PWD`

When `PWD` names the same directory as the process current directory, the Rust equivalent of `logical_getcwd` succeeds and returns that logical path.

Traceability:

- `pwd.c`
- `logical_getcwd`

### SC-4: Reject invalid logical `PWD`

When `PWD` is missing or names a different directory than the process current directory, the Rust equivalent of `logical_getcwd` fails rather than returning the invalid candidate.

Traceability:

- `pwd.c`
- `logical_getcwd`

### SC-5: Identity-based validation

Validation outcomes for logical cwd depend on filesystem identity equivalence derived from metadata, not merely on path-string equality.

Traceability:

- `pwd.c`
- `logical_getcwd`
- `stat`
- `dev_ino`