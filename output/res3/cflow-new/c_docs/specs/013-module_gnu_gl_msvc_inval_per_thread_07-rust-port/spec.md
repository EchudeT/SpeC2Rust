# spec.md

## Overview

This module provides per-thread tracking for Microsoft C runtime invalid-parameter handling within the GNU compatibility layer used by the project. Its observed behavior is limited to:

- supplying an invalid-parameter handler function compatible with the MSVC runtime callback shape, and
- providing access to a per-thread state object associated with invalid-parameter handling.

The Rust rewrite must preserve that functional boundary: it must support retrieval of the current thread's invalid-parameter state and support an invalid-parameter handler path that updates that state for the current thread.

## Scope

In scope for this module:

- per-thread invalid-parameter state access
- invalid-parameter handler behavior tied to that per-thread state
- preservation of thread-local separation of state

Out of scope for this module spec:

- general-purpose error reporting frameworks
- process-wide policy configuration beyond the behavior evidenced by this module
- any additional public APIs not evidenced by the analyzed source

## Source Basis

This specification is derived from the following analyzed source elements:

- File: `gnu/msvc-inval.c`
- Functions:
  - `gl_msvc_inval_current`
  - `gl_msvc_invalid_parameter_handler`
- Core type:
  - `struct gl_msvc_inval_per_thread`

## Feature Specification

### Feature: Per-thread invalid-parameter state access

The module exposes functionality to obtain the invalid-parameter tracking state for the calling thread.

Behavior required from the Rust version:

- A caller must be able to request the current thread's `gl_msvc_inval_per_thread` state.
- If the calling thread has no existing state object yet, the module must make one available for that thread before returning.
- Repeated requests from the same thread during the lifetime of that thread's state must resolve to that thread's own state rather than another thread's state.
- Requests from different threads must resolve to distinct per-thread state instances.

Traceability:

- `gl_msvc_inval_current` in `gnu/msvc-inval.c`
- `struct gl_msvc_inval_per_thread` references in `gnu/msvc-inval.c`

### Feature: Invalid-parameter handler integration with per-thread state

The module defines an MSVC-compatible invalid-parameter handler function. The observed function signature includes standard invalid-parameter callback inputs such as expression, function, file, line, and an additional integer payload.

Behavior required from the Rust version:

- The module must provide handler behavior corresponding to the analyzed invalid-parameter callback.
- When invoked, the handler must act on the current thread's invalid-parameter state, not on shared global state for all threads.
- The handler must tolerate the callback inputs defined by the analyzed signature, including cases where descriptive inputs may be absent, because the source signature accepts pointer-based descriptive fields.

This spec does not require preserving any descriptive text payload beyond what is evidenced by the source analysis; only the per-thread handling relationship is required.

Traceability:

- `gl_msvc_invalid_parameter_handler` in `gnu/msvc-inval.c`
- `struct gl_msvc_inval_per_thread` references near handler usage in `gnu/msvc-inval.c`

## User Scenarios & Testing

### Scenario 1: First access on a thread

A caller running on a thread asks the module for the current invalid-parameter state before any prior use on that thread.

Expected outcome:

- the call succeeds in returning that thread's state object
- the returned state is usable by later invalid-parameter handling on the same thread

Suggested test:

- call the Rust equivalent of `gl_msvc_inval_current` once on a fresh thread
- verify a state object is returned for that thread
- verify a subsequent call on the same thread refers to the same logical thread-local state

Traceability:

- `gl_msvc_inval_current`

### Scenario 2: Repeated access on the same thread

A caller requests the current invalid-parameter state multiple times within one thread.

Expected outcome:

- all requests resolve to the same thread-local state instance for that thread

Suggested test:

- invoke current-state access twice from the same thread
- verify the two results identify the same logical per-thread state

Traceability:

- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

### Scenario 3: Independent state across threads

Two threads both use the module.

Expected outcome:

- each thread receives its own invalid-parameter state
- operations on one thread's state do not become the other thread's current state

Suggested test:

- start two threads
- obtain the current state in each
- verify the states are distinct by thread identity or equivalent observable separation

Traceability:

- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

### Scenario 4: Handler updates current thread state

The invalid-parameter handler is invoked while running on a thread that has module state.

Expected outcome:

