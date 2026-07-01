# spec.md

## Overview

This specification defines the required behavior for the Rust rewrite of `module_src_c.c_22` from project `cflow-new`, targeting branch `085-module_src_c.c_22-rust-port`.

The analyzed module is responsible for a narrow set of preprocessing and scanner-support behaviors centered on:

- opening and closing preprocessor input streams,
- finalizing preprocessing state,
- advancing to the next scanner input source,
- retrieving scanner tokens,
- decoding numeric escape content,
- handling backslash-driven line continuation or escape processing,
- updating source location state.

The Rust version must preserve the observable behavior of these responsibilities as evidenced by the following functions in `src/c.c`:

- `pp_finalize`
- `pp_open`
- `pp_close`
- `yywrap`
- `get_token`
- `source`
- `getnum`
- `backslash`
- `update_loc`

## Feature Specification

### Summary

The module provides scanner-side support for consuming source text, especially when input may pass through a preprocessing stage. It manages source stream lifecycle, token acquisition, source switching, character escape handling, and source location updates needed by the rest of the parser/scanner pipeline.

### In-Scope Behavior

The Rust rewrite must implement the following module behaviors:

1. **Preprocessor stream lifecycle**
   - Open an input stream for preprocessing or scanner consumption by source name.
   - Close a previously opened stream.
   - Finalize preprocessing-related state when processing is complete.

2. **Scanner input progression**
   - Supply the next token to scanner consumers.
   - Signal scanner end-of-input behavior through wrapper logic that advances or terminates when the current source is exhausted.

3. **Source switching**
   - Accept a source name and make it the active input source for scanning/preprocessing.
   - Report success or failure as an integer result compatible with the original module’s source-loading behavior.

4. **Escape and numeric decoding support**
   - Decode numeric sequences using a specified base and bounded digit count.
   - Process backslash-related input behavior used by the scanner.

5. **Location tracking**
   - Update source location state as input is consumed so downstream diagnostics and parser state remain aligned with the current position in the source.

### Out of Scope

The Rust rewrite must not claim or add behaviors not evidenced by this module analysis, including:

- new public interfaces beyond those needed to preserve the above behaviors,
- thread-safety guarantees,
- persistence or serialization,
- error recovery systems beyond original observable return/fail behavior,
- performance or benchmarking guarantees,
- preprocessing features not directly implied by these functions.

## User Scenarios & Testing

### Scenario 1: Open and scan a source file

A caller provides a source file name to begin scanning. The module opens the source, makes it active, and returns tokens through repeated token retrieval until input is exhausted.

**Expected support in Rust**
- Opening a named source succeeds when the source is available.
- Token retrieval continues until end-of-input is reached.
- End-of-input handling follows the module’s wrapper behavior rather than requiring the caller to manually reset scanner state.

**Test focus**
- Valid source path opens successfully.
- Tokens can be retrieved after opening.
- End-of-input is handled consistently through the scanner wrapper path.

### Scenario 2: Fail to open a source

A caller requests scanning of a source that cannot be opened.

**Expected support in Rust**
- Source-open or source-activation reports failure.
- No invalid active stream is left behind.

**Test focus**
- Invalid path returns failure from source-opening behavior.
- Closing/finalization after failed open does not corrupt state.

### Scenario 3: Switch to another source at scanner boundary

While scanner input is being consumed, the current source ends and the scanner asks whether another source should be activated.

**Expected support in Rust**
- Wrapper logic can determine whether scanning should continue with another source or terminate cleanly.
- Source switching uses the module’s source activation behavior.

**Test focus**
- Exhaust current source and invoke end-of-input handling.
- Verify wrapper result matches whether a new source is available.

### Scenario 4: Decode numeric escaped input

The scanner encounters a numeric sequence whose interpretation depends on base and maximum digit count.

**Expected support in Rust**
- Numeric conversion respects the provided base.
- Conversion stops at the allowed count boundary.

**Test focus**
- Base-sensitive decoding produces expected integer values.
- Input longer than the digit limit is bounded by the count parameter.

### Scenario 5: Handle backslash-driven scanner behavior

The scanner encounters backslash input that must be interpreted according to this module’s escape or continuation rules.

**Expected support in Rust**
- Backslash processing returns the same kind of scanner decision/result as the C module.
- Source position remains consistent after processing.

**Test focus**
- Invoke backslash handling on representative scanner input.
- Verify resulting scanner state and location updates remain correct.

### Scenario 6: Maintain correct source locations

As input is consumed, the module updates location state so that subsequent parser or diagnostic consumers see the correct source position.

**Expected support in Rust**
- Location updates track consumed input consistently.
- Line-sensitive input changes are reflected in location state.

**Test focus**
- Consume input containing line changes.
- Verify location state changes after update operations.

## Requirements

### Functional Requirements

#### FR-1: Preprocessor finalization
The module shall provide a finalization operation that releases or ends preprocessing-related activity after scanning/preprocessing completes.

