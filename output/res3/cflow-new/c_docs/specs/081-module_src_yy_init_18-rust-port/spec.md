# spec.md

## Title

Rust Functional Specification for `module_src_yy_init_18`

## Document Metadata

- Project: `cflow-new`
- Module: `module_src_yy_init_18`
- Category: `module_cluster`
- Source basis: `src/c.c`
- Rust branch target: `081-module_src_yy_init_18-rust-port`
- Generation date: `2026-06-17`

## 1. Feature Specification

### 1.1 Purpose

This module is responsible for initializing scanner runtime state in two areas:

- preparing an individual lexical input buffer for use with a specific input stream; and
- resetting scanner-global state to a known initial condition.

The Rust rewrite must preserve the observable behavior of these initialization steps as evidenced by:

- `yy_init_buffer`
- `yy_init_globals`
- `struct yy_buffer_state`

### 1.2 In-Scope Functionality

The Rust version must implement the following functional boundaries:

1. **Buffer initialization**
   - Accept a buffer-state object and an associated input stream reference.
   - Initialize that buffer into a valid starting state for subsequent scanner use.
   - Associate the buffer with the provided stream.
   - Reset buffer position and status fields needed for fresh scanning.

2. **Global scanner-state initialization**
   - Reset scanner-global variables and pointers to their startup values.
   - Clear active-buffer and scanner bookkeeping state so later scanner operations begin from a clean baseline.
   - Return a status result indicating initialization completion.

### 1.3 Out of Scope

The Rust version is not required by this module spec to add or expose:

- new scanning features;
- new public APIs beyond what is needed to preserve this module’s role;
- thread-safety guarantees;
- persistence, serialization, or checkpointing;
- recovery behavior beyond the initialization/reset behavior evidenced here.

## 2. User Scenarios & Testing

### 2.1 Scenario: Initialize a newly created scanner buffer

A scanner runtime has allocated or otherwise obtained a buffer-state object and wants to prepare it for reading from a file-like input source.

**Expected behavior**
- The module accepts the buffer and input stream association.
- The buffer becomes usable as a fresh scanner buffer.
- Existing per-buffer state that would interfere with fresh scanning is reset.

**Test focus**
- Verify the buffer references the intended input stream after initialization.
- Verify the buffer is in an initial state rather than a partially used state.
- Verify initialization succeeds for a valid buffer object.

### 2.2 Scenario: Reinitialize a buffer before reusing it

A previously used buffer is reassigned or reset for a new scan start.

**Expected behavior**
- Reinitialization clears prior scanning progress stored in the buffer.
- The buffer is returned to the same logical initial condition expected for fresh use with its assigned stream.

**Test focus**
- Start from a deliberately modified buffer state.
- Invoke buffer initialization again.
- Confirm reusable fields are reset to initial values required for scanner startup.

### 2.3 Scenario: Reset scanner-global state during startup

Before scanning begins, the scanner runtime must clear global state left from prior use or static defaults.

**Expected behavior**
- Global scanner state is reset to a known baseline.
- Pointers or references to current buffer/runtime bookkeeping are cleared or restored to startup values.
- A success status is returned.

**Test focus**
- Seed global state with non-default values.
- Run global initialization.
- Verify all scanner-global fields covered by this module are restored to their baseline values.
- Verify the function returns the expected success indicator.

### 2.4 Scenario: Reset scanner-global state after teardown or before another scan session

A scanner instance lifecycle requires reestablishing initial globals after earlier activity.

**Expected behavior**
- The same reset behavior used at startup can be applied again.
- No stale active-buffer or scanner bookkeeping state remains afterward.

**Test focus**
- Simulate a prior scan session by mutating global scanner state.
- Invoke global initialization more than once.
- Confirm repeated calls restore the same baseline state each time.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1 Buffer-to-stream association
The module shall support initializing a scanner buffer with a specified input stream association, as evidenced by `yy_init_buffer(YY_BUFFER_STATE b, FILE * file)` in `src/c.c`.

#### FR-2 Fresh buffer-state reset
The module shall reset the target buffer to a fresh scanner-start condition during buffer initialization, as evidenced by `yy_init_buffer` and the use of `struct yy_buffer_state` in `src/c.c`.

#### FR-3 Reinitialization behavior
The module shall allow the same buffer object to be initialized again, with prior per-buffer scanning progress overwritten by startup state, as evidenced by the reset role of `yy_init_buffer`.

#### FR-4 Global scanner baseline reset
The module shall provide scanner-global initialization that restores scanner-global state and bookkeeping variables to startup values, as evidenced by `yy_init_globals(void)` in `src/c.c`.

#### FR-5 Active-buffer/global pointer clearing
The module shall reset active-buffer-related global scanner references as part of global initialization, where such references are part of scanner-global state managed by `yy_init_globals`.

#### FR-6 Initialization status result
The module shall produce a success/failure status result for global initialization consistent with the source function contract returning `int`, as evidenced by `yy_init_globals(void)`.

### 3.2 Key Entities

#### `yy_buffer_state`
Primary per-buffer scanner state entity.

**Role**
- Represents the state of one scanner input buffer.
- Holds the buffer’s association with an input stream and the mutable state needed to begin or resume scanning.

**Relevant relationship**
- `yy_init_buffer` initializes and resets this entity for use with a file input source.

#### Scanner-global runtime state
Module-managed scanner-wide state reset by global initialization.

**Role**
- Tracks scanner-wide bookkeeping such as current/active buffer references and other initialization-sensitive globals.

**Relevant relationship**
- `yy_init_globals` restores this state to its startup baseline.
- Buffer initialization and global initialization operate at different scopes: per-buffer versus scanner-wide.

## 4. Success Criteria

### 4.1 Behavioral Equivalence Criteria

1. **Buffer initialization correctness**
   - Given a valid buffer-state object and input stream, the Rust implementation places the buffer into a fresh initialized state suitable for scanner startup.
   - Traceability: `yy_init_buffer`, `yy_buffer_state`.

2. **Buffer reuse correctness**
   - Reinitializing an already used buffer resets its startup-relevant state and does not preserve stale scan-progress fields that would affect a new scan start.

3. **Global reset correctness**
   - After mutating scanner-global state away from defaults, calling the Rust equivalent of global initialization restores the baseline values managed by this module.
   - Traceability: `yy_init_globals`.

4. **Repeatable global initialization**
   - Repeated invocations of global initialization produce the same resulting baseline state.

5. **Status result preservation**
   - The Rust implementation exposes an initialization outcome equivalent to the source module’s global initialization status behavior.

### 4.2 Test Completion Criteria

The Rust port is considered complete for this module when:

- all scenarios in Section 2 are covered by automated tests;
- each functional requirement in Section 3.1 has at least one direct test;
- tests demonstrate correct per-buffer initialization and scanner-global reset behavior using Rust representations of the source entities;
- no tested behavior requires capabilities beyond those evidenced by `src/c.c`.