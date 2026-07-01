# Specification: main_root_clear_ungetc_09

- **Project**: `pwd`
- **Module**: `main_root_clear_ungetc_09`
- **Category**: `main_cluster`
- **Source basis**: `fflush.c`
- **Rust target branch**: `009-main_root_clear_ungetc_09-rust-port`
- **Generation date**: `2026-06-07`

## 1. Overview

This module provides internal support for clearing a stream's pushed-back input state created by `ungetc`, while preserving correct stream position behavior where required.

The analyzed C module contains two internal functions that operate on a `FILE *` stream:

- a position-preserving clearing operation
- a general clearing operation

The Rust rewrite must implement the same functional boundary: clearing any unread pushed-back input associated with a stream and ensuring that stream position is handled consistently with the original module behavior.

## 2. Feature Specification

### 2.1 Functional Intent

The module exists to remove pending `ungetc` state from an open stream.

When a stream has bytes or characters previously pushed back onto the input stream, this module clears that temporary input state so the stream no longer exposes those pushed-back bytes as pending input.

### 2.2 Required Rust Behavior

The Rust version must implement behavior equivalent to the C module for the following cases:

1. **Clear pushed-back input from a stream**
   - If a stream contains buffered `ungetc` data, the module clears it.

2. **Preserve effective stream position when required**
   - For the position-preserving path, clearing pushed-back input must not leave the stream at an incorrect file position relative to the underlying stream state expected after removing pushed-back data.

3. **Support a general clear operation**
   - The general clear operation must clear pushed-back input state on a stream and perform any position handling needed by the original module's behavior.

4. **Operate as internal stream-state management**
   - The module is internal support logic for stream handling, not a user-facing feature with expanded API guarantees beyond the analyzed behavior.

### 2.3 Out of Scope

The Rust rewrite must not introduce unevidenced capabilities, including:

- new public APIs beyond what is needed for the port
- guarantees about thread safety
- persistence, serialization, or recovery behavior
- behavior unrelated to clearing `ungetc` state
- broader flushing semantics not evidenced by this module fragment alone

## 3. User Scenarios & Testing

## 3.1 Usage Scenarios

### Scenario 1: Stream with pushed-back input needs cleanup

A caller managing a stream has previously used `ungetc`-like behavior, leaving pushed-back input pending. Before continuing normal stream processing, internal logic clears that pushed-back state so future reads observe the underlying stream content rather than stale pushed-back bytes.

**Expected result**: pushed-back input is removed from the stream state.

### Scenario 2: Cleanup must not disturb effective file position

A stream has pushed-back input that affects the logical read state. Internal cleanup uses the position-preserving path so the pushed-back state is removed without leaving the stream position inconsistent with the intended underlying file position.

**Expected result**: pushed-back state is cleared and the stream remains correctly positioned.

### Scenario 3: Stream without pushed-back input is processed

Internal code invokes the clear operation on a stream that has no pending `ungetc` state.

**Expected result**: the operation completes without introducing incorrect position changes or synthetic pushed-back state.

## 3.2 Test Scenarios

1. **Clearing pending pushed-back input**
   - Prepare a stream with pushed-back input.
   - Invoke the equivalent of the clear operation.
   - Verify that subsequent reads do not return the previously pushed-back data unless it is present in the underlying stream itself.

2. **Position-preserving clear**
   - Prepare a seekable stream where position can be observed.
   - Create pushed-back input.
   - Invoke the position-preserving clear path.
   - Verify that the resulting stream position matches the expected underlying position after removal of pushed-back state.

3. **No-op safety on clean stream**
   - Invoke the clear operation on a stream with no pushed-back input.
   - Verify that stream usability is preserved and no incorrect repositioning occurs.

4. **Repeated clearing**
   - Clear a stream's pushed-back state more than once.
   - Verify that the stream remains usable and no new pushed-back state is introduced.

## 4. Requirements

### 4.1 Functional Requirements

- **FR-1**: The module shall clear pending `ungetc`/pushed-back input state associated with a stream.
  **Traceability**: `clear_ungetc_buffer`, `clear_ungetc_buffer_preserving_position` in `fflush.c`.

- **FR-2**: The module shall support a mode of clearing pushed-back input that preserves correct stream position behavior while removing that state.
  **Traceability**: `clear_ungetc_buffer_preserving_position` in `fflush.c`.

- **FR-3**: The module shall provide a general clearing operation for stream pushed-back input state.
  **Traceability**: `clear_ungetc_buffer` in `fflush.c`.

- **FR-4**: The module shall operate on stream objects corresponding to C `FILE *` semantics as the unit whose pushed-back state is managed.
  **Traceability**: both functions accept `FILE *fp` in `fflush.c`.

- **FR-5**: Applying the clear operation to a stream shall leave the stream without residual pushed-back input managed by this module.
  **Traceability**: purpose implied by both function names and signatures in `fflush.c`.

### 4.2 Key Entities

- **Stream handle**
  - Represents the stream being manipulated.
  - In the source module this is a `FILE *`.
  - All module behavior is scoped to this entity.

- **Pushed-back input state**
  - Represents input previously returned to the stream through `ungetc`-style behavior.
  - This is the state the module clears.

- **Stream position**
  - Represents the stream's effective read position relevant to underlying file access.
  - The position-preserving operation maintains correct position semantics while clearing pushed-back input.

### 4.3 Entity Relationships

- A **stream handle** may contain **pushed-back input state**.
- Clearing operations act on the **stream handle** to remove its **pushed-back input state**.
- The position-preserving operation additionally constrains how **stream position** is affected during that clearing.

## 5. Success Criteria

- **SC-1**: For a stream with pending pushed-back input, after module clearing, subsequent reads no longer consume the previously pushed-back data from internal pushback state.
  **Traceability**: `clear_ungetc_buffer`, `clear_ungetc_buffer_preserving_position`.

- **SC-2**: For the position-preserving path on a seekable stream, observed stream position after clearing matches the expected underlying position semantics of the original C behavior.
  **Traceability**: `clear_ungetc_buffer_preserving_position`.

- **SC-3**: Invoking the general clear path on a stream without pushed-back input does not corrupt stream usability or introduce incorrect pushed-back state.
  **Traceability**: `clear_ungetc_buffer`.

- **SC-4**: Repeated invocation of the clear logic on the same stream does not reintroduce pushed-back data and leaves the stream in a consistent state for continued I/O.
  **Traceability**: `clear_ungetc_buffer`, `clear_ungetc_buffer_preserving_position`.

## 6. Notes for Rust Port Validation

The Rust rewrite should be validated against behavior-level equivalence, not C-level implementation structure. The key acceptance focus is:

- pushed-back input state is actually cleared
- position-sensitive clearing preserves correct stream position semantics
- clean streams remain usable after the operation