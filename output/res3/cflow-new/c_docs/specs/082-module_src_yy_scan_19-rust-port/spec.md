# spec.md

## Title
Rust Functional Specification for `module_src_yy_scan_19`

## Document Metadata
- Project: `cflow-new`
- Module: `module_src_yy_scan_19`
- Category: `module_cluster`
- Source basis: `src/c.c`
- Rust branch: `082-module_src_yy_scan_19-rust-port`
- Generation date: 2026-06-17

## Overview
This module provides scanner-buffer creation for in-memory input supplied by callers as either a NUL-terminated string or a byte sequence with an explicit length. Its functional role is to convert caller-owned textual or byte input into a scanner buffer state that the lexer can consume as input.

The Rust rewrite must preserve the observable behavior of the C module functions that create such scan buffers:
- `yy_scan_string`
- `yy_scan_bytes`

The module’s scope is limited to creating and returning scanner-compatible buffer state from provided in-memory input. No broader lexer behavior, parsing behavior, or unrelated buffer management capabilities are specified here unless directly required by these entry points.

## Feature Specification

### Feature: Create scanner input from a NUL-terminated string
The module must accept a caller-provided C-style string input and produce a scanner buffer state representing that input for subsequent lexical scanning.

Behavior required from the source evidence:
- The string-oriented entry point consumes a pointer to a NUL-terminated character sequence.
- The resulting scanner buffer corresponds to the bytes of that string up to, but not including, the terminating NUL.
- The function returns a scanner buffer state handle representing the prepared input.

Traceability:
- `yy_scan_string` in `src/c.c:2456-2460`
- `struct yy_buffer_state` references in `src/c.c`

### Feature: Create scanner input from a byte sequence with explicit length
The module must accept a pointer to bytes plus an explicit byte count and produce a scanner buffer state representing exactly that byte range for subsequent lexical scanning.

Behavior required from the source evidence:
- The byte-oriented entry point consumes a byte pointer and an integer length.
- The resulting scanner buffer corresponds exactly to the specified number of bytes, regardless of embedded NUL bytes.
- The function returns a scanner buffer state handle representing the prepared input.

Traceability:
- `yy_scan_bytes` in `src/c.c:2471-2499`
- `struct yy_buffer_state` references in `src/c.c`

### Feature: Return scanner-owned buffer state suitable for lexer consumption
Both entry points must produce a scanner buffer state object that is usable as lexer input state.

Behavior required from the source evidence:
- The return type of both public entry points is `YY_BUFFER_STATE`.
- The created state is a buffer-state abstraction rather than a raw byte slice.
- The Rust rewrite must preserve the concept of an allocated or otherwise materialized scanner buffer object that represents the supplied input.

Traceability:
- `yy_scan_string` and `yy_scan_bytes`
- `struct yy_buffer_state` in `src/c.c`

## User Scenarios & Testing

### Scenario 1: Scan from a normal text string
A caller has a conventional NUL-terminated text string and needs the lexer to read from that in-memory text instead of a file or stream.

Expected support:
- The caller can pass the string to the string entry point.
- A buffer state handle is returned.
- The scanner buffer represents the text content before the terminating NUL.

Suggested tests:
- Input `"abc"` produces a non-null/non-error buffer state.
- The represented input length is 3 bytes.
- The represented content matches `abc`.

Traceability:
- `yy_scan_string`

### Scenario 2: Scan from bytes containing embedded NUL
A caller needs to scan in-memory data where the input may contain `\0` bytes and therefore cannot rely on NUL termination to define its extent.

Expected support:
- The caller can pass a pointer to bytes and an explicit length to the byte entry point.
- A buffer state handle is returned.
- The scanner buffer includes all bytes within the specified length, including embedded NUL values.

Suggested tests:
- Input bytes `['a', '\0', 'b']` with length `3` produce a non-null/non-error buffer state.
- The represented input length is exactly 3 bytes.
- The represented content preserves the embedded NUL.

Traceability:
- `yy_scan_bytes`

### Scenario 3: String entry point behaves as a convenience wrapper over byte scanning semantics
A caller using ordinary strings expects the string-scanning API to map naturally to byte scanning over the string’s content length.

Expected support:
- The string entry point uses the input’s content length determined by its terminating NUL.
- The produced scanner buffer corresponds to the same visible byte content that would be obtained by passing the same bytes and length explicitly.

Suggested tests:
- For input `"token"`, the string entry point yields content equivalent to bytes `['t','o','k','e','n']` with length `5`.
- The terminal NUL is not counted as part of caller-visible content.

Traceability:
- `yy_scan_string`
- `yy_scan_bytes`

### Scenario 4: Empty input
A caller may provide an empty input source.

Expected support:
- Empty string input can be wrapped as a valid scanner buffer state.
- Empty byte input with length `0` can be wrapped as a valid scanner buffer state.

Suggested tests:
- `""` via the string entry point returns a valid buffer state representing zero bytes of content.
- A zero-length byte call returns a valid buffer state representing zero bytes of content.

