# spec.md

## Overview

- **Project**: `pwd`
- **Module**: `main_root_clear_ungetc_09`
- **Category**: `main_cluster`
- **Rust branch**: `009-main_root_clear_ungetc_09-rust-port`
- **Source evidence**: `fflush.c`
- **Analyzed functions**:
  - `clear_ungetc_buffer_preserving_position`
  - `clear_ungetc_buffer`

This module is responsible for clearing any pushed-back input state associated with a stream. Its behavior is limited to removing `ungetc`-style buffered input, with one variant explicitly preserving the stream position while doing so.

The Rust rewrite must preserve this functional scope: clear pushed-back input state on a stream, and support the position-preserving behavior evidenced by the source module.

---

## Feature Specification

### Summary

This module provides internal stream-state cleanup behavior for input streams that may contain data previously pushed back for rereading. The cleanup behavior has two forms:

1. **Clear pushed-back input while preserving current stream position**
2. **Clear pushed-back input for a stream**

The Rust version must implement equivalent behavior for stream-like state managed by the rewritten module, without adding new externally visible capabilities beyond this cleanup role.

### In-Scope Behavior

- Detect and remove stream state corresponding to pushed-back input.
- Support a mode in which the pushed-back input state is cleared without changing the logical file position observed by subsequent stream operations.
- Apply cleanup directly to the target stream object supplied to the operation.

### Out-of-Scope Behavior

The following are not evidenced by the input and must not be introduced as required behavior:

- New public APIs beyond what is needed by the Rust rewrite
- General flushing semantics beyond the specific ungetc-buffer clearing role shown here
- Thread-safety guarantees
- Cross-process or persistent state handling
- Serialization, logging, recovery, or diagnostics features
- Behavior for unrelated stream transformations

---

## User Scenarios & Testing

### Scenario 1: Clearing pushed-back input before further stream use

A caller has a stream with one or more characters previously pushed back onto it. Before continuing normal stream processing, the module clears that pushed-back input state so subsequent reads no longer consume the pushed-back characters.

**Expected result**:
- The stream no longer exposes the pushed-back characters after cleanup.
- Normal stream use can continue from the underlying stream state after the pushback is removed.

**Testing focus**:
- Create a stream state with pushed-back input.
- Invoke the clearing behavior.
- Verify that subsequent reads do not return the previously pushed-back data.

### Scenario 2: Clearing pushed-back input while preserving file position

A caller needs to remove pushed-back input but must not alter the current stream position as observed for future file-position-sensitive operations.

**Expected result**:
- Pushed-back input is removed.
- The logical stream position after cleanup matches the position before cleanup.

**Testing focus**:
- Record stream position.
- Create pushed-back input state.
- Invoke the position-preserving clearing behavior.
- Verify that the stream position is unchanged after cleanup.
- Verify that pushed-back data is no longer returned by reads.

### Scenario 3: No-op behavior when there is no pushed-back input

A caller invokes cleanup on a stream that has no pushed-back input state.

**Expected result**:
- The operation completes without introducing new buffered input state.
- Stream position remains valid and usable.
- Subsequent reads behave the same as they would have before the call.

**Testing focus**:
- Apply both cleanup behaviors to a stream with no pushback state.
- Verify no extra data appears and stream position remains consistent.

---

## Requirements

### Functional Requirements

#### FR-1: Clear pushed-back input state
The module shall provide behavior that clears `ungetc`-style pushed-back input associated with a target stream.

**Traceability**:
- `fflush.c`
- `clear_ungetc_buffer`

#### FR-2: Preserve stream position during clearing
The module shall provide behavior that clears pushed-back input while preserving the stream position of the target stream.

**Traceability**:
- `fflush.c`
- `clear_ungetc_buffer_preserving_position`

#### FR-3: Operate on the supplied stream object
The cleanup behavior shall act on the specific stream instance provided by the caller, rather than on global or unrelated stream state.

**Traceability**:
- `fflush.c`
- `clear_ungetc_buffer_preserving_position (FILE *fp)`
- `clear_ungetc_buffer (FILE *fp)`

#### FR-4: Support safe continuation of stream use after cleanup
After pushed-back input is cleared, the stream shall remain usable for subsequent stream operations consistent with the stream position and remaining underlying input state.

**Traceability**:
- `fflush.c`
- `clear_ungetc_buffer`
- `clear_ungetc_buffer_preserving_position`

### Key Entities

#### Stream handle
The core entity is the stream object passed as `FILE *fp` in the C source. It represents the target input stream whose pushed-back input state may be cleared.

#### Pushed-back input state
This is the stream-associated state created by `ungetc`-style operations. The module’s purpose is to remove this state.

#### Stream position
This is the logical position within the stream that must be preserved by the position-preserving cleanup behavior.

### Entity Relationships

- A **stream handle** may contain **pushed-back input state**.
- Clearing operations modify the **pushed-back input state** associated with a given **stream handle**.
- The position-preserving operation must remove **pushed-back input state** without changing the **stream position** of that same stream handle.

---

## Success Criteria

### SC-1: Pushed-back input is removed
When the Rust implementation is applied to a stream containing pushed-back input, subsequent reads shall not return data solely due to that prior pushback state.

**Traceability**:
- `clear_ungetc_buffer`

### SC-2: Position is preserved when required
When the position-preserving behavior is used, the stream position measured before and after the operation shall be identical.

**Traceability**:
- `clear_ungetc_buffer_preserving_position`

### SC-3: Streams without pushback remain unaffected in observable behavior
Applying the Rust implementation to a stream with no pushed-back input shall not introduce new readable data and shall not corrupt subsequent stream use.

**Traceability**:
- `clear_ungetc_buffer`
- `clear_ungetc_buffer_preserving_position`

### SC-4: Behavior is limited to stream-local cleanup
Tests shall show that the operation affects only the supplied stream instance and does not alter unrelated stream state.

**Traceability**:
- Function parameters in `fflush.c` for both analyzed functions