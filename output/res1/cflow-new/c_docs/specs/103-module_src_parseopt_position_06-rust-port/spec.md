# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_position_06`

## Overview

This module defines the position-tracking and whitespace-break selection behavior used by the word-wrapping logic in `src/parseopt/wordwrap.c`. The analyzed functionality covers:

- tracking a logical output position,
- comparing and combining positions,
- locating the last usable whitespace break within buffered text,
- flushing buffered content as wrapped output based on that break analysis.

The Rust rewrite must preserve the same observable behavior for these responsibilities within the word-wrap subsystem of `cflow-new`, on branch `103-module_src_parseopt_position_06-rust-port`.

## Scope

In scope for this specification:

- the `position` value semantics used by the module,
- the relation between buffered text scanning and whitespace break detection,
- line flushing behavior that depends on the detected break position and tracked positions.

Out of scope:

- unrelated option parsing behavior,
- terminal width discovery beyond what is already consumed by this module,
- any new public API or capabilities not evidenced by `src/parseopt/wordwrap.c`.

## Feature Specification

### Summary

The module provides the internal behavior required to manage wrapped output positions and to decide where a buffered line can be broken at whitespace. It must support:

1. initializing and updating logical positions,
2. combining positions from multiple segments,
3. testing position equality,
4. scanning buffered content to find the last whitespace location suitable for wrapping,
5. flushing buffered content according to that wrap decision.

### Functional Behavior

#### Position tracking

The module must represent a logical position associated with buffered output. This position is used by the wrapping logic to measure and compare where output segments begin or end.

The Rust version must implement the same behavior as the C helpers:

- initialize a position from an input width/count,
- increment a position by a byte count,
- add one position into another,
- test whether two positions are equal.

These operations are internal building blocks for the wrapping path and must preserve the semantics expected by the surrounding word-wrap state.

#### Whitespace break discovery

When the wrapping logic needs to decide whether buffered text can be split, the module must scan the relevant buffered range and identify the last whitespace location that can serve as a wrap boundary.

The Rust version must preserve these behaviors evidenced by `wordwrap_last_ws`:

- inspect buffered text up to a specified size,
- track the most recent whitespace position encountered,
- return the resulting position information for the last usable whitespace,
- use caller-supplied prior whitespace position state as part of the decision flow.

This behavior exists to support line breaking at whitespace rather than at arbitrary byte positions.

#### Line flush based on wrap position

When buffered text is ready to be emitted, the module must flush a line in a way that respects the tracked positions and whitespace break analysis.

The Rust version must preserve these behaviors evidenced by `flush_line`:

- evaluate the buffered range to determine how much content should be flushed,
- use the last-whitespace position information when selecting the break point,
- update internal position-related state consistently with the emitted content,
- report success or failure through the function result.

The exact internal mechanics may differ in Rust, but the externally observable line-breaking decisions and state progression must remain consistent with the C module’s role.

## User Scenarios & Testing

### Scenario 1: Initialize a fresh wrapping position

A caller creates or resets word-wrap state for a new line or segment. The module initializes a position from a starting count so that later wrapping calculations begin from the correct logical location.

The Rust version must support tests that verify:

- a newly initialized position matches the supplied starting value,
- subsequent operations use that initialized value as their baseline.

Traceability: `position_init`, `struct position`.

### Scenario 2: Advance position as buffered bytes are processed

As text is appended or emitted, the wrapping subsystem advances the current position by the number of bytes processed.

The Rust version must support tests that verify:

- incrementing by a positive byte count advances the position consistently,
- repeated increments accumulate correctly.

Traceability: `position_incr`, `struct position`.

### Scenario 3: Combine positions from adjacent segments

The wrapping logic needs to merge position information from one segment into another when calculating the total position over buffered content.

The Rust version must support tests that verify:

- adding one position to another produces the same combined logical result as the C behavior,
- the destination position reflects the cumulative value after multiple additions.

Traceability: `position_add`, `struct position`.

### Scenario 4: Detect whether two positions are the same

The module compares tracked positions to decide whether two logical locations are identical.

The Rust version must support tests that verify:

- equality returns true for positions with the same logical value,
- equality returns false for positions with different logical values.

Traceability: `position_eq`, `struct position`.

### Scenario 5: Find the last whitespace in buffered text

A buffered line contains a mix of words and spaces. Before wrapping, the module scans the buffer and identifies the last whitespace that can serve as the wrap point within the requested size.

The Rust version must support tests that verify:

