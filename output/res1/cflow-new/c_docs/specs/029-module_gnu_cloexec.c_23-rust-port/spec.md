# spec.md

## Title

Functional Specification for `module_gnu_cloexec.c_23` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_cloexec.c_23`
- Category: `module_cluster`
- Source file: `gnu/cloexec.c`
- Rust branch: `029-module_gnu_cloexec.c_23-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides small file-descriptor utility behavior centered on the close-on-exec flag. Its scope is limited to:

- setting or clearing the close-on-exec state of an existing file descriptor
- creating a duplicate file descriptor with close-on-exec enabled

The Rust rewrite must preserve this behavioral scope and remain a narrow utility module for descriptor flag management.

## Feature Specification

### Summary

The Rust version must implement the same functional boundary as the source module:

1. Accept an open file descriptor and request that its close-on-exec flag be enabled or disabled.
2. Accept an open file descriptor and return a duplicate descriptor that is marked close-on-exec.

### Functional Behavior

#### Close-on-exec flag update

The module must support updating the close-on-exec state for a supplied file descriptor.

Expected behavior:

- Input includes:
  - a file descriptor
  - a boolean-like request indicating whether close-on-exec should be enabled or disabled
- The operation applies the requested state to that descriptor.
- The operation reports success or failure as an integer status compatible with the source module’s contract.

This behavior is traced to `set_cloexec_flag`.

#### Close-on-exec duplicate creation

The module must support producing a duplicate of a supplied file descriptor where the returned duplicate has close-on-exec enabled.

Expected behavior:

- Input includes an existing file descriptor.
- Output is either:
  - a new file descriptor referring to the same underlying open file description, with close-on-exec enabled on the duplicate, or
  - a failure indication compatible with the source module’s contract.

This behavior is traced to `dup_cloexec`.

### Out of Scope

The Rust rewrite must not introduce functionality not evidenced by the source module, including:

- managing ownership lifetimes beyond what is required to express the existing descriptor operations
- defining new public configuration systems
- adding persistence, serialization, recovery, concurrency, or benchmarking features
- expanding the module into general descriptor management beyond close-on-exec setting and cloexec duplication

## User Scenarios & Testing

### Scenario 1: Mark an existing descriptor close-on-exec

A caller has an already-open descriptor and needs to ensure it will not remain open across an `exec`-style process image replacement.

Expected support:

- caller passes the descriptor and requests close-on-exec enabled
- module returns success when the flag update succeeds
- subsequent inspection of descriptor flags shows close-on-exec enabled

Testing guidance:

- open a descriptor
- call the Rust equivalent of the close-on-exec setter with `true`
- verify success status
- verify the descriptor’s close-on-exec bit is set

Traceability: `set_cloexec_flag`

### Scenario 2: Clear close-on-exec on an existing descriptor

A caller has a descriptor whose close-on-exec behavior must be disabled.

Expected support:

- caller passes the descriptor and requests close-on-exec disabled
- module returns success when the flag update succeeds
- subsequent inspection shows the flag cleared

Testing guidance:

- start from a descriptor with close-on-exec enabled
- call the Rust equivalent with `false`
- verify success status
- verify the descriptor’s close-on-exec bit is cleared

Traceability: `set_cloexec_flag`

### Scenario 3: Duplicate a descriptor for exec-safe use

A caller needs a duplicate descriptor that will not leak across `exec`.

Expected support:

- caller passes an existing descriptor
- module returns a distinct descriptor number on success
- the returned descriptor is valid and has close-on-exec enabled

Testing guidance:

- open a descriptor
- call the Rust equivalent of the cloexec duplication function
- verify the returned descriptor is non-negative / success-valued according to the API contract
- verify the returned descriptor is different from the input descriptor
- verify the returned descriptor’s close-on-exec bit is set
- close both descriptors after the test

Traceability: `dup_cloexec`

### Scenario 4: Failure propagation for invalid descriptor input

A caller passes an invalid descriptor.

Expected support:

- module reports failure rather than silently succeeding
- no success result is produced for duplication
- no success status is produced for flag modification

Testing guidance:

- use an invalid descriptor such as `-1`
- invoke each operation
- verify failure is reported according to the module contract

Traceability: `set_cloexec_flag`, `dup_cloexec`

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide an operation that accepts a file descriptor and a requested close-on-exec state and attempts to apply that state to the descriptor.
  Traceability: `set_cloexec_flag`

- **FR-2**: The close-on-exec state-setting operation shall support both enabling and disabling the flag based on caller input.
  Traceability: `set_cloexec_flag`

- **FR-3**: The close-on-exec state-setting operation shall return a success/failure status consistent with the source module’s integer result contract.
  Traceability: `set_cloexec_flag`

- **FR-4**: The module shall provide an operation that duplicates an existing file descriptor and returns the duplicate on success.
  Traceability: `dup_cloexec`

- **FR-5**: The duplication operation shall ensure that the returned duplicate has close-on-exec enabled.
  Traceability: `dup_cloexec`

- **FR-6**: The duplication operation shall report failure using the same result style as the source module when duplication cannot be completed.
  Traceability: `dup_cloexec`

- **FR-7**: Both operations shall operate on caller-supplied file descriptors without requiring module-owned descriptor data structures.
  Traceability: `set_cloexec_flag`, `dup_cloexec`

### Key Entities

- **File descriptor**
  - Integer handle supplied by the caller.
  - Serves as the primary input entity for both supported operations.

- **Close-on-exec state**
  - Boolean descriptor property controlled by the setter operation.
  - Required property on descriptors produced by the duplication operation.

- **Operation result**
  - Integer-style success/failure outcome.
  - For flag updates, indicates whether the requested descriptor state change succeeded.
  - For duplication, represents either the new descriptor or a failure indication.

### Entity Relationships

- A **file descriptor** has a **close-on-exec state**.
- The setter operation modifies the **close-on-exec state** of a supplied **file descriptor**.
- The duplication operation creates a new **file descriptor** whose **close-on-exec state** must be enabled.
- Each operation produces an **operation result** describing success or failure.

## Success Criteria

- **SC-1**: A test that opens a valid descriptor, invokes the Rust close-on-exec setter with enable=true, and then reads descriptor flags shall confirm the close-on-exec flag is set and that the function reports success.
  Traceability: `set_cloexec_flag`

- **SC-2**: A test that opens a valid descriptor with close-on-exec already enabled, invokes the Rust setter with enable=false, and then reads descriptor flags shall confirm the close-on-exec flag is cleared and that the function reports success.
  Traceability: `set_cloexec_flag`

- **SC-3**: A test that duplicates a valid descriptor through the Rust cloexec-duplication operation shall receive a distinct valid descriptor whose close-on-exec flag is set.
  Traceability: `dup_cloexec`

- **SC-4**: A test that supplies an invalid descriptor to the Rust setter shall observe failure reporting rather than a success status.
  Traceability: `set_cloexec_flag`

- **SC-5**: A test that supplies an invalid descriptor to the Rust duplication operation shall observe failure reporting rather than a valid duplicate descriptor.
  Traceability: `dup_cloexec`

- **SC-6**: The Rust module’s exposed functionality shall remain limited to the two behaviors evidenced in `gnu/cloexec.c`: close-on-exec flag update and cloexec-enabled duplication.
  Traceability: `gnu/cloexec.c`, `set_cloexec_flag`, `dup_cloexec`