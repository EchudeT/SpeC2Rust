# spec.md

## Title

Rust Functional Specification for `module_src_c.c_22`

## Metadata

- Project: `cflow-new`
- Module: `module_src_c.c_22`
- Category: `module_cluster`
- Source file: `src/c.c`
- Rust branch target: `085-module_src_c.c_22-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides scanner-adjacent input and source-location support for C-language processing. Its responsibilities are limited to:

- opening and closing preprocessor input sources,
- finalizing preprocessor-related state,
- handling scanner end-of-input transitions,
- retrieving the next token from the active scanner input,
- requesting that a named source file become the current scanner source,
- decoding numeric escape sequences with a caller-supplied base and digit limit,
- interpreting backslash-led escaped input,
- updating current source-location state as input is consumed.

The Rust rewrite must preserve this functional boundary and behavior as evidenced by the functions in `src/c.c`:
`pp_finalize`, `pp_open`, `pp_close`, `yywrap`, `get_token`, `source`, `getnum`, `backslash`, and `update_loc`.

## Feature Specification

### Feature 1: Preprocessor Input Lifecycle

The module manages preprocessor-facing file input lifecycle for scanner use.

Supported behavior:

- Open a named input source for preprocessing/scanning.
- Return a usable file handle or equivalent success/failure result to the caller.
- Close an opened preprocessing input source.
- Finalize preprocessor-related state when processing is complete.

The Rust version must preserve the observable contract that:
- source opening is driven by a file name,
- open failure is detectable by the caller,
- close/finalize operations are available as distinct actions.

Traceability:
- `pp_open`
- `pp_close`
- `pp_finalize`

### Feature 2: Scanner End-of-Input Handling

The module participates in scanner end-of-input behavior.

Supported behavior:

- Provide a wrap/end-of-input decision function used when the scanner reaches the end of the current buffer or source.
- Allow the scanner/control flow to determine whether scanning should stop or continue after end-of-input handling.

The Rust version must preserve equivalent end-of-input behavior for the scanner integration layer.

Traceability:
- `yywrap`

### Feature 3: Token Retrieval from Active Input

The module exposes retrieval of the next token from the current scanner/preprocessor input stream.

Supported behavior:

- Read from the currently active input state.
- Return a token code/result suitable for parser or caller consumption.
- Cooperate with source switching and end-of-input handling.

The Rust version must provide equivalent token retrieval semantics for the active source.

Traceability:
- `get_token`

### Feature 4: Source Activation by Name

The module allows a caller to request scanning from a named source.

Supported behavior:

- Accept a source name/path.
- Attempt to activate that source for subsequent tokenization.
- Return success/failure status to the caller.

The Rust version must preserve the ability to switch or start scanning using a named source and report whether activation succeeded.

Traceability:
- `source`

### Feature 5: Escape and Numeric Sequence Interpretation

The module interprets escaped input sequences, including numeric escapes.

Supported behavior:

- Decode a numeric sequence using:
  - a caller-provided numeric base,
  - a caller-provided maximum digit count.
- Interpret backslash-led escapes and return the resulting character/value or scanner-significant result.

The Rust version must preserve:
- bounded numeric consumption based on the requested digit count,
- base-dependent numeric decoding,
- escape interpretation through a dedicated backslash-processing entry point.

Traceability:
- `getnum`
- `backslash`

### Feature 6: Source Location Tracking

The module updates source-location state as input is processed.

Supported behavior:

- Track current location information while scanning advances.
- Update that location state based on consumed input.
- Keep location updates consistent with token retrieval and source transitions.

The Rust version must preserve equivalent location-update behavior observable by scanner/parser consumers.

Traceability:
- `update_loc`

## User Scenarios & Testing

### Scenario 1: Open and scan a named source

A caller provides a source file name, activates it, and begins token retrieval.

Expected support:
- Opening/activation succeeds for a readable source.
- Repeated token retrieval proceeds from that source.
- Location state advances as input is consumed.
- End-of-input handling occurs when the source is exhausted.

Traceability:
- `pp_open`
- `source`
- `get_token`
- `yywrap`
- `update_loc`

Suggested tests:
- Activate a valid input file and verify token retrieval produces results until exhaustion.
- Verify location state changes after consuming input.
- Verify end-of-input path is reached cleanly.

### Scenario 2: Detect source-open failure

A caller requests a missing or unreadable source.

Expected support:
- Source opening/activation reports failure.
- No false success is returned.

Traceability:
- `pp_open`
- `source`

Suggested tests:
- Attempt to open a nonexistent file and verify failure is reported.
- Attempt to activate an unreadable source and verify failure is reported.

### Scenario 3: Handle scanner end-of-input

The scanner reaches the end of the active input.

Expected support:
- End-of-input handling returns the same stop/continue decision semantics as the C module.
- The caller/scanner can terminate or proceed accordingly.

Traceability:
- `yywrap`

Suggested tests:
- Exhaust a source and verify the wrap handler returns the expected result.
- If multiple source transitions are part of current behavior, verify the same transition outcome; otherwise verify termination.

### Scenario 4: Decode escaped characters

The scanner encounters backslash-led escaped text.

Expected support:
- Escape handling produces the correct decoded result for supported escapes.
- Numeric escapes respect base and digit-count limits.

Traceability:
- `backslash`
- `getnum`

Suggested tests:
- Verify common escapes accepted by the module decode correctly.
- Verify octal/hex-style numeric decoding consumes no more than the permitted number of digits.
- Verify decoding differs appropriately when called with different bases.

### Scenario 5: Finalize after scanning

After scanning work is complete, the caller closes open inputs and finalizes preprocessor state.

Expected support:
- Opened inputs can be closed.
- Finalization completes without requiring further token input.
- Cleanup actions remain distinct from token retrieval.

Traceability:
- `pp_close`
- `pp_finalize`

Suggested tests:
- Open and close a source successfully.
- Finalize after scanning completion and verify no further active processing is required.

## Requirements

### Functional Requirements

#### FR-1: Open preprocessing input by name
The module shall accept a source name and attempt to open it as preprocessing/scanner input, with caller-visible success or failure.

Traceability:
- `pp_open`

#### FR-2: Close preprocessing input
The module shall provide a way to close an opened preprocessing/scanner input.

Traceability:
- `pp_close`

#### FR-3: Finalize preprocessing-related state
The module shall provide a finalization operation for preprocessing/scanner-related state after processing is complete.

Traceability:
- `pp_finalize`

#### FR-4: Provide scanner end-of-input handling
The module shall provide wrap/end-of-input behavior callable by the scanner when current input is exhausted.

Traceability:
- `yywrap`

#### FR-5: Return the next token from active input
The module shall provide token retrieval from the currently active scanner source.

Traceability:
- `get_token`

#### FR-6: Activate a named source for scanning
The module shall accept a source name/path and attempt to make it the active source for subsequent scanning, returning success or failure.

Traceability:
- `source`

#### FR-7: Decode numeric escape content with bounded consumption
The module shall decode numeric input using a specified base and a maximum number of digits to consume.

Traceability:
- `getnum`

#### FR-8: Interpret backslash-led escapes
The module shall process escape sequences beginning with backslash and return the decoded result needed by scanner logic.

Traceability:
- `backslash`

#### FR-9: Update source-location state during input processing
The module shall update current source-location information as scanning progresses.

Traceability:
- `update_loc`

### Key Entities

#### Entity 1: Scanner Buffer State
The module interacts with scanner buffer state represented in C as `struct yy_buffer_state`.

Role:
- Represents the current scanner input buffer context.
- Supports token retrieval and end-of-input behavior.
- Participates in source activation and wrap handling.

Relationships:
- Used indirectly by token retrieval and end-of-input flow.
- Tied to active source/input lifecycle.

Traceability:
- `struct yy_buffer_state`
- `get_token`
- `yywrap`
- `source`

#### Entity 2: Preprocessor/Scanner Input Handle
The module works with an open input stream handle represented in C through `FILE *`.

Role:
- Represents an opened source for preprocessing/scanning.
- Is created by open operations and consumed by close operations.

Relationships:
- Produced by source opening.
- Released by input closing.

Traceability:
- `pp_open`
- `pp_close`

#### Entity 3: Source Location State
The module maintains current source position information updated as input is consumed.

Role:
- Tracks where scanning currently occurs in the active source.
- Supports accurate progression across consumed text.

Relationships:
- Advanced by `update_loc`.
- Must remain consistent with token retrieval and source activation.

Traceability:
- `update_loc`
- `get_token`
- `source`

#### Entity 4: Escape-Decoding State
The module contains transient state/logic for interpreting escaped characters and numeric escape fragments.

Role:
- Converts escaped textual sequences into decoded values.
- Enforces numeric decoding rules based on base and digit limit.

Relationships:
- `backslash` uses numeric decoding support from `getnum`.
- Feeds decoded values back into scanner behavior.

Traceability:
- `getnum`
- `backslash`

## Success Criteria

### SC-1: Source open/activation parity
For valid readable inputs and invalid/missing inputs, the Rust module shall return success/failure outcomes matching the C module’s observable behavior for source opening and activation.

Traceability:
- `pp_open`
- `source`

### SC-2: Token retrieval parity
Given the same active input, the Rust module shall produce equivalent token-return behavior to the C module through the token retrieval entry point.

Traceability:
- `get_token`

### SC-3: End-of-input parity
When the active input is exhausted, the Rust module shall return the same end-of-input/wrap decision as the C module.

Traceability:
- `yywrap`

### SC-4: Escape decoding parity
For supported backslash escapes and numeric escapes, the Rust module shall return decoded results matching the C module, including honoring numeric base and digit-count limits.

Traceability:
- `getnum`
- `backslash`

### SC-5: Location update parity
As identical input is consumed, the Rust module shall update source-location state consistently with the C module’s observable location progression.

Traceability:
- `update_loc`

### SC-6: Input close/finalize availability
The Rust module shall support explicit input close and finalization operations corresponding to the C module’s lifecycle boundaries.

Traceability:
- `pp_close`
- `pp_finalize`

## Out of Scope

The Rust rewrite specification does not require any capability not evidenced by this module analysis, including:
- new public APIs beyond those needed to preserve current module behavior,
- thread-safety guarantees,
- serialization or persistence,
- recovery or fallback behaviors not present in the C module,
- performance or benchmark targets,
- cross-language FFI surface design.