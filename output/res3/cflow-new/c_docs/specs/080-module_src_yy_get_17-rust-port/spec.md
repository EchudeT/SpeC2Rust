# spec.md

## Title

Rust Functional Specification for `module_src_yy_get_17`

## Metadata

- Project: `cflow-new`
- Module: `module_src_yy_get_17`
- Category: `module_cluster`
- Source basis: `src/c.c`
- Rust branch: `080-module_src_yy_get_17-rust-port`
- Generation date: 2026-06-17

## Overview

This module covers scanner buffer-advance behavior used during lexical analysis. Its scope is limited to the functionality evidenced by `yy_get_next_buffer` and `yy_get_previous_state`, together with the scanner buffer and transition structures they depend on.

The Rust rewrite must preserve the scanner behavior needed to:

- determine the scanner state immediately preceding end-of-buffer handling,
- prepare the next buffer region for continued scanning,
- distinguish between continued scanning, end-of-file, and scanner failure conditions,
- operate against the existing scanner buffer model represented by `yy_buffer_state`,
- use transition metadata represented by `yy_trans_info` as part of scanner state progression.

This specification does not require new public APIs or capabilities beyond the behavior implied by the source module.

## Feature Specification

### Summary

The module provides internal scanner support for advancing across buffer boundaries and reconstructing scanner state from the already matched text. In the Rust version, this functionality must support a lexer runtime that uses a refillable character buffer with end-of-buffer sentinels and a state-machine-based transition model.

### In-Scope Behavior

The Rust module must implement the following behaviors:

1. **Reconstruct previous scanner state from buffered input**
   - Given the scanner’s current matched text region, compute the lexical state reached just before end-of-buffer processing is needed.
   - This reconstruction must follow the scanner transition model represented by `yy_trans_info`.

2. **Advance scanning when the current buffer region is exhausted**
   - When the scanner reaches the end of the active buffer content, determine whether more input remains, whether the scan is complete, or whether the condition should be treated as a fatal/no-progress case.
   - Preserve any unmatched or partially matched text that must remain available across the buffer transition.

3. **Manage buffer continuation semantics**
   - Support the standard scanner distinction between:
     - more data available for scanning,
     - end of input,
     - inability to continue due to invalid buffer advancement state.
   - Maintain the buffer content arrangement required for subsequent scanning after refill or compaction.

4. **Honor buffer ownership and status encoded in scanner buffer state**
   - Use the active `yy_buffer_state` to decide how continuation and end-of-input handling behave.
   - Respect whether the buffer is a normal input buffer versus a special end condition, as evidenced by the presence of scanner buffer state structures in the source.

### Behavioral Boundaries

The Rust rewrite must not assume responsibilities beyond this module’s evidenced scope. In particular, this specification does not require:

- token classification logic,
- scanner rule actions,
- creation or destruction of buffers outside what is necessary to support these functions,
- external I/O API redesign,
- concurrency guarantees.

## User Scenarios & Testing

### Scenario 1: Continue scanning after reaching end of current buffer contents

A scanner has consumed the active buffer up to the end-of-buffer sentinel while processing a long input stream. The runtime invokes the buffer-advance logic. The module preserves the necessary remaining text, prepares the next readable region, and signals that scanning can continue.

**Expected outcome**
- The scanner receives a “continue scanning” result.
- Previously matched text needed for correctness remains intact.
- The scanner can resume without losing state continuity.

**Test focus**
- Buffer with remaining source input available.
- Non-empty matched prefix crossing the buffer boundary.
- Subsequent scan step begins from the reconstructed state.

### Scenario 2: Detect end of input at buffer boundary

A scanner reaches the end of the available input and no additional characters can be supplied. The module processes the buffer boundary and reports end-of-file semantics.

**Expected outcome**
- The scanner receives an end-of-input result.
- No false continuation is reported.
- State handling remains valid for final token/end processing.

**Test focus**
- Buffer marked or behaving as final input region.
- No additional readable characters beyond sentinels.
- Stable EOF result on repeated boundary handling.

### Scenario 3: Reconstruct state before end-of-buffer transition

A token match extends up to a buffer boundary, and the lexer must know the state reached by consuming the buffered characters before deciding how to proceed. The module recomputes the previous state from the current text.

**Expected outcome**
- The reconstructed state matches the state that would have been reached by replaying the buffered characters through the scanner automaton.
- Subsequent transition handling is consistent with continued lexing.

**Test focus**
- Buffered text containing multiple character classes or transition paths.
- Validation against expected DFA/NFA-derived state progression from the original scanner tables.

### Scenario 4: Handle non-progress/failure condition at buffer advancement

