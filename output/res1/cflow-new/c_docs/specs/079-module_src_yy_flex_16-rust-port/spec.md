# spec.md

## Title

Rust Functional Specification for `module_src_yy_flex_16`

## Summary

This module provides the local string helper behavior used by the generated scanner logic in `src/c.c`. The analyzed functionality is limited to two internal helper operations:

- copying exactly `n` characters from one character sequence to another (`yy_flex_strncpy`)
- computing the length of a NUL-terminated character sequence (`yy_flex_strlen`)

The Rust rewrite must preserve the observable behavior these helpers provide to the surrounding scanner module, without adding unrelated capabilities.

## Scope

In scope for this module:

- behavior equivalent to `yy_flex_strncpy`
- behavior equivalent to `yy_flex_strlen`
- compatibility with the scanner module’s use of C-style character buffers and buffer-state-driven processing

Out of scope:

- redesign of scanner logic
- new public APIs beyond what is needed to support the existing module behavior
- changes to buffer-state semantics not evidenced by the source analysis
- additional string manipulation features

## Source Basis

This specification is derived from the following analyzed elements:

- File: `src/c.c`
- Functions:
  - `yy_flex_strncpy` at `src/c.c:2688-2694`
  - `yy_flex_strlen` at `src/c.c:2698-2705`
- Related data structures referenced by the module context:
  - `struct yy_buffer_state`
  - `struct yy_trans_info`
  - `struct obstack`

## Feature Specification

### Feature: Fixed-length scanner string copy

The module must support copying a specified number of characters from a source character sequence into a destination character sequence.

Behavioral expectations:

- Exactly `n` characters are copied in order from source to destination.
- The operation is driven by the requested count, not by early termination on a NUL byte.
- The copy operation does not itself require or guarantee appending a trailing NUL unless that trailing byte is included within the copied range.
- The copied bytes must preserve the original byte values for the copied span.

This feature exists to support scanner-internal manipulation of text fragments stored in C-style buffers.

Traceability:

- `yy_flex_strncpy` in `src/c.c:2688-2694`

### Feature: NUL-terminated scanner string length calculation

The module must support calculating the length of a character sequence by scanning until the first NUL terminator.

Behavioral expectations:

- The returned length is the number of characters before the first NUL byte.
- The NUL terminator itself is not counted.
- The function operates on a C-style string representation expected by the surrounding scanner code.

This feature exists to support scanner-internal logic that depends on string lengths derived from generated-lexer buffers and text storage.

Traceability:

- `yy_flex_strlen` in `src/c.c:2698-2705`

### Feature: Compatibility with scanner buffer-oriented context

The Rust rewrite must preserve interoperability with the module context in which scanner text is held in buffer-oriented structures.

Behavioral expectations:

- The helper logic must work correctly with character data originating from scanner buffer state.
- The helper logic must not impose semantics that conflict with the surrounding generated-scanner model of `yy_buffer_state`.

Traceability:

- `struct yy_buffer_state` occurrences in `src/c.c`
- helper functions located in the same scanner source file `src/c.c`

## User Scenarios & Testing

### Scenario 1: Copying a known scanner token fragment

A scanner workflow has a token fragment already available as a character sequence and needs to copy a fixed number of bytes into another buffer.

Expected result:

- the destination contains the same first `n` bytes as the source
- bytes beyond `n` are not part of the helper’s required behavior
- no implicit truncation occurs before `n` due to embedded NUL within the copied range

Test coverage:

- copy zero bytes
- copy one byte
- copy multiple bytes
- copy a range containing a NUL byte before the end of the requested length

Traceability:

- `yy_flex_strncpy`

### Scenario 2: Measuring a scanner text buffer as a C string

A scanner workflow needs the length of text stored as a NUL-terminated character sequence.

Expected result:

- the returned value equals the number of bytes before the first NUL terminator

Test coverage:

- empty string returns `0`
- single-character string returns `1`
- multi-character string returns the full visible length
- string with embedded data followed by NUL stops at the first NUL

Traceability:

- `yy_flex_strlen`

### Scenario 3: Using helper behavior with scanner-managed buffers

Scanner-managed character data associated with buffer state is passed through these helper operations.

