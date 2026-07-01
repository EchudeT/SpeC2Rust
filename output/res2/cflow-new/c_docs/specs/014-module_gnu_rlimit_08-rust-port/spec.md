# spec.md

## Overview

This module provides a `getdtablesize()` facility for environments where that capability may need a compatibility implementation. Its observable purpose is to return the process file descriptor table size limit as an `int`, using the operating system resource limit information represented by `struct rlimit`.

The Rust rewrite must preserve the same functional boundary: expose equivalent behavior for obtaining the current descriptor-table size value used by callers as the process limit for open file descriptors.

## Scope

In scope:

- Behavior equivalent to `getdtablesize(void) -> int`.
- Reading the process descriptor limit from system resource-limit state.
- Returning the limit as an integer result suitable for callers that need a descriptor-table size.

Out of scope:

- Defining broader resource-limit management APIs.
- Changing process limits.
- Adding new public APIs beyond the behavior evidenced by this module.

## Feature Specification

### Feature: Report process descriptor-table size

The module shall provide functionality equivalent to `getdtablesize()`.

Observed functional intent from the source:

- The module obtains file descriptor table size information from process resource limits.
- The relevant resource-limit entity is `struct rlimit`.
- The result is returned as an `int`.

Rust version requirements for this feature:

- It must provide behaviorally equivalent retrieval of the process file descriptor table size.
- It must use the process's resource-limit information as the source of truth where this module does so.
- It must return an integer result consistent with the C module's contract.

## User Scenarios & Testing

### Scenario 1: Caller queries descriptor capacity before opening files

A caller needs to know how many file descriptors the current process may have open and calls the module's descriptor-table-size function.

Expected result:

- The call returns an integer representing the process descriptor-table size limit used by the original module.

Test coverage:

- Invoke the Rust implementation in a normal process context.
- Verify that the returned value is a positive integer when the host environment exposes a positive descriptor limit.

### Scenario 2: Caller uses the value for bounds or sizing logic

A caller uses the returned descriptor-table size to make decisions such as iteration bounds or defensive checks around descriptor usage.

Expected result:

- The returned value is stable for the duration of the call and directly usable as an `int`-like sizing value.
- The Rust behavior matches the C module's single-call query semantics.

Test coverage:

- Call the Rust implementation and assert the result can be consumed as an integer without additional translation by the caller-facing API.

### Scenario 3: Environment with resource-limit-backed descriptor sizing

The operating environment provides file descriptor limits through resource-limit state.

Expected result:

- The module derives its answer from that resource-limit state rather than from unrelated configuration.

Test coverage:

- On a platform/configuration where resource limits are available, compare the Rust implementation's result with the process open-file limit obtained independently through the same OS facility.
- Confirm equivalence or the same effective integer value after conversion to the exposed return type.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide functionality equivalent to `getdtablesize(void)` that returns a process descriptor-table size as an integer.
  **Traceability:** `gnu/getdtablesize.c`, `getdtablesize`.

- **FR-2**: The module shall obtain the descriptor-table size from process resource-limit information represented by `struct rlimit` where this module uses that source.
  **Traceability:** `gnu/getdtablesize.c`, `getdtablesize`, `struct rlimit`.

- **FR-3**: The module shall expose a result compatible with the C contract's `int` return behavior.
  **Traceability:** `gnu/getdtablesize.c`, `getdtablesize`.

- **FR-4**: The module shall be read-only with respect to resource limits; it shall report the descriptor-table size and not modify process limits as part of this functionality.
  **Traceability:** `gnu/getdtablesize.c`, `getdtablesize` behavior boundary.

### Key Entities

- **Descriptor-table size result**: The integer value returned to the caller representing the process file descriptor table size.
  **Traceability:** `getdtablesize` return type `int`.

- **Resource-limit record (`struct rlimit`)**: The operating-system resource-limit structure used by the module as the source of descriptor-limit information.
  **Traceability:** `gnu/getdtablesize.c:114`, `struct rlimit`.

Relationship:

- The module reads descriptor-limit information from the resource-limit record and converts or returns it as the descriptor-table size result.

## Success Criteria

- **SC-1**: The Rust module provides one behaviorally equivalent descriptor-table-size query corresponding to `getdtablesize()` and returns an integer result.
  **Mapped requirements:** FR-1, FR-3.

- **SC-2**: In environments where resource limits for open files are available, the Rust result matches the value derived from the same process resource-limit source used by the original module.
  **Mapped requirements:** FR-2.

- **SC-3**: The Rust implementation performs no observable modification of process resource limits during a descriptor-table-size query.
  **Mapped requirements:** FR-4.

- **SC-4**: The Rust implementation supports the documented usage scenarios of direct querying and immediate caller use of the returned sizing value.
  **Mapped requirements:** FR-1, FR-3.