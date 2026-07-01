# spec.md

## Title

Functional Specification for `main_root_fclose.c_17` Rust Port

## Metadata

- Project: `pwd`
- Module: `main_root_fclose.c_17`
- Category: `main_cluster`
- Source file: `fclose.c`
- Rust branch: `017-main_root_fclose.c_17-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides close-time handling for a `FILE *` stream, centered on replacement `fclose` behavior. Its purpose is to close a stream while preserving and reporting relevant error conditions that may arise from either buffered output flushing or the final close operation itself.

The Rust port must implement equivalent observable behavior for stream-close operations represented by this module: a close attempt is made, close-related failure is reported through the function result, and the error outcome must reflect the module’s prioritization of pre-close stream error state versus close-system-call failure as evidenced by the source functions.

## Scope

In scope:

- Replacement close behavior corresponding to `rpl_fclose`.
- A non-throwing close helper behavior corresponding to the internal helper used by the module.
- Preservation of close-operation result semantics when stream state already indicates an error before final close.

Out of scope:

- General-purpose stream I/O APIs beyond close behavior.
- Creation, opening, reading, writing, seeking, buffering policy, or stream ownership models not evidenced by this module.
- Any new public API beyond the behavior required to port the module’s existing role.

## Feature Specification

### Feature: Replacement stream close with error-state preservation

The module defines stream-closing behavior that does more than simply invoke a raw close. When asked to close a stream, it must:

1. Inspect whether the stream already has an error condition relevant to the close operation.
2. Attempt to close the stream regardless of that prior state.
3. Return failure if either:
   - the stream had a relevant pre-existing error condition, or
   - the close attempt itself fails.
4. Preserve the precedence of the pre-existing stream error condition when both a prior stream error and a close failure are involved, so that the resulting reported error corresponds to the earlier stream error rather than being overwritten by a later close failure.

This behavior is evidenced by:
- `fclose_nothrow`: helper for performing a close attempt without introducing unwanted exception-style behavior.
- `rpl_fclose`: replacement close routine implementing the module’s externally relevant behavior.

### Feature: Non-throwing close attempt helper

The module includes an internal helper whose role is to perform the underlying close attempt and return its success or failure as an integer status. The Rust port must preserve this helper’s functional role inside the module, even if expressed idiomatically in Rust, because the replacement close behavior depends on separating:

- detection of prior stream error state, and
- execution of the actual close attempt.

No broader helper functionality is evidenced.

## User Scenarios & Testing

### Scenario 1: Closing a clean stream succeeds

A caller closes a stream that has no pending stream error and whose underlying close completes successfully.

Expected behavior:
- The module attempts the close.
- The close operation returns success.
- No failure is reported.

Test coverage:
- Construct or simulate a stream/handle state with no prior error.
- Ensure the close path succeeds.
- Verify success result.

### Scenario 2: Stream has an error before close, and close itself succeeds

A caller closes a stream after an earlier write/flush-related condition has already placed the stream in an error state, but the final underlying close operation succeeds.

Expected behavior:
- The module still performs the close attempt.
- The overall result is failure, because the stream had a relevant error before the close completed.
- The reported error outcome corresponds to the prior stream error condition.

Test coverage:
- Simulate a stream state where an error indicator is set before close.
- Ensure underlying close succeeds.
- Verify failure result and preservation of the prior error condition.

### Scenario 3: Stream is clean before close, but close itself fails

A caller closes a stream with no prior stream error, but the underlying close operation fails.

Expected behavior:
- The module reports failure.
- The reported error outcome reflects the close failure.

Test coverage:
- Simulate clean pre-close state.
- Force underlying close failure.
- Verify failure result and close-failure error reporting.

### Scenario 4: Stream already has an error, and close also fails

A caller closes a stream that is already in an error state, and the underlying close operation also fails.

Expected behavior:
- The module reports failure.
- The earlier stream error condition remains the effective reported error condition rather than being replaced by the later close failure.

Test coverage:
- Simulate a pre-existing stream error.
- Force underlying close failure.
- Verify failure result.
- Verify error precedence remains with the pre-existing stream error.

### Scenario 5: Internal helper returns close status without adding extra behavior

Within the module, the internal non-throwing close helper is used to perform the close step.

Expected behavior:
- The helper provides a status result usable by the replacement close routine.
- The replacement routine derives final behavior by combining helper result with pre-close stream error observation.

Test coverage:
- Unit-test the helper through module-internal tests or equivalent abstraction.
- Verify success and failure statuses are distinguishable and are consumed correctly by the replacement close logic.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide replacement stream-close behavior corresponding to `rpl_fclose` in `fclose.c`.
  Traceability: `fclose.c`, `rpl_fclose`.

- **FR-2**: Before finalizing the close result, the module shall account for whether the stream is already in an error state prior to the underlying close attempt.
  Traceability: `fclose.c`, `rpl_fclose`.

- **FR-3**: The module shall attempt to close the stream even when a relevant pre-close stream error state is present.
  Traceability: `fclose.c`, `rpl_fclose`.

- **FR-4**: The module shall report success only when there is no relevant pre-close stream error and the underlying close attempt succeeds.
  Traceability: `fclose.c`, `rpl_fclose`.

- **FR-5**: The module shall report failure when the underlying close attempt fails.
  Traceability: `fclose.c`, `fclose_nothrow`, `rpl_fclose`.

- **FR-6**: The module shall report failure when a relevant pre-close stream error exists, even if the underlying close attempt succeeds.
  Traceability: `fclose.c`, `rpl_fclose`.

- **FR-7**: When both a relevant pre-close stream error and a close failure occur, the module shall preserve the earlier stream error as the effective reported error condition instead of replacing it with the close failure.
  Traceability: `fclose.c`, `rpl_fclose`.

- **FR-8**: The module shall use an internal close helper corresponding to `fclose_nothrow` to obtain underlying close status as a non-public module behavior.
  Traceability: `fclose.c`, `fclose_nothrow`.

### Key Entities

- **Stream handle / stream object**: The close target represented in the C source as `FILE *`. It is the primary input to both module functions and carries the state relevant to pre-close error detection and final close.
  Traceability: `fclose.c`, `fclose_nothrow`, `rpl_fclose`.

- **Pre-close stream error state**: The stream-associated error condition observed before or during close-result determination. This state influences final failure reporting and error precedence.
  Traceability: `fclose.c`, `rpl_fclose`.

- **Close result status**: The integer success/failure outcome produced by the helper and by the replacement close routine.
  Traceability: `fclose.c`, `fclose_nothrow`, `rpl_fclose`.

- **Effective reported error condition**: The error outcome visible after `rpl_fclose` completes, determined by precedence between pre-existing stream error and close failure.
  Traceability: `fclose.c`, `rpl_fclose`.

## Success Criteria

- **SC-1**: In a test case with no pre-close stream error and successful underlying close, the Rust port returns success.
  Traceability: `rpl_fclose`.

- **SC-2**: In a test case with a pre-close stream error and successful underlying close, the Rust port returns failure.
  Traceability: `rpl_fclose`.

- **SC-3**: In a test case with no pre-close stream error and failed underlying close, the Rust port returns failure.
  Traceability: `fclose_nothrow`, `rpl_fclose`.

- **SC-4**: In a test case with both a pre-close stream error and a failed underlying close, the Rust port returns failure and preserves the pre-close error as the effective reported error condition.
  Traceability: `rpl_fclose`.

- **SC-5**: The Rust port includes module-level tests covering all four observable combinations of:
  - pre-close error present / absent, and
  - underlying close success / failure.
  Traceability: `rpl_fclose`.

- **SC-6**: The Rust port keeps the helper close behavior internal to the module and uses it to support the replacement close logic without introducing unrelated public functionality.
  Traceability: `fclose_nothrow`, `rpl_fclose`.

## Non-Goals

The Rust port is not required by this module specification to provide:

- additional public stream-management APIs,
- thread-safety guarantees,
- exception or panic recovery semantics beyond preserving close-result behavior,
- serialization or persistence features,
- any behavior not evidenced by `fclose.c`.

## Notes for Port Validation

Behavioral equivalence should be judged by externally observable close outcomes and error precedence, not by reproducing C-level implementation structure exactly. Internal Rust representation may differ, but it must preserve the module’s functional boundaries and results described above.