- when whitespace exists in the scanned range, the returned position corresponds to the last such whitespace,
- when multiple whitespace characters exist, the latest eligible one is chosen,
- prior whitespace state passed by the caller is correctly incorporated into the returned result.

Traceability: `wordwrap_last_ws`, `struct wordwrap_file`, `struct position`.

### Scenario 6: Flush a line using whitespace-aware wrapping

A buffered line reaches a width where output must be emitted. The module determines whether to break at the last tracked whitespace and flushes the appropriate portion of the line.

The Rust version must support tests that verify:

- line flushing uses whitespace-aware break selection rather than ignoring available whitespace,
- internal tracked position state changes consistently after the flush,
- the flush operation returns a status indicating success or failure.

Traceability: `flush_line`, `wordwrap_last_ws`, `struct wordwrap_file`, `struct position`.

## Requirements

### Functional Requirements

#### FR-1: Position initialization
The module shall initialize a `position` value from a caller-provided unsigned starting count for use by the wrapping subsystem.

Traceability: `position_init`, `struct position`.

#### FR-2: Position increment
The module shall advance a `position` by a caller-provided byte count.

Traceability: `position_incr`, `struct position`.

#### FR-3: Position accumulation
The module shall support adding one `position` value into another to form a cumulative logical position.

Traceability: `position_add`, `struct position`.

#### FR-4: Position equality check
The module shall support testing whether two `position` values are equal.

Traceability: `position_eq`, `struct position`.

#### FR-5: Last-whitespace search within buffered content
The module shall examine buffered content associated with a `wordwrap_file` up to a caller-provided size and determine the last whitespace break position relevant to wrapping.

Traceability: `wordwrap_last_ws`, `struct wordwrap_file`, `struct position`.

#### FR-6: Use of caller-provided whitespace state
The module shall accept prior whitespace position state during last-whitespace analysis and incorporate that state into the computed result.

Traceability: `wordwrap_last_ws`, `struct position`.

#### FR-7: Whitespace-aware line flush
The module shall flush buffered line content for a `wordwrap_file` using the wrap decision derived from tracked positions and last-whitespace analysis.

Traceability: `flush_line`, `wordwrap_last_ws`, `struct wordwrap_file`, `struct position`.

#### FR-8: Flush result reporting
The module shall report line-flush success or failure through its return value.

Traceability: `flush_line`.

#### FR-9: Position state consistency across flushes
The module shall maintain consistent `position`-related state in `wordwrap_file` as content is flushed.

Traceability: `flush_line`, `struct wordwrap_file`, `struct position`.

### Key Entities

#### `position`
A compact internal value representing a logical output position used by the wrapping subsystem.

Relationships:

- stored within `wordwrap_file`,
- initialized, incremented, accumulated, and compared by the position helper functions,
- produced and consumed during whitespace search and line flush decisions.

Traceability: `struct position`, `position_init`, `position_incr`, `position_add`, `position_eq`, `wordwrap_last_ws`, `flush_line`.

#### `wordwrap_file`
The internal word-wrap state holder for buffered output and position-tracking data.

Relationships:

- owns or references the buffered content scanned for whitespace,
- contains `position` fields used for wrap calculations,
- is the primary state object passed to whitespace search and line flush operations.

Traceability: `struct wordwrap_file`, `wordwrap_last_ws`, `flush_line`.

## Success Criteria

### SC-1: Position helper parity
For representative inputs used by the wrapping subsystem, Rust position initialization, increment, addition, and equality behavior shall match the C module’s observable results.

Traceability: `position_init`, `position_incr`, `position_add`, `position_eq`.

### SC-2: Correct last-whitespace selection
Given buffered text and a scan size, the Rust implementation shall identify the same last usable whitespace position as the C implementation for cases with no whitespace, one whitespace, and multiple whitespace characters.

Traceability: `wordwrap_last_ws`, `struct position`, `struct wordwrap_file`.

### SC-3: Flush behavior parity
For buffered line-flush cases that depend on whitespace-aware wrapping, the Rust implementation shall emit or select the same flush boundary and return the same success/failure outcome as the C implementation.

Traceability: `flush_line`, `wordwrap_last_ws`.

### SC-4: State progression consistency
After line flush operations, the Rust implementation shall leave position-related `wordwrap_file` state consistent with the amount of content flushed and suitable for continued wrapping.

Traceability: `flush_line`, `struct wordwrap_file`, `struct position`.

### SC-5: Scenario coverage
Automated tests for the Rust rewrite shall cover all scenarios listed in this specification.

Traceability: all scenarios above.