**Traceability:** `pp_finalize` in `src/c.c`

#### FR-2: Named source opening
The module shall provide an operation that opens a source by name and returns a stream/result representing whether opening succeeded.

**Traceability:** `pp_open` in `src/c.c`

#### FR-3: Source stream closing
The module shall provide an operation that closes a previously opened source stream.

**Traceability:** `pp_close` in `src/c.c`

#### FR-4: End-of-input wrapper behavior
The module shall provide scanner wrapper behavior that is invoked at end of current input and returns an integer status indicating whether scanning should continue or terminate.

**Traceability:** `yywrap` in `src/c.c`

#### FR-5: Token retrieval
The module shall provide token retrieval for scanner consumers and return token codes as integer results.

**Traceability:** `get_token` in `src/c.c`

#### FR-6: Source activation by name
The module shall provide a source-loading/activation operation that accepts a source name and returns integer success/failure status.

**Traceability:** `source` in `src/c.c`

#### FR-7: Numeric sequence decoding
The module shall decode a numeric sequence using:
- a caller-provided base, and
- a caller-provided maximum digit count.

The operation shall return the resulting integer value.

**Traceability:** `getnum` in `src/c.c`

#### FR-8: Backslash handling
The module shall provide scanner support for backslash-driven input handling and return an integer result usable by scanner logic.

**Traceability:** `backslash` in `src/c.c`

#### FR-9: Location updates
The module shall provide an operation that updates current source location state in response to consumed input.

**Traceability:** `update_loc` in `src/c.c`

#### FR-10: Consistent interaction among source, token, and location operations
The module’s source opening/activation, token retrieval, backslash handling, and location update behaviors shall operate coherently on the active scanner input so that scanning can proceed across a source from open/activation through exhaustion.

**Traceability:** `pp_open`, `yywrap`, `get_token`, `source`, `backslash`, `update_loc` in `src/c.c`

### Key Entities

#### 1. Active source/input stream
A named source stream is the current input being scanned or preprocessed. It is created/opened by source-opening behavior, may be activated by source-selection behavior, and is later closed or exhausted.

**Traceability:** `pp_open`, `pp_close`, `source`

#### 2. Scanner token result
Token retrieval produces integer token codes for downstream scanner/parser consumption.

**Traceability:** `get_token`

#### 3. Scanner end-of-input status
The wrapper result indicates whether scanning terminates or continues with another input source.

**Traceability:** `yywrap`

#### 4. Numeric decode state
Numeric decoding uses a base and a digit-count limit to produce an integer value from scanner input.

**Traceability:** `getnum`

#### 5. Source location state
Location state records the current position in the active source and is updated as input is consumed.

**Traceability:** `update_loc`

#### 6. Scanner/preprocessor buffer state
The module depends on scanner buffer state represented by `struct yy_buffer_state`, which ties active input data to scanner progression and source transitions.

**Traceability:** `struct yy_buffer_state` entries in `src/c.c`

#### 7. Auxiliary dynamic storage
The module references `struct obstack`, indicating auxiliary dynamic storage used by surrounding scanner/preprocessor logic that the rewrite must preserve behaviorally where this module depends on it.

**Traceability:** `struct obstack` entries in `src/c.c`

## Success Criteria

1. **Source open/close parity**
   - The Rust module can open a valid named source and close it through the module’s corresponding lifecycle operations.
   - Invalid source opening is reported as failure.
   - **Traceability:** `pp_open`, `pp_close`, `source`

2. **Token retrieval parity**
   - After successful source activation, the Rust module returns token results through repeated token retrieval until end-of-input behavior is reached.
   - **Traceability:** `get_token`, `yywrap`

3. **End-of-input transition parity**
   - When current input is exhausted, the Rust module produces wrapper status that correctly distinguishes continuation from termination according to available next-source behavior.
   - **Traceability:** `yywrap`, `source`

4. **Numeric decoding parity**
   - For representative inputs, numeric decoding in the Rust module matches C-module results for the same base and digit-count limit.
   - **Traceability:** `getnum`

5. **Backslash handling parity**
   - For representative scanner inputs involving backslash processing, the Rust module returns the same class of scanner result and preserves scanning continuity.
   - **Traceability:** `backslash`

6. **Location tracking parity**
   - Source location state is updated consistently as input is consumed, including across line-sensitive input changes.
   - **Traceability:** `update_loc`

7. **Finalization parity**
   - Finalization can be invoked after scanning/preprocessing activity without leaving the module in an unusable or inconsistent state for that processing session.
   - **Traceability:** `pp_finalize`

8. **Integrated scanning workflow**
   - The Rust rewrite supports the end-to-end workflow of opening or activating a source, retrieving tokens, handling scanner escapes/continuations, updating locations, and terminating or switching source at end-of-input.
   - **Traceability:** `pp_open`, `get_token`, `backslash`, `update_loc`, `yywrap`, `source`, `pp_finalize`