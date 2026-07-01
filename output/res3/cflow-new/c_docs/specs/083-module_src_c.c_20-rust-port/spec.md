# spec.md

## Title

Rust Port Functional Specification: `module_src_c.c_20`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_c.c_20`
- **Category**: `module_cluster`
- **Source file**: `src/c.c`
- **Rust branch**: `083-module_src_c.c_20-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides scanner runtime support centered on input buffer lifecycle, active-buffer switching, stacked buffer management, scanner state refresh, limited scanner position/introspection accessors, and fatal error termination. The Rust rewrite must preserve the observable behavior of these facilities as used by the generated scanner runtime in `src/c.c`.

The evidenced functionality in this module is:

- restoring and reloading scanner state when the current buffer is marked new
- attempting scanner transition on NUL input
- pushing characters back into scanner input
- restarting scanning on a file input
- switching the active scanner buffer
- loading scanner state from the active buffer
- deleting and flushing buffers
- pushing and popping buffer states on a stack
- reporting fatal scanner errors as non-returning failures
- exposing current line number, input stream, output stream, and token length

This specification covers only those functional boundaries evidenced by the analyzed functions and data structures.

## Feature Specification

### Summary

The Rust version must implement the scanner runtime buffer-management behavior needed by the generated lexer logic in this module. It must maintain an active input buffer, support a stack of buffers for nested scanning contexts, refresh scanner working state from the active buffer, permit scanner pushback behavior, handle restart and flush operations, and provide read-only access to selected scanner status values.

### In-Scope Features

1. **Active buffer management**
   - Maintain a current scanner buffer state.
   - Replace the current buffer with another buffer.
   - Refresh scanner working state from the current buffer after changes.

2. **Buffer lifecycle operations**
   - Flush a buffer so it is treated as emptied/reset for future scanning.
   - Delete a buffer and release its ownership from scanner management.

3. **Buffer stack operations**
   - Push a new active buffer while preserving the previous active buffer.
   - Pop the current buffer and restore the previous active buffer.

4. **Scanner restart behavior**
   - Restart scanning against a specified input source by reinitializing scanner use of buffer state as required by `yyrestart`.

5. **Pushback / unread behavior**
   - Support scanner logic that returns a character to the input stream region via `yyunput`.

6. **NUL transition support**
   - Support scanner state progression logic for NUL-input handling through `yy_try_NUL_trans`.

7. **Scanner state accessors**
   - Provide access to:
     - current line number
     - current input handle/reference
     - current output handle/reference
     - current token/text length

8. **Fatal failure path**
   - Provide a non-returning fatal error path for scanner runtime failures.

### Out of Scope

The Rust port specification does not require any capability not evidenced in the analyzed module, including but not limited to:

- new public APIs beyond the module’s evidenced behavior
- thread-safety guarantees
- serialization
- recovery from fatal runtime errors
- performance targets or benchmark parity
- functionality of unrelated lexer token rules outside the listed runtime support functions

## User Scenarios & Testing

### Scenario 1: Restart scanning on a new input source

A caller needs the scanner to stop using its current input source and begin scanning from a specified file/input handle.

**Expected behavior**
- Restart operation rebinds scanner operation to the provided input source.
- The scanner’s active buffer state is made consistent for continued scanning.
- Subsequent scanner actions use the restarted source rather than stale prior state.

**Test guidance**
- Initialize scanner state with one input source.
- Invoke restart with another source.
- Verify the scanner’s current input reference reflects the new source.
- Verify scanning can continue without requiring manual buffer reloading by the caller.

### Scenario 2: Switch to a prepared buffer

A caller has an existing scanner buffer and wants it to become the active buffer immediately.

**Expected behavior**
- The specified buffer becomes current.
- Scanner working pointers/state are refreshed from that buffer.
- If another buffer was active, its relevant state is preserved before switching.

**Test guidance**
- Create or obtain two distinct buffer states.
- Make the first active, then switch to the second.
- Verify the current buffer-dependent state matches the second buffer.

### Scenario 3: Push and later restore nested buffer context

A scanner temporarily enters another input context and must later resume the previous one.

**Expected behavior**
- Pushing a buffer makes the new buffer active and retains the previous active buffer on a stack.
- Popping removes the current active buffer and restores the previous active buffer.
- Active scanner state follows the stack top after each operation.

**Test guidance**
- Start with buffer A active.
- Push buffer B, then confirm B is active.
- Pop the buffer state, then confirm A is active again.
- Repeat with multiple nested pushes to verify stack behavior.

### Scenario 4: Flush a buffer before reuse

A caller wants to discard current buffered content and reset the buffer so later scanning treats it as fresh input state.

