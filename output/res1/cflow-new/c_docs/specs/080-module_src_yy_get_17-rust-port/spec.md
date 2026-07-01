# spec.md

## Title

Rust Functional Specification for `module_src_yy_get_17`

## Metadata

- Project: `cflow-new`
- Module: `module_src_yy_get_17`
- Category: `module_cluster`
- Source basis: `src/c.c`
- Rust branch target: `080-module_src_yy_get_17-rust-port`
- Generation date: `2026-06-11`

## Overview

This module defines scanner-internal buffer transition behavior for generated lexical analysis state handling. Its scope is limited to:

- determining how scanning proceeds when the active input buffer reaches its current end, and
- reconstructing the scanner state immediately preceding the current buffer position.

The Rust rewrite must preserve the observable behavior of these scanner-internal operations as evidenced by:

- `yy_get_next_buffer`
- `yy_get_previous_state`
- `struct yy_buffer_state`
- `struct yy_trans_info`

This module is not a complete scanner by itself. It serves the scanner runtime by coordinating buffer contents, end-of-buffer markers, and state reconstruction needed for continued token matching.

## Feature Specification

### Feature: End-of-buffer transition handling

The module must support the scanner’s need to continue operation when the current scan position reaches the end of the active buffer.

Behavior required from the Rust version:

- Inspect the active buffer state and determine whether scanning can continue from already-buffered data, requires loading additional input, or has reached final end-of-input.
- Preserve the unprocessed text fragment at the end of the current buffer so it remains available after buffer maintenance.
- Maintain the buffer contract expected by the scanner, including the presence and placement of end-of-buffer sentinel characters.
- Update scanner positions so subsequent matching resumes from the correct location in the active buffer.
- Distinguish the outcomes corresponding to:
  - more buffered input available immediately,
  - input refill required and possible,
  - true end-of-input / last match handling.

The Rust rewrite must match the source module’s functional decisions for these end-of-buffer cases.

### Feature: Previous scanner state reconstruction

The module must support reconstruction of the scanner state associated with the text already traversed in the active buffer prior to the current scan pointer.

Behavior required from the Rust version:

- Start from the scanner’s recorded start state.
- Replay state transitions across the relevant buffered characters up to, but not beyond, the current scan position.
- Respect end-of-buffer transition handling during state replay so the resulting state matches scanner expectations at buffer boundaries.
- Return the state value to be used by scanner control flow after backing up or before requesting further buffer activity.

The Rust rewrite must preserve the same state reconstruction semantics as the source module for the same buffer contents and scanner context.

## User Scenarios & Testing

### Scenario 1: Scanning continues with remaining buffered text

A scanner has consumed characters near the end of the active buffer, but enough text remains to continue without final termination.

Expected module behavior:

- preserve the pending lexeme fragment,
- normalize the buffer contents for continued scanning,
- leave scanner pointers positioned so matching resumes correctly,
- report the nonterminal continuation outcome expected by scanner control flow.

Testing guidance:

- prepare a buffer with a partial lexeme ending near the buffer limit,
- invoke end-of-buffer handling,
- verify that pending characters are retained in order,
- verify that continuation is selected rather than final end-of-input.

### Scenario 2: Scanning reaches end of input

A scanner consumes the active buffer and no further input is available.

Expected module behavior:

- distinguish true end-of-input from a refillable boundary,
- preserve the scanner’s final text boundary semantics,
- return the outcome corresponding to end-of-input / last-match handling.

Testing guidance:

- prepare an active buffer marked as not refillable or exhausted,
- invoke end-of-buffer handling,
- verify that the end-of-input path is selected,
- verify that sentinel placement and scanner positions remain valid for final processing.

### Scenario 3: Scanner state must be reconstructed before buffer transition

The scanner needs the state that would result from replaying transitions over the buffered text preceding the current position.

Expected module behavior:

- begin from the current start state,
- apply transitions across buffered characters up to the current point,
- produce the same resulting state as the original C scanner logic.

Testing guidance:

- provide a known start state and a controlled buffer segment,
- compute the reconstructed state using the Rust module,
- compare the result with the state produced by the original module for the same setup.

### Scenario 4: Buffer boundary includes end-of-buffer marker handling

The current scan path traverses text adjacent to end-of-buffer markers.

