# spec.md

## Title

Rust Functional Specification for `module_src_c.c_22`

## Metadata

- Project: `cflow-new`
- Module: `module_src_c.c_22`
- Category: `module_cluster`
- Source file: `src/c.c`
- Rust branch target: `085-module_src_c.c_22-rust-port`
- Generation date: 2026-06-11

## Overview

This module provides the input-side behavior that connects preprocessor-managed source reading with lexical token retrieval and source-location tracking.

The Rust rewrite must preserve the observable behavior of this module as evidenced by the analyzed functions in `src/c.c`:

- opening and closing preprocessed input streams
- finalizing preprocessing state
- advancing to the next input source when the current one is exhausted
- retrieving the next token from the active scanner
- parsing numeric escape fragments for character handling
- interpreting backslash-led input sequences
- updating source location state as input is consumed

The specification is limited to these behaviors and to the data relationships implied by the scanner/preprocessor state in this module.

## Feature Specification

### Feature: Preprocessor-backed input lifecycle

The module manages the lifecycle of input streams used by the scanner.

It must support:

- opening an input stream for a named source through preprocessing-aware logic
- closing an input stream previously opened by the module
- final cleanup of preprocessing-related state after scanning is complete

This behavior is evidenced by `pp_open`, `pp_close`, and `pp_finalize`.

### Feature: Scanner end-of-input transition

The module handles the transition that occurs when the scanner reaches the end of the current input source.

It must support:

- scanner wrap-up logic that decides whether scanning should terminate or continue with another source
- cooperation with source-switching behavior when additional input sources are available

This behavior is evidenced by `yywrap` and `source`.

### Feature: Token retrieval from the active input source

The module provides token acquisition from the active scanner input.

It must support:

- requesting the next token from the scanner
- returning the scanner result in the same role as the C implementation’s token getter

This behavior is evidenced by `get_token`.

### Feature: Source activation and switching

The module activates a named source for scanning.

It must support:

- accepting a source name
- attempting to make that source the active scanner input
- reporting success or failure in the same functional role as the C implementation

This behavior is evidenced by `source`.

### Feature: Escape-sequence numeric parsing

The module parses numeric fragments used by escape-sequence handling.

It must support:

- reading numeric input using a caller-provided base
- respecting a caller-provided count limit
- returning the parsed numeric result in the same functional role as the C implementation

This behavior is evidenced by `getnum`.

### Feature: Backslash sequence handling

The module interprets input beginning with a backslash in the context of scanning.

It must support:

- consuming and interpreting backslash-led input according to the current scanner state
- returning the same category of result as the C implementation for downstream scanner use

This behavior is evidenced by `backslash`, with numeric support from `getnum`.

### Feature: Source-location maintenance

The module updates current location state as input is processed.

It must support:

- advancing location metadata based on consumed input
- keeping scanner-visible location information consistent with the active source position

This behavior is evidenced by `update_loc`.

## User Scenarios & Testing

### Scenario 1: Open, scan, and close one source

A caller activates a single named source, retrieves tokens until the scanner reports end of input, and then closes and finalizes preprocessing state.

The Rust version must support this flow:

1. A source name is provided.
2. The module opens the corresponding preprocessed input.
3. Tokens are retrieved repeatedly from the active scanner.
4. End-of-input logic completes without leaving the scanner in an invalid state.
5. The input is closed.
6. Finalization completes without requiring additional cleanup steps outside the module’s defined lifecycle functions.

Relevant evidence: `pp_open`, `get_token`, `yywrap`, `pp_close`, `pp_finalize`, `source`.

### Scenario 2: End of one source triggers source transition

A caller is scanning input and the current source is exhausted. The module must perform wrap logic and, when another source is available through the module’s source-selection behavior, continue scanning from that next source rather than terminating prematurely.

The Rust version must support:

1. detection of end of the active source
2. invocation of wrap behavior
3. continued scanning when a next source can be activated
4. termination only when no further source can be supplied

Relevant evidence: `yywrap`, `source`.

### Scenario 3: Backslash-led text is interpreted correctly

While scanning, the input contains a backslash-led sequence. The module must interpret that sequence using its backslash handling and any required numeric parsing.

The Rust version must support test cases covering at least:

- a backslash sequence that does not require numeric parsing
- a backslash sequence that uses numeric parsing with a specified base and digit limit
- result propagation back to scanner logic

Relevant evidence: `backslash`, `getnum`.

### Scenario 4: Location state advances with consumed input

As tokens and characters are consumed from the active source, the module updates source-location state so that subsequent scanner-visible position information reflects the consumed input.

The Rust version must support tests in which:

- input consumption changes the current location
- repeated updates remain consistent across multiple consumed segments
- location updates remain tied to the active source stream

Relevant evidence: `update_loc`.

### Scenario 5: Open failure or source activation failure is reported

A caller attempts to activate a named source that cannot be opened or cannot become the active scanner input.

The Rust version must support:

- a failed source-open or source-activation outcome
- a returned failure result in the same functional role as the C implementation
- no false success reporting for unavailable input

Relevant evidence: `pp_open`, `source`.

## Requirements

### Functional Requirements

#### FR-1: Preprocessed input opening
The module shall provide behavior equivalent to opening a named input source for scanner consumption through preprocessing-aware logic.

