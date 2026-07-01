# spec.md

## Title

Rust Functional Specification for `module_src_c.c_20`

## Metadata

- Project: `cflow-new`
- Source module: `src/c.c`
- Module category: `module_cluster`
- Target Rust branch: `083-module_src_c.c_20-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides scanner runtime support centered on lexical input buffer management and scanner state access. The evidenced functionality covers:

- maintaining the active scanner input buffer,
- switching between buffers,
- pushing and popping nested buffers,
- flushing, reloading, restarting, and deleting buffers,
- handling end-of-buffer and NUL-transition behavior used by the scanner runtime,
- pushing characters back into scanner input,
- reporting fatal scanner runtime errors,
- exposing basic scanner state getters for line number, input stream, output stream, and current token length.

The Rust rewrite must preserve the functional behavior of this runtime support as used by the generated scanner logic in `src/c.c`. The rewrite is limited to the responsibilities evidenced by the listed functions and data structures and must not introduce unrelated capabilities.

## Feature Specification

### Feature: Active scanner buffer lifecycle management

The module manages a current input buffer representing the scanner's source of characters. The Rust version must support loading an initial or replacement input source into scanner state, making it the active buffer, and synchronizing scanner runtime variables with that active buffer.

This includes behavior evidenced by:

- `yyrestart`
- `yy_switch_to_buffer`
- `yy_load_buffer_state`
- `yy_flush_buffer`
- `yy_delete_buffer`

Expected functional behavior:

- restarting scanning against a specified input source resets scanner use of the current source and prepares continued scanning from that source,
- switching to a different buffer updates scanner state so subsequent scanning uses the new active buffer,
- loading buffer state refreshes scanner runtime state from the active buffer,
- flushing a buffer clears buffered input state so it is treated as needing fresh input,
- deleting a buffer releases it from scanner use and avoids leaving it as an invalid active reference.

### Feature: Nested buffer stack support

The module supports temporary switching to another input buffer and later returning to the previous one. This is used for scanner workflows that require nested input sources.

This includes behavior evidenced by:

- `yypush_buffer_state`
- `yypop_buffer_state`
- `yy_switch_to_buffer`

Expected functional behavior:

- pushing a buffer preserves the current active buffer on an internal stack and makes the new buffer active,
- popping restores the prior buffer from the stack,
- popping when the current buffer is removed leaves scanner state consistent with the next available buffer state.

### Feature: Scanner input pushback and special transition handling

The module supports scanner runtime operations needed by generated lexical automata when handling end-of-buffer and NUL cases.

This includes behavior evidenced by:

- `yy_try_NUL_trans`
- `yyunput`
- the identified `if` block at `src/c.c:1598-1618` related to restoring scanner offset and checking for `YY_BUFFER_NEW`.

Expected functional behavior:

- the scanner can attempt a transition on NUL input according to current scanner state and receive the resulting state decision,
- characters can be pushed back into the input stream so they are re-read by subsequent scanner steps,
- buffer-new and end-of-buffer related state is handled so scanner operation remains correct across buffer boundaries and pushback activity.

### Feature: Fatal runtime error signaling

The module contains a scanner fatal-error path for unrecoverable runtime conditions.

This is evidenced by:

- `yy_fatal_error`

Expected functional behavior:

- unrecoverable scanner runtime failures terminate the current scanner operation immediately through the module's fatal error mechanism,
- the Rust rewrite must preserve fail-fast behavior for equivalent fatal conditions rather than silently continuing.

### Feature: Scanner state query accessors

The module exposes read access to selected scanner state values.

This includes behavior evidenced by:

- `yyget_lineno`
- `yyget_in`
- `yyget_out`
- `yyget_leng`

Expected functional behavior:

- callers can obtain the current scanner line number,
- callers can obtain the current scanner input source handle/reference,
- callers can obtain the current scanner output handle/reference,
- callers can obtain the current matched token length.

The Rust rewrite must preserve the ability of scanner-integrated code to retrieve these values from the maintained scanner state.

## User Scenarios & Testing

### Scenario 1: Restart scanning on a new input source

A caller has an existing scanner instance and needs scanning to continue from a different input source.

The Rust module must support:

1. receiving a new input source through restart,
2. making that source the active scanning buffer,
3. ensuring subsequent scanner actions use the new active source,
4. preserving consistent line/input/output/token-length queries after restart.

Tests should verify that after restart, the active buffer and input handle/reference reflect the replacement source and scanning does not continue reading stale buffered data.

### Scenario 2: Temporarily scan nested input and return

A scanner is reading one source, then needs to switch to another source temporarily, such as nested included content, before resuming the original source.

The Rust module must support:

1. pushing the current buffer,
2. activating a new buffer,
3. scanning against the nested buffer,
4. popping the nested buffer,
5. restoring the previous active buffer correctly.

Tests should verify stack order, active-buffer restoration, and continued scanning from the resumed source.

### Scenario 3: Flush buffered input before continued scanning

A caller needs the scanner to discard buffered contents and force fresh input handling for a buffer.

The Rust module must support flushing a buffer and leaving scanner state ready for subsequent refilling/use.

Tests should verify that a flushed active buffer is treated as newly needing input and does not retain stale buffered content.

### Scenario 4: Delete an inactive or replaced buffer

A caller has finished with a buffer and needs it removed.

The Rust module must support deleting a buffer without leaving scanner state pointing to freed or invalid buffer state.

Tests should verify correct behavior when deleting a non-active buffer and when deleting a buffer that had recently been switched away from.

### Scenario 5: Push back a character into scanner input

During lexical processing, scanner logic determines that one character must be returned to the input stream.

The Rust module must support unput behavior so the character becomes the next character seen by scanning logic.

Tests should verify that after pushback, the next read path observes the pushed-back character and scanner position remains consistent.

### Scenario 6: Handle NUL or end-of-buffer transition logic

The generated scanner enters a path where it must evaluate a NUL transition or react to a new/end-of-buffer state.

The Rust module must support the same state progression decisions as the source module for these runtime cases.

Tests should verify that NUL-transition handling returns a valid transition result for the current state and that buffer-new/end-of-buffer conditions do not corrupt scanner state.

### Scenario 7: Query scanner state during operation

Scanner-integrated code queries current line number, input source, output source, or token length.

The Rust module must return values consistent with current scanner state.

Tests should verify that getters reflect the active scanner state before and after buffer switches, restart, and token processing steps that change token length.

## Requirements

### Functional Requirements

- **FR-1 Active buffer restart**
  The module shall support restarting scanner input against a specified input source and making it the basis for subsequent scanning behavior.
  **Traceability:** `yyrestart` in `src/c.c:2088-2102`

- **FR-2 Buffer switching**
  The module shall support replacing the current active buffer with another buffer and updating scanner state accordingly.
  **Traceability:** `yy_switch_to_buffer` in `src/c.c:2112-2144`; `yy_load_buffer_state` in `src/c.c:2147-2160`

- **FR-3 Buffer-state loading**
  The module shall support synchronizing scanner runtime state from the currently active buffer whenever required by buffer lifecycle operations.
  **Traceability:** `yy_load_buffer_state` in `src/c.c:2147-2160`

- **FR-4 Buffer deletion**
  The module shall support deleting a buffer state object and maintaining valid scanner state after deletion.
  **Traceability:** `yy_delete_buffer` in `src/c.c:2204-2220`

- **FR-5 Buffer flushing**
  The module shall support resetting a buffer's buffered-input condition so that subsequent scanning treats it as requiring fresh input handling.
  **Traceability:** `yy_flush_buffer` in `src/c.c:2268-2292`

- **FR-6 Buffer stack push**
  The module shall support pushing the current buffer state and activating a new buffer as the current scanner input.
  **Traceability:** `yypush_buffer_state` in `src/c.c:2302-2329`

- **FR-7 Buffer stack pop**
  The module shall support popping the current buffer state and restoring the previous buffer as active input when available.
  **Traceability:** `yypop_buffer_state` in `src/c.c:2338-2355`

- **FR-8 Character pushback**
  The module shall support returning a character to scanner input so it can be processed again by subsequent scanner logic.
  **Traceability:** `yyunput` in `src/c.c:1953-1993`

- **FR-9 NUL-transition handling**
  The module shall support scanner runtime evaluation of transitions on NUL input based on current scanner state.
  **Traceability:** `yy_try_NUL_trans` in `src/c.c:1923-1948`

- **FR-10 End-of-buffer/new-buffer state handling**
  The module shall preserve correct scanner behavior when the active buffer is in a newly initialized or end-of-buffer related state.
  **Traceability:** `if` block in `src/c.c:1598-1618`

- **FR-11 Fatal error signaling**
  The module shall provide a fatal scanner runtime error path for unrecoverable conditions and shall not continue normal scanning after such an error is raised.
  **Traceability:** `yy_fatal_error` in `src/c.c:2507-2511`

- **FR-12 Line number query**
  The module shall expose the current scanner line number.
  **Traceability:** `yyget_lineno` in `src/c.c:2542-2546`

- **FR-13 Input handle/reference query**
  The module shall expose the scanner's current input source handle/reference.
  **Traceability:** `yyget_in` in `src/c.c:2551-2554`

- **FR-14 Output handle/reference query**
  The module shall expose the scanner's current output destination handle/reference.
  **Traceability:** `yyget_out` in `src/c.c:2559-2562`

- **FR-15 Current token length query**
  The module shall expose the current matched token length maintained by scanner state.
  **Traceability:** `yyget_leng` in `src/c.c:2567-2570`

### Key Entities

- **Scanner buffer state (`yy_buffer_state`)**
  The primary runtime entity representing one scanner input buffer. It participates in active-buffer selection, flushing, deletion, restart, and stack-based nesting.
  **Traceability:** `struct yy_buffer_state` occurrences at `src/c.c:191`, `233-298`, `2176`, `2377`, `2378`, `2383`, `2396`, `2398`, `2404`, `2427`

- **Scanner transition information (`yy_trans_info`)**
  Transition metadata used by generated scanner state movement, including special handling paths such as NUL transitions.
  **Traceability:** `struct yy_trans_info` at `src/c.c:440-444`

- **Scanner-managed buffer stack relationship**
  A logical relationship in which one active `yy_buffer_state` may be saved while another becomes active, allowing nested scanning and later restoration.
  **Traceability:** `yypush_buffer_state` `src/c.c:2302-2329`; `yypop_buffer_state` `src/c.c:2338-2355`; `yy_switch_to_buffer` `src/c.c:2112-2144`

- **Scanner state access values**
  Runtime state values exposed through getters: current line number, current input source, current output destination, and current token length.
  **Traceability:** `yyget_lineno`, `yyget_in`, `yyget_out`, `yyget_leng` at `src/c.c:2542-2570`

- **Obstack presence**
  The source file contains `struct obstack`, but no evidenced functionality from the analyzed function set in this module requires separate Rust feature definition beyond ensuring no conflict with the scanner runtime responsibilities above.
  **Traceability:** `struct obstack` at `src/c.c:760`, `2850`

## Success Criteria

- **SC-1** Restarting with a new input source causes subsequent scanner buffer state and input-source queries to reflect that new source.
  **Traceability:** `yyrestart`, `yyget_in`

- **SC-2** Switching to a different buffer updates the active scanner buffer without requiring process restart and keeps scanner state internally consistent.
  **Traceability:** `yy_switch_to_buffer`, `yy_load_buffer_state`

- **SC-3** Pushing two or more buffers and then popping them restores previously active buffers in last-in, first-out order.
  **Traceability:** `yypush_buffer_state`, `yypop_buffer_state`

- **SC-4** Flushing a buffer clears its ready-to-scan buffered contents such that subsequent scanning treats the buffer as needing fresh input handling.
  **Traceability:** `yy_flush_buffer`

- **SC-5** Deleting a buffer does not leave the scanner operating on an invalid buffer reference.
  **Traceability:** `yy_delete_buffer`

- **SC-6** After character pushback, the next scanner read path observes the pushed-back character before later input.
  **Traceability:** `yyunput`

- **SC-7** NUL-transition evaluation returns a deterministic result for the current scanner state and does not corrupt active buffer state.
  **Traceability:** `yy_try_NUL_trans`

- **SC-8** New-buffer and end-of-buffer state handling preserves correct scanner progression across buffer boundaries.
  **Traceability:** `src/c.c:1598-1618` identified buffer-status logic

- **SC-9** Fatal scanner runtime conditions trigger the module's fatal error path and do not continue normal execution from that point.
  **Traceability:** `yy_fatal_error`

- **SC-10** Getter functions return values consistent with current scanner state before and after buffer restart, switch, push, and pop operations.
  **Traceability:** `yyget_lineno`, `yyget_in`, `yyget_out`, `yyget_leng`

## Out of Scope

The Rust rewrite specification does not require capabilities not evidenced by the analyzed module interface and structures, including:

- new public APIs beyond those functionally represented here,
- thread-safety guarantees,
- serialization or persistence of scanner state,
- recovery semantics beyond the existing fatal-error path,
- performance or benchmark targets,
- non-evidenced `obstack` feature expansion.