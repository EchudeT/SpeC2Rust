# spec.md

## Title

Rust Functional Specification for `module_src_parser.c_29`

## Metadata

- Project: `cflow-new`
- Source module: `src/parser.c`
- Module category: `module_cluster`
- Target Rust branch: `092-module_src_parser.c_29-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides parser-token stack management and parser-facing token inspection utilities for the source parser. Its evidenced responsibilities are:

- maintain an ordered stack/list of parser tokens,
- support parser backtracking through mark/restore operations,
- support token insertion and deletion within the token sequence,
- support forward and backward movement through the token stream,
- initialize and clear parser token state,
- produce token-oriented debug and error reporting tied to token location and type.

The Rust rewrite must preserve the observable behavior of these responsibilities as exercised through the functions identified in `src/parser.c`, without adding unrelated capabilities.

## Scope

### In Scope

The Rust version must implement the functionality evidenced by these parser services:

- token sequence storage and lifetime management,
- token push/insert/delete operations,
- parser position save and restore,
- advancing to next token and putting one token back,
- full or partial cleanup of parser token state,
- parser initialization,
- token formatting for debugging,
- file/location-aware parser error reporting based on a token.

### Out of Scope

The following are not evidenced by this module input and must not be introduced as module requirements:

- new parsing algorithms beyond token stack handling,
- concurrency guarantees,
- persistence or serialization,
- public FFI surface,
- recovery systems beyond existing stack/state restoration behavior,
- benchmarking requirements.

## Feature Specification

### 1. Token Sequence Management

The module maintains parser tokens as an ordered sequence used by the parser. Tokens carry, at minimum, a token type, source line association, and token text/value reference, as evidenced by `tokins`, `tokpush`, `print_token`, `file_error`, and debug functions.

The Rust version must support:

- appending a token to the current token sequence,
- inserting a token at a specified position in the sequence,
- deleting a contiguous token range from the sequence,
- clearing token contents and parser-visible token state,
- cleaning up allocated token storage/state when parsing work is reset or completed.

### 2. Parser Position Save/Restore

The module supports parser backtracking by recording a stack/token position and later restoring to it, as evidenced by `mark` and `restore`.

The Rust version must preserve the functional behavior that:

- a caller can capture a parser position,
- parsing/token navigation may advance after that point,
- restoring returns the parser state to the previously captured position for subsequent token consumption.

### 3. Token Navigation

The module exposes token-stream traversal primitives, as evidenced by `nexttoken` and `putback`.

The Rust version must support:

- moving forward to obtain/use the next token in sequence,
- moving backward by one token position after forward movement,
- maintaining consistent parser-visible position before and after insertions, deletions, and restore operations.

### 4. Parser Initialization and Reset

The module initializes parser token machinery through `init_parse` and provides reset-style operations through `clearstack` and `cleanup_stack`.

The Rust version must support:

- initialization into a defined empty/ready parser-token state,
- reinitialization or clearing such that subsequent parsing starts from a clean token sequence,
- cleanup that leaves no stale parser-visible token entries.

### 5. Debug and Diagnostic Support

The module includes token-to-string conversion and debugging output helpers through `token_type_str`, `print_token`, `dbgtok`, `debugtoken`, and error reporting through `file_error`.

The Rust version must preserve the module’s functional diagnostic behavior:

- token types can be rendered into human-readable form for diagnostics,
- individual tokens can be formatted for debugging output,
- diagnostics may include caller-supplied formatting context,
- parser file/token errors are reported with token-associated source context when available.

The exact output wording need not be specified beyond preserving the information content evidenced by these functions: token identity/type, token text where applicable, and source location context for file-related errors.

## User Scenarios & Testing

### Scenario 1: Build a Token Stream During Parsing

A parser stage receives lexical tokens and appends them one by one for later parser consumption.

Expected support:

- tokens can be pushed in order,
- subsequent forward traversal returns them in the same order,
- an empty initialized state becomes non-empty after pushes.

Suggested tests:

- initialize state, push multiple tokens, then traverse with repeated next-token operations and confirm ordering,
- confirm token metadata needed for diagnostics remains associated with each token.

### Scenario 2: Insert Synthetic or Adjusted Tokens

A parser transformation needs to insert a token at a specific position in an existing sequence.

Expected support:

- insertion at a valid position preserves surrounding token order,
- inserted token becomes visible during later traversal from that region.

Suggested tests:

- create a sequence of known tokens, insert one in the middle, and verify traversal order reflects the insertion,
- insert at beginning and end boundaries and verify expected placement.

### Scenario 3: Delete a Contiguous Token Range

A parser adjustment removes one or more tokens from the stored sequence.

Expected support:

- deletion removes all tokens in the specified range,
- tokens outside the range remain in their original relative order,
- traversal after deletion does not expose removed tokens.

Suggested tests:

- remove a middle range and verify remaining order,
- remove a single-token range,
- verify parser position behavior remains valid after deletion.

### Scenario 4: Backtrack to an Earlier Parser Position

The parser speculatively consumes tokens, then decides to revert.

Expected support:

- a position can be marked before speculative reads,
- traversal can advance beyond the mark,
- restoring makes the parser continue again from the saved position.

Suggested tests:

- mark at start, consume several tokens, restore, and confirm the next token seen matches the first post-mark token,
- mark in the middle of a sequence, advance, restore, and confirm position accuracy.

### Scenario 5: Put Back the Most Recently Advanced Token

The parser needs one-token lookahead and then wants to retract that advance.

Expected support:

- after advancing once, putback repositions to the prior token boundary,
- the next advance returns the same token again.

Suggested tests:

- push a small sequence, call nexttoken once, call putback, then call nexttoken again and verify the same token is observed,
- verify repeated invalid backward movement is handled consistently with the original module’s behavior.

### Scenario 6: Reset Parser State Between Parsing Sessions

A parser completes one parsing run and starts another.

Expected support:

- clear/reset removes old token visibility,
- cleanup leaves the module ready for reinitialization or fresh use,
- initialization after cleanup yields an empty valid state.

Suggested tests:

- populate state, clear or clean up, then verify traversal yields no prior tokens,
- run two separate parse sessions and verify no cross-session token leakage.

### Scenario 7: Produce Token Debug Output and File Errors

A parser encounters unexpected input and emits diagnostics.

Expected support:

- token type names are available for debug display,
- token-specific debug output includes token-identifying information,
- file errors can report a message tied to the token’s source line/context.

Suggested tests:

- construct representative tokens and verify debug formatting contains token type and token text where applicable,
- trigger file error reporting and verify the emitted diagnostic includes the provided message and token location context.

## Requirements

### Functional Requirements

#### FR-1: Token storage and ordered access
The module shall maintain an ordered parser token sequence that supports subsequent parser traversal.

Traceability: `tokpush`, `tokins`, `nexttoken`, `print_token` in `src/parser.c`.

#### FR-2: Token append
The module shall allow a token defined by type, source line, and token text/value to be appended to the current sequence.

Traceability: `tokpush` in `src/parser.c`.

#### FR-3: Token insertion
The module shall allow insertion of a token defined by type, source line, and token text/value at a caller-specified sequence position.

Traceability: `tokins` in `src/parser.c`.

#### FR-4: Token range deletion
The module shall allow deletion of a contiguous range of tokens identified by beginning and ending positions.

Traceability: `tokdel` in `src/parser.c`.

#### FR-5: Parser position capture
The module shall allow the parser to capture a token-stack/parser position for later restoration.

Traceability: `mark` and `Stackpos` usage in `src/parser.c`.

#### FR-6: Parser position restoration
The module shall restore parser token state to a previously captured position so that subsequent token traversal resumes from that point.

Traceability: `restore` and `Stackpos` usage in `src/parser.c`.

#### FR-7: Forward token traversal
The module shall advance through the token sequence one token at a time.

Traceability: `nexttoken` in `src/parser.c`.

#### FR-8: Single-step backward traversal
The module shall support moving parser position backward by one token after advancement.

Traceability: `putback` in `src/parser.c`.

#### FR-9: Initialization
The module shall initialize parser token state into a defined ready state before use.

Traceability: `init_parse` in `src/parser.c`.

#### FR-10: Clearing and cleanup
The module shall support clearing active token-stack state and cleanup of parser token resources/state.

Traceability: `clearstack`, `cleanup_stack` in `src/parser.c`.

#### FR-11: Token type naming for diagnostics
The module shall provide human-readable token type naming for diagnostic or debug output.

Traceability: `token_type_str` in `src/parser.c`.

#### FR-12: Token debug formatting
The module shall support formatting/output of token information for debugging, including token-specific detail.

Traceability: `print_token`, `dbgtok`, `debugtoken` in `src/parser.c`.

#### FR-13: Token-aware file error reporting
The module shall support reporting an error message associated with a token and its source location context.

Traceability: `file_error` in `src/parser.c`.

### Key Entities

#### Token entry
A token entry represents one parser token in the maintained sequence. Based on function signatures and diagnostic helpers, each token entry includes:

- token type,
- source line reference,
- token text/value reference.

Traceability: `TOKSTK *` usage across `print_token`, `dbgtok`, `debugtoken`, `file_error`; token construction arguments in `tokins` and `tokpush`.

#### Token sequence / token stack
The token sequence is the ordered collection of token entries manipulated by append, insert, delete, clear, cleanup, and traversal operations.

Traceability: `tokpush`, `tokins`, `tokdel`, `clearstack`, `cleanup_stack`, `nexttoken`, `putback`.

#### Parser position (`Stackpos`)
A parser position identifies a restorable location within token-stack state.

Traceability: `mark(Stackpos pos)`, `restore(Stackpos pos)`.

#### Diagnostic rendering context
The module contains diagnostic helpers that derive human-readable information from token entries and token types.

Traceability: `token_type_str`, `print_token`, `dbgtok`, `debugtoken`, `file_error`.

## Success Criteria

1. The Rust module can be initialized into an empty usable state and subsequently reused after clear/cleanup without exposing stale tokens.
   - Traceability: `init_parse`, `clearstack`, `cleanup_stack`.

2. Appending tokens in sequence and traversing with forward token advancement yields tokens in insertion order.
   - Traceability: `tokpush`, `nexttoken`.

3. Inserting a token at a specified position causes subsequent traversal to reflect the new token at that position while preserving the relative order of unaffected tokens.
   - Traceability: `tokins`, `nexttoken`.

4. Deleting a contiguous token range removes exactly that range from subsequent traversal results.
   - Traceability: `tokdel`, `nexttoken`.

5. After capturing a parser position, advancing, and restoring, subsequent traversal resumes from the captured position.
   - Traceability: `mark`, `restore`, `nexttoken`.

6. After one forward advance, a single putback causes the next forward advance to return the same token again.
   - Traceability: `nexttoken`, `putback`.

7. Token diagnostics produced by the Rust module include token type identity and token-associated content comparable in information to the C module’s debug helpers.
   - Traceability: `token_type_str`, `print_token`, `dbgtok`, `debugtoken`.

8. File/token error reporting includes the supplied message and token-related source context when a token is provided.
   - Traceability: `file_error`.

9. All required behaviors are implemented strictly within the evidenced functional boundary of `src/parser.c`, with no dependence on unevidenced new capabilities.
   - Traceability: all listed functions and token-related types in the module input.