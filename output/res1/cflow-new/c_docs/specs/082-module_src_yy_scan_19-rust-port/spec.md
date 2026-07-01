# spec.md

## Title

Functional Specification: `module_src_yy_scan_19` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_yy_scan_19`
- **Category**: `module_cluster`
- **Source files analyzed**: `src/c.c`
- **Primary source functions**:
  - `yy_scan_string`
  - `yy_scan_bytes`
- **Primary source data structure**:
  - `struct yy_buffer_state`
- **Rust branch target**: `082-module_src_yy_scan_19-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module provides scanner input setup for in-memory data. Its responsibility is to accept either a C string or an arbitrary byte sequence, create a scanner buffer representing that input, and return a scanner buffer handle that can be used by the surrounding lexical scanner machinery.

The Rust rewrite must preserve the module’s observable behavior as evidenced by the analyzed functions:

- creating a scan buffer from a NUL-terminated string input
- creating a scan buffer from a byte slice with explicit length, including data that may contain embedded NUL bytes
- returning a buffer state handle representing the prepared scanner input

The specification is limited to this input-buffer creation role and does not extend the module beyond behaviors evidenced by the analyzed source.

## Feature Specification

### Feature: Scan buffer creation from string input

The module shall provide functionality equivalent to `yy_scan_string` for preparing scanner input from a NUL-terminated string.

Behavior required:

- Accept a string input whose logical length is determined from its terminating NUL.
- Prepare a scanner buffer that represents exactly the string content before the terminating NUL.
- Return a buffer state handle associated with that prepared input.

This feature exists to support scanner invocation on ordinary text input already resident in memory.

### Feature: Scan buffer creation from explicit byte input

The module shall provide functionality equivalent to `yy_scan_bytes` for preparing scanner input from a memory region and explicit byte count.

Behavior required:

- Accept a pointer/reference to bytes and an explicit length.
- Treat the provided length as authoritative, independent of byte values within the region.
- Support byte sequences containing embedded NUL bytes.
- Prepare a scanner buffer that represents exactly the provided number of bytes.
- Return a buffer state handle associated with that prepared input.

This feature exists to support scanner invocation on in-memory data that is not necessarily a conventional C string.

### Feature: Scanner-compatible buffer state production

The module shall create and return values corresponding to `YY_BUFFER_STATE` / `struct yy_buffer_state` so the produced buffer can participate in the scanner’s normal input-buffer workflow.

Behavior required:

- The returned handle must identify a valid buffer state created for the supplied input.
- The produced buffer state must be suitable for downstream scanner consumption consistent with the source module’s role.

## User Scenarios & Testing

### Scenario 1: Scan ordinary source text already stored as a string

A caller has source text in memory as a normal NUL-terminated string and needs the scanner to read from it.

Expected support:

- The caller passes the string to the Rust equivalent of `yy_scan_string`.
- The module returns a valid buffer state handle.
- Scanner processing of that buffer sees the same character sequence as the original string content, excluding the terminating NUL.

Suggested tests:

- Input: empty string.
- Input: short ASCII string.
- Input: multi-line string.
- Verify returned handle is valid for scanner use.
- Verify scanned token stream matches the same text provided via normal file-based scanning, where applicable in the surrounding scanner.

### Scenario 2: Scan bytes with embedded NUL values

A caller has in-memory bytes that include one or more `\0` bytes and needs the scanner to process the exact region.

Expected support:

- The caller passes the byte storage and explicit length to the Rust equivalent of `yy_scan_bytes`.
- The module returns a valid buffer state handle.
- The scanner reads exactly the specified number of bytes rather than stopping at the first NUL.

Suggested tests:

- Input bytes: `b"a\0b"` with length 3.
- Input bytes: leading NUL.
- Input bytes: trailing NUL included within explicit length.
- Verify the resulting scan behavior reflects all bytes within the requested span.

### Scenario 3: Distinguish string-based and length-based semantics

A caller uses either string input or explicit byte input depending on source data origin.

Expected support:

- String-based setup uses terminator-based length.
- Byte-based setup uses caller-supplied length.
- The two entry points remain semantically distinct.

Suggested tests:

- For data `b"a\0b\0"`, verify:
  - string-based scanning sees only `a`
  - byte-based scanning with length 4 sees all 4 bytes

### Scenario 4: Handle zero-length input

A caller provides no content.

Expected support:

- Empty string input can be scanned.
- Explicit byte input with length 0 can be scanned.
- A valid buffer state handle is still produced if the source behavior does so for empty inputs.

Suggested tests:

- Empty string through string API.
- Non-null byte storage with length 0 through bytes API.
- Confirm scanner reaches end of input immediately without reading unrelated memory.

## Requirements

### Functional Requirements

#### FR-1: Provide string-based scan buffer creation
The Rust module shall provide functionality equivalent to source function `yy_scan_string` in `src/c.c`, creating a scanner buffer from a NUL-terminated string input.

**Traceability**: `yy_scan_string` in `src/c.c:2456-2460`

#### FR-2: Derive string length from terminator
For string-based scan buffer creation, the Rust module shall determine the input extent from the terminating NUL rather than from an explicit caller-provided length.

