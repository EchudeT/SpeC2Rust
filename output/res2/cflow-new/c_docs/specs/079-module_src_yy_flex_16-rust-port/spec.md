# spec.md

## Title
Rust Functional Specification for `module_src_yy_flex_16`

## Metadata
- Project: `cflow-new`
- Module: `module_src_yy_flex_16`
- Category: `module_cluster`
- Source basis: `src/c.c`
- Rust branch: `079-module_src_yy_flex_16-rust-port`
- Generation date: `2026-06-17`

## Overview
This module covers scanner-support functionality evidenced in `src/c.c` around Flex-generated runtime support, with directly identified functions for bounded string copying and string length calculation, and related scanner buffer state structures used by the same runtime area.

The Rust rewrite must preserve the functional behavior of this module as a scanner-internal support component. Its required scope is limited to:

- copying up to a specified number of bytes from one character sequence to another,
- determining the length of a NUL-terminated character sequence,
- supporting these operations in the context of scanner buffer state handling represented by `yy_buffer_state`,
- preserving behavior expected by the surrounding generated scanner runtime.

No broader string library behavior, public API expansion, or unrelated buffer-management features are required unless they are necessary to preserve the evidenced module behavior.

## Feature Specification

### Summary
The module provides low-level helper behavior for scanner runtime string handling. The Rust version must implement equivalent functionality for:

- bounded character-sequence copy (`yy_flex_strncpy`),
- character-sequence length calculation (`yy_flex_strlen`).

These helpers exist alongside scanner buffer state structures and must remain compatible with scanner-internal use where textual data and buffer contents are manipulated.

### In-Scope Behavior
The Rust rewrite must support the following evidenced behaviors:

1. **Bounded copy of character data**
   - Provide behavior equivalent to copying characters from a source sequence into a destination sequence for exactly the requested count boundary used by the scanner runtime.
   - Preserve byte-for-byte ordering of copied character data.
   - Support use with ordinary C-style text data represented as NUL-terminated byte sequences, while not depending on Unicode semantics.

2. **Length calculation for NUL-terminated character data**
   - Provide behavior equivalent to counting characters until the first terminating NUL byte.
   - Return the length as an integer-sized value suitable for scanner-internal use.

3. **Compatibility with scanner buffer-oriented runtime usage**
   - The helper behavior must remain usable wherever scanner runtime logic interacts with `yy_buffer_state` and related text/buffer storage.
   - Rust behavior must not assume ownership or text invariants beyond the byte-oriented, NUL-terminated usage evidenced by the C module.

### Out of Scope
The Rust rewrite must not introduce unsupported capabilities not evidenced by the module input, including:

- new public string utility APIs,
- Unicode-aware transformations,
- thread-safety guarantees,
- persistence, serialization, or recovery features,
- alternate memory models beyond what is required to preserve module behavior.

## User Scenarios & Testing

### Scenario 1: Copy scanner text into an internal destination buffer
A scanner-internal path needs to copy character data from one buffer region to another using a maximum count.

**Expected support in Rust**
- The module copies the source bytes in order into the destination region up to the requested count.
- The operation behaves consistently for scanner text represented as raw bytes/characters.
- The result is suitable for subsequent scanner runtime use.

**Testing approach**
- Provide a source byte string and destination storage.
- Invoke the Rust equivalent of bounded copy with a specified count.
- Verify that the destination contains the expected first `n` bytes from the source and that no copied byte differs from the source ordering.

### Scenario 2: Determine the length of scanner text
A scanner-internal path needs the length of a NUL-terminated text fragment.

**Expected support in Rust**
- The module counts bytes until the first NUL terminator.
- Returned length matches the visible pre-terminator content size.

**Testing approach**
- Provide byte strings with a terminating NUL at different positions.
- Verify that the Rust equivalent returns the count of bytes before the first NUL.
- Verify the zero-length case for an immediate terminator.

### Scenario 3: Use helper functions with buffer-state-managed text
Scanner runtime logic represented by `yy_buffer_state` stores or references text that must be measured or copied.

**Expected support in Rust**
- String helper behavior remains usable with byte data associated with scanner buffers.
- No assumptions are made that conflict with buffer-based scanner text handling.

**Testing approach**
- Construct representative buffer-associated byte content.
- Use length and copy operations on that content.
- Verify outputs remain correct when the text originates from scanner buffer state context rather than standalone literals.

## Requirements

### Functional Requirements

#### FR-1: Bounded copy support
The module shall provide functionality equivalent to `yy_flex_strncpy` from `src/c.c:2688-2694` for copying character data from a source sequence to a destination sequence with an explicit count limit.

#### FR-2: Byte-order preservation during copy
The bounded copy behavior shall preserve the exact order and values of copied bytes from source to destination, matching scanner-internal character-sequence handling evidenced by `yy_flex_strncpy` in `src/c.c:2688-2694`.

#### FR-3: NUL-terminated length calculation
The module shall provide functionality equivalent to `yy_flex_strlen` from `src/c.c:2698-2705` for determining the number of bytes preceding the first NUL terminator in a character sequence.

#### FR-4: Scanner-runtime compatibility
The helper functions shall remain compatible with scanner runtime usage associated with `yy_buffer_state` structures evidenced in `src/c.c` at lines 191, 233-298, 2176, 2377, 2378, 2383, 2396, 2398, 2404, and 2427.

#### FR-5: Byte-oriented text handling
The module shall treat scanner text as byte-oriented character data, not as Unicode text, consistent with the `char *` and `const char *` function signatures in `src/c.c:2688-2705`.

### Key Entities

#### `yy_buffer_state`
A scanner buffer-state structure evidenced multiple times in `src/c.c`. It represents the scanner’s active or managed input buffer context. In this module specification, it is the primary surrounding runtime entity whose text storage or referenced content may be measured or copied by the helper functions.

#### `yy_trans_info`
A scanner transition-related structure evidenced at `src/c.c:440-444`. It is part of the surrounding scanner runtime context. No independent functionality is required here beyond preserving compatibility with the scanner runtime environment in which the helper functions operate.

#### `obstack`
A storage-management structure evidenced at `src/c.c:760` and `src/c.c:2850`. It is a surrounding runtime data structure relevant to the scanner/parser environment. The Rust rewrite does not need to generalize or expose obstack behavior beyond maintaining compatibility with the module’s evidenced scanner-support role.

#### Relationships
- `yy_flex_strncpy` and `yy_flex_strlen` operate on character data used by the scanner runtime.
- `yy_buffer_state` is the most directly relevant runtime structure because buffer-managed text may be the subject of copying and length calculation.
- `yy_trans_info` and `obstack` are contextual runtime entities in the same module area, but no additional direct behavior is evidenced for these specific helper functions.

## Success Criteria

1. **Copy equivalence**
   - For representative scanner text inputs, the Rust bounded-copy behavior produces the same copied byte sequence as the C behavior described by `yy_flex_strncpy` (`src/c.c:2688-2694`).

2. **Length equivalence**
   - For representative NUL-terminated inputs, the Rust length-calculation behavior returns the same pre-terminator byte count as the C behavior described by `yy_flex_strlen` (`src/c.c:2698-2705`).

3. **Zero-length and empty-string correctness**
   - The Rust implementation correctly handles a copy count of zero and correctly reports length zero for a sequence beginning with a NUL terminator, matching the helper-function semantics evidenced in `src/c.c:2688-2705`.

4. **Scanner buffer context compatibility**
   - Tests using byte content associated with scanner buffer-state usage succeed without requiring semantics outside the scanner runtime context evidenced by `yy_buffer_state` in `src/c.c`.

5. **No unsupported functional expansion**
   - The Rust rewrite delivers the helper functionality and scanner-runtime compatibility described in this specification without adding unrelated capabilities not evidenced by the source module.