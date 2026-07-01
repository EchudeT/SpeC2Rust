# spec.md

## Title

Rust Port Functional Specification: `module_src_parser.c_29`

## Metadata

- **Project**: `cflow-new`
- **Source module**: `src/parser.c`
- **Module category**: `module_cluster`
- **Rust branch**: `092-module_src_parser.c_29-rust-port`
- **Generation date**: 2026-06-17

## Overview

This module provides parser-token stack management and parser-facing token navigation support for `cflow-new`. Its evidenced responsibilities are:

- maintaining an ordered in-memory token stack,
- inserting, deleting, pushing, clearing, marking, and restoring token positions,
- advancing to the next token and putting a token back,
- initializing parser state,
- producing token-oriented debug and error output.

The Rust rewrite must preserve the module’s observable behavior as a parser support component. The specification is limited to functionality evidenced by `src/parser.c` and its listed functions and data structures.

## Scope

### In Scope

- Token stack lifecycle and mutation behavior
- Parser position save/restore behavior
- Sequential token consumption and single-step backtracking behavior
- Module initialization and cleanup behavior
- Token debug formatting and file/token-context error reporting

### Out of Scope

- Defining a broader lexer
- Adding new parsing features or public APIs
- Concurrency guarantees
- Serialization, persistence, or external protocol support
- Recovery behavior beyond evidenced stack/state restoration operations

## Feature Specification

### Feature 1: Token Stack Management

The module must manage a mutable ordered collection of parser tokens used by the parser.

Supported stack operations must include:

- appending a token to the end of the current token sequence,
- inserting a token at a specified position,
- deleting a contiguous token range,
- clearing active token contents,
- cleaning up token storage associated with the stack.

The Rust version must preserve the sequence semantics required by later token reads and parser backtracking.

### Feature 2: Parser Position Control

The module must support saving and restoring parser stack position through a position marker abstraction.

This includes:

- capturing a current stack position for later reuse,
- restoring parser state to a previously saved position.

The restore behavior must make subsequent token navigation reflect the restored position.

### Feature 3: Token Navigation

The module must support parser consumption of the token sequence.

Required navigation behavior:

- retrieve or advance to the next token in sequence,
- back up by one token position when the parser needs to reconsider the last token.

The Rust version must preserve ordered traversal behavior and support parser workflows that depend on temporary lookahead followed by one-step reversal.

### Feature 4: Parser State Initialization

The module must provide parser-state initialization that places token navigation and stack handling into a defined starting state before use.

### Feature 5: Debug and Error Reporting Support

The module must support token-oriented diagnostics for development and parser error reporting.

Documented support includes:

- converting token types to human-readable names,
- rendering token information for debugging,
- debug printing of individual tokens and token-related messages,
- reporting a file/token-context error using a message and token reference.

The Rust version must preserve equivalent diagnostic intent so that token state and parser failures remain inspectable.

## User Scenarios & Testing

### Scenario 1: Build a token sequence for parsing

A parser-supporting component initializes parser state, then pushes tokens in parse order. The parser calls next-token operations repeatedly and receives tokens in the same order they were inserted.

**Test expectations**

- After initialization and a sequence of pushes, token traversal returns tokens in insertion order.
- No skipped or reordered tokens occur during ordinary traversal.

### Scenario 2: Insert or delete tokens during parser preprocessing

A parser stage modifies the token sequence by inserting a token at a chosen position or deleting a contiguous range, then resumes parsing.

**Test expectations**

- Inserted tokens appear at the specified position during subsequent traversal.
- Deleted ranges no longer appear during traversal.
- Tokens outside the affected range preserve their relative order.

### Scenario 3: Save parser position, inspect ahead, then restore

The parser marks its current position, consumes one or more subsequent tokens for lookahead, then restores the saved position and re-reads from there.

**Test expectations**

- Restoring a marked position causes subsequent traversal to begin again from that saved point.
- Tokens read after restore match the tokens originally available at that position.

### Scenario 4: Consume a token and put it back

The parser reads the next token, determines it is not yet ready to commit to that token, and performs a one-step putback.

**Test expectations**

- A successful putback causes the immediately following next-token call to return the same token again.
- Putback does not reorder surrounding tokens.

### Scenario 5: Reset or clean parser token state

After a parse phase or before shutdown, the parser clears or cleans the token stack.

**Test expectations**

- After clearing, active token traversal behaves as empty or reset state.
- After cleanup, no stale token contents remain available through normal traversal.

### Scenario 6: Diagnose token stream state

During debugging or error handling, the parser requests token-type naming, token printing, or token-context error reporting.

**Test expectations**

- Known token types produce stable human-readable names.
- Debug output includes token-identifying information sufficient to distinguish tokens.
- Error reporting associates the supplied message with the supplied token context.

## Requirements

### Functional Requirements

#### FR-1: Maintain an ordered token sequence
The module shall maintain parser tokens as an ordered sequence that can be appended to, inserted into, and traversed in order.

**Traceability**: `tokpush`, `tokins`, `nexttoken` in `src/parser.c`