**Traceability**: `yy_scan_string` in `src/c.c:2456-2460`

#### FR-3: Provide explicit-length byte scan buffer creation
The Rust module shall provide functionality equivalent to source function `yy_scan_bytes` in `src/c.c`, creating a scanner buffer from a byte region and explicit byte length.

**Traceability**: `yy_scan_bytes` in `src/c.c:2471-2499`

#### FR-4: Preserve explicit-length semantics for byte input
For byte-based scan buffer creation, the Rust module shall treat the provided length as the exact number of input bytes to expose to the scanner, regardless of embedded NUL bytes within the region.

**Traceability**: `yy_scan_bytes` in `src/c.c:2471-2499`

#### FR-5: Return scanner buffer state handles
Each successful scan-buffer creation operation shall return a buffer state handle corresponding to the scanner buffer created for that input.

**Traceability**: return type `YY_BUFFER_STATE` for `yy_scan_string` and `yy_scan_bytes`; `struct yy_buffer_state` in `src/c.c`

#### FR-6: Represent input as scanner-consumable buffer state
The created buffer state shall represent the supplied in-memory input in a form consumable by the scanner workflow that uses `struct yy_buffer_state`.

**Traceability**: `struct yy_buffer_state` occurrences in `src/c.c`; `yy_scan_string`; `yy_scan_bytes`

#### FR-7: Support empty input
The Rust module shall support creation of scanner buffers for empty string input and for explicit byte input with length zero, consistent with the source functions’ accepted interfaces.

**Traceability**: `yy_scan_string`; `yy_scan_bytes`

### Key Entities

#### Entity: Scanner Buffer State
The central entity is the scanner buffer state, corresponding to `YY_BUFFER_STATE` and backed by `struct yy_buffer_state`.

Role:

- Represents one prepared scanner input buffer.
- Serves as the return value from both module entry points.
- Connects in-memory input data to the surrounding scanner.

Relationship to module behavior:

- `yy_scan_string` produces one scanner buffer state from string input.
- `yy_scan_bytes` produces one scanner buffer state from explicit byte input.

**Traceability**: `struct yy_buffer_state` in `src/c.c`; return type of `yy_scan_string` and `yy_scan_bytes`

#### Entity: String Input
A NUL-terminated string used as input to string-based scan buffer creation.

Role:

- Supplies text content whose extent is determined by the first terminating NUL.

Relationship to buffer state:

- One string input is converted into one scanner buffer state by `yy_scan_string`.

**Traceability**: parameter `const char * yystr` in `yy_scan_string`

#### Entity: Byte Sequence Input
An in-memory byte region paired with an explicit integer length.

Role:

- Supplies arbitrary bytes, including possible embedded NUL values.

Relationship to buffer state:

- One byte region plus length is converted into one scanner buffer state by `yy_scan_bytes`.

**Traceability**: parameters `const char * yybytes, int _yybytes_len` in `yy_scan_bytes`

## Success Criteria

### SC-1: String input parity
For representative inputs, the Rust implementation of string-based scan buffer creation shall produce scanner behavior equivalent to the source module for the same NUL-terminated string content.

**Measured by**: tests using empty, single-line, and multi-line strings through the Rust equivalent of `yy_scan_string`.

**Traceability**: `yy_scan_string`

### SC-2: Embedded-NUL byte handling parity
For representative byte sequences containing embedded NUL bytes, the Rust implementation of explicit-length scan buffer creation shall expose exactly the specified byte count to the scanner.

**Measured by**: tests using byte inputs with embedded, leading, and trailing NUL bytes through the Rust equivalent of `yy_scan_bytes`.

**Traceability**: `yy_scan_bytes`

### SC-3: Correct distinction between APIs
The Rust implementation shall preserve the semantic difference between string-based input length determination and explicit-length byte input.

**Measured by**: a test case where identical underlying memory yields different effective scan extents when passed to the string-based entry point versus the byte-based entry point.

**Traceability**: `yy_scan_string`, `yy_scan_bytes`

### SC-4: Valid buffer state production
Both Rust entry points shall return a valid scanner buffer state handle on successful creation, suitable for use by the scanner path that consumes buffer states.

**Measured by**: integration tests that create a buffer through each entry point and successfully perform scanner consumption from the returned state.

**Traceability**: `YY_BUFFER_STATE`; `struct yy_buffer_state`; `yy_scan_string`; `yy_scan_bytes`

### SC-5: Empty-input support
The Rust implementation shall correctly handle empty input for both entry points without exposing bytes beyond the declared input extent.

**Measured by**: tests for empty string input and zero-length byte input that immediately observe end-of-input behavior.

**Traceability**: `yy_scan_string`; `yy_scan_bytes`

## Out of Scope

The Rust port specification does not require any capabilities not evidenced by the analyzed source for this module, including:

- new public APIs beyond the two evidenced entry points
- thread-safety guarantees
- serialization or persistence of buffer state
- recovery mechanisms beyond source-equivalent behavior
- performance targets or benchmarking commitments
- functionality centered on unrelated scanner internals not required to preserve the observable behavior of these entry points