# spec.md

## Title
Rust Functional Specification for `module_src_parseopt_wordwrap_word_10`

## Overview
This module is a focused part of the project's word-wrapping subsystem. The analyzed functionality centers on explicit marking of word boundaries within a wrapping session through:

- `wordwrap_word_start`
- `wordwrap_word_end`

The Rust rewrite must preserve the observable behavior of this boundary-tracking functionality as used inside the existing word-wrap flow in `src/parseopt/wordwrap.c`.

## Scope
In scope for this specification:

- Representing the word-wrap file/session state needed by word boundary operations.
- Supporting the start of a word region within the current output/layout state.
- Supporting the end of a word region within the current output/layout state.
- Preserving position/bookmark information associated with these boundaries.

Out of scope:

- Defining new wrapping policies or formatting features not evidenced in the analyzed module.
- Adding public APIs beyond the functionality corresponding to the analyzed module behavior.
- Extending the module with concurrency, persistence, FFI, or recovery features.

## Feature Specification

### Summary
The module provides internal word-boundary control for the word-wrap engine. It allows the engine to mark the current position as the beginning of a word and later mark the end of that word. These markers are maintained within the wrapping state object and are used by the broader wrapping logic to reason about whole-word handling.

### Required Behavior
The Rust version must implement behavior equivalent to the C module for the following functions:

- Starting a word boundary mark for an active wrapping state.
- Ending a word boundary mark for an active wrapping state.

The behavior must be state-based: both operations act on a mutable wrapping session/file object rather than on standalone values.

### Functional Boundaries
The Rust port must preserve these boundaries:

1. **Word start capture**
   - When the wrapping engine indicates that a new word begins, the module records the current position in the wrapping state as the word start reference.

2. **Word end capture**
   - When the wrapping engine indicates that the current word ends, the module records the current position in the wrapping state as the word end reference.

3. **Position-based tracking**
   - Word boundaries are represented using the module's position-tracking structures already evidenced in the C source.

4. **State-local behavior**
   - Boundary updates apply only to the provided wrap-state object and do not imply global effects.

## User Scenarios & Testing

### Scenario 1: Mark the beginning of a word during wrapped output generation
A caller managing wrapped output reaches the first character of a word and invokes the word-start operation on the current wrapping state.

Expected support in Rust:
- The wrapping state stores a word-start position corresponding to the current tracked position at the moment of the call.

Suggested test:
- Initialize a wrapping state with a known current position.
- Invoke the Rust equivalent of `wordwrap_word_start`.
- Verify that the stored word-start position matches the current position.

### Scenario 2: Mark the end of a word after consuming its characters
A caller finishes processing a word and invokes the word-end operation.

Expected support in Rust:
- The wrapping state stores a word-end position corresponding to the current tracked position at the moment of the call.

Suggested test:
- Advance or set the current position to a known value after simulated word emission.
- Invoke the Rust equivalent of `wordwrap_word_end`.
- Verify that the stored word-end position matches the current position.

### Scenario 3: Track multiple words in sequence
A caller processes several words in order, repeatedly marking starts and ends as wrapping proceeds.

Expected support in Rust:
- Each new call updates the wrap-state word boundary markers to the latest current position at the time of invocation.

Suggested test:
- Repeatedly mutate current position and call start/end operations.
- After each call, verify that the corresponding stored boundary reflects the most recent position.

### Scenario 4: Boundary tracking remains local to one wrapping session
Two wrapping sessions are active independently.

Expected support in Rust:
- Calling word boundary operations on one state must not alter the boundary positions stored in another state.

Suggested test:
- Create two independent wrapping-state instances with different positions.
- Call start/end on only one instance.
- Verify only the targeted instance changes.

## Requirements

### Functional Requirements

#### FR-1: Maintain wrapping-session state for word boundary operations
The Rust module must define and use a wrapping-session/state entity corresponding to the C `struct wordwrap_file`, sufficient to hold current position and stored word boundary positions.

Traceability:
- `src/parseopt/wordwrap.c`
- `struct wordwrap_file` at lines 72-95
- `wordwrap_word_start`
- `wordwrap_word_end`

#### FR-2: Record word start from current position
The Rust implementation must provide behavior equivalent to `wordwrap_word_start`, updating the wrapping-session state so that the current position becomes the stored start position of the current word.

Traceability:
- `wordwrap_word_start` at lines 568-572
- `struct wordwrap_file`
- `struct position`

#### FR-3: Record word end from current position
The Rust implementation must provide behavior equivalent to `wordwrap_word_end`, updating the wrapping-session state so that the current position becomes the stored end position of the current word.

Traceability:
- `wordwrap_word_end` at lines 577-581
- `struct wordwrap_file`
- `struct position`

#### FR-4: Use position entities consistently for boundary tracking
Word-start and word-end values must be represented as position data associated with the wrapping state, matching the role of `struct position` in the source module.

Traceability:
- `struct position` occurrences in `src/parseopt/wordwrap.c`
- `struct wordwrap_file` fields referencing positions
- `wordwrap_word_start`
- `wordwrap_word_end`

#### FR-5: Preserve per-instance state isolation
Boundary updates must affect only the wrapping-state instance supplied to the operation.

Traceability:
- `wordwrap_word_start (WORDWRAP_FILE wf)`
- `wordwrap_word_end (WORDWRAP_FILE wf)`
- `struct wordwrap_file`

### Key Entities

#### Word-Wrap Session / File State
A mutable state object corresponding to `struct wordwrap_file`. It owns the position information used by the wrapping engine, including the current position and stored word-boundary positions.

Relationship:
- Contains or references position values used by word boundary functions.

Traceability:
- `struct wordwrap_file` at lines 72-95

#### Position
A position-tracking entity corresponding to `struct position`. It represents a location in the wrapping/output process and is used for both the current state and the saved word-start/word-end markers.

Relationship:
- Stored within the wrapping session state.
- Copied or assigned by the word-start and word-end operations.

Traceability:
- `struct position` occurrences throughout `src/parseopt/wordwrap.c`

## Success Criteria

1. **Word start correctness**
   - Given a wrapping-state instance with a known current position, invoking the Rust equivalent of `wordwrap_word_start` results in the stored word-start position exactly matching that current position.

2. **Word end correctness**
   - Given a wrapping-state instance with a known current position, invoking the Rust equivalent of `wordwrap_word_end` results in the stored word-end position exactly matching that current position.

3. **Sequential update correctness**
   - Across repeated start/end operations with changing current positions, the stored boundary positions always reflect the latest invocation's current position.

4. **State isolation correctness**
   - In tests using multiple wrapping-state instances, performing word-boundary operations on one instance leaves the others unchanged.

5. **Traceable structural equivalence**
   - The Rust rewrite contains state and position entities sufficient to express the same boundary-tracking behavior evidenced by `struct wordwrap_file`, `struct position`, `wordwrap_word_start`, and `wordwrap_word_end`.