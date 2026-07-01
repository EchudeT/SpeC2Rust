# spec.md

## Overview

This module provides small file-descriptor utilities related to the close-on-exec setting and descriptor duplication. The Rust rewrite must preserve the observable behavior of the C module in `gnu/cloexec.c`, specifically for:

- updating whether a file descriptor has the close-on-exec flag set
- duplicating a file descriptor in a way that yields a close-on-exec duplicate

The module scope is limited to descriptor-level operations. No broader resource management or higher-level stream abstraction is evidenced by this module.

## Feature Specification

### Supported behavior

The Rust version must implement two functional behaviors:

1. **Set or clear close-on-exec on an existing descriptor**
   - Accept an existing file descriptor and a boolean target state.
   - Attempt to make the descriptor match the requested close-on-exec state.
   - Report success or failure through the function result.

2. **Create a close-on-exec duplicate of a descriptor**
   - Accept an existing file descriptor.
   - Return a new descriptor referring to the same underlying open file description, with close-on-exec enabled on the returned descriptor.
   - Report failure through the function result when duplication cannot be completed.

### Behavioral boundaries

- The module operates only on file descriptors supplied by the caller.
- The module does not define ownership tracking or lifetime management for descriptors beyond the direct operation requested.
- The module does not introduce policy beyond:
  - toggling the close-on-exec flag for an existing descriptor
  - producing a duplicated descriptor intended to be close-on-exec

## User Scenarios & Testing

### Scenario 1: Mark an existing descriptor close-on-exec

A caller has an already-open descriptor and wants it closed automatically across an `exec`-style process replacement.

**Expected support**
- The module accepts the descriptor and a request to enable close-on-exec.
- If the operation succeeds, subsequent inspection of descriptor flags shows close-on-exec enabled.
- If the descriptor is invalid or the platform operation fails, the module reports failure.

**Test considerations**
- Open a descriptor.
- Call the Rust equivalent of `set_cloexec_flag(fd, true)`.
- Verify success result.
- Verify the descriptor now has the close-on-exec flag set.

### Scenario 2: Clear close-on-exec on an existing descriptor

A caller needs a descriptor to remain open across `exec` and explicitly clears the flag.

**Expected support**
- The module accepts the descriptor and a request to disable close-on-exec.
- If the operation succeeds, subsequent inspection of descriptor flags shows close-on-exec disabled.
- If the operation cannot be completed, the module reports failure.

**Test considerations**
- Open a descriptor and ensure the flag is set first.
- Call the Rust equivalent of `set_cloexec_flag(fd, false)`.
- Verify success result.
- Verify the descriptor no longer has the close-on-exec flag set.

### Scenario 3: Duplicate a descriptor with close-on-exec applied

A caller needs another descriptor for the same resource but does not want that duplicate inherited across `exec`.

**Expected support**
- The module accepts a valid descriptor.
- It returns a new descriptor distinct from the original.
- The new descriptor refers to the same underlying open file description as the original.
- The new descriptor has close-on-exec enabled.
- On failure, no valid duplicate is returned.

**Test considerations**
- Open a descriptor.
- Call the Rust equivalent of `dup_cloexec(fd)`.
- Verify the returned descriptor differs from the source descriptor.
- Verify both descriptors can access the same underlying file/resource as expected for duplication.
- Verify the returned descriptor has close-on-exec enabled.

### Scenario 4: Invalid descriptor handling

A caller passes an invalid descriptor.

**Expected support**
- The module reports failure for both supported operations when given an invalid descriptor.
- No success result is produced for an invalid descriptor input.

**Test considerations**
- Call both operations with a known-invalid descriptor such as `-1`.
- Verify failure is reported.

## Requirements

### Functional Requirements

#### FR-1: Close-on-exec flag update
The module shall provide an operation that accepts a file descriptor and a boolean value and attempts to set the descriptor’s close-on-exec state to match that value.

**Traceability:** `gnu/cloexec.c`, `set_cloexec_flag`

#### FR-2: Success/failure reporting for flag update
The close-on-exec update operation shall report whether the requested change succeeded or failed.

**Traceability:** `gnu/cloexec.c`, `set_cloexec_flag`

#### FR-3: Close-on-exec descriptor duplication
The module shall provide an operation that accepts a file descriptor and attempts to create a duplicate descriptor whose close-on-exec flag is enabled.

**Traceability:** `gnu/cloexec.c`, `dup_cloexec`

#### FR-4: Duplicate descriptor result reporting
The duplication operation shall return a usable duplicated descriptor on success and report failure when duplication cannot be completed.

**Traceability:** `gnu/cloexec.c`, `dup_cloexec`

#### FR-5: Existing-descriptor operation scope
Both operations shall act only on the descriptor supplied by the caller or on the duplicate created from that descriptor; the module shall not require any additional module-defined state.

**Traceability:** `gnu/cloexec.c`, `set_cloexec_flag`, `dup_cloexec`

### Key Entities

#### File descriptor
An integer OS file descriptor supplied by the caller. It is the primary input to both module operations.

**Relationships**
- `set_cloexec_flag` updates metadata associated with the supplied descriptor.
- `dup_cloexec` uses the supplied descriptor as the source for creating a second descriptor.

#### Close-on-exec state
A boolean descriptor attribute indicating whether the descriptor should be closed on successful process replacement via `exec`.

**Relationships**
- `set_cloexec_flag` changes this state on an existing descriptor to a caller-specified value.
- `dup_cloexec` guarantees this state is enabled on the returned duplicate.

#### Duplicated descriptor
A new file descriptor returned from the duplication operation and associated with the same underlying open file description as the source descriptor.

**Relationships**
- Produced by `dup_cloexec`.
- Must be distinct from the input descriptor.
- Must have close-on-exec enabled.

## Success Criteria

1. **Flag enable correctness**
   - When called with a valid descriptor and enable=true, the Rust implementation returns success and the descriptor is externally observable as close-on-exec.
   - **Traceability:** `set_cloexec_flag`

2. **Flag disable correctness**
   - When called with a valid descriptor and enable=false, the Rust implementation returns success and the descriptor is externally observable as not close-on-exec.

3. **Invalid descriptor rejection for flag update**
   - When called with an invalid descriptor, the flag update operation reports failure.

4. **Duplicate creation correctness**
   - When called with a valid descriptor, the duplication operation returns a descriptor different from the input and that returned descriptor is externally observable as close-on-exec.
   - **Traceability:** `dup_cloexec`

5. **Duplicate failure reporting**
   - When duplication cannot be performed, the Rust implementation reports failure and does not yield a valid duplicate descriptor.

6. **No extra module state required**
   - The Rust rewrite can perform both supported operations without requiring module-managed persistent state or auxiliary user-provided structures beyond the file descriptor input.
   - **Traceability:** `gnu/cloexec.c`, `set_cloexec_flag`, `dup_cloexec`