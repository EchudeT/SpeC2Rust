# spec.md

## Title

Functional Specification for `module_gnu_cloexec.c_23` Rust Port

## Document Information

- Project: `cflow-new`
- Module: `module_gnu_cloexec.c_23`
- Category: `module_cluster`
- Source file: `gnu/cloexec.c`
- Rust branch: `029-module_gnu_cloexec.c_23-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides file-descriptor close-on-exec control utilities.

Its functionality is limited to:

- setting or clearing the close-on-exec flag on an existing file descriptor
- duplicating a file descriptor while ensuring the duplicate has close-on-exec enabled

The Rust rewrite must preserve these behaviors and their observable outcomes, including success and failure signaling through integer file descriptor results or error-indicating return values consistent with the source module’s contract.

## Scope

### In Scope

- Control of the close-on-exec state for a supplied file descriptor
- Creation of a duplicate file descriptor configured for close-on-exec
- Propagation of failure when the requested operation cannot be completed

### Out of Scope

- Opening files or sockets
- Ownership abstractions beyond the module’s functional contract
- Descriptor lifecycle management outside the two provided operations
- Any API expansion beyond the source module behavior

## Feature Specification

### Feature: Set close-on-exec state on a descriptor

The module must provide behavior equivalent to setting the close-on-exec flag on a given file descriptor to either enabled or disabled.

Expected behavior:

- Accept an existing file descriptor and a requested boolean state
- Attempt to apply the requested close-on-exec state to that descriptor
- Report success when the descriptor’s close-on-exec state has been updated as requested
- Report failure when the operation cannot be performed, including cases where the descriptor is invalid or the underlying descriptor flag operation fails

This feature corresponds to `set_cloexec_flag`.

### Feature: Duplicate a descriptor with close-on-exec enabled

The module must provide behavior equivalent to duplicating a file descriptor such that the returned duplicate is configured with close-on-exec enabled.

Expected behavior:

- Accept an existing file descriptor
- Create a duplicate descriptor
- Ensure the resulting duplicate is marked close-on-exec
- Return the duplicate descriptor on success
- Report failure if duplication fails or if close-on-exec cannot be established on the duplicate

This feature corresponds to `dup_cloexec`.

## User Scenarios & Testing

### Scenario 1: Enable close-on-exec on an existing descriptor

A caller has an already-open file descriptor that must not remain open across an `exec`-family process replacement.

The module is used to enable close-on-exec on that descriptor.

The Rust version must support:

- successful completion for a valid descriptor where the flag can be set
- failure reporting for an invalid descriptor
- observable postcondition that the descriptor is configured not to survive `exec`

### Scenario 2: Disable close-on-exec on an existing descriptor

A caller has a descriptor whose close-on-exec state must be cleared.

The module is used to disable the flag on that descriptor.

The Rust version must support:

- successful completion for a valid descriptor where the flag can be cleared
- failure reporting for an invalid descriptor
- observable postcondition that the descriptor is configured to remain open across `exec`, subject to normal operating system semantics

### Scenario 3: Duplicate a descriptor for safe exec-boundary handling

A caller needs a second descriptor referring to the same open file description, but wants the duplicate protected from leakage across `exec`.

The module is used to obtain a duplicate with close-on-exec enabled.

The Rust version must support:

- returning a new descriptor distinct from the input descriptor on success
- failure reporting when the input descriptor cannot be duplicated
- observable postcondition that the returned descriptor has close-on-exec enabled

### Scenario 4: Error propagation from underlying descriptor operations

A caller supplies a descriptor that cannot be used for the requested operation.

The module is used in either mode and must not mask failure.

The Rust version must support:

- reporting operation failure rather than pretending success
- not returning a usable duplicate when duplication or close-on-exec setup fails

### Testing Coverage Expectations

Tests for the Rust port should cover:

- setting close-on-exec to enabled on a valid descriptor
- setting close-on-exec to disabled on a valid descriptor
- failure when attempting to modify an invalid descriptor
- successful duplication of a valid descriptor
- confirmation that the duplicated descriptor has close-on-exec enabled
- failure when duplication is requested for an invalid descriptor
- failure behavior when duplicate creation succeeds but close-on-exec establishment cannot be completed, if such a condition can be induced in the target test environment

## Requirements

### Functional Requirements

#### FR-1: Close-on-exec state update

The module shall support updating the close-on-exec flag for a supplied file descriptor to a caller-selected enabled or disabled state.

Traceability: `gnu/cloexec.c`, `set_cloexec_flag`.

#### FR-2: Success/failure result for state update

The module shall report whether the close-on-exec state update operation succeeded or failed.

Traceability: `gnu/cloexec.c`, `set_cloexec_flag`.

#### FR-3: Descriptor duplication with close-on-exec

The module shall support creating a duplicate of a supplied file descriptor and returning that duplicate with close-on-exec enabled.

Traceability: `gnu/cloexec.c`, `dup_cloexec`.

#### FR-4: Failure result for duplicate operation

The module shall report failure when descriptor duplication cannot be completed or when the duplicate cannot be placed into the required close-on-exec state.

Traceability: `gnu/cloexec.c`, `dup_cloexec`.

#### FR-5: No additional module responsibilities

The Rust port shall remain limited to close-on-exec flag control and close-on-exec-configured descriptor duplication.

Traceability: module boundary established by `gnu/cloexec.c`, `set_cloexec_flag`, `dup_cloexec`.

### Key Entities

#### Entity: File descriptor

- An integer operating-system file descriptor supplied by the caller
- Used as the target of close-on-exec state updates
- Used as the source descriptor for duplication

Traceability: `set_cloexec_flag(int desc, bool value)`, `dup_cloexec(int fd)`.

#### Entity: Close-on-exec state

- A boolean descriptor attribute indicating whether the descriptor should be closed during `exec`
- Applied to an existing descriptor in the state-update operation
- Required to be enabled on descriptors returned by the duplication operation

Traceability: `set_cloexec_flag(..., bool value)`, `dup_cloexec(int fd)`.

#### Entity: Duplicated descriptor

- A newly returned integer file descriptor produced from an existing descriptor
- Must refer to the duplicated descriptor resource according to normal descriptor duplication semantics
- Must have close-on-exec enabled when returned successfully

Traceability: `dup_cloexec(int fd)`.

## Success Criteria

### SC-1: Correct state mutation

For a valid descriptor, invoking the Rust equivalent of `set_cloexec_flag` with `true` results in the descriptor having close-on-exec enabled, and invoking it with `false` results in that flag being cleared.

Traceability: `set_cloexec_flag`.

### SC-2: Failure signaling for invalid update targets

For an invalid or unusable descriptor, the Rust equivalent of `set_cloexec_flag` reports failure and does not claim success.

Traceability: `set_cloexec_flag`.

### SC-3: Correct duplicate creation

For a valid descriptor, invoking the Rust equivalent of `dup_cloexec` returns a new valid descriptor distinct from the input descriptor.

Traceability: `dup_cloexec`.

### SC-4: Close-on-exec guarantee on duplicates

For every successful `dup_cloexec` call, the returned descriptor is observably configured with close-on-exec enabled.

Traceability: `dup_cloexec`.

### SC-5: Failure signaling for duplicate errors

When duplication cannot be completed, or when the duplicated descriptor cannot be established with close-on-exec enabled, the Rust equivalent of `dup_cloexec` reports failure and does not return a successful result.

Traceability: `dup_cloexec`.

## Acceptance Notes

- The Rust port may express results using Rust-native error handling, but its observable behavior must remain equivalent to the source module’s success and failure contract.
- Acceptance is based on externally verifiable descriptor behavior, especially the close-on-exec state and duplication outcome.
- No acceptance criteria apply to behaviors outside the two source operations.