Expected result:

- helper behavior remains valid for text stored in scanner buffers
- no behavior change is introduced by the Rust port relative to the C module’s helper semantics

Test coverage:

- prepare representative scanner-style NUL-terminated buffers and verify length calculation
- prepare representative scanner-style fixed-length slices and verify exact-byte copying

Traceability:

- `yy_buffer_state`
- `yy_flex_strncpy`
- `yy_flex_strlen`

## Requirements

### Functional Requirements

#### FR-1: Fixed-count copy
The module shall provide behavior equivalent to copying exactly `n` characters from a source character sequence to a destination sequence.

Traceability:

- `yy_flex_strncpy` in `src/c.c:2688-2694`

#### FR-2: Order preservation during copy
The module shall preserve source byte order across the copied range.

Traceability:

- `yy_flex_strncpy` in `src/c.c:2688-2694`

#### FR-3: No implicit NUL-termination guarantee
The module shall not require automatic insertion of a trailing NUL when performing fixed-count copy, except where a NUL byte is already included within the copied range.

Traceability:

- `yy_flex_strncpy` in `src/c.c:2688-2694`

#### FR-4: NUL-terminated length calculation
The module shall provide behavior equivalent to returning the number of characters preceding the first NUL byte in a character sequence.

Traceability:

- `yy_flex_strlen` in `src/c.c:2698-2705`

#### FR-5: Empty-string handling
The module shall return length `0` when the first byte of the provided character sequence is NUL.

Traceability:

- `yy_flex_strlen` in `src/c.c:2698-2705`

#### FR-6: Scanner-context compatibility
The module shall support use with character data associated with the scanner source file’s buffer-state-driven processing context.

Traceability:

- `src/c.c`
- `struct yy_buffer_state`
- `yy_flex_strncpy`
- `yy_flex_strlen`

### Key Entities

#### `yy_buffer_state`
Represents scanner buffer state in the surrounding module context. It is the principal buffer-oriented entity from which character data used by the helper operations may originate.

Relationship to this module:

- provides the scanner-managed text context in which the helper functions are used
- establishes that the helpers must remain compatible with buffer-backed character data

Traceability:

- `struct yy_buffer_state` occurrences in `src/c.c`

#### `yy_trans_info`
Represents scanner transition information in the generated scanner context.

Relationship to this module:

- contextual entity in the same scanner source file
- indicates that the string helpers belong to lexer/scanner infrastructure rather than general-purpose text utilities

Traceability:

- `struct yy_trans_info` in `src/c.c:440-444`

#### `obstack`
Represents storage management context used elsewhere in the same source file.

Relationship to this module:

- contextual storage entity that may coexist with scanner text handling
- no additional behavior beyond source-evidenced module context is required from the Rust rewrite for this specification

Traceability:

- `struct obstack` in `src/c.c:760`
- `struct obstack` in `src/c.c:2850`

## Success Criteria

### SC-1: Exact fixed-count copy behavior
Tests demonstrate that the Rust rewrite copies exactly the requested number of bytes for representative values of `n`, including `0`, `1`, and multi-byte counts.

Traceability:

- FR-1
- `yy_flex_strncpy`

### SC-2: Copy preserves byte values including embedded NUL
Tests demonstrate that copied output matches source bytes position-for-position across the copied span, including cases where a NUL byte appears before the end of the requested range.

Traceability:

- FR-2
- FR-3
- `yy_flex_strncpy`

### SC-3: Correct C-string length results
Tests demonstrate that the Rust rewrite returns the number of bytes before the first NUL byte for empty, single-character, and multi-character inputs.

Traceability:

- FR-4
- FR-5
- `yy_flex_strlen`

### SC-4: Scanner-style buffer compatibility
Tests using representative scanner-style character buffers confirm that the helper behavior remains correct when applied to data shaped like scanner-managed text.

Traceability:

- FR-6
- `yy_buffer_state`
- `yy_flex_strncpy`
- `yy_flex_strlen`

### SC-5: No unsupported capability expansion
The Rust rewrite exposes only the helper behavior evidenced by the analyzed source for this module specification and does not require additional functional capabilities to satisfy module parity.

Traceability:

- Scope
- Source Basis