The scanner requests the next buffer state, but the active conditions do not permit valid continuation or EOF processing according to the module’s internal contract.

**Expected outcome**
- The module returns the failure/status code corresponding to this condition.
- The scanner does not incorrectly continue scanning.

**Test focus**
- Boundary case where no valid movement or refill outcome exists.
- Verification that failure is distinguishable from normal continuation and EOF.

## Requirements

### Functional Requirements

#### FR-1: Previous-state reconstruction
The module shall compute the scanner state immediately preceding end-of-buffer handling by replaying or otherwise evaluating the already buffered matched text against the scanner transition model.

**Traceability**
- Function: `yy_get_previous_state` (`src/c.c:1884-1915`)
- Type: `struct yy_trans_info` (`src/c.c:440-444`)

#### FR-2: Buffer-boundary advancement
The module shall process scanner arrival at the end of the active buffer and determine the next scanner outcome for continuation.

**Traceability**
- Function: `yy_get_next_buffer` (`src/c.c:1741-1878`)
- Type: `struct yy_buffer_state` (`src/c.c:191`, `233-298`, and later references)

#### FR-3: Distinct continuation outcomes
The module shall provide distinct outcomes for:
- continued scanning with more buffer content available,
- end of input,
- failure or non-progress condition.

**Traceability**
- Function: `yy_get_next_buffer` (`src/c.c:1741-1878`)

#### FR-4: Preservation of required buffered text across refill/shift
The module shall preserve the portion of buffered text needed to maintain correct scanning semantics when moving to the next buffer region.

**Traceability**
- Function: `yy_get_next_buffer` (`src/c.c:1741-1878`)
- Type: `struct yy_buffer_state`

#### FR-5: Dependence on active buffer state
The module shall base boundary-handling decisions on the active scanner buffer state, including buffer content and status relevant to end-of-input behavior.

**Traceability**
- Function: `yy_get_next_buffer` (`src/c.c:1741-1878`)
- Type: `struct yy_buffer_state`

#### FR-6: Compatibility with state-machine-driven lexing
The module shall operate consistently with a transition-table scanner model in which the current scanner state and buffered characters determine the next state.

**Traceability**
- Functions: `yy_get_next_buffer`, `yy_get_previous_state`
- Type: `struct yy_trans_info`

### Key Entities

#### `yy_buffer_state`
Represents the active scanner buffer and its control state. It is the primary entity for deciding whether more input can be scanned, how buffered text is retained, and how end-of-input is recognized.

**Role in this module**
- Supplies the current buffer content context for boundary processing.
- Carries the state needed to interpret whether refill/continuation is possible.
- Serves as the storage context whose contents are rearranged or examined during next-buffer handling.

#### `yy_trans_info`
Represents scanner transition information used by the lexical state machine.

**Role in this module**
- Provides the transition basis for reconstructing the prior scanner state from buffered text.
- Connects character consumption history to scanner-state recovery.

#### Relationship between entities
- `yy_get_previous_state` uses buffered text context together with `yy_trans_info` to recover a scanner state.
- `yy_get_next_buffer` uses `yy_buffer_state` to decide how scanning continues at the buffer boundary.
- Together, these entities and behaviors maintain scanner continuity across end-of-buffer events.

## Success Criteria

1. **Correct previous-state recovery**
   - For test inputs that reach a buffer boundary, the Rust implementation returns the same logical pre-boundary scanner state as the C module’s `yy_get_previous_state`.
   - Traceability: `yy_get_previous_state`, `yy_trans_info`

2. **Correct continuation signaling**
   - For buffer-boundary cases with additional input available, the Rust implementation signals continuation rather than EOF or failure.
   - Traceability: `yy_get_next_buffer`

3. **Correct EOF signaling**
   - For buffer-boundary cases with no additional input available, the Rust implementation signals EOF behavior consistent with the C module.
   - Traceability: `yy_get_next_buffer`, `yy_buffer_state`

4. **Correct failure/non-progress signaling**
   - For boundary cases that the source module treats as non-continuable, the Rust implementation returns the corresponding distinct failure/status outcome.

5. **Buffered text continuity across boundary handling**
   - In tests where a token or match spans a buffer boundary, the Rust implementation preserves the required buffered text so that scanning after the boundary is behaviorally equivalent to the C module.

6. **No scope expansion beyond evidenced behavior**
   - The Rust module implements the internal scanner buffer/state behaviors covered by this specification without introducing unrelated module responsibilities.
   - Traceability: module scope defined by `yy_get_next_buffer`, `yy_get_previous_state`, `yy_buffer_state`, and `yy_trans_info`