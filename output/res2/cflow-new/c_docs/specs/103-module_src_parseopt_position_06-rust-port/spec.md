# spec.md

## Title

Functional Specification for `module_src_parseopt_position_06` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_parseopt_position_06`
- Category: `module_cluster`
- Source basis: `src/parseopt/wordwrap.c`
- Rust branch: `103-module_src_parseopt_position_06-rust-port`
- Generation date: `2026-06-17`

## Overview

This module defines and uses position-tracking behavior inside the word-wrapping subsystem. Its functional role is to represent a location within buffered text, update that location as bytes are consumed, compare locations, identify the last usable whitespace position within a range, and flush buffered text according to wrapping boundaries.

The Rust rewrite must preserve the same observable behavior within the word-wrapping flow:

- maintain position state for buffered text,
- advance and combine positions consistently,
- test position equality,
- locate a whitespace break candidate within a bounded buffered region,
- flush buffered content while respecting tracked positions and wrapping decisions.

This specification is limited to behavior evidenced by `src/parseopt/wordwrap.c`.

## Scope

### In Scope

- Position initialization, advancement, accumulation, and equality checks.
- Use of position state within a word-wrap file context.
- Detection of the last whitespace position in a buffered segment.
- Flushing of buffered text based on tracked positions and wrap boundaries.

### Out of Scope

- Any capabilities not evidenced in this module slice, including new public APIs, persistence, concurrency guarantees, recovery features, or foreign-function interfaces.
- General terminal sizing behavior except where it is already consumed by the existing word-wrapping state.
- Any formatting features beyond the position-aware wrapping and flush behavior shown in `wordwrap.c`.

## Feature Specification

### Feature: Position Tracking for Buffered Word Wrapping

The module must provide behavior equivalent to a small position model used by the word-wrapping logic. A position represents progress within buffered output and is updated as text is examined or emitted.

The Rust version must support:

- creating a position from an initial unsigned value,
- incrementing a position by a byte count,
- adding one position value into another,
- checking whether two positions are equal.

These operations must behave consistently wherever the wrapping logic uses them to track current location, whitespace location, and buffer offsets.

### Feature: Whitespace Break Discovery

The module must support scanning a bounded portion of the word-wrap buffer to determine the last whitespace location that can serve as a wrap point.

The Rust version must implement equivalent behavior to `wordwrap_last_ws`:

- inspect buffered content associated with a word-wrap file state,
- limit inspection to the supplied size bound,
- use position tracking during the scan,
- return the discovered position of the last matching whitespace candidate within that bound,
- use the provided prior whitespace position input as part of the decision flow.

Only behavior evidenced by the function’s role in the wrapping pipeline is required; no broader tokenization or Unicode-aware expansion is implied.

### Feature: Buffered Line Flush with Wrap-Aware Position Use

The module must support flushing buffered text from the word-wrap file state while using tracked positions and whitespace boundaries to decide how much text is emitted.

The Rust version must implement equivalent behavior to `flush_line`:

- operate on the current word-wrap file state and a requested size,
- determine whether a buffered line segment can be flushed directly or should be split at a previously identified whitespace position,
- use position values maintained in the file state and intermediate scan results,
- update the word-wrap file state to reflect the flushed content,
- report success or failure through an integer-equivalent result consistent with the C behavior.

The Rust port must preserve externally observable wrapping behavior for the same buffered input and width-related state.

## User Scenarios & Testing

### Scenario 1: Initialize and Advance a Position During Buffer Processing

A wrapping routine starts tracking a new segment at a known starting offset and advances the tracked location as bytes are processed.

Expected support:

- a position can be initialized from a starting count,
- repeated byte increments produce the same resulting position progression as the C module,
- equality checks can confirm that the current location matches an expected location.

### Scenario 2: Accumulate Relative Position Information

A wrapping routine computes a local position while scanning a subsection of text, then adds that local position to a base position representing the start of the subsection.

Expected support:

- one position can be added into another,
- the resulting combined position matches the position that would have been obtained by direct progression from the base.

### Scenario 3: Detect the Last Whitespace Break in a Bounded Range

A buffered line contains one or more whitespace characters before a wrap limit. The wrapping logic needs the last valid whitespace location within that limited region.

Expected support:

- the module can scan up to the provided size boundary,
- the last whitespace position within that bound is returned,
- if multiple whitespace characters exist, the last one within the bound is selected,
- the returned position is suitable for subsequent flush decisions.

### Scenario 4: Flush a Buffered Line Without Splitting

The buffered content to be flushed does not require wrapping at a whitespace boundary within the requested size.

Expected support:

- the flush operation emits or advances through the requested content as a direct flush path,
- position-related state in the word-wrap file is updated consistently,
- the result indicates success when the underlying flush succeeds.

### Scenario 5: Flush a Buffered Line by Breaking at Whitespace

The buffered content exceeds the preferred direct flush extent, but a prior whitespace location exists within the usable range.

Expected support:

- the flush operation uses the last whitespace position as the break point,
- content before the break is flushed first,
- internal state reflects consumption of the flushed portion and the remaining buffered content,
- the behavior matches the C module’s wrap-aware line flushing outcome.

### Scenario 6: No Usable Whitespace Found in Range

A bounded scan is requested, but no whitespace candidate exists within that range.

Expected support:

- the whitespace search returns a result consistent with the module’s position-tracking rules when no newer usable whitespace is found,
- the flush logic remains able to choose its non-whitespace handling path as in the C implementation.

### Testing Notes

The Rust version must be testable with fixture-driven cases built from buffered text and position state. At minimum, tests should cover:

- position initialization, increment, addition, and equality,
- bounded whitespace search with zero, one, and multiple whitespace candidates,
- flush behavior when the requested size falls before or after the last usable whitespace,
- state updates before and after flush operations,
- success/failure result propagation from flush logic.

## Requirements

### Functional Requirements

#### FR-1: Position Initialization
The module shall initialize a position from a supplied unsigned starting value, producing a valid initial position state for later wrapping operations.

Traceability: `position_init`, `struct position`

#### FR-2: Position Increment
The module shall advance a position by a supplied byte count during buffered text processing.

Traceability: `position_incr`, `struct position`

#### FR-3: Position Accumulation
The module shall add one position value into another so that relative scan progress can be merged into a base buffered position.

Traceability: `position_add`, `struct position`

#### FR-4: Position Equality Test
The module shall determine whether two positions are equal for control-flow decisions in the wrapping process.

Traceability: `position_eq`, `struct position`

#### FR-5: Word-Wrap State Integration
The module shall maintain position values as part of the word-wrap file state and use them during whitespace discovery and flush decisions.

Traceability: `struct wordwrap_file`, `wordwrap_last_ws`, `flush_line`

#### FR-6: Bounded Last-Whitespace Search
The module shall search buffered content associated with a word-wrap file state up to a supplied size limit and determine the last whitespace position found within that bound.

Traceability: `wordwrap_last_ws`, `struct wordwrap_file`, `struct position`

#### FR-7: Prior Whitespace Context Use
The module shall accept a prior whitespace position input during the bounded last-whitespace search and incorporate it into the returned position result consistent with the source behavior.

Traceability: `wordwrap_last_ws`

#### FR-8: Wrap-Aware Flush Decision
The module shall flush buffered content for a supplied size using position-tracked information and whitespace break discovery to choose the flush boundary.

Traceability: `flush_line`, `wordwrap_last_ws`, `struct wordwrap_file`

#### FR-9: Flush State Update
After a flush operation, the module shall update the word-wrap file state so that buffered positions remain consistent with the amount of content flushed.

Traceability: `flush_line`, `struct wordwrap_file`, `struct position`

#### FR-10: Flush Result Reporting
The module shall report success or failure from the flush operation in a form equivalent to the C module’s integer return behavior.

Traceability: `flush_line`

### Key Entities

#### `position`
A compact position value used to represent progress within buffered word-wrap processing. It is the core unit for:

- initialization from a starting count,
- movement by byte increments,
- accumulation with another position,
- equality comparison,
- marking whitespace and flush boundaries.

Relationships:

- used directly by position helper functions,
- stored within the word-wrap file state,
- returned by whitespace search logic,
- consumed by flush logic to select breakpoints and update progress.

Traceability: `struct position`, `position_init`, `position_incr`, `position_add`, `position_eq`, `wordwrap_last_ws`, `flush_line`

#### `wordwrap_file`
The state holder for the word-wrapping subsystem in this module slice. It contains position fields that represent current tracked locations relevant to buffered wrapping and flushing.

Relationships:

- supplies the buffered context scanned by last-whitespace detection,
- owns the position state updated during flushing,
- is the principal mutable context passed to both `wordwrap_last_ws` and `flush_line`.

Traceability: `struct wordwrap_file`, `wordwrap_last_ws`, `flush_line`

## Success Criteria

### SC-1: Position Operation Equivalence
For representative inputs, the Rust implementation produces the same position results as the C logic for initialization, increment, accumulation, and equality decisions.

Traceability: `position_init`, `position_incr`, `position_add`, `position_eq`

### SC-2: Correct Last-Whitespace Selection
For buffered text with multiple whitespace candidates within a supplied bound, the Rust implementation returns the last candidate within that bound, matching the C module’s behavior.

Traceability: `wordwrap_last_ws`

### SC-3: Stable Behavior When No New Whitespace Exists
For bounded scans with no usable whitespace in range, the Rust implementation returns a position result consistent with the C behavior and does not invent a new break location.

Traceability: `wordwrap_last_ws`

### SC-4: Wrap-Aware Flush Equivalence
Given the same buffered content, size input, and word-wrap state, the Rust implementation flushes at the same effective boundary as the C module, including whitespace-based breaks when applicable.

Traceability: `flush_line`, `wordwrap_last_ws`

### SC-5: State Consistency After Flush
After each successful flush, the Rust implementation leaves the word-wrap state’s tracked positions consistent with the amount of buffered content consumed.

Traceability: `flush_line`, `struct wordwrap_file`, `struct position`

### SC-6: Result Code Preservation
The Rust implementation preserves success/failure outcomes of flush operations in a way that is behaviorally equivalent to the C module.

Traceability: `flush_line`

### SC-7: Scenario Coverage
Automated tests for the Rust port cover all scenarios listed in this document and demonstrate conformance for both normal and boundary cases.

Traceability: all functions and types listed in this specification