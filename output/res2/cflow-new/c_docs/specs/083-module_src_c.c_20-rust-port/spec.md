# spec.md

## Title

Rust Port Functional Specification: `module_src_c.c_20`

## Metadata

- Project: `cflow-new`
- Module category: `module_cluster`
- Source module: `src/c.c`
- Rust branch: `083-module_src_c.c_20-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides scanner buffer and scanner state support used by the generated lexical scanner in `src/c.c`. Its evidenced responsibilities are:

- managing the active scanner input buffer,
- switching between buffers,
- pushing and popping nested buffers,
- reloading scanner state from the active buffer,
- flushing or deleting buffers,
- handling scanner pushback and NUL-transition behavior,
- restarting scanning on a new input stream,
- exposing basic scanner accessors for line number, input stream, output stream, and current token length,
- terminating on fatal scanner errors.

The Rust rewrite must preserve the same functional behavior boundaries for scanner-state and buffer management that are evidenced by the listed functions and buffer-related data structures.

## Scope

### In Scope

The Rust module must implement the functional behavior represented by these scanner support capabilities:

- scanner restart against an input source,
- active buffer replacement,
- active buffer stack push/pop behavior,
- buffer-state load and flush behavior,
- buffer destruction behavior,
- scanner pushback behavior,
- scanner NUL-transition handling,
- fatal error termination for unrecoverable scanner support failures,
- read-only accessors for current scanner metadata:
  - line number,
  - input handle,
  - output handle,
  - current token length.

### Out of Scope

The Rust rewrite specification does not require any behavior not evidenced by this module analysis, including:

- new public APIs beyond the evidenced scanner-support surface,
- thread-safety guarantees,
- serialization or persistence,
- recovery-oriented fatal error alternatives,
- performance targets or benchmark parity,
- scanner grammar changes,
- semantics outside buffer/state support.

## Feature Specification

### Feature 1: Active Scanner Buffer Management

The module manages a current scanner buffer and allows scanner execution to be redirected to another buffer. When the active buffer changes, the scanner-visible state must be synchronized with that buffer so subsequent scanning operations read from the newly active source.

Evidenced by:
- `yy_switch_to_buffer`
- `yy_load_buffer_state`
- `yyrestart`
- `yy_flush_buffer`
- `struct yy_buffer_state`

#### Required behavior

- The module must maintain the notion of one current active buffer.
- Switching to another buffer must make that buffer the active source for subsequent scanning.
- Restarting with a new input stream must reset scanner operation onto a buffer associated with that stream.
- Flushing a buffer must reset its consumable content/state so scanning resumes from a clean state for that buffer.
- Loading buffer state must synchronize scanner runtime state from the active buffer before continued scanning.

### Feature 2: Buffer Stack for Nested Input Sources

The module supports nested buffer usage through push and pop operations. This enables scanner workflows where one input source temporarily interrupts another and later returns control to the previous source.

Evidenced by:
- `yypush_buffer_state`
- `yypop_buffer_state`
- `yy_switch_to_buffer`
- `struct yy_buffer_state`

#### Required behavior

- The module must support pushing a new buffer so it becomes active while preserving the previous active buffer for later restoration.
- The module must support popping the active buffer and restoring the previously pushed buffer as active.
- Popping must correctly handle the active buffer lifecycle expected by the module, including transition away from the current buffer.

### Feature 3: Buffer Lifecycle Control

The module provides explicit deletion and reset operations for scanner buffers.

Evidenced by:
- `yy_delete_buffer`
- `yy_flush_buffer`
- `struct yy_buffer_state`

#### Required behavior

- The module must allow a buffer object to be deleted so it is no longer used by scanner operations.
- The module must allow buffer contents/state to be cleared/reset without requiring deletion.
- Operations involving deleted or replaced buffers must not leave the scanner in an undefined active-buffer state.

### Feature 4: Scanner Pushback Support

The module supports pushing a character back into scanner input so the scanner can reconsider input during tokenization.

Evidenced by:
- `yyunput`

#### Required behavior

- The module must support reinserting a character into scanner input relative to the current scanner position.
- After pushback, subsequent scanning must observe the pushed-back character in the correct order relative to surrounding buffered input.

### Feature 5: NUL-Transition Handling

The module includes scanner support for transition behavior when the scanner encounters a NUL boundary during state-machine execution.

Evidenced by:
- `yy_try_NUL_trans`
- buffer/state support functions in the same module

#### Required behavior

- The module must support the scanner’s attempt to continue or resolve state transition logic when a NUL condition is encountered in buffered input.
- The result of NUL-transition handling must be usable by scanner control flow to determine the next scanner state.

### Feature 6: Fatal Scanner Error Signaling

The module contains a fatal error path for unrecoverable scanner-support errors.

Evidenced by:
- `yy_fatal_error`

#### Required behavior

- The module must provide a fatal error mechanism for unrecoverable internal scanner support failures.
- Fatal errors must stop normal scanner progress rather than silently continuing.

### Feature 7: Scanner Metadata Accessors

The module exposes accessors for current scanner metadata.

Evidenced by:
- `yyget_lineno`
- `yyget_in`
- `yyget_out`
- `yyget_leng`

#### Required behavior

- The module must expose the current scanner line number.
- The module must expose the current scanner input handle/reference.
- The module must expose the current scanner output handle/reference.
- The module must expose the current token length.

## User Scenarios & Testing

### Scenario 1: Restart scanning on a new input source

A caller has an existing scanner instance and wants to begin scanning a different input stream.

Expected support:
- invoking restart causes the scanner to use the new input source,
- scanner state is synchronized to the restarted buffer before continued scanning.

Test coverage:
- initialize scanner support with one input source,
- restart with a second input source,
- verify subsequent scanner reads come from the second source,
- verify scanner metadata accessors reflect the new active input context where applicable.

Traceability:
- `yyrestart`
- `yy_load_buffer_state`
- `yyget_in`

### Scenario 2: Temporarily scan nested input, then return

A scanner encounters a workflow requiring temporary scanning from another buffer, then resuming the original buffer afterward.

Expected support:
- pushing a new buffer makes it active immediately,
- popping restores the previously active buffer,
- scanner continues from the restored buffer after pop.

Test coverage:
- start with buffer A active,
- push buffer B,
- verify buffer B is used,
- pop buffer B,
- verify scanning resumes from buffer A.

Traceability:
- `yypush_buffer_state`
- `yypop_buffer_state`
- `yy_switch_to_buffer`

### Scenario 3: Replace the active buffer directly

A caller wants to switch scanner processing from one prepared buffer to another without using stack nesting.

Expected support:
- direct switch changes the active buffer,
- scanner runtime state reloads to match the selected buffer.

Test coverage:
- prepare two buffers,
- switch from A to B,
- verify the active source is B,
- verify scanner state reflects B immediately.

Traceability:
- `yy_switch_to_buffer`
- `yy_load_buffer_state`

### Scenario 4: Flush a buffer before reuse

A caller wants to clear a buffer’s current content/state and continue with it as a fresh scanner source.

Expected support:
- flushing resets the buffer for clean reuse,
- if the flushed buffer is active, scanner state updates consistently.

Test coverage:
- use a buffer with consumed or partially consumed input,
- flush it,
- verify buffer is reset to initial scanning condition,
- if active, verify subsequent scanning starts from the flushed state.

Traceability:
- `yy_flush_buffer`
- `yy_load_buffer_state`

### Scenario 5: Delete a no-longer-needed buffer

A caller has finished with a buffer and removes it from scanner use.

Expected support:
- deleted buffers are no longer valid for scanner operation,
- deleting buffers does not leave scanner state inconsistent.

Test coverage:
- create or obtain a buffer,
- make it inactive or transition away as needed,
- delete it,
- verify scanner continues correctly with any remaining active/restored buffer state.

Traceability:
- `yy_delete_buffer`
- `struct yy_buffer_state`

### Scenario 6: Push back one character during scanning

Scanner logic needs to return one character to the input stream so it can be rescanned.

Expected support:
- the character is restored ahead of the current scan point,
- the next scan step sees that character again.

Test coverage:
- consume input up to a known point,
- invoke pushback with a known character,
- verify subsequent scanner progression sees the pushed character before later buffered content.

Traceability:
- `yyunput`

### Scenario 7: Resolve NUL transition during scanning

Scanner execution reaches a NUL condition within the buffer and must determine the next automaton state.

Expected support:
- scanner support performs the NUL-transition attempt,
- a next-state result is produced for scanner control flow.

Test coverage:
- place scanner in a state where NUL-transition logic is exercised,
- invoke the transition path,
- verify a deterministic next-state result is returned and usable.

Traceability:
- `yy_try_NUL_trans`

### Scenario 8: Query scanner metadata

A caller needs the current scanner line number, current input/output handles, or token length.

Expected support:
- accessors return the current values maintained by scanner state.

Test coverage:
- perform scanner activity affecting line and token length,
- query line number and length,
- verify returned values match scanner state,
- verify input/output accessors return the currently configured handles.

Traceability:
- `yyget_lineno`
- `yyget_in`
- `yyget_out`
- `yyget_leng`

### Scenario 9: Fatal internal scanner support failure

An unrecoverable scanner-support condition is encountered.

Expected support:
- fatal error path terminates normal scanner progress.

Test coverage:
- exercise a controlled unrecoverable internal condition in test scaffolding,
- verify the module enters its fatal-failure path rather than continuing silently.

Traceability:
- `yy_fatal_error`

## Requirements

### Functional Requirements

#### FR-1: Active buffer tracking
The module shall maintain a current active scanner buffer used as the source of scanner state and input progression.

Traceability:
- `yy_switch_to_buffer`
- `yy_load_buffer_state`
- `struct yy_buffer_state`

#### FR-2: Buffer switching
The module shall support replacing the active scanner buffer with another buffer and shall make the replacement effective for subsequent scanning.

Traceability:
- `yy_switch_to_buffer`
- `yy_load_buffer_state`

#### FR-3: Scanner restart
The module shall support restarting scanner operation against a provided input stream, resetting scanner execution onto the corresponding active buffer context.

Traceability:
- `yyrestart`

#### FR-4: Buffer-state synchronization
The module shall support loading scanner runtime state from the current active buffer before scanning continues.

Traceability:
- `yy_load_buffer_state`

#### FR-5: Buffer flushing
The module shall support resetting a buffer’s scanning content/state without deleting the buffer object.

Traceability:
- `yy_flush_buffer`

#### FR-6: Buffer deletion
The module shall support deleting a scanner buffer so it is no longer available for scanner use.

Traceability:
- `yy_delete_buffer`
- `struct yy_buffer_state`

#### FR-7: Nested buffer push
The module shall support pushing a new buffer onto a buffer stack and making it the active scanner buffer.

Traceability:
- `yypush_buffer_state`

#### FR-8: Nested buffer pop
The module shall support popping the current buffer from the buffer stack and restoring the previous buffer as active when one exists.

Traceability:
- `yypop_buffer_state`

#### FR-9: Character pushback
The module shall support unput/pushback of a character into scanner input so the next scanner progression can observe it again.

Traceability:
- `yyunput`

#### FR-10: NUL transition support
The module shall support scanner transition resolution for NUL conditions and return a scanner state result usable by scanner control flow.

Traceability:
- `yy_try_NUL_trans`

#### FR-11: Fatal error termination
The module shall provide a fatal error path for unrecoverable internal scanner-support failures and shall not continue normal scanner execution after such a failure.

Traceability:
- `yy_fatal_error`

#### FR-12: Line number accessor
The module shall provide read access to the current scanner line number.

Traceability:
- `yyget_lineno`

#### FR-13: Input handle accessor
The module shall provide read access to the current scanner input handle/reference.

Traceability:
- `yyget_in`

#### FR-14: Output handle accessor
The module shall provide read access to the current scanner output handle/reference.

Traceability:
- `yyget_out`

#### FR-15: Token length accessor
The module shall provide read access to the current scanner token length.

Traceability:
- `yyget_leng`

### Key Entities

#### Entity 1: Scanner Buffer State
Represents a scanner buffer object that stores the state needed for scanning from one input source and for moving between active, flushed, stacked, and deleted states.

Traceability:
- `struct yy_buffer_state` at multiple referenced locations
- `yy_switch_to_buffer`
- `yy_delete_buffer`
- `yy_flush_buffer`
- `yypush_buffer_state`
- `yypop_buffer_state`
- `yyrestart`

Relationships:
- one buffer may be the current active buffer,
- buffers may participate in a stack-like nesting relationship,
- scanner runtime state is loaded from the active buffer.

#### Entity 2: Scanner Runtime State
Represents the scanner’s current execution context as reflected by active-buffer selection, current input position, token length, line number, and current input/output handles.

Traceability:
- `yy_load_buffer_state`
- `yyunput`
- `yy_try_NUL_trans`
- `yyget_lineno`
- `yyget_in`
- `yyget_out`
- `yyget_leng`

Relationships:
- runtime state depends on the active buffer,
- pushback modifies effective upcoming input within runtime state,
- NUL-transition handling consumes current scanner state and returns a next state for control flow.

#### Entity 3: Transition Information
Represents scanner automaton transition data used to resolve input-driven scanner state changes, including NUL-related transition behavior.

Traceability:
- `struct yy_trans_info`
- `yy_try_NUL_trans`

Relationships:
- used by scanner runtime state to determine next-state behavior during scanning.

## Success Criteria

1. The Rust module supports direct active-buffer replacement, and tests verify that after switching buffers, subsequent scanner activity uses the new buffer.
   - Traceability: `yy_switch_to_buffer`, `yy_load_buffer_state`

2. The Rust module supports scanner restart on a new input source, and tests verify that subsequent scanner activity is sourced from the restarted input.
   - Traceability: `yyrestart`

3. The Rust module supports nested buffer push/pop behavior, and tests verify that pushing activates the new buffer and popping restores the previous one.
   - Traceability: `yypush_buffer_state`, `yypop_buffer_state`

4. The Rust module supports buffer flushing, and tests verify that a flushed buffer returns to a clean scanning state without requiring deletion.
   - Traceability: `yy_flush_buffer`

5. The Rust module supports buffer deletion, and tests verify that deleted buffers are not used for subsequent scanner operations.
   - Traceability: `yy_delete_buffer`, `struct yy_buffer_state`

6. The Rust module supports single-character pushback, and tests verify that the next scanner progression re-observes the pushed-back character.
   - Traceability: `yyunput`

7. The Rust module supports NUL-transition resolution, and tests verify that a deterministic next-state result is returned for scanner control flow when the NUL path is exercised.
   - Traceability: `yy_try_NUL_trans`

8. The Rust module provides scanner metadata accessors, and tests verify correct reporting of line number, input handle/reference, output handle/reference, and current token length.
   - Traceability: `yyget_lineno`, `yyget_in`, `yyget_out`, `yyget_leng`

9. The Rust module provides a fatal internal error path, and tests verify that an unrecoverable scanner-support failure does not continue normal execution.
   - Traceability: `yy_fatal_error`