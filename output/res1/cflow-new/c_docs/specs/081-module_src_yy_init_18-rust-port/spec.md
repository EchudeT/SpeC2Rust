# spec.md

## Title
Rust Functional Specification for `module_src_yy_init_18`

## Metadata
- Project: `cflow-new`
- Module: `module_src_yy_init_18`
- Category: `module_cluster`
- Source file: `src/c.c`
- Source functions in scope:
  - `yy_init_buffer`
  - `yy_init_globals`
- Generation date: 2026-06-11
- Rust branch target: `081-module_src_yy_init_18-rust-port`

## Overview
This module is responsible for scanner initialization at two levels:

1. **Process or scanner-global initialization** via `yy_init_globals`, which resets scanner state to a defined baseline.
2. **Per-buffer initialization** via `yy_init_buffer`, which prepares a scanner buffer to be used with an input stream.

The Rust rewrite must preserve the observed functional boundaries of these responsibilities: resetting scanner-wide state to its initial condition, and initializing a buffer object so it can participate correctly in subsequent lexical scanning.

## In Scope
- Initialization of scanner global state.
- Initialization and reset of a scanner buffer against a provided file/input handle.
- Establishment of initial buffer state required before scanning proceeds.
- Behavior tied to interactive-input detection when a buffer is associated with an input stream.

## Out of Scope
- Tokenization logic.
- Buffer creation and destruction beyond the initialization behavior evidenced here.
- Input reading loops and refill algorithms beyond the initial state setup.
- New public APIs, concurrency guarantees, serialization, recovery features, or FFI behavior not evidenced by the source functions in scope.

## Feature Specification

### Feature 1: Global scanner state initialization
The module shall provide behavior equivalent to `yy_init_globals`, whose purpose is to initialize or reinitialize scanner-wide state variables to their baseline values.

This initialization must:
- Clear or reset scanner state that tracks active buffers and input/output stream associations.
- Reset scanner text/length tracking state to its empty or initial condition.
- Reset line-oriented and status-oriented scanner bookkeeping to startup values.
- Leave the scanner in a consistent pre-scan state suitable for later buffer selection and token scanning.
- Report success/failure consistently with the source behavior of the initialization function.

The Rust version must preserve the observable effect that calling global initialization returns the scanner to a clean initial state, regardless of prior scanner activity.

### Feature 2: Buffer initialization for an input source
The module shall provide behavior equivalent to `yy_init_buffer`, whose purpose is to initialize a buffer object for use with a given input stream.

This initialization must:
- Associate the buffer with the supplied input source.
- Reset buffer position and status so the next scan begins from an initial state rather than stale prior contents.
- Establish the buffer’s line/column or line-related startup counters to their expected initial values where present in the source state model.
- Reset end-of-buffer and fill-related state so later scanner operations treat the buffer as newly initialized.
- Set the buffer’s interactive/non-interactive status according to the supplied input source when such detection is available in the source behavior.
- Avoid altering unrelated scanner features outside the buffer and global initialization responsibilities evidenced here.

The Rust version must preserve the observable effect that a newly initialized or reinitialized buffer behaves like fresh scanner input state tied to the provided stream.

## User Scenarios & Testing

### Scenario 1: Fresh scanner startup
A caller constructs or obtains scanner state and invokes global initialization before any scanning begins.

Expected result:
- No active scanning residue remains.
- Scanner state reflects startup defaults.
- Subsequent buffer initialization can proceed without depending on prior state.

Test coverage:
- Initialize from an already clean state.
- Initialize after simulated prior scanner activity.
- Verify all tracked initialization-sensitive fields are reset to baseline.

### Scenario 2: Bind a buffer to an input stream before first scan
A caller has a buffer object and an input stream and initializes the buffer before scanning.

Expected result:
- The buffer is associated with that stream.
- The buffer is marked as newly initialized.
- The next scan starts from beginning-of-buffer semantics rather than from preserved prior contents.

Test coverage:
- Initialize a fresh buffer with a valid stream.
- Verify stream association and initial buffer status fields.
- Verify initial counters and flags match startup expectations.

### Scenario 3: Reinitialize an existing buffer for reuse
A caller reuses an existing scanner buffer with the same or a different input stream.

Expected result:
- Any prior buffer progress is discarded for initialization purposes.
- The buffer returns to the same initial state as a freshly prepared buffer.
- The current input stream association reflects the new provided stream.

Test coverage:
- Reinitialize after simulated partial scanning.
- Reinitialize with a changed stream handle.
- Verify stale end-of-buffer, position, and status fields do not persist.

### Scenario 4: Interactive-input classification affects buffer startup state
A caller initializes a buffer against an input source that is classified differently for interactivity.

Expected result:
- The buffer’s interactive flag/state matches the input source classification supported by the source behavior.
- Later scanner behavior can rely on that initialized classification.

Test coverage:
- Initialize using an input source treated as interactive.
- Initialize using an input source treated as non-interactive.
- Verify only the interaction-related flag differs where expected.

### Scenario 5: Global reset after buffer-related activity
A caller performs buffer-related setup or simulated scanning state changes, then invokes global initialization.