**Expected behavior**
- Flush clears/reset scanner-visible buffered content for the target buffer.
- If the flushed buffer is active, scanner working state becomes consistent with the reset buffer state.

**Test guidance**
- Use a buffer with scanner progress already advanced.
- Flush it.
- Verify its status is reset for subsequent scanning behavior.
- If active, verify state reload has occurred.

### Scenario 5: Delete an inactive or active buffer

A caller no longer needs a buffer and requests deletion.

**Expected behavior**
- The buffer is removed from scanner management.
- If the deleted buffer is active or stack-referenced, scanner state remains valid according to the runtime’s supported deletion semantics.
- No further use of the deleted buffer is permitted.

**Test guidance**
- Delete an inactive buffer and verify scanner operation continues on the remaining active buffer.
- Delete the active buffer only in the manner supported by the original module’s semantics and verify no stale active reference remains.

### Scenario 6: Push back a character during scanning

The scanner needs to return a character to the input region so it can be processed again as part of lexer control flow.

**Expected behavior**
- The pushback operation updates scanner input state so the character becomes available for subsequent scanning.
- Scanner text/input position remains coherent after pushback.

**Test guidance**
- Simulate scanner progress to a buffer position.
- Invoke pushback with a character.
- Verify the next scanner read sees the pushed-back character in the expected position.

### Scenario 7: Handle NUL transition during state machine execution

The scanner encounters NUL-input handling in DFA progression.

**Expected behavior**
- NUL transition handling returns the resulting scanner state according to the runtime transition tables/state model.
- The operation does not corrupt active buffer state.

**Test guidance**
- Execute the NUL-transition path from a known scanner state.
- Verify a valid resulting state is produced for both accepting and jam/non-advancing cases supported by the runtime.

### Scenario 8: Read scanner status values

A caller needs current scanner metadata.

**Expected behavior**
- Accessors return:
  - current line number
  - current input reference
  - current output reference
  - current token length

**Test guidance**
- Set up scanner activity that changes line count and token length.
- Verify each accessor returns the current scanner value without modifying state.

### Scenario 9: Fatal runtime error terminates execution path

The scanner hits an unrecoverable internal error.

**Expected behavior**
- Fatal error reporting does not return control to the caller.
- The error path is clearly distinguishable from ordinary scanner results.

**Test guidance**
- Trigger the fatal error path under controlled test conditions.
- Verify execution ends via panic/abort/error termination semantics chosen for the Rust port, with no normal return.

## Requirements

### Functional Requirements

#### FR-1: Active buffer switching
The module shall support making a specified buffer the active scanner buffer and updating scanner working state to match it.

**Traceability**
- `yy_switch_to_buffer` (`src/c.c:2112-2144`)
- `yy_load_buffer_state` (`src/c.c:2147-2160`)
- `struct yy_buffer_state`

#### FR-2: Scanner state reload from current buffer
The module shall support reloading scanner operational state from the current active buffer whenever buffer-related operations require it.

**Traceability**
- `yy_load_buffer_state` (`src/c.c:2147-2160`)
- conditional buffer-new restore logic in `if` block (`src/c.c:1598-1618`)
- `struct yy_buffer_state`

#### FR-3: Scanner restart on input source
The module shall support restarting scanner processing using a specified input source and making scanner buffer state consistent for continued scanning.

**Traceability**
- `yyrestart` (`src/c.c:2088-2102`)
- `struct yy_buffer_state`

#### FR-4: Buffer flush/reset
The module shall support flushing a buffer so its content/status is reset for subsequent scanning use.

**Traceability**
- `yy_flush_buffer` (`src/c.c:2268-2292`)
- `struct yy_buffer_state`

#### FR-5: Buffer deletion
The module shall support deleting a buffer and removing it from scanner management.

**Traceability**
- `yy_delete_buffer` (`src/c.c:2204-2220`)
- `struct yy_buffer_state`

#### FR-6: Buffer stack push
The module shall support pushing a new buffer onto the scanner buffer stack and making it active.

**Traceability**
- `yypush_buffer_state` (`src/c.c:2302-2329`)
- `struct yy_buffer_state`

#### FR-7: Buffer stack pop
The module shall support popping the current scanner buffer from the buffer stack and restoring the previous active buffer when present.

**Traceability**
- `yypop_buffer_state` (`src/c.c:2338-2355`)
- `struct yy_buffer_state`

#### FR-8: Character pushback into scanner input
The module shall support returning a character to scanner input state so it can be re-read by subsequent scanner logic.

**Traceability**
- `yyunput` (`src/c.c:1953-1993`)