#### FR-2: Support deletion of contiguous token ranges
The module shall remove tokens in a caller-specified contiguous range so that removed tokens are no longer returned by subsequent traversal.

**Traceability**: `tokdel` in `src/parser.c`

#### FR-3: Support parser position save and restore
The module shall provide a position marker abstraction that allows current parser position to be saved and later restored.

**Traceability**: `mark`, `restore` in `src/parser.c`

#### FR-4: Support sequential token consumption
The module shall provide token navigation that advances through the token sequence one token at a time in deterministic order.

**Traceability**: `nexttoken` in `src/parser.c`

#### FR-5: Support one-step token backtracking
The module shall support reversing the most recent token advancement so the parser can re-read that token on the next navigation step.

**Traceability**: `putback` in `src/parser.c`

#### FR-6: Support initialization to a defined parser state
The module shall expose initialization behavior that resets parser token handling to a defined starting state before parsing begins.

**Traceability**: `init_parse` in `src/parser.c`

#### FR-7: Support stack clearing and cleanup
The module shall support both clearing active token contents and performing cleanup of token-stack-associated storage/state.

**Traceability**: `clearstack`, `cleanup_stack` in `src/parser.c`

#### FR-8: Provide token-type naming for diagnostics
The module shall provide human-readable token type naming for diagnostic use.

**Traceability**: `token_type_str` in `src/parser.c`

#### FR-9: Provide token debug rendering
The module shall provide debug-oriented rendering of token information and token-related debug messages.

**Traceability**: `print_token`, `dbgtok`, `debugtoken` in `src/parser.c`

#### FR-10: Provide token-context error reporting
The module shall report an error message associated with a supplied token context.

**Traceability**: `file_error` in `src/parser.c`

### Key Entities

#### Token Stack Entry
A token stack entry represents one parser token and its associated metadata used by navigation, diagnostics, and error reporting.

Evidenced token-related fields include:

- token type,
- line association,
- token text/value reference.

**Traceability**: token parameters used by `tokins`, `tokpush`; token access by `print_token`, `dbgtok`, `debugtoken`, `file_error`

#### Stack Position
A stack position identifies a restorable parser location within the token sequence.

It is used to:

- mark the current parser location,
- restore parsing to an earlier location.

**Traceability**: `mark`, `restore`

#### Token Sequence / Stack State
The token sequence is the module-managed ordered collection of token stack entries together with the current navigation position.

It is affected by:

- insertion and append operations,
- range deletion,
- clearing and cleanup,
- next-token and putback navigation,
- initialization and restoration.

**Traceability**: `tokdel`, `tokins`, `tokpush`, `clearstack`, `cleanup_stack`, `nexttoken`, `putback`, `init_parse`, `mark`, `restore`

#### Balance State
The source file defines a balance-state structure. Its presence indicates parser state related to balancing or nesting tracking within this module’s file scope.

The Rust port must preserve any externally observable behavior dependent on such state, but this specification does not assign additional functionality beyond what is evidenced by the listed parser-state and token-stack functions.

**Traceability**: `struct balance_state` definitions in `src/parser.c`

## Success Criteria

1. **Ordered traversal preserved**: Given a known sequence of pushed tokens, repeated next-token calls return the same ordered sequence in the Rust version.
   - **Traceability**: `tokpush`, `nexttoken`

2. **Insertion behavior preserved**: Given insertion at a specific position, subsequent traversal reflects the inserted token at that position and preserves the order of unaffected tokens.
   - **Traceability**: `tokins`, `nexttoken`

3. **Deletion behavior preserved**: Given deletion of a contiguous token range, subsequent traversal never returns tokens from the deleted range.
   - **Traceability**: `tokdel`, `nexttoken`

4. **Mark/restore behavior preserved**: After marking, advancing, and restoring, the next tokens observed match those originally available at the mark position.
   - **Traceability**: `mark`, `restore`, `nexttoken`

5. **Putback behavior preserved**: After consuming one token and calling putback, the next next-token call returns that same token again.
   - **Traceability**: `nexttoken`, `putback`

6. **Reset behavior preserved**: After initialization or clearing, token traversal begins from a defined empty/reset state consistent with the original module’s behavior.
   - **Traceability**: `init_parse`, `clearstack`, `nexttoken`

7. **Cleanup behavior preserved**: After cleanup, the module does not expose previously active token contents through normal token traversal.
   - **Traceability**: `cleanup_stack`, `nexttoken`

8. **Diagnostic mapping preserved**: Token-type diagnostic naming remains available for supported token kinds used by this module.
   - **Traceability**: `token_type_str`

9. **Diagnostic output capability preserved**: The Rust module can emit token-focused debug information and token-context error information corresponding to the original module’s diagnostic roles.
   - **Traceability**: `print_token`, `dbgtok`, `debugtoken`, `file_error`

## Constraints and Porting Notes

- The Rust rewrite must preserve module behavior, not necessarily C-level storage techniques.
- Internal memory-management mechanisms may differ, provided functional behavior of stack operations, navigation, and diagnostics remains equivalent.
- No additional functional surface should be introduced unless required to preserve the evidenced behavior of `src/parser.c`.