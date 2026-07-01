# spec.md

## Title
Rust Functional Specification for `main_root_quoting_options_02`

## Metadata
- Project: `pwd`
- Module: `main_root_quoting_options_02`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Primary function in scope: `quotearg_n_custom_mem`
- Core types in scope: `quoting_options`, `slotvec`
- Rust branch target: `002-main_root_quoting_options_02-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides argument quoting behavior driven by quoting options, with the specific in-scope behavior centered on producing a quoted representation of an input memory region using caller-supplied left and right quote delimiters.

The Rust rewrite must preserve the observable behavior of the C module portion evidenced here:
- accept an argument as a byte sequence with explicit length,
- apply custom quoting delimiters through quoting options,
- select the quoting operation by slot index `n`,
- return the quoted result for that slot.

The module also relies on persistent quoting configuration and slot-based result storage as evidenced by the presence of `quoting_options` and `slotvec`.

## Feature Specification

### In-Scope Feature
The Rust module shall implement custom-memory quoting equivalent to `quotearg_n_custom_mem`:
- input consists of:
  - a slot number `n`,
  - a left quote string,
  - a right quote string,
  - an argument buffer,
  - an explicit argument size;
- behavior consists of:
  - creating quoting options configured for custom quoting,
  - applying those options to the provided memory region,
  - producing a quoted output associated with the requested slot.

### Required Behavioral Boundaries
The Rust rewrite must preserve these evidenced functional boundaries:
1. **Explicit-length input handling**
   The argument is treated as a memory region identified by pointer plus size in C; therefore the Rust version must support quoting data by explicit length rather than relying only on terminator-based string length.
2. **Custom delimiter quoting**
   The output must use the caller-supplied left and right quote strings.
3. **Slot-indexed result selection**
   The operation is parameterized by integer slot `n`, indicating that the quoted result is managed in a slot-oriented manner.
4. **Quoting options mediation**
   Custom quoting is expressed through a `quoting_options` entity, so Rust behavior must preserve the notion that quoting style is determined by options rather than hardcoded formatting.

### Out of Scope
The Rust specification does not require any capability not evidenced by the input, including:
- new public APIs beyond behavior needed to cover the in-scope function,
- guarantees about concurrency,
- serialization or persistence,
- FFI interfaces,
- recovery features,
- performance targets.

## User Scenarios & Testing

### Scenario 1: Quote a fixed-size argument with custom delimiters
A caller has an input buffer and its exact size, and needs a quoted representation using custom opening and closing quote strings.

**Expected behavior**
- The full specified memory region is processed.
- The resulting representation is surrounded by the provided left and right delimiters.
- The operation succeeds through the slot designated by `n`.

**Test focus**
- Provide a byte sequence and explicit size.
- Verify the output begins with the left quote and ends with the right quote.
- Verify no truncation occurs when the input contains data covered by the explicit size.

### Scenario 2: Distinguish slot-based results
A caller performs quoting operations using different slot numbers and expects each operation to address its designated result slot.

**Expected behavior**
- Calling the operation with different `n` values yields results associated with those slot selections.
- Reusing a slot updates that slot’s current quoted result.

**Test focus**
- Quote one input with slot `0`, another with slot `1`.
- Verify both slot-directed operations produce the expected quoted content.
- Repeat with the same slot and verify the returned result reflects the later call for that slot.

### Scenario 3: Quote data that is not represented solely by C-string termination
A caller needs quoting for input where the effective length is externally specified.

**Expected behavior**
- The operation respects `argsize`.
- The quoted output corresponds to exactly the specified input extent.

**Test focus**
- Pass input with a length chosen independently of any trailing terminator assumptions.
- Verify the quoted output matches the intended slice length.

### Scenario 4: Apply custom quoting through options-defined behavior
A caller relies on quoting semantics being driven by a quoting options object configured for custom left and right delimiters.

**Expected behavior**
- The output reflects custom quoting configuration rather than default quoting.
- Changing the custom delimiters changes the output framing accordingly.

**Test focus**
- Run the same input with one pair of delimiters, then another.
- Verify only the delimiter framing changes as dictated by the provided custom quote strings.

## Requirements

### Functional Requirements
- **FR-1: Custom memory quoting**
  - The module shall provide behavior equivalent to `quotearg_n_custom_mem`, accepting a slot index, custom left quote, custom right quote, input memory, and explicit input length.
  - Traceability: `quotearg.c`, `quotearg_n_custom_mem`.

- **FR-2: Explicit-length processing**
  - The module shall process the argument according to the provided size parameter rather than requiring implicit termination-based length discovery.

- **FR-3: Caller-defined quote delimiters**
  - The module shall support custom left and right quote strings that determine the surrounding quotation applied to the result.
  - Traceability: `quotearg.c`, `quotearg_n_custom_mem`; `quoting_options`.

- **FR-4: Option-driven quoting configuration**
  - The module shall represent quoting behavior through a quoting-options entity sufficient to express custom quoting.
  - Traceability: `quotearg.c`; `quoting_options`; use of local `quoting_options` in the function scope referenced at and around line 1025.

- **FR-5: Slot-based result behavior**
  - The module shall preserve slot-oriented quoting behavior indicated by parameter `n` and the presence of slot storage structures.
  - Traceability: `quotearg.c`, `quotearg_n_custom_mem`; `slotvec`.

### Key Entities
- **`quoting_options`**
  - Represents the configuration controlling how quoting is applied.
  - In this module scope, it must be able to represent custom quoting delimiters.
  - Relationship: used to parameterize the quoting operation performed by `quotearg_n_custom_mem`.

- **`slotvec`**
  - Represents slot-based storage or management for quoted results.
  - Relationship: works with the slot index `n` to identify or retain the result associated with a particular quoting slot.

- **Quoted result associated with slot `n`**
  - Observable output of the module operation.
  - Relationship: produced by applying `quoting_options` to the specified input memory and stored or selected according to `slotvec` behavior.

## Success Criteria
- **SC-1**
  - For a given input memory region and explicit length, the Rust module produces a quoted result that uses the supplied left and right delimiters.
  - Traceability: `quotearg_n_custom_mem`.

- **SC-2**
  - Tests demonstrate that the Rust module respects explicit input length when forming the quoted result.

- **SC-3**
  - Tests demonstrate that varying the custom left and right quote strings changes the output framing accordingly.
  - Traceability: `quotearg_n_custom_mem`; `quoting_options`.

- **SC-4**
  - Tests demonstrate slot-directed behavior by invoking the operation with at least two slot indices and observing correct result association per slot.
  - Traceability: `quotearg_n_custom_mem`; `slotvec`.

- **SC-5**
  - The Rust rewrite exposes no required behavior beyond the evidenced boundaries of custom memory quoting, option-driven custom delimiters, and slot-based result selection.
  - Traceability: `quotearg.c`, `quotearg_n_custom_mem`, `quoting_options`, `slotvec`.