Expected module behavior:

- treat end-of-buffer markers according to scanner transition rules,
- avoid consuming sentinel data as normal input,
- return a reconstructed state and next-buffer outcome consistent with generated scanner expectations.

Testing guidance:

- set up buffered data with scanner pointers near sentinel positions,
- exercise both state reconstruction and next-buffer handling,
- verify equivalence with C behavior at the boundary.

## Requirements

### Functional Requirements

#### FR-1: Active buffer end processing
The module shall process the active scanner buffer when the current scan position reaches the effective end of buffered input.
Traceability: `yy_get_next_buffer`, `struct yy_buffer_state`

#### FR-2: Pending text preservation
The module shall preserve the unconsumed or partially matched text segment that must remain available across buffer maintenance.
Traceability: `yy_get_next_buffer`, `struct yy_buffer_state`

#### FR-3: Scanner position update
The module shall update internal scanning positions after buffer maintenance so subsequent matching resumes from the correct location.
Traceability: `yy_get_next_buffer`, `struct yy_buffer_state`

#### FR-4: End-of-buffer outcome selection
The module shall distinguish scanner continuation from terminal end-of-input handling when processing the active buffer boundary.
Traceability: `yy_get_next_buffer`

#### FR-5: Buffer sentinel validity
The module shall maintain valid end-of-buffer sentinel placement required by scanner operation after buffer content movement or refill-related handling.
Traceability: `yy_get_next_buffer`, `struct yy_buffer_state`

#### FR-6: Start-state-based replay
The module shall reconstruct the prior scanner state by beginning from the scanner’s configured start state and replaying transitions across buffered characters preceding the current position.
Traceability: `yy_get_previous_state`

#### FR-7: Transition-table-based state reconstruction
The module shall determine the reconstructed prior state using the scanner transition information associated with the generated automaton.
Traceability: `yy_get_previous_state`, `struct yy_trans_info`

#### FR-8: Boundary-correct state replay
The module shall handle replay across text adjacent to end-of-buffer conditions without treating sentinel markers as ordinary input.
Traceability: `yy_get_previous_state`, `yy_get_next_buffer`

### Key Entities

#### `yy_buffer_state`
Represents the active scanner buffer and its runtime status. Within this module, it is the source of the current buffer contents, scan bounds, and buffer-end conditions used during:

- end-of-buffer processing, and
- prior-state reconstruction.

Relationship:
- consumed by `yy_get_next_buffer` to maintain scanning continuity,
- consulted indirectly by `yy_get_previous_state` through current scan position and buffered text range.

#### `yy_trans_info`
Represents generated scanner transition information used to move between scanner states during replay.

Relationship:
- used by `yy_get_previous_state` to reconstruct the scanner state from buffered characters.

## Success Criteria

1. For representative scanner contexts derived from `src/c.c`, the Rust implementation returns the same end-of-buffer category as the C implementation for:
   - continuation with buffered data,
   - refill/continued scanning path,
   - terminal end-of-input path.
   Traceability: `yy_get_next_buffer`

2. When end-of-buffer handling occurs, the Rust implementation preserves the same retained character sequence and effective scan restart position as the C implementation for the same initial buffer state.
   Traceability: `yy_get_next_buffer`, `struct yy_buffer_state`

3. After end-of-buffer processing, the Rust implementation leaves valid end-of-buffer sentinel structure consistent with scanner expectations for all tested buffer-edge cases covered by the original logic.
   Traceability: `yy_get_next_buffer`, `struct yy_buffer_state`

4. For controlled buffered input samples and start states, the Rust implementation reconstructs the same previous scanner state value as the C implementation.
   Traceability: `yy_get_previous_state`, `struct yy_trans_info`

5. For buffer-boundary test cases involving end-of-buffer markers, the Rust implementation matches the C implementation’s behavior without interpreting sentinel markers as ordinary input during state reconstruction.
   Traceability: `yy_get_previous_state`, `yy_get_next_buffer`

6. The Rust rewrite remains functionally limited to scanner-internal buffer transition and previous-state reconstruction behavior evidenced by the source module, with no required additional public capabilities beyond those functions’ roles.
   Traceability: `yy_get_next_buffer`, `yy_get_previous_state`