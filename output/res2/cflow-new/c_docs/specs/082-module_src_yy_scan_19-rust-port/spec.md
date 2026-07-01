# spec.md

## Title

Rust Functional Specification for `module_src_yy_scan_19`

## Document Metadata

- Project: `cflow-new`
- Module: `module_src_yy_scan_19`
- Category: `module_cluster`
- Source file: `src/c.c`
- Rust branch: `082-module_src_yy_scan_19-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides scanner-input buffer creation for in-memory text supplied by a caller. Its exposed behavior is represented by two functions:

- `yy_scan_string`
- `yy_scan_bytes`

The Rust rewrite must preserve the functional role of these entry points: turning caller-provided byte or string data into a scanner buffer state that can be consumed by the lexer machinery used by the larger module in `src/c.c`.

The specification is limited to behavior evidenced by the analyzed functions and the referenced scanner buffer state type. It does not define unrelated lexer behavior, parsing behavior, concurrency guarantees, or new public interfaces.

## Feature Specification

### Feature Summary

The module creates scanner buffer objects from caller-supplied in-memory input so that the lexer can scan that input as if it were a normal scanner source.

Two forms of input are supported:

1. A NUL-terminated C string input.
2. A byte sequence with an explicit length.

### Required Functional Behavior

#### 1. Scan a NUL-terminated string

The Rust version must support creating a scanner buffer state from a caller-provided string equivalent to `yy_scan_string(const char *yystr)`.

Expected behavior:

- The entire input string up to its terminating NUL is treated as scanner input.
- The produced scanner buffer state is suitable for downstream scanner consumption.
- The operation returns a buffer-state handle representing the scanned input.

Traceability:
- Function: `yy_scan_string` in `src/c.c:2456-2460`
- Related entity: `struct yy_buffer_state`

#### 2. Scan an explicit byte buffer

The Rust version must support creating a scanner buffer state from a caller-provided byte sequence equivalent to `yy_scan_bytes(const char *yybytes, int _yybytes_len)`.

Expected behavior:

- The exact number of bytes provided by the caller is treated as scanner input.
- Input handling must not depend on embedded NUL bytes being absent, because the byte-oriented API accepts an explicit length.
- The produced scanner buffer state is suitable for downstream scanner consumption.
- The operation returns a buffer-state handle representing the scanned input.

Traceability:
- Function: `yy_scan_bytes` in `src/c.c:2471-2499`
- Related entity: `struct yy_buffer_state`

#### 3. Distinguish string-based and byte-based input length semantics

The Rust version must preserve the functional distinction between the two entry points:

- string scanning derives content length from string termination;
- byte scanning derives content length from the explicit caller-provided length.

Traceability:
- Functions: `yy_scan_string`, `yy_scan_bytes`

#### 4. Produce buffer state objects compatible with scanner buffer handling

The Rust version must represent the created scan input as a scanner buffer state corresponding to the source module’s `struct yy_buffer_state`.

Expected behavior:

- The returned object identifies a concrete scanner input buffer.
- The object is usable by the surrounding scanner subsystem that operates on buffer states.

Traceability:
- Functions: `yy_scan_string`, `yy_scan_bytes`
- Type: `struct yy_buffer_state`

## User Scenarios & Testing

### Scenario 1: Scan a normal text string

A caller has a conventional text string and wants the lexer to read from that string instead of a file or preexisting buffer.

Expected support:

- The caller passes the string to the string-based scan entry point.
- A scanner buffer state is returned.
- The lexer can then consume the buffer contents in order.

Suggested tests:

- Create a buffer from a simple ASCII string.
- Verify the returned buffer handle is valid for scanner use.
- Verify scanner consumption reaches the end of the provided string content.

Traceability:
- Function: `yy_scan_string`

### Scenario 2: Scan bytes with embedded NUL

A caller has input data that includes one or more `\0` bytes and must preserve the full byte sequence as scanner input.

Expected support:

- The caller passes the byte pointer and explicit length to the byte-based scan entry point.
- The full provided length is accepted as input, including bytes after embedded NUL.
- A scanner buffer state is returned for scanner use.

Suggested tests:

- Provide a byte sequence containing embedded NUL bytes.
- Verify the resulting scanner input length corresponds to the explicit length, not the first NUL.
- Verify scanner-visible content includes data after embedded NUL.

Traceability:
- Function: `yy_scan_bytes`

### Scenario 3: Distinguish string semantics from byte semantics

A caller uses both APIs and expects them to differ only in how input extent is determined.

Expected support:

- For the same underlying memory, the string-based path stops at the first terminating NUL.
- The byte-based path uses the explicit byte count.

Suggested tests:

- Use memory containing `abc\0def`.
- With string scanning, verify only `abc` is treated as input.
- With byte scanning and length 7, verify all seven bytes are treated as input.

Traceability:
- Functions: `yy_scan_string`, `yy_scan_bytes`

### Scenario 4: Use returned buffer state with the scanner subsystem

A caller creates a scan buffer and passes control to the lexer’s existing buffer-processing flow.

Expected support:

- The returned buffer state integrates with the scanner subsystem’s existing expectations for buffer objects.

Suggested tests:

- After creating a buffer through each API, pass it into the scanner flow used elsewhere in the module.
- Verify no adaptation layer or alternate object type is required.

Traceability:
- Functions: `yy_scan_string`, `yy_scan_bytes`
- Type: `struct yy_buffer_state`

## Requirements

### Functional Requirements

#### FR-1: String input scanning

The module shall provide behavior equivalent to creating a scanner buffer from a NUL-terminated input string.

Traceability:
- `yy_scan_string` (`src/c.c:2456-2460`)

#### FR-2: Byte input scanning with explicit length

The module shall provide behavior equivalent to creating a scanner buffer from caller-supplied bytes using an explicit byte length.

Traceability:
- `yy_scan_bytes` (`src/c.c:2471-2499`)

#### FR-3: Exact byte-count preservation for byte scanning

For byte-based scanning, the module shall treat exactly the supplied length as input extent.

Traceability:
- `yy_scan_bytes` (`src/c.c:2471-2499`)

#### FR-4: String-termination-based extent for string scanning

For string-based scanning, the module shall determine input extent from NUL-terminated string content.

Traceability:
- `yy_scan_string` (`src/c.c:2456-2460`)

#### FR-5: Return scanner buffer state handles

Each scan-creation operation shall return a scanner buffer state corresponding to `YY_BUFFER_STATE` / `struct yy_buffer_state`.

Traceability:
- `yy_scan_string`
- `yy_scan_bytes`
- `struct yy_buffer_state`

#### FR-6: Scanner compatibility of created buffers

Created buffers shall be usable by the scanner subsystem that consumes `yy_buffer_state` instances in the source module.

Traceability:
- `yy_scan_string`
- `yy_scan_bytes`
- `struct yy_buffer_state`

### Key Entities

#### `yy_buffer_state`

Core scanner buffer object used to represent an in-memory input source prepared for lexing.

Role in this module:

- It is the output form produced by both scanning entry points.
- It carries the scanner-facing representation of caller-supplied string or byte input.

Traceability:
- `struct yy_buffer_state` references in `src/c.c`
- Return type of `yy_scan_string`
- Return type of `yy_scan_bytes`

#### Input memory supplied by the caller

Caller-provided in-memory content passed through one of two API shapes:

- NUL-terminated string input
- Byte pointer plus explicit byte length

Role in this module:

- Serves as the source material from which a scanner buffer state is created.
- Determines whether input extent is string-terminated or length-explicit.

Traceability:
- Parameters of `yy_scan_string`
- Parameters of `yy_scan_bytes`

#### Relationship between entities

- Caller-provided input is converted into a `yy_buffer_state`.
- The choice of API determines how the input boundary is computed.
- The resulting buffer state is then consumed by the larger scanner subsystem.

## Success Criteria

### Functional Acceptance Criteria

1. A Rust caller can create a scanner buffer from a normal text string through functionality equivalent to `yy_scan_string`.
   - Measured by a test that produces a valid buffer handle for a NUL-terminated input string.

Traceability:
- `yy_scan_string`

2. A Rust caller can create a scanner buffer from arbitrary bytes through functionality equivalent to `yy_scan_bytes`.
   - Measured by a test that produces a valid buffer handle for a byte slice and explicit length.

Traceability:
- `yy_scan_bytes`

3. Byte-based scanning preserves explicit-length semantics.
   - Measured by a test using embedded NUL bytes where all bytes up to the provided length remain part of scanner input.

Traceability:

4. String-based scanning preserves NUL-terminated semantics.
   - Measured by a test where scanning from `abc\0def` via the string entry point uses only the prefix before the terminating NUL.

Traceability:

5. Returned Rust buffer objects map cleanly to the scanner’s buffer-state concept.
   - Measured by integration tests showing that buffers created by both APIs can be consumed by the surrounding scanner flow without alternate representations.

Traceability:
- `struct yy_buffer_state`

6. The Rust rewrite introduces no required public functionality beyond the two evidenced scan-creation behaviors and their buffer-state output.
   - Measured by API review against this specification.

Traceability: