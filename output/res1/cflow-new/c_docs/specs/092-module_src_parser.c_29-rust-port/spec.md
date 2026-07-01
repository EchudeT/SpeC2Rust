# spec.md

## Title

Rust Functional Specification: `module_src_parser.c_29`

## Overview

This module provides parser token-stack support for the `cflow-new` parser layer. Its evidenced responsibilities are:

- storing parser tokens with associated type, source line, and token text
- moving through the stored token stream
- inserting, deleting, and rewinding tokens within that stream
- resetting and cleaning parser stack state
- reporting token-oriented debug and file-location error information
- initializing parser state before use

The Rust rewrite on branch `092-module_src_parser.c_29-rust-port` must preserve this functional behavior for the parser workflow represented by `src/parser.c`.

## Scope

In scope for this module:

- token stack lifecycle management
- token cursor navigation
- token insertion and deletion within the stack
- save/restore of parser position
- parser-state initialization and clearing
- token debug formatting and token-related error reporting

Out of scope unless required by direct compatibility with this module’s current behavior:

- lexical analysis itself
- higher-level grammar recognition
- new parsing features or public APIs not evidenced by `src/parser.c`

## Source Basis

This specification is derived from the following evidenced module surface in `src/parser.c`:

- token/debug/error helpers: `print_token`, `token_type_str`, `dbgtok`, `debugtoken`, `file_error`
- token stack state operations: `mark`, `restore`, `tokdel`, `tokins`, `tokpush`, `cleanup_stack`, `clearstack`, `nexttoken`, `putback`, `init_parse`
- core internal entities: token-stack record structure(s), parser stack state, and `balance_state`

## Feature Specification

### 1. Token stack storage and mutation

The module maintains an ordered sequence of parser tokens. Each stored token carries, at minimum, token kind, source line, and token text. The Rust version must support:

- appending a token to the current token sequence
- inserting a token at a specified position in the current sequence
- deleting a contiguous token range from the current sequence
- removing or releasing accumulated token-stack contents during cleanup/reset operations

Behavior must preserve token ordering except where explicitly changed by insertion or deletion.

Traceability: `tokpush`, `tokins`, `tokdel`, `cleanup_stack`, `clearstack`.

### 2. Token stream navigation

The module exposes sequential navigation over the token sequence used by parser logic. The Rust version must support:

- advancing to the next token in sequence
- moving one step backward in the token stream when parser logic needs to put back a token
- maintaining a current parser position that reflects these operations

Navigation behavior must remain consistent after stack mutations where the original C module supports such use.

Traceability: `nexttoken`, `putback`.

### 3. Parser position save and restore

The module allows parser logic to save a current token-stack position and later restore it. The Rust version must support:

- marking the current parser/token-stack position
- restoring the parser/token-stack state to a previously marked position

This behavior is required for parser backtracking or tentative parsing flows evidenced by the presence of `mark` and `restore`.

Traceability: `mark`, `restore`, `Stackpos` usage.

### 4. Parser-state initialization and reset

The module initializes parser state before token operations begin and provides stack-clearing behavior for reuse or shutdown. The Rust version must support:

- explicit parser-state initialization
- clearing token-stack contents and position state
- cleanup of transient stack storage so subsequent parser activity starts from a known empty state

Traceability: `init_parse`, `cleanup_stack`, `clearstack`.

### 5. Token diagnostics and debug visibility

The module includes token-oriented debugging helpers. The Rust version must preserve equivalent functional diagnostic behavior required by current parser development and troubleshooting flows:

- producing a readable representation of a token record
- mapping token type identifiers to human-readable token type text
- emitting token debug output, including delimiter-aware and formatted forms

The exact output formatting may vary only if all token identity information presently conveyed by the C module remains available and understandable.

Traceability: `print_token`, `token_type_str`, `dbgtok`, `debugtoken`.

### 6. Token-location error reporting

The module reports file/token-related errors using token context. The Rust version must preserve the ability to emit an error message associated with a token position or token record so callers can identify where parser failure occurred.

Traceability: `file_error`.

## User Scenarios & Testing

### Scenario 1: Build a token sequence during parsing

A parser stage initializes this module, pushes tokens as they are recognized, and then iterates through them using next-token access.

The Rust version must support a test where:

1. parser state is initialized
2. multiple tokens are appended with distinct types, lines, and text
3. successive next-token operations traverse them in insertion order

Expected result:
- traversal order matches append order
- token metadata remains associated with each token

Traceability: `init_parse`, `tokpush`, `nexttoken`.

### Scenario 2: Perform tentative parsing with rollback

A parser reaches a point where it must try a parse path, save its position, and later restore that position if the path fails.

The Rust version must support a test where:

1. tokens exist in the stack
2. the current position is marked
3. one or more next-token operations advance the cursor
4. restore returns the cursor to the marked position
5. subsequent next-token behavior matches what would have occurred from that earlier point

Expected result:
- restored traversal reproduces the same subsequent token sequence

Traceability: `mark`, `restore`, `nexttoken`.

### Scenario 3: Put back one token after reading too far

A parser consumes a token, determines it advanced too far, and steps back once.

The Rust version must support a test where:

1. at least two tokens are present
2. next-token is called
3. putback is called
4. next-token is called again

Expected result:
- the second next-token returns the same token that was just put back

Traceability: `nexttoken`, `putback`.

### Scenario 4: Insert a synthetic token into the stream

Parser logic may need to inject a token into an existing sequence at a defined position.

The Rust version must support a test where:

1. a token sequence is built
2. a token is inserted at a specific valid position
3. stream traversal reflects the inserted token at that location
4. surrounding tokens preserve their relative order

Traceability: `tokins`, `nexttoken`.

### Scenario 5: Delete a token range

Parser logic removes a contiguous span of tokens from the maintained sequence.

The Rust version must support a test where:

1. a sequence of several tokens exists
2. a middle range is deleted
3. traversal skips the deleted range and preserves the remaining order

Traceability: `tokdel`, `nexttoken`.

### Scenario 6: Reset parser token state between uses

A caller clears or cleans up the parser stack, then reuses the module.

The Rust version must support a test where:

1. tokens are created and traversed
2. clear or cleanup is invoked
3. a new parse session is initialized or resumed from empty state
4. old tokens are no longer visible in traversal

Traceability: `cleanup_stack`, `clearstack`, `init_parse`.

### Scenario 7: Produce token diagnostics

A developer enables parser debugging or a parse error occurs, requiring token-readable output.

The Rust version must support tests where:

- a token can be rendered in human-readable form
- token type identifiers are converted to readable names
- an error report can include token context, including source-line association when present

Traceability: `print_token`, `token_type_str`, `dbgtok`, `debugtoken`, `file_error`.

## Requirements

### Functional Requirements

#### FR-1: Initialize parser token state
The module shall provide an initialization operation that prepares parser/token-stack state for use from a known starting condition.

Traceability: `init_parse`.

#### FR-2: Store tokens with parser-relevant metadata
The module shall store tokens as records containing token type, source line, and token text sufficient to support traversal, mutation, debugging, and error reporting.

Traceability: `tokpush`, `tokins`, `print_token`, `file_error`.

#### FR-3: Append tokens
The module shall support appending a token to the current end of the maintained token sequence.

Traceability: `tokpush`.

#### FR-4: Insert tokens at a position
The module shall support insertion of a token at a specified token-sequence position.

Traceability: `tokins`.

#### FR-5: Delete contiguous token ranges
The module shall support deleting a contiguous range of tokens identified by begin and end positions.

Traceability: `tokdel`.

#### FR-6: Advance through tokens sequentially
The module shall support retrieving or advancing to the next token from the current parser position.

Traceability: `nexttoken`.

#### FR-7: Rewind one token step
The module shall support stepping the current parser position backward to undo a prior token advance.

Traceability: `putback`.

#### FR-8: Save parser position
The module shall support capturing the current parser position in a saved position value.

Traceability: `mark`, `Stackpos`.

#### FR-9: Restore parser position
The module shall support restoring the parser state to a previously saved position value.

Traceability: `restore`, `Stackpos`.

#### FR-10: Clear active token-stack contents
The module shall support clearing token-stack state so that subsequent token traversal operates on an empty or reset state.

Traceability: `clearstack`.

#### FR-11: Clean up token-stack resources/state
The module shall support cleanup of accumulated token-stack state/resources used during parsing.

Traceability: `cleanup_stack`.

#### FR-12: Provide human-readable token type names
The module shall support conversion from token type identifiers to readable token type strings for diagnostics.

Traceability: `token_type_str`.

#### FR-13: Provide human-readable token rendering
The module shall support rendering a token record in readable diagnostic form.

Traceability: `print_token`, `dbgtok`, `debugtoken`.

#### FR-14: Report errors using token context
The module shall support emitting parser/file error information associated with a token record.

Traceability: `file_error`.

### Key Entities

#### Token record
A token record represents one parser token in the maintained sequence. Based on the evidenced operations, it includes at least:

- token type identifier
- source line number
- token text
- position within the maintained token sequence, directly or indirectly through stack storage

Relationships:
- token records are stored in an ordered token stack/sequence
- navigation and mutation operations act on these records
- diagnostic and error functions read from these records

Traceability: `TOKSTK` usage across `print_token`, `dbgtok`, `debugtoken`, `file_error`, `tokins`, `tokpush`.

#### Parser position (`Stackpos`)
A saved parser position identifies a point in the token sequence to which parsing can later return.

Relationships:
- created or captured during mark operations
- consumed by restore operations
- tied to the current token cursor/state

Traceability: `mark`, `restore`.

#### Token stack / parser stack state
This is the module-maintained ordered storage and cursor state for parser tokens.

Relationships:
- owns token records
- is initialized, traversed, mutated, cleared, and cleaned up by this module
- underlies next-token, putback, insert, delete, mark, and restore behavior

Traceability: `tokdel`, `tokins`, `tokpush`, `cleanup_stack`, `clearstack`, `nexttoken`, `putback`, `init_parse`.

#### Balance state
`balance_state` appears as an internal parser-support structure in this file. The Rust rewrite must preserve any behavior dependence this module has on such state where it affects parser stack correctness, but no expanded capability is specified beyond evidenced compatibility.

Traceability: `struct balance_state` declarations in `src/parser.c`.

## Success Criteria

1. **Initialization correctness**
   After initialization, token traversal begins from an empty or defined starting state with no residual tokens from prior use.
   Traceability: `init_parse`, `clearstack`, `cleanup_stack`.

2. **Append and traversal correctness**
   Given a sequence of appended tokens, sequential traversal returns them in the same order with preserved type, line, and text metadata.
   Traceability: `tokpush`, `nexttoken`.

3. **Insertion correctness**
   Inserting a token at a valid position causes traversal to include that token at the requested location without disturbing the relative order of unaffected tokens.
   Traceability: `tokins`, `nexttoken`.

4. **Deletion correctness**
   Deleting a token range removes exactly that contiguous span from subsequent traversal results.
   Traceability: `tokdel`, `nexttoken`.

5. **Single-step rewind correctness**
   After one advance and one putback, the next advance returns the same token as before the rewind.
   Traceability: `nexttoken`, `putback`.

6. **Mark/restore correctness**
   After saving a parser position, advancing, and restoring, subsequent traversal reproduces the same token sequence that was available immediately after the mark.
   Traceability: `mark`, `restore`, `nexttoken`.

7. **Reset/cleanup correctness**
   After clear or cleanup, previously stored tokens are no longer returned by traversal, and the module can be used again for a fresh token sequence.
   Traceability: `cleanup_stack`, `clearstack`, `init_parse`.

8. **Diagnostic coverage**
   Token diagnostics provide readable token-type and token-instance information sufficient to identify the token involved in parser debugging output.
   Traceability: `print_token`, `token_type_str`, `dbgtok`, `debugtoken`.

9. **Error-context preservation**
   Error reporting can associate a message with token context, including source-line information when present in the token record.
   Traceability: `file_error`.

10. **Behavioral compatibility boundary**
    The Rust rewrite does not require new functionality beyond the evidenced token-stack management, navigation, reset, and diagnostics responsibilities present in `src/parser.c`.
    Traceability: full evidenced function set for this module.