Traceability: `pp_open` in `src/c.c`.

#### FR-2: Input closing
The module shall provide behavior equivalent to closing an opened input stream associated with this module.

Traceability: `pp_close` in `src/c.c`.

#### FR-3: Preprocessing finalization
The module shall provide a finalization step that completes cleanup of preprocessing-related module state after scanning use is finished.

Traceability: `pp_finalize` in `src/c.c`.

#### FR-4: Token retrieval
The module shall provide a token retrieval operation that obtains the next token from the active scanner input source.

Traceability: `get_token` in `src/c.c`.

#### FR-5: Source activation
The module shall provide a source-selection operation that accepts a source name and attempts to make that source active for scanning, reporting success or failure.

Traceability: `source` in `src/c.c`.

#### FR-6: End-of-input wrap behavior
The module shall provide scanner wrap behavior that is invoked at end of the current input and determines whether scanning continues with another source or terminates.

Traceability: `yywrap`, `source` in `src/c.c`.

#### FR-7: Numeric fragment parsing for escapes
The module shall provide numeric parsing behavior that accepts a base and count limit and returns the parsed value for use in escape handling.

Traceability: `getnum` in `src/c.c`.

#### FR-8: Backslash sequence interpretation
The module shall provide interpretation of backslash-led input sequences for scanner use, including use of numeric escape parsing when required.

Traceability: `backslash`, `getnum` in `src/c.c`.

#### FR-9: Source-location updates
The module shall provide behavior that updates current source-location state in response to consumed input.

Traceability: `update_loc` in `src/c.c`.

### Key Entities

#### Active input stream
The module works with an opened input stream representing the current source being scanned.

Relationship to requirements:
- opened by preprocessor-aware logic
- consumed by token retrieval
- closed during input lifecycle completion

Traceability: `pp_open`, `pp_close`, `source`.

#### Scanner buffer state
The module depends on scanner buffer state represented by `struct yy_buffer_state`, which holds the scanner’s current buffered-input context.

Relationship to requirements:
- source activation and wrap behavior operate against scanner input state
- token retrieval reads from the active scanner context
- location updates are tied to consumed scanner input

Traceability: `struct yy_buffer_state` entries in `src/c.c`; `yywrap`, `get_token`, `source`, `update_loc`.

#### Preprocessor/scanner accumulation state
The module references `struct obstack`, indicating participation in accumulated lexical or preprocessing state used by the scanner environment.

Relationship to requirements:
- part of the surrounding state finalized by preprocessing cleanup
- part of the scanner/preprocessor context in which tokens and source text are managed

Traceability: `struct obstack` entries in `src/c.c`; `pp_finalize`.

#### Source location state
The module maintains mutable location information for the active input source.

Relationship to requirements:
- advanced as input is consumed
- must remain consistent with the current scanner source

Traceability: `update_loc`.

#### Escape parsing state
The module uses transient parsing state for backslash-led and numeric escape handling.

Relationship to requirements:
- `backslash` interprets the escape sequence
- `getnum` parses numeric content under caller-supplied constraints

Traceability: `backslash`, `getnum`.

## Success Criteria

### Behavioral parity criteria

1. The Rust module can open a named source for scanning and report failure when opening cannot be performed.
   - Traceability: `pp_open`, `source`

2. The Rust module can close previously opened input handled by the module without requiring caller-visible extra cleanup beyond the defined lifecycle.
   - Traceability: `pp_close`

3. The Rust module provides a finalization step that completes preprocessing-related cleanup after scanning is finished.
   - Traceability: `pp_finalize`

4. The Rust module returns scanner tokens through a token retrieval operation corresponding to the C module’s role.
   - Traceability: `get_token`

5. When the current input source is exhausted, the Rust module performs wrap behavior that either continues with another source or terminates scanning when no further source is available.
   - Traceability: `yywrap`, `source`

6. The Rust module correctly handles backslash-led input sequences, including cases that require numeric parsing with a specified base and count limit.
   - Traceability: `backslash`, `getnum`

7. The Rust module updates source-location state as input is consumed, and tests can observe location advancement across multiple updates.
   - Traceability: `update_loc`

### Testability criteria

1. Automated tests cover successful source activation, unsuccessful source activation, token retrieval, end-of-input wrap handling, backslash handling, numeric escape parsing, and location updates.
   - Traceability: `source`, `get_token`, `yywrap`, `backslash`, `getnum`, `update_loc`

2. For representative inputs used by this module’s scope, the Rust rewrite produces the same control-flow outcomes as the C module for:
   - source open success/failure
   - continuation vs termination at wrap
   - parsed numeric escape results
   - location advancement after consumption
   - Traceability: `pp_open`, `source`, `yywrap`, `getnum`, `update_loc`

3. The module lifecycle sequence of open/use/close/finalize completes without leaving active scanner input unmanaged.
   - Traceability: `pp_open`, `get_token`, `pp_close`, `pp_finalize`

## Out of Scope

The Rust rewrite specification does not require any capability not evidenced by this module slice, including:

- new public APIs beyond behavior needed to preserve the existing module role
- thread-safety guarantees
- serialization or persistence
- recovery workflows beyond the observed success/failure behavior
- performance targets or benchmark parity
- features outside the scanner/preprocessor input, escape handling, and location-tracking responsibilities evidenced here