#### FR-9: NUL transition handling
The module shall support scanner state transition logic for NUL-input processing and return the resulting scanner state.

**Traceability**
- `yy_try_NUL_trans` (`src/c.c:1923-1948`)
- `struct yy_trans_info`

#### FR-10: Fatal error non-return
The module shall support a fatal scanner runtime error path that does not return normally.

**Traceability**
- `yy_fatal_error` (`src/c.c:2507-2511`)

#### FR-11: Line number accessor
The module shall provide read access to the scanner’s current line number.

**Traceability**
- `yyget_lineno` (`src/c.c:2542-2546`)

#### FR-12: Input handle accessor
The module shall provide read access to the scanner’s current input stream/handle.

**Traceability**
- `yyget_in` (`src/c.c:2551-2554`)

#### FR-13: Output handle accessor
The module shall provide read access to the scanner’s current output stream/handle.

**Traceability**
- `yyget_out` (`src/c.c:2559-2562`)

#### FR-14: Current token length accessor
The module shall provide read access to the scanner’s current token/text length.

**Traceability**
- `yyget_leng` (`src/c.c:2567-2570`)

### Key Entities

#### `yy_buffer_state`
Represents a scanner input buffer and its runtime status. This is the central entity for buffer lifecycle, active-buffer switching, flush/reset, deletion, restart support, and stack-based nested scanning.

**Relationships**
- One buffer may be the current active scanner buffer.
- Multiple buffers may participate in a push/pop stack.
- Buffer operations require scanner state reload when the active buffer changes or is reset.

**Traceability**
- `struct yy_buffer_state` occurrences in `src/c.c`
- `yyrestart`
- `yy_switch_to_buffer`
- `yy_load_buffer_state`
- `yy_delete_buffer`
- `yy_flush_buffer`
- `yypush_buffer_state`
- `yypop_buffer_state`

#### Scanner transition state
Represents the DFA/runtime state used by transition logic, including NUL handling.

**Relationships**
- Consumed by `yy_try_NUL_trans`.
- Works with transition table information represented by `yy_trans_info`.

**Traceability**
- `yy_try_NUL_trans` (`src/c.c:1923-1948`)
- `struct yy_trans_info` (`src/c.c:440-444`)

#### Scanner input/output and status values
Represents scanner-maintained metadata exposed through accessors: line number, input handle, output handle, and current token length.

**Relationships**
- These values are read-only through accessor functions in this module.
- Input/output references are associated with the active scanner runtime context.

**Traceability**
- `yyget_lineno`
- `yyget_in`
- `yyget_out`
- `yyget_leng`

## Success Criteria

1. **Buffer switching correctness**
   - Given two valid buffers, switching to the second causes subsequent scanner state reads and operations to use the second as the active buffer.
   - Traceability: `yy_switch_to_buffer`, `yy_load_buffer_state`

2. **Restart correctness**
   - After restart with a specified input source, the scanner reports that source as current input and continues operation without stale prior buffer state.
   - Traceability: `yyrestart`, `yyget_in`

3. **Flush correctness**
   - Flushing a buffer resets it to scanner-usable post-flush state; if the flushed buffer is active, the active scanner state is also refreshed.
   - Traceability: `yy_flush_buffer`, `yy_load_buffer_state`

4. **Deletion correctness**
   - Deleting a buffer removes it from scanner management without leaving the scanner in an invalid active-buffer state under supported usage.
   - Traceability: `yy_delete_buffer`, `struct yy_buffer_state`

5. **Stack behavior correctness**
   - With nested push/pop operations, the active buffer always matches the top of the stack, and popping restores the previous buffer in LIFO order.
   - Traceability: `yypush_buffer_state`, `yypop_buffer_state`

6. **Pushback correctness**
   - After a pushback operation, the next scanner consumption path can observe the pushed-back character in the expected re-read position.
   - Traceability: `yyunput`

7. **NUL transition correctness**
   - For known scanner states, NUL-transition handling returns the same resulting state behavior as the original module for corresponding transition-table conditions.
   - Traceability: `yy_try_NUL_trans`, `struct yy_trans_info`

8. **Accessor correctness**
   - Accessors return current scanner values for line number, input, output, and token length without mutating scanner state.
   - Traceability: `yyget_lineno`, `yyget_in`, `yyget_out`, `yyget_leng`

9. **Fatal error behavior**
   - Fatal error reporting does not return normally.
   - Traceability: `yy_fatal_error`

10. **Observed behavior parity**
    - For the covered runtime operations in this specification, Rust-port behavior matches the original C module’s externally observable semantics in module-level tests.
    - Traceability: all functions listed in this specification