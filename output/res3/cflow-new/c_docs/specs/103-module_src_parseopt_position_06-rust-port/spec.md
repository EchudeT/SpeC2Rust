# spec.md

## Title

Functional Specification for `module_src_parseopt_position_06` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_position_06`
- Category: `module_cluster`
- Source file: `src/parseopt/wordwrap.c`
- Rust branch: `103-module_src_parseopt_position_06-rust-port`
- Generation date: `2026-06-17`

## Overview

This module defines the position-tracking and line-flush behavior used by the word-wrapping logic in `src/parseopt/wordwrap.c`.

The Rust rewrite must preserve the observed functional role of this module:

- track logical output position using a compact position record,
- compare and combine positions as wrapping decisions are made,
- locate the last whitespace position within a bounded region of buffered text,
- flush buffered text as a wrapped output line while updating tracked positions consistently.

The specification is limited to behavior evidenced by the analyzed functions and data structures in this module slice. It does not introduce new capabilities beyond the source behavior.

## Feature Specification

### Position tracking

The module maintains a `position` value representing text position state used by the word-wrap engine. The Rust version must support:

- initializing a position from a starting unsigned value,
- incrementing a position by a byte count,
- adding one position to another,
- testing two positions for equality.

These operations are used internally to maintain and compare state while scanning buffered text and while flushing wrapped output.

### Whitespace boundary discovery

The module identifies the most recent whitespace boundary within a specified prefix of the word-wrap buffer.

The Rust version must implement behavior equivalent to:

- scanning a bounded amount of buffered content,
- finding the last whitespace position relevant for wrapping,
- returning the resulting position,
- using a caller-provided prior whitespace position as input context.

This behavior is required so wrapping can prefer a whitespace split point instead of breaking at an arbitrary byte boundary.

### Buffered line flush for word wrapping

The module flushes buffered content as a line segment according to word-wrap state.

The Rust version must implement behavior equivalent to:

- consuming a specified amount of buffered content for output,
- using tracked positions and whitespace information to determine the flushed segment,
- updating the word-wrap state after flush,
- reporting success or failure via an integer-style status result consistent with the C behavior.

This functionality is part of the word-wrap engine’s handling of line emission when the current buffered line reaches a wrap decision point.

## User Scenarios & Testing

### Scenario 1: Start tracking a fresh output position

A caller or enclosing word-wrap routine creates a new position state before scanning or emitting text.

Expected support:

- a position can be initialized from a starting offset/count,
- subsequent operations can use that initialized value without extra setup.

Test focus:

- initialization produces the expected baseline state,
- equality comparison against another equally initialized position succeeds.

### Scenario 2: Advance position while scanning bytes

The word-wrap logic scans buffered text and must advance the current position as bytes are examined.

Expected support:

- position state advances by the supplied byte count,
- repeated increments accumulate correctly.

Test focus:

- incrementing by several values produces the same result as incrementing by their sum,
- increments affect later equality and addition results consistently.

### Scenario 3: Combine tracked positions

The wrap engine needs to accumulate position state from one tracked value into another.

Expected support:

- one position can be added into another,
- the result reflects combined position state used by later comparisons and output accounting.

Test focus:

- adding a zero-equivalent initialized position does not change the target,
- adding two independently advanced positions yields the expected accumulated state.

### Scenario 4: Detect the last whitespace before a wrap limit

The wrap engine has buffered content and a size limit representing the candidate flush region. It must find the last whitespace in that region.

Expected support:

- the scan respects the provided size bound,
- the returned position corresponds to the last whitespace found within that bound,
- prior whitespace context supplied by the caller is incorporated consistently.

Test focus:

- input containing whitespace before the limit returns the last such position,
- input with multiple whitespace characters returns the latest one in range,
- input without whitespace does not falsely report a new later whitespace position.

### Scenario 5: Flush a wrapped line segment

When the word-wrap engine decides to emit part of the buffer, it flushes a line segment.

Expected support:

- flush uses the requested size,
- internal position state is updated to reflect consumed content,
- output state remains coherent for subsequent wrapping steps,
- success or failure is reported.

Test focus:

- flushing a non-empty segment advances the tracked state,
- repeated flushes consume successive segments coherently,
- flush returns a failure status when the underlying operation fails in the same situations as the C module.

## Requirements

### Functional Requirements

#### FR-1: Position initialization
The Rust module shall provide behavior equivalent to `position_init` for creating a valid `position` state from an unsigned starting value.

Traceability: `position_init`, `struct position`

#### FR-2: Position advancement
The Rust module shall provide behavior equivalent to `position_incr` for advancing a `position` by a signed byte-count input used by the wrapping logic.

Traceability: `position_incr`, `struct position`

#### FR-3: Position accumulation
The Rust module shall provide behavior equivalent to `position_add` for combining one `position` into another.

Traceability: `position_add`, `struct position`

#### FR-4: Position equality test
The Rust module shall provide behavior equivalent to `position_eq` for determining whether two position values represent the same tracked state.

Traceability: `position_eq`, `struct position`

#### FR-5: Last-whitespace search within bounded buffered content
The Rust module shall provide behavior equivalent to `wordwrap_last_ws` for determining the last whitespace position within a caller-specified prefix of the word-wrap buffer, using the provided prior whitespace position as context.

Traceability: `wordwrap_last_ws`, `struct position`, `struct wordwrap_file`

#### FR-6: Wrapped-line flush
The Rust module shall provide behavior equivalent to `flush_line` for flushing a specified amount of buffered content from the word-wrap state and returning an integer-style success/failure result.

Traceability: `flush_line`, `struct wordwrap_file`, `struct position`

#### FR-7: State consistency across wrap operations
The Rust module shall preserve consistency between the word-wrap buffer state and tracked positions before and after whitespace search and line flush operations.

Traceability: `wordwrap_last_ws`, `flush_line`, `struct wordwrap_file`, `struct position`

### Key Entities

#### `position`
A compact state object used to track logical progress through buffered text for wrapping purposes.

Observed relationships:

- owned and manipulated by helper operations for initialization, increment, accumulation, and equality testing,
- stored within `wordwrap_file`,
- used as both input and output of whitespace-search logic,
- used during line flush accounting.

Traceability: `struct position`, `position_init`, `position_incr`, `position_add`, `position_eq`, `wordwrap_last_ws`, `flush_line`

#### `wordwrap_file`
The word-wrap state holder for buffered output processing.

Observed relationships:

- contains `position` fields used by wrapping logic,
- is the primary state object passed to whitespace search and line flush routines,
- represents the current buffered text and associated wrap-tracking state.

Traceability: `struct wordwrap_file`, `wordwrap_last_ws`, `flush_line`

## Success Criteria

### SC-1: Position helper equivalence
For representative inputs used by the wrapping logic, Rust position initialization, increment, addition, and equality behavior matches the C module’s observable results.

Traceability: `position_init`, `position_incr`, `position_add`, `position_eq`

### SC-2: Correct bounded whitespace detection
Given buffered text and a size bound, the Rust implementation returns the same last-whitespace position as the C module for cases with:
- no whitespace in range,
- one whitespace in range,
- multiple whitespace candidates in range.

Traceability: `wordwrap_last_ws`

### SC-3: Flush behavior equivalence
For the same buffered state and flush size, the Rust implementation produces the same success/failure outcome and equivalent post-flush wrap state as the C module.

Traceability: `flush_line`, `struct wordwrap_file`, `struct position`

### SC-4: Coherent multi-step wrapping state
Across sequences of scan/advance, whitespace detection, and flush operations, the Rust implementation maintains internally consistent position state and does not lose track of the current wrap boundary.

Traceability: `position_incr`, `position_add`, `wordwrap_last_ws`, `flush_line`

### SC-5: Scope fidelity
The Rust port implements only the functionality evidenced in this module slice: position tracking helpers, bounded last-whitespace discovery, and wrapped-line flushing behavior.

Traceability: `src/parseopt/wordwrap.c`, analyzed function set