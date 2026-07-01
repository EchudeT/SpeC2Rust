# spec.md

## Title

Functional Specification for `module_src_yy_init_18` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_yy_init_18`
- Category: `module_cluster`
- Source file: `src/c.c`
- Rust branch: `081-module_src_yy_init_18-rust-port`
- Generation date: `2026-06-17`

## Overview

This module covers scanner initialization behavior centered on two internal responsibilities:

1. initializing a scanner buffer against an input stream; and
2. initializing scanner-global state to defined defaults.

The Rust rewrite must preserve the observable behavior of these responsibilities as used by the scanner runtime in `src/c.c`. The module does not define high-level parsing behavior by itself; it provides initialization support required before scanning proceeds.

## Feature Specification

### Purpose

The module prepares scanner state so that a lexical buffer can be used safely and predictably by the surrounding scanner runtime.

### In-Scope Functionality

The Rust version must implement the following functional behavior evidenced by the source module:

- Initialize a buffer-state object for use with a specific input file/stream.
- Reset the buffer’s scanning status so that future reads/scans begin from a clean initial condition.
- Mark the buffer as interactive or non-interactive based on the characteristics of the associated input stream.
- Set the buffer’s line and column tracking state to defined initial values during buffer initialization.
- Initialize scanner-global variables to their default startup state.
- Return a success/failure status from global initialization consistent with the source module’s role as an initializer.

### Out of Scope

The Rust port specification for this module does not require any capability beyond what is evidenced here, including:

- defining new public APIs not needed for these initialization responsibilities;
- extending scanner semantics beyond buffer/global initialization;
- adding concurrency behavior, persistence, or recovery features.

## User Scenarios & Testing

### Scenario 1: Scanner startup with no prior state

A scanner runtime starts for the first time and requires its global variables to be placed into a known default state before any buffer is used.

**Expected behavior**
- Global scanner state is initialized successfully.
- Subsequent scanner operations can rely on default values being present rather than uninitialized state.

**Test focus**
- Invoke global initialization from a fresh scanner context.
- Verify success status is returned.
- Verify relevant global state fields used by the scanner are set to defined defaults rather than retaining arbitrary prior values.

### Scenario 2: Creating or reusing a buffer for a file-backed input source

The scanner runtime has a buffer-state object and associates it with an input file/stream before scanning begins.

**Expected behavior**
- The buffer is reset to an initial scanning condition.
- The file/stream association is stored in the buffer state.
- Buffer line/column counters are set to their initialization values.
- Interactive-mode flags reflect the input stream characteristics.

**Test focus**
- Initialize a buffer using a regular file-like stream.
- Verify the buffer references that stream after initialization.
- Verify scanning status is reset and counters are initialized.
- Verify the non-interactive path is selected when the input is not interactive.

### Scenario 3: Buffer initialization for an interactive input source

The scanner runtime initializes a buffer that reads from an interactive source such as a terminal-like stream.

**Expected behavior**
- The buffer is initialized the same way as other inputs, but interactive behavior flags reflect that the stream is interactive.

**Test focus**
- Provide a stream/context that should be classified as interactive.
- Verify the interactive flag/path is enabled in the initialized buffer state.

### Scenario 4: Reinitializing an existing buffer

A previously used buffer is prepared again for scanning, potentially with a new or reused file/stream.

**Expected behavior**
- Any prior scanning progress stored in the buffer does not remain as active scanning state after reinitialization.
- Initial line/column and status values are restored as defined by the module.

**Test focus**
- Mutate buffer state to represent in-progress use.
- Call buffer initialization again.
- Verify initialization values overwrite prior transient scanning state.

## Requirements

### Functional Requirements

#### FR-1 Buffer initialization
The module shall initialize a scanner buffer-state object for association with a provided input file/stream, as evidenced by `yy_init_buffer` in `src/c.c`.

#### FR-2 Buffer reset to startup state
During buffer initialization, the module shall place the buffer into a clean startup scanning state appropriate for subsequent scanner use, as evidenced by `yy_init_buffer` and the `yy_buffer_state` structure in `src/c.c`.

#### FR-3 Stream association retention
During buffer initialization, the module shall record the provided input file/stream as the buffer’s active source, as evidenced by `yy_init_buffer` and `yy_buffer_state`.

#### FR-4 Interactive-source classification
During buffer initialization, the module shall set the buffer’s interactive/non-interactive mode according to the provided input stream’s characteristics, as evidenced by `yy_init_buffer` and `yy_buffer_state`.

#### FR-5 Line and column initialization
During buffer initialization, the module shall initialize buffer line and column tracking fields to defined starting values, as evidenced by `yy_init_buffer` and `yy_buffer_state`.

#### FR-6 Global state initialization
The module shall initialize scanner-global state variables to defined default values before scanner use, as evidenced by `yy_init_globals` in `src/c.c`.

#### FR-7 Initialization status reporting
The module shall report the result of global initialization via an integer-like success/failure status compatible with the source behavior of `yy_init_globals`.

### Key Entities

#### `yy_buffer_state`
Core scanner buffer state used by this module.

**Role**
- Represents the current buffer used by the scanner runtime.
- Holds the input stream association.
- Holds buffer status and mode flags relevant to initialization.
- Holds position-tracking fields such as line and column state.

**Relationship to module behavior**
- `yy_init_buffer` operates on this entity and establishes its initial usable state.

#### Scanner-global state
A set of scanner runtime variables initialized by `yy_init_globals`.

**Role**
- Stores process-local scanner defaults required before scanning begins.

**Relationship to module behavior**
- `yy_init_globals` sets these values so later scanner operations observe defined startup state.

## Success Criteria

### Functional correctness

- A Rust implementation of global initialization returns a status value and sets scanner-global defaults in a way that allows dependent scanner logic to start from defined state, matching the role of `yy_init_globals`.
- A Rust implementation of buffer initialization associates the target buffer with the supplied stream/source and resets the buffer to a valid startup condition, matching the role of `yy_init_buffer`.
- Buffer initialization in Rust sets line and column tracking to the module-defined initial values reflected by the source behavior.
- Buffer initialization in Rust distinguishes interactive from non-interactive input in buffer state, consistent with the source behavior.

### Testability

The port is acceptable when automated tests demonstrate all of the following:

1. Fresh global initialization succeeds and leaves scanner-global state defined.
2. File-backed buffer initialization stores the source and resets startup state.
3. Interactive-input buffer initialization sets interactive mode differently from non-interactive initialization when given correspondingly classified inputs.
4. Reinitializing a previously used buffer restores initialization values for startup-sensitive fields.

### Traceability

Each success criterion must be verifiable against the behavior evidenced by:
- `yy_init_buffer` in `src/c.c`
- `yy_init_globals` in `src/c.c`
- `struct yy_buffer_state` definitions/usages in `src/c.c`