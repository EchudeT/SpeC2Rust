# spec.md

## Title
Functional Specification for `module_gnu_close.c_24` Rust Port

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_close.c_24`
- Category: `module_cluster`
- Source file: `gnu/close.c`
- Rust branch: `030-module_gnu_close.c_24-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides close-operation behavior for file descriptors, centered on a replacement close routine and an internal helper that performs a non-throwing close attempt.

The Rust port must preserve the observable behavior of the C module:
- provide the module’s close wrapper behavior for a supplied file descriptor,
- use an internal close attempt that does not alter externally visible error state when the close attempt succeeds,
- report success or failure through the function result in the same style as the C module.

No broader resource-management abstractions or additional APIs are required by the evidence for this module.

## Scope
Included in scope:
- behavior equivalent to `rpl_close(int fd)`,
- internal helper behavior equivalent to `close_nothrow(int fd)`,
- handling of valid and invalid file descriptor close attempts,
- propagation of close success or failure through integer return status.

Out of scope:
- opening file descriptors,
- buffered I/O,
- ownership-tracking abstractions beyond what is needed to implement the close behavior,
- any functionality not evidenced by `gnu/close.c`.

## Feature Specification

### Feature: Replacement close behavior
The module must expose functionality equivalent to the C module’s replacement close routine for a file descriptor.

Behavior that must be preserved:
- Accept a file descriptor as input.
- Attempt to close that descriptor.
- Return a success status when the descriptor is successfully closed.
- Return a failure status when the close operation fails.
- Preserve the module’s intended distinction between the public close wrapper and the internal non-throwing close helper.

### Feature: Non-throwing internal close attempt
The module must contain internal behavior equivalent to the helper routine used by the C source.

Behavior that must be preserved:
- Perform a close attempt on the supplied file descriptor.
- Be used as internal support for the module’s externally visible close behavior.
- Avoid introducing additional externally visible failure signaling beyond the integer result expected from the source module.

## User Scenarios & Testing

### Scenario 1: Closing an open file descriptor succeeds
A caller has an open file descriptor and invokes the module’s close functionality.

Expected outcome:
- The module attempts to close the descriptor.
- The operation reports success through its return value.
- The descriptor is no longer open after the call.

Suggested test:
- Open a temporary file or pipe endpoint.
- Call the Rust port’s close function with the obtained descriptor.
- Verify a success return code.
- Verify subsequent direct use of that descriptor fails as a closed descriptor.

### Scenario 2: Closing an invalid file descriptor reports failure
A caller passes a file descriptor that is not open or otherwise invalid.

Expected outcome:
- The module attempts the close operation.
- The operation reports failure through its return value.

Suggested test:
- Pass a clearly invalid descriptor such as `-1`, or a descriptor already closed by a prior successful call.
- Verify the Rust port returns failure.

### Scenario 3: Internal helper semantics support wrapper behavior
The module’s public close behavior relies on an internal non-throwing close attempt.

Expected outcome:
- The helper remains internal-only.
- The public wrapper’s behavior is consistent with using that helper to perform the close attempt.

Suggested test:
- Validate observable wrapper results across successful and failing close cases.
- Confirm no extra public surface is introduced for helper-only behavior.

## Requirements

### Functional Requirements
- **FR-1**: The Rust module shall implement functionality equivalent to `rpl_close(int fd)` from `gnu/close.c`, accepting a file descriptor and returning an integer-style success or failure status traceable to the close attempt.
- **FR-2**: The Rust module shall implement internal helper behavior equivalent to `close_nothrow(int fd)` from `gnu/close.c` to support the wrapper’s close operation.
- **FR-3**: When invoked with a file descriptor that can be closed successfully, the module shall report success.
- **FR-4**: When invoked with a file descriptor for which close fails, the module shall report failure.
- **FR-5**: The Rust port shall preserve the functional boundary present in the source module: one externally relevant replacement close operation and one internal helper for non-throwing close behavior.
- **FR-6**: The Rust port shall not require or expose additional module capabilities beyond file-descriptor close behavior evidenced by `gnu/close.c`.

### Key Entities
- **File descriptor**: Integer handle supplied by the caller and used as the sole operation target for this module.
- **Close result status**: Integer-style success/failure outcome produced by the close operation.
- **Internal helper / public wrapper relationship**: The helper performs the close attempt logic used by the replacement close routine; the wrapper is the module’s externally relevant behavior.

## Success Criteria
- **SC-1**: A test using an open descriptor shows the Rust port returns success when closing it, matching the source module’s functional intent.
- **SC-2**: A test using an invalid or already closed descriptor shows the Rust port returns failure.
- **SC-3**: The Rust implementation contains behavior corresponding to both source functions in `gnu/close.c`: an internal close helper and the replacement close routine.
- **SC-4**: The Rust port introduces no additional public module responsibilities beyond descriptor close behavior evidenced by the source file.
- **SC-5**: All specified behavior is traceable to `gnu/close.c`, specifically `close_nothrow` and `rpl_close`.