- the handler records or marks the invalid-parameter event in the current thread's state
- the effect is visible through that thread's state rather than another thread's state

Suggested test:

- obtain current state on a thread
- invoke the Rust equivalent of the invalid-parameter handler on that thread
- verify the current thread's state reflects that a handler event occurred

Traceability:

- `gl_msvc_invalid_parameter_handler`
- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

### Scenario 5: Handler accepts callback inputs in MSVC shape

The handler is called with the callback parameters represented in the analyzed source signature, including descriptive pointers and line number.

Expected outcome:

- the handler accepts those inputs without requiring additional parameters or alternate shapes
- absent descriptive pointers do not prevent the handler from performing its per-thread state effect

Suggested test:

- invoke the handler with representative values and with null/absent descriptive fields
- verify the per-thread state effect still occurs

Traceability:

- `gl_msvc_invalid_parameter_handler`

## Requirements

### Functional Requirements

#### FR-1: Provide current-thread invalid-parameter state access

The module shall provide functionality equivalent to `gl_msvc_inval_current` that returns the invalid-parameter tracking state for the calling thread.

Traceability:

- `gl_msvc_inval_current`

#### FR-2: Ensure state availability for first use on a thread

When current-thread state is requested for a thread that does not yet have associated invalid-parameter state, the module shall make state available for that thread before returning.

Traceability:

- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

#### FR-3: Preserve per-thread separation of invalid-parameter state

The module shall maintain invalid-parameter state separately for each thread such that retrieving current state on one thread does not return the current state of another thread.

Traceability:

- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

#### FR-4: Provide invalid-parameter handler behavior matching analyzed callback role

The module shall provide behavior equivalent to `gl_msvc_invalid_parameter_handler` for handling an invalid-parameter callback with the analyzed parameter shape.

Traceability:

- `gl_msvc_invalid_parameter_handler`

#### FR-5: Bind handler effects to the calling thread's state

When the invalid-parameter handler is invoked, the module shall apply its effect to the current thread's `gl_msvc_inval_per_thread` state.

Traceability:

- `gl_msvc_invalid_parameter_handler`
- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

#### FR-6: Accept nullable descriptive callback inputs

The invalid-parameter handler shall accept the descriptive callback inputs present in the analyzed signature without requiring all descriptive pointer fields to be populated.

Traceability:

- `gl_msvc_invalid_parameter_handler`

### Key Entities

#### `gl_msvc_inval_per_thread`

The core module entity is the per-thread invalid-parameter state object.

Observed role:

- represents invalid-parameter handling state for one thread
- is the state returned by current-thread lookup
- is the state acted upon by the invalid-parameter handler for the calling thread

Relationships:

- one calling thread maps to one current `gl_msvc_inval_per_thread` state at a time through current-state access
- the invalid-parameter handler uses the current thread's associated `gl_msvc_inval_per_thread`

Traceability:

- `struct gl_msvc_inval_per_thread`
- `gl_msvc_inval_current`
- `gl_msvc_invalid_parameter_handler`

## Success Criteria

### SC-1: Current-state retrieval works on first use

A test calling the Rust equivalent of `gl_msvc_inval_current` on a thread with no prior module use obtains a valid thread-associated state result.

Traceability:

- `gl_msvc_inval_current`

### SC-2: Same-thread retrieval is stable

A test making repeated current-state requests from one thread observes the same logical per-thread state across those calls.

Traceability:

- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

### SC-3: Cross-thread retrieval is separated

A test obtaining current state from two different threads observes distinct per-thread states rather than a single shared current state.

Traceability:

- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

### SC-4: Handler affects current thread state

A test invoking the Rust equivalent of `gl_msvc_invalid_parameter_handler` on a thread after obtaining that thread's current state can observe that the handler affected that same thread's invalid-parameter state.

Traceability:

- `gl_msvc_invalid_parameter_handler`
- `gl_msvc_inval_current`
- `struct gl_msvc_inval_per_thread`

### SC-5: Handler accepts analyzed callback argument shape

A test can invoke the Rust equivalent of the invalid-parameter handler with the analyzed callback argument categories, including absent descriptive pointers, and still obtain the expected per-thread state effect.

Traceability:

- `gl_msvc_invalid_parameter_handler`