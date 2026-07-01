# spec.md

## Title
Functional Specification for `module_src_parseopt_wordwrap_set_09`

## Overview
This module defines margin-setting behavior for the word-wrapping output object used by `src/parseopt/wordwrap.c`. The Rust rewrite must preserve the observable behavior of changing the left and right margins on an existing word-wrap handle and reporting success or failure through the same functional outcomes as the C module.

The scope of this specification is limited to the functionality evidenced by:

- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`
- the `wordwrap_file` state object
- the position-tracking structures used within that state

This specification does not define unrelated word-wrapping operations beyond the state changes and validation implied by these margin-setting functions.

## Feature Specification

### Summary
The module provides controlled updates to the wrapping margins of a word-wrap state object. It allows a caller to set:

- a left margin, which defines the indentation boundary used by the wrapping state
- a right margin, which defines the wrap boundary used by the wrapping state

Both operations act on an existing word-wrap object and return an integer status indicating whether the requested update was accepted.

### Supported behavior
The Rust version must implement the following behavior:

1. Accept a word-wrap state handle and a requested margin value.
2. Validate the request against the current state of the word-wrap object.
3. If the request is valid, update the corresponding stored margin in the word-wrap state.
4. If the request is invalid, leave the effective state unchanged for that operation and return a failure status.
5. Preserve the logical relationship between left and right margins so that the word-wrap configuration remains usable.

### Behavioral boundaries
The Rust rewrite must preserve only the behavior evidenced for margin-setting:

- setting the left margin is a distinct operation from setting the right margin
- each operation returns status to the caller
- the operations apply to the `wordwrap_file` state object
- margin values are unsigned quantities
- validity depends on the current word-wrap configuration, not solely on the new value in isolation

No additional configuration features are in scope unless required to support these observed behaviors.

## User Scenarios & Testing

### Scenario 1: Set a valid left margin
A caller has an initialized word-wrap object with an already valid right margin. The caller requests a new left margin that remains compatible with the current right margin.

Expected result:

- the function reports success
- the left margin stored in the word-wrap state is updated
- the right margin remains unchanged

### Scenario 2: Reject an invalid left margin
A caller requests a left margin that would violate the word-wrap configuration constraints relative to the current state.

Expected result:

- the function reports failure
- the word-wrap state does not adopt the invalid left margin

### Scenario 3: Set a valid right margin
A caller has an initialized word-wrap object with an already valid left margin. The caller requests a new right margin that remains compatible with the current left margin.

Expected result:

- the function reports success
- the right margin stored in the word-wrap state is updated
- the left margin remains unchanged

### Scenario 4: Reject an invalid right margin
A caller requests a right margin that would violate the configuration constraints relative to the current state.

Expected result:

- the function reports failure
- the word-wrap state does not adopt the invalid right margin

### Scenario 5: Sequential reconfiguration
A caller adjusts one margin and then the other, each time respecting the configuration rules.

Expected result:

- each valid call succeeds independently
- the final state reflects both accepted updates
- no unrelated word-wrap state is corrupted by the sequence

### Testing guidance
The Rust rewrite must be testable through state-based checks on a word-wrap object.

Minimum test coverage should include:

- success case for `wordwrap_set_left_margin`
- failure case for `wordwrap_set_left_margin`
- success case for `wordwrap_set_right_margin`
- failure case for `wordwrap_set_right_margin`
- verification that failed updates do not alter the previously valid margin state
- verification that valid updates affect only the targeted margin field

## Requirements

### Functional Requirements

#### FR-1: Left margin update
The module shall provide an operation corresponding to `wordwrap_set_left_margin` that accepts a word-wrap state object and an unsigned left-margin value and returns an integer status.

Traceability: `src/parseopt/wordwrap.c`, `wordwrap_set_left_margin`

#### FR-2: Right margin update
The module shall provide an operation corresponding to `wordwrap_set_right_margin` that accepts a word-wrap state object and an unsigned right-margin value and returns an integer status.

Traceability: `src/parseopt/wordwrap.c`, `wordwrap_set_right_margin`

#### FR-3: Validation of left-margin requests
The left-margin operation shall accept only values that preserve a valid wrapping configuration relative to the current word-wrap state.

Traceability: `src/parseopt/wordwrap.c`, `wordwrap_set_left_margin`, `struct wordwrap_file`

#### FR-4: Validation of right-margin requests
The right-margin operation shall accept only values that preserve a valid wrapping configuration relative to the current word-wrap state.

Traceability: `src/parseopt/wordwrap.c`, `wordwrap_set_right_margin`, `struct wordwrap_file`

#### FR-5: State update on success
When a margin-setting request is valid, the module shall store the new margin value in the word-wrap state and report success.

Traceability: `src/parseopt/wordwrap.c`, `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

#### FR-6: No state change on failure
When a margin-setting request is invalid, the module shall report failure and shall not apply the requested invalid margin as the active stored value.

Traceability: `src/parseopt/wordwrap.c`, `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

#### FR-7: Independent margin updates
Updating the left margin shall not modify the right margin except as required by preserving existing valid state, and updating the right margin shall not modify the left margin except as required by preserving existing valid state.

Traceability: `src/parseopt/wordwrap.c`, `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

### Key Entities

#### `wordwrap_file`
The central state object for this module. It holds the active word-wrap configuration, including the margins whose values are updated by the two covered functions. The Rust rewrite must preserve this role as the authoritative state for margin validation and storage.

Traceability: `struct wordwrap_file` in `src/parseopt/wordwrap.c`

#### `position`
A supporting state structure used within the word-wrap module to track positional information relevant to wrapping behavior. Although this specification does not require exposing it publicly, the Rust rewrite must preserve any internal positional consistency needed so that accepted margin changes remain compatible with the word-wrap state model.

Traceability: `struct position` instances in `src/parseopt/wordwrap.c`

## Success Criteria

### SC-1: API-equivalent functional coverage
The Rust module implements behaviorally equivalent left- and right-margin update operations for the word-wrap state object, with integer-like success/failure outcomes corresponding to the C functions.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`

### SC-2: Correct acceptance of valid requests
For test cases where the requested left or right margin is valid relative to the current state, the Rust implementation returns success and the stored margin reflects the new value.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

### SC-3: Correct rejection of invalid requests
For test cases where the requested left or right margin is invalid relative to the current state, the Rust implementation returns failure.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`

### SC-4: State preservation on failed updates
For rejected margin changes, tests confirm that the previously stored valid margin state remains unchanged.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

### SC-5: No cross-margin corruption
Tests confirm that a successful left-margin update does not incorrectly alter the right margin, and a successful right-margin update does not incorrectly alter the left margin.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

### SC-6: Sequential update correctness
Tests confirm that multiple valid margin updates applied in sequence produce a final state consistent with the accepted operations.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`