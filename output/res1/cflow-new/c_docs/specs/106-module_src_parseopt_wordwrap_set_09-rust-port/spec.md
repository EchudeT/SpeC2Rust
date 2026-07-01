# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_wordwrap_set_09`

## Document Control

- Project: `cflow-new`
- Module category: `module_cluster`
- Source module: `src/parseopt/wordwrap.c`
- Rust branch target: `106-module_src_parseopt_wordwrap_set_09-rust-port`
- Generation date: `2026-06-11`

## Overview

This module segment defines margin-configuration behavior for the word-wrapping output facility represented by `WORDWRAP_FILE` and its backing `struct wordwrap_file`.

The Rust rewrite must preserve the observable behavior of setting:

- the left margin, via `wordwrap_set_left_margin`
- the right margin, via `wordwrap_set_right_margin`

These operations are configuration actions on an existing word-wrap context. They must update the wrap configuration used by the broader word-wrapping subsystem and report success or failure using the same success/failure contract as the C module.

## Feature Specification

### Feature: Configure left margin for a word-wrap context

The module must provide behavior equivalent to `wordwrap_set_left_margin(WORDWRAP_FILE wf, unsigned left)`.

This behavior applies to an existing word-wrap context and sets the left-margin value used by that context for subsequent formatted output. The operation must validate the requested configuration against the current state of the same context and reject invalid settings.

Observed scope supported by the source evidence:

- operate on a `WORDWRAP_FILE` / `struct wordwrap_file` instance
- accept an unsigned margin value
- return an integer status indicating whether the change was accepted
- affect margin configuration held by the word-wrap context

### Feature: Configure right margin for a word-wrap context

The module must provide behavior equivalent to `wordwrap_set_right_margin(WORDWRAP_FILE wf, unsigned right)`.

This behavior applies to an existing word-wrap context and sets the right-margin value used by that context for subsequent formatted output. The operation must validate the requested configuration against the current state of the same context and reject invalid settings.

Observed scope supported by the source evidence:

- operate on a `WORDWRAP_FILE` / `struct wordwrap_file` instance
- accept an unsigned margin value
- return an integer status indicating whether the change was accepted
- affect margin configuration held by the word-wrap context

### Functional boundary

The Rust port for this module is limited to margin-setting behavior. It must not introduce new public capabilities beyond configuration of left and right margins on the existing word-wrap state object.

## User Scenarios & Testing

### Scenario 1: Set a valid left margin on an existing wrap context

A caller has already created or obtained a valid word-wrap context.
The caller requests a new left margin value that is acceptable relative to the context’s current wrap configuration.

Expected result:

- the operation succeeds
- the context stores the new left margin
- later formatting performed by the same context uses the updated left margin

### Scenario 2: Reject an invalid left margin

A caller attempts to set a left margin that violates the module’s accepted margin relationship for the current context state.

Expected result:

- the operation returns failure
- the context’s previously active left margin remains unchanged

### Scenario 3: Set a valid right margin on an existing wrap context

A caller has a valid word-wrap context and supplies a right margin value that is acceptable relative to the current configuration.

Expected result:

- the operation succeeds
- the context stores the new right margin
- later formatting performed by the same context uses the updated right margin

### Scenario 4: Reject an invalid right margin

A caller attempts to set a right margin that violates the module’s accepted margin relationship for the current context state.

Expected result:

- the operation returns failure
- the context’s previously active right margin remains unchanged

### Scenario 5: Sequential reconfiguration

A caller adjusts one margin and then the other on the same wrap context.

Expected result:

- each call is validated against the current state at the time of that call
- accepted calls update only the requested margin
- rejected calls do not partially apply configuration changes

### Testing coverage required

The Rust version must be testable for at least the following:

- successful update of left margin
- successful update of right margin
- failure path for invalid left-margin input
- failure path for invalid right-margin input
- preservation of previous state after a rejected update
- correct cumulative state after multiple valid updates on one context

## Requirements

### Functional Requirements

#### FR-1: Left-margin update

The module shall accept a word-wrap context handle and a requested left-margin value, and shall attempt to update the context’s stored left-margin configuration.

Traceability: `wordwrap_set_left_margin`, `struct wordwrap_file`

#### FR-2: Right-margin update

The module shall accept a word-wrap context handle and a requested right-margin value, and shall attempt to update the context’s stored right-margin configuration.

Traceability: `wordwrap_set_right_margin`, `struct wordwrap_file`

#### FR-3: Validation before commit

The module shall validate a requested margin value against the current context state before committing the update.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

#### FR-4: Failure signaling

The module shall return an integer-compatible success/failure result for each margin-setting operation, preserving the source module’s contract that the caller can distinguish accepted from rejected updates.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`

#### FR-5: No state change on rejected update

If a requested left or right margin is rejected by validation, the module shall leave the context’s prior margin configuration unchanged.

Traceability: `wordwrap_set_left_margin`, `wordwrap_set_right_margin`, `struct wordwrap_file`

#### FR-6: Context-scoped configuration

Margin changes shall apply only to the targeted word-wrap context instance and shall be stored as part of that context’s state for use by the broader word-wrapping behavior.

Traceability: `WORDWRAP_FILE`, `struct wordwrap_file`

### Key Entities

#### `WORDWRAP_FILE` / `struct wordwrap_file`

The central word-wrap context object. It owns the configuration state affected by this module, including the margins used by the broader word-wrapping subsystem. The margin-setting functions operate on this entity.

Relationship:
- receives left/right margin updates
- retains updated values for later use by output/wrapping operations elsewhere in the same subsystem

#### `struct position`

A position-tracking structure used within the same source module as part of the word-wrap subsystem’s internal state model. For this module segment, it provides context that margin configuration belongs to a larger layout/wrapping state machine, but the exposed behavior here is limited to margin-setting on the wrap context.

Relationship:
- associated with `struct wordwrap_file`
- not directly modified by the two specified public configuration operations as a standalone entity

## Success Criteria

### Behavioral correctness

- A valid call corresponding to `wordwrap_set_left_margin` updates the left margin of the targeted wrap context and reports success.
- A valid call corresponding to `wordwrap_set_right_margin` updates the right margin of the targeted wrap context and reports success.
- An invalid requested left margin is rejected and reported as failure.
- An invalid requested right margin is rejected and reported as failure.
- After any rejected margin-setting call, the wrap context retains its pre-call margin values.

### Scope fidelity

- The Rust implementation exposes behavior equivalent to the source module’s two evidenced margin-setting operations and does not require unsupported new capabilities to use them.

### Traceable test completion

- Automated tests demonstrate all scenarios listed in **User Scenarios & Testing** against the Rust implementation.
- Each functional requirement in **Requirements** has at least one corresponding test case or assertion.
- Test outcomes show that accepted updates are persisted in the target context and rejected updates are not persisted.