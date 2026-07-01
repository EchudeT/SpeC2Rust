# spec.md

## Title
Functional Specification for `module_src_parseopt_wordwrap_set_09` Rust Port

## Overview
This module covers margin-setting behavior for the word wrapping component implemented in `src/parseopt/wordwrap.c`, specifically the functions:

- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`

The Rust rewrite must preserve the functional behavior of updating the active wrapping margins on an existing wordwrap context (`WORDWRAP_FILE` / `struct wordwrap_file`) and reporting success or failure through the same decision boundaries evidenced by the C module.

The scope of this specification is limited to margin configuration behavior. It does not define the full word wrapping engine beyond what is necessary to describe how these setters affect the wordwrap state.

## Feature Specification

### Summary
The module provides controlled mutation of the left and right wrapping margins for a wordwrap state object. The Rust version must support setting either margin independently while enforcing the same validity constraints implied by the original C logic.

### In Scope
- Accepting a wordwrap state handle/context.
- Setting the left margin to a caller-supplied unsigned value.
- Setting the right margin to a caller-supplied unsigned value.
- Rejecting invalid margin updates.
- Returning a status result indicating whether the update was accepted.

### Out of Scope
- Creation or destruction of the wordwrap context unless needed internally by the Rust port.
- Text emission, wrapping, indentation output, or terminal width discovery beyond the dependency that margins belong to the wordwrap state.
- Any new configuration APIs or behaviors not evidenced by the source functions listed above.

### Required Behavior
1. The module must allow the left margin stored in the wordwrap state to be updated.
2. The module must allow the right margin stored in the wordwrap state to be updated.
3. Each update must be validated against the current state of the opposite margin before being accepted.
4. Invalid updates must leave the effective wordwrap margin state unchanged.
5. The outcome of each setter call must be observable through its return/status value.

## User Scenarios & Testing

### Scenario 1: Set a valid left margin
A caller has an initialized wordwrap context with an existing right margin. The caller requests a new left margin that satisfies the module's margin validity rules.

Expected result:
- The call succeeds.
- The left margin in the wordwrap state becomes the requested value.
- The right margin remains unchanged.

Test coverage:
- Initialize a context with a known right margin.
- Apply a valid left margin.
- Verify success status and stored state.

### Scenario 2: Reject an invalid left margin
A caller requests a left margin value that violates the relation required between left and right margins.

Expected result:
- The call fails.
- The wordwrap state retains the previous left and right margins.

Test coverage:
- Initialize a context with known margins.
- Attempt to set an invalid left margin.
- Verify failure status and unchanged state.

### Scenario 3: Set a valid right margin
A caller has an initialized wordwrap context with an existing left margin. The caller requests a new right margin that satisfies the module's validity rules.

Expected result:
- The call succeeds.
- The right margin in the wordwrap state becomes the requested value.
- The left margin remains unchanged.

Test coverage:
- Initialize a context with a known left margin.
- Apply a valid right margin.
- Verify success status and stored state.

### Scenario 4: Reject an invalid right margin
A caller requests a right margin value that violates the relation required between left and right margins.

Expected result:
- The call fails.
- The wordwrap state retains the previous margins.

Test coverage:
- Initialize a context with known margins.
- Attempt to set an invalid right margin.
- Verify failure status and unchanged state.

### Scenario 5: Sequential reconfiguration
A caller changes one margin and then the other, relying on the updated state for subsequent validation.

Expected result:
- Each call is validated against the current stored margins at the time of the call.
- A later call may succeed or fail depending on the earlier accepted change.

Test coverage:
- Set one valid margin.
- Attempt both valid and invalid updates to the opposite margin afterward.
- Verify that validation uses current state, not original initialization values.

## Requirements

### Functional Requirements

#### FR-1: Left margin setter
The Rust module must provide behavior equivalent to `wordwrap_set_left_margin` for updating the left margin in a wordwrap state object.

Traceability:
- `src/parseopt/wordwrap.c`
- `wordwrap_set_left_margin`

#### FR-2: Right margin setter
The Rust module must provide behavior equivalent to `wordwrap_set_right_margin` for updating the right margin in a wordwrap state object.

Traceability:
- `src/parseopt/wordwrap.c`
- `wordwrap_set_right_margin`

#### FR-3: Margin relation validation
The module must validate any proposed left or right margin against the currently stored opposite margin before committing the change.

Traceability:
- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`
- `struct wordwrap_file`

#### FR-4: No state change on rejected update
If a requested margin value is invalid, the module must not partially apply or otherwise alter the stored margin configuration.

Traceability:
- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`
- `struct wordwrap_file`

#### FR-5: Deterministic status reporting
Each setter operation must report success or failure in a deterministic way corresponding to whether the margin update was accepted.

Traceability:
- `wordwrap_set_left_margin`
- `wordwrap_set_right_margin`

### Key Entities

#### `wordwrap_file`
The central wordwrap state object stores the active configuration used by the wrapping subsystem, including the left and right margin values affected by this module.

Relationship to module behavior:
- Both setter functions operate on this state object.
- Validation depends on reading the current opposite margin from this object.
- Successful updates mutate this object.

Traceability:
- `struct wordwrap_file` in `src/parseopt/wordwrap.c`

#### `position`
The source file defines several `struct position` uses associated with tracking layout state in the wider wordwrap subsystem. For this module's scope, these structures are part of the surrounding wordwrap state but are not directly configured by the two margin-setting functions.

Relationship to module behavior:
- They belong to the broader context that margins influence indirectly during wrapping/layout.
- This module does not require new behavior on these structures beyond preserving compatibility with the owning wordwrap state.

Traceability:
- `struct position` definitions/usages in `src/parseopt/wordwrap.c`

## Success Criteria

1. A valid call corresponding to `wordwrap_set_left_margin` updates only the left margin field of an existing wordwrap state and returns success.
2. A valid call corresponding to `wordwrap_set_right_margin` updates only the right margin field of an existing wordwrap state and returns success.
3. An invalid left-margin request is rejected, returns failure, and leaves the stored margins unchanged.
4. An invalid right-margin request is rejected, returns failure, and leaves the stored margins unchanged.
5. Validation behavior for each setter is based on the current wordwrap state at call time.
6. Rust tests cover at least the five user scenarios in this specification using direct state inspection or equivalent observable outcomes.
7. The Rust port does not introduce additional externally visible behavior beyond the margin-setting functionality evidenced by `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, and `struct wordwrap_file`.