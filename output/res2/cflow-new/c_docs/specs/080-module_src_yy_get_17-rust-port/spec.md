# spec.md

## Title

Rust Functional Specification for `module_src_yy_get_17`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_yy_get_17`
- **Category**: `module_cluster`
- **Rust branch**: `080-module_src_yy_get_17-rust-port`
- **Source evidence date**: `2026-06-17`

## Overview

This module covers scanner buffer-advance and prior-state reconstruction behavior used by the generated lexical scanner in `src/c.c`. The Rust rewrite must preserve the functional role of the original logic that:

- determines how scanning continues when the current buffer reaches an end-of-buffer condition, and
- reconstructs the scanner state that existed immediately before the current end-of-buffer position.

The module’s scope is limited to behavior evidenced by:

- `yy_get_next_buffer` in `src/c.c:1741-1878`
- `yy_get_previous_state` in `src/c.c:1884-1915`
- scanner buffer state structures named `yy_buffer_state`
- scanner transition structure `yy_trans_info`

No additional capabilities are in scope beyond these scanner-internal responsibilities.

## Feature Specification

### Feature: End-of-buffer transition handling

The Rust module must implement the scanner behavior that is triggered when the current scan position reaches the end-of-buffer sentinel region.

This behavior includes:

- preserving the already matched text that remains valid at the time of refill or termination decision,
- determining whether additional input must be made available before scanning can continue,
- distinguishing between:
  - scanning can continue from a refilled or shifted buffer,
  - the scanner has reached final end-of-input for the current source, or
  - the current match requires a special end-of-buffer retry path,
- updating scanner buffer position information so later scanning resumes from the correct location in the active buffer.

The Rust version must preserve the observable scanner control outcome of this step, including the same class of continuation/termination decisions as the C module.

### Feature: Previous-state reconstruction

The Rust module must implement recovery of the scanner automaton state that corresponds to the text immediately preceding the current end-of-buffer position.

This behavior includes:

- traversing the currently matched text from the active buffer start position up to the current character position,
- applying the scanner’s transition information to reconstruct the automaton state reached by that text,
- producing the prior scanner state needed so end-of-buffer handling can decide the next scanner action correctly.

The Rust version must preserve the same state reconstruction result for the same scanner tables, buffer contents, and current buffer positions.

### Feature: Coordination with scanner buffer state

The Rust module must operate against scanner buffer state data that represents:

- the active character buffer,
- the current scan position,
- the amount of valid text present in the buffer,
- buffer ownership and refill-related flags where needed by end-of-buffer handling.

The Rust rewrite must preserve the buffer-state-driven behavior required by the two source functions without broadening module scope.

## User Scenarios & Testing

### Scenario 1: Continue scanning when more input is available

A scanner is processing input and reaches the end-of-buffer sentinel while the underlying input source has not been exhausted.

Expected module behavior:

- the remaining matched text is retained correctly,
- buffer state is adjusted so unread space can be reused or new input can be appended,
- the scanner receives the control result indicating that scanning should continue with newly available input,
- the prior scanner state used for continuation matches the state implied by the already matched text.

Testing guidance:

- initialize a buffer with partial token text ending at the end-of-buffer region,
- provide additional input availability,
- verify that the returned control path is “continue scanning,” not final EOF,
- verify that the preserved prefix text and resumed position yield the same subsequent tokenization as the C behavior.

### Scenario 2: Final end-of-input is reached

A scanner reaches the end-of-buffer sentinel and no further input is available for the current buffer source.

Expected module behavior:

- the module identifies true end-of-input,
- scanner state is left in a form consistent with end-of-input processing,
- the returned control result indicates final input exhaustion rather than refill continuation.

Testing guidance:

- initialize a buffer whose current position reaches the logical end of valid input,
- configure no additional input source data,
- verify that the module reports the terminal outcome expected by the original scanner logic.

### Scenario 3: End-of-buffer retry path for an in-progress match

A scanner reaches end-of-buffer during a match where the scanner must resolve behavior based on the state before the sentinel region.

Expected module behavior:

- the previous automaton state is reconstructed from the text already scanned,
- the module returns the control outcome corresponding to an end-of-buffer retry path when that is what the scanner tables imply,
- no already matched characters are lost or spuriously added.

Testing guidance:

- construct a case where the scanner’s current match spans close to the buffer end,
- invoke previous-state reconstruction,
- verify that the reconstructed state matches the state produced by replaying the same text from the scanner start state,
- verify that the next-buffer decision follows the same path as the C module.

### Scenario 4: Buffer contains unconsumed matched text when refill is needed

A scanner has consumed part of the buffer, but some matched text before the current position must be kept when the buffer is advanced.

Expected module behavior:

- only the required retained text is preserved,
- buffer-relative positions after compaction/refill remain consistent,
- subsequent scanning observes the same matched text boundaries as before the move.

Testing guidance:

- place the current scan position near the end of the buffer with a non-empty matched segment,
- trigger next-buffer handling,
- verify that retained text bytes remain in order and scanning resumes at the correct logical offset.

## Requirements

### Functional Requirements

#### FR-1: End-of-buffer outcome determination

The module shall determine the next scanner control outcome when the current scan position reaches the end-of-buffer condition.

**Traceability**: `yy_get_next_buffer` (`src/c.c:1741-1878`)

#### FR-2: Matched-text preservation across buffer advancement

The module shall preserve the text segment still required by the scanner when buffer advancement or refill processing occurs.

**Traceability**: `yy_get_next_buffer` (`src/c.c:1741-1878`), `yy_buffer_state`

#### FR-3: Buffer position update for continued scanning

The module shall update active buffer position information so scanning can resume correctly after buffer advancement or refill.

**Traceability**: `yy_get_next_buffer` (`src/c.c:1741-1878`), `yy_buffer_state`

#### FR-4: Final input exhaustion detection

The module shall distinguish true end-of-input from cases where scanning can continue after buffer handling.

**Traceability**: `yy_get_next_buffer` (`src/c.c:1741-1878`), `yy_buffer_state`

#### FR-5: Previous automaton state reconstruction

The module shall reconstruct the scanner automaton state corresponding to the text immediately preceding the end-of-buffer position.

**Traceability**: `yy_get_previous_state` (`src/c.c:1884-1915`), `yy_trans_info`

#### FR-6: Transition-driven state replay

The module shall derive the previous automaton state by replaying transitions over the already scanned text represented in the active buffer.

**Traceability**: `yy_get_previous_state` (`src/c.c:1884-1915`), `yy_trans_info`, `yy_buffer_state`

#### FR-7: Consistent coordination between prior-state reconstruction and next-buffer handling

The module shall make the reconstructed previous state available in the form needed for end-of-buffer handling decisions, so both operations remain behaviorally consistent.

**Traceability**: `yy_get_next_buffer` (`src/c.c:1741-1878`), `yy_get_previous_state` (`src/c.c:1884-1915`)

### Key Entities

#### `yy_buffer_state`

Scanner buffer state representing the active input buffer and its scanning positions.

Functional role evidenced by the module:

- holds the character storage used by the scanner,
- identifies the current scan position within that storage,
- represents how much valid input is present,
- carries buffer-related status needed to decide whether refill or termination applies.

Relationship to module behavior:

- consumed by end-of-buffer handling to preserve text, advance buffer contents, and classify next action,
- consumed by previous-state reconstruction to locate the text region whose transitions must be replayed.

**Traceability**: `yy_buffer_state` declarations in `src/c.c`, `yy_get_next_buffer`, `yy_get_previous_state`

#### `yy_trans_info`

Scanner transition information used to move between lexical automaton states based on input characters.

Functional role evidenced by the module:

- defines the transition mapping used during scanner state replay,
- supports reconstruction of the state preceding end-of-buffer.

Relationship to module behavior:

- read by previous-state reconstruction while replaying matched text from the active buffer.

**Traceability**: `yy_trans_info` (`src/c.c:440-444`), `yy_get_previous_state`

## Success Criteria

1. **Outcome parity for buffer advancement**: For inputs that trigger end-of-buffer processing, the Rust module returns the same class of scanner outcome as the C module: continue-after-refill, final end-of-input, or end-of-buffer retry path.
   **Traceability**: `yy_get_next_buffer`

2. **State reconstruction parity**: For the same scanner tables, active buffer contents, and current position, the Rust module reconstructs the same prior scanner state as the C module.
   **Traceability**: `yy_get_previous_state`, `yy_trans_info`

3. **Text preservation correctness**: When buffer advancement occurs with retained matched text, the Rust module preserves exactly the required text bytes in the correct order for subsequent scanning.
   **Traceability**: `yy_get_next_buffer`, `yy_buffer_state`

4. **Resume-position correctness**: After nonterminal buffer handling, scanning resumes from a buffer position that yields the same subsequent tokenization behavior as the C module for the same input.
   **Traceability**: `yy_get_next_buffer`, `yy_buffer_state`

5. **Terminal EOF correctness**: When no further input is available, the Rust module reaches the terminal end-of-input outcome without misclassifying the condition as a refill continuation.
   **Traceability**: `yy_get_next_buffer`

6. **Integrated behavior correctness**: In cases where end-of-buffer handling depends on the previously reached automaton state, the combined Rust behavior matches the original module’s decision path.
   **Traceability**: `yy_get_next_buffer`, `yy_get_previous_state`

## Out of Scope

The Rust rewrite specification does not require, because they are not evidenced by the provided module scope:

- new public APIs beyond what is needed to preserve module behavior,
- thread-safety guarantees,
- serialization or persistence of scanner state,
- error recovery features beyond the original scanner behavior,
- FFI-specific interfaces,
- performance or benchmark targets.