Expected result:
- Scanner-global fields return to baseline.
- No stale current-buffer or text-tracking state remains active unless re-established later by normal flow.

Test coverage:
- Set non-default scanner globals, then reset.
- Verify post-reset state matches the same baseline as fresh startup.

## Requirements

### Functional Requirements

#### FR-1: Scanner global reset
The Rust module shall implement scanner-global initialization behavior equivalent to `yy_init_globals` from `src/c.c`, resetting scanner-wide state to startup defaults.

Traceability:
- Function: `yy_init_globals`
- File: `src/c.c`

#### FR-2: Clean pre-scan state
The scanner-global initialization shall leave the scanner in a coherent pre-scan condition with no residual active-buffer/text bookkeeping from earlier activity.

Traceability:
- Function: `yy_init_globals`
- File: `src/c.c`

#### FR-3: Result signaling for global initialization
The Rust implementation shall preserve the success/failure outcome semantics of `yy_init_globals` as observable from its integer return contract.

Traceability:
- Function: `yy_init_globals`
- File: `src/c.c`

#### FR-4: Buffer-to-input association
The Rust module shall implement buffer initialization behavior equivalent to `yy_init_buffer`, including associating a buffer state with the supplied input stream.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`
- File: `src/c.c`

#### FR-5: Buffer state reset on initialization
Buffer initialization shall reset the buffer’s scanning state so it behaves as newly prepared input rather than continuing from prior contents or prior read position.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`
- File: `src/c.c`

#### FR-6: Initial line-related state
Buffer initialization shall establish the initial line-related bookkeeping required by the source buffer state model.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`
- File: `src/c.c`

#### FR-7: Initial buffer status and end-of-buffer state
Buffer initialization shall establish initial buffer status and end-of-buffer-related state needed for subsequent scanner operation.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`
- File: `src/c.c`

#### FR-8: Interactive mode initialization
When the source behavior determines whether the input stream is interactive, buffer initialization shall set the corresponding buffer flag/state consistently.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`
- File: `src/c.c`

#### FR-9: Reinitialization equivalence
Reinitializing an existing buffer with `yy_init_buffer`-equivalent behavior shall produce the same initialization-state invariants as initializing a fresh buffer object of the same kind.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`
- File: `src/c.c`

### Key Entities

#### `yy_buffer_state`
The core buffer entity representing scanner input state. Within this module’s scope, it is the object being initialized for use with a file/input stream. Its relevant relationships are:
- It is associated with an input source during buffer initialization.
- It stores buffer-local scanner state such as status, position/readiness, and line-related bookkeeping.
- It carries the interactive/non-interactive classification used by later scanner behavior.

Traceability:
- Type: `struct yy_buffer_state`
- Functions: `yy_init_buffer`
- File: `src/c.c`

#### Scanner-global state
A set of scanner-wide variables or state holders reset by global initialization. Within this module’s scope, these globals represent:
- Current/active buffer tracking.
- Input/output stream associations.
- Current token text/length tracking.
- Startup bookkeeping such as line/status defaults.

Traceability:
- Function: `yy_init_globals`
- File: `src/c.c`

#### Input stream / file handle
An external input source supplied when initializing a buffer. Within this module’s scope, it determines:
- Which source the buffer is tied to.
- Whether interactive-input classification is applied during initialization.

Traceability:
- Function: `yy_init_buffer`
- File: `src/c.c`

## Success Criteria

### SC-1: Global reset correctness
After invoking the Rust equivalent of `yy_init_globals`, all scanner-global fields touched by the source initialization function match their expected startup values in tests derived from `src/c.c`.

Traceability:
- Function: `yy_init_globals`

### SC-2: Stable reinitialization
Invoking the Rust equivalent of global initialization multiple times yields the same postcondition each time, independent of prior non-default scanner state.

Traceability:
- Function: `yy_init_globals`

### SC-3: Buffer initialization correctness
After invoking the Rust equivalent of `yy_init_buffer`, the target buffer reflects the provided input source and the initialization-sensitive fields match source-equivalent startup values.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`

### SC-4: No stale buffer progress after reinit
Tests that simulate prior scanning state on a buffer and then reinitialize it must show that prior progress/status does not persist in fields reset by source-equivalent initialization.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`

### SC-5: Interactive classification preservation
Where test fixtures provide input sources classified as interactive versus non-interactive, the Rust buffer initialization must set the corresponding buffer state consistently with the source behavior.

Traceability:
- Function: `yy_init_buffer`
- Type: `struct yy_buffer_state`

### SC-6: Scenario support completeness
The Rust port shall support all scenarios listed in this specification without requiring capabilities beyond the functional boundaries evidenced by `yy_init_buffer` and `yy_init_globals`.

Traceability:
- Functions: `yy_init_buffer`, `yy_init_globals`

## Non-Goals
- Defining a full lexer API.
- Altering scanner semantics outside initialization.
- Adding validation, diagnostics, or recovery behavior not evidenced by the source.
- Introducing module capabilities unrelated to scanner-global reset or buffer initialization.