Traceability:
- `yy_scan_string`
- `yy_scan_bytes`

## Requirements

### Functional Requirements

#### FR-1: String-based scanner buffer creation
The Rust module shall provide behavior equivalent to `yy_scan_string`: accept a NUL-terminated string input and create a scanner buffer state for the string’s content up to the terminating NUL.

Traceability:
- `yy_scan_string` in `src/c.c:2456-2460`

#### FR-2: Byte-based scanner buffer creation
The Rust module shall provide behavior equivalent to `yy_scan_bytes`: accept a byte pointer and explicit byte length and create a scanner buffer state for exactly that byte range.

Traceability:
- `yy_scan_bytes` in `src/c.c:2471-2499`

#### FR-3: Embedded-NUL preservation for explicit-length input
For explicit-length byte input, the Rust module shall preserve embedded NUL bytes as part of the represented input when they fall within the supplied length.

Traceability:
- `yy_scan_bytes` signature and purpose in `src/c.c:2471-2499`

#### FR-4: String input extent determined by terminator
For string input, the Rust module shall determine input extent from the first terminating NUL and shall not treat that terminating NUL as part of the caller-provided content.

Traceability:
- `yy_scan_string` in `src/c.c:2456-2460`

#### FR-5: Return scanner buffer state handle
Both entry points shall return a scanner buffer state handle/object corresponding to the created scan input.

Traceability:
- Return type `YY_BUFFER_STATE` for both functions
- `struct yy_buffer_state` references in `src/c.c`

#### FR-6: Support empty inputs
Both entry points shall support empty input ranges and produce a valid scanner buffer state representing zero bytes of caller content.

Traceability:
- `yy_scan_string`
- `yy_scan_bytes`

### Key Entities

#### Scanner Buffer State
The central entity is the scanner buffer state, represented in C as `struct yy_buffer_state` and exposed through `YY_BUFFER_STATE`. It is the returned object from both public entry points and represents lexer-readable input derived from caller-provided memory.

Relationship to module behavior:
- Created by both scan-buffer creation functions.
- Encapsulates the input prepared for lexical scanning.

Traceability:
- `struct yy_buffer_state` references throughout `src/c.c`
- `yy_scan_string`
- `yy_scan_bytes`

#### String Input
A NUL-terminated character sequence supplied by the caller for the string-scanning entry point.

Relationship to module behavior:
- Used only with the string-oriented API.
- Its terminating NUL defines the extent of caller-visible content.

Traceability:
- `yy_scan_string`

#### Byte Input with Explicit Length
A caller-provided memory range defined by pointer plus byte count for the byte-scanning entry point.

Relationship to module behavior:
- Used with the explicit-length API.
- Defines content by length rather than by terminator, allowing embedded NUL bytes.

Traceability:
- `yy_scan_bytes`

## Success Criteria

### SC-1: Correct string-content mapping
Given a NUL-terminated input string, the Rust implementation creates a scanner buffer state whose represented content matches the bytes before the terminating NUL.

Measured by:
- Tests with non-empty and empty strings.
- No extra caller-visible byte beyond the content length is included.

Traceability:
- `yy_scan_string`

### SC-2: Correct explicit-length mapping
Given a byte pointer and length `N`, the Rust implementation creates a scanner buffer state whose represented content is exactly `N` bytes long and byte-for-byte identical to the supplied range.

Measured by:
- Tests over multiple lengths, including `0`.
- Equality checks on represented content.

Traceability:
- `yy_scan_bytes`

### SC-3: Embedded-NUL handling
For explicit-length inputs containing embedded NUL bytes, the Rust implementation preserves those bytes within the created scanner buffer state.

Measured by:
- A test using input such as `[0x61, 0x00, 0x62]` with length `3`.
- Verification that all three bytes remain present and ordered.

Traceability:
- `yy_scan_bytes`

### SC-4: API-level result creation
Each supported entry point returns a scanner buffer state object/handle for valid input cases covered by the source module’s intended use.

Measured by:
- Construction tests confirming a returned buffer-state value for normal and empty inputs.

Traceability:
- `yy_scan_string`
- `yy_scan_bytes`
- `struct yy_buffer_state`

### SC-5: Behavioral consistency between string and byte forms for ordinary text
For ordinary text without embedded NUL, scanning via the string entry point and via the byte entry point with the corresponding length yields equivalent represented content.

Measured by:
- Comparative tests using sample inputs such as `"a"`, `"token"`, and `""`.

Traceability:
- `yy_scan_string`
- `yy_scan_bytes`

## Out of Scope
The following are not specified by this document because they are not evidenced as module-specific responsibilities from the provided source slice:
- General lexer tokenization behavior
- Parser behavior
- File-based scanner input setup
- New public APIs beyond Rust equivalents of the evidenced functionality
- Thread-safety guarantees
- Serialization or persistence
- Error recovery strategies not directly evidenced by the listed functions