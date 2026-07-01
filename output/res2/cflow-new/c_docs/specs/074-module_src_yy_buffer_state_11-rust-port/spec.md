# spec.md

## Title

Rust Functional Specification for `module_src_yy_buffer_state_11`

## Metadata

- Project: `cflow-new`
- Module: `module_src_yy_buffer_state_11`
- Category: `module_cluster`
- Source file: `src/c.c`
- Rust branch: `074-module_src_yy_buffer_state_11-rust-port`
- Generation date: `2026-06-17`

## Overview

This module manages lexer buffer objects and the internal buffer stack used by the scanner runtime. Its evidenced responsibilities are:

- creating a new scanner buffer associated with a file input source (`yy_create_buffer`)
- ensuring that the internal stack used to hold buffer states exists and can grow (`yyensure_buffer_stack`)
- creating a scanner buffer view over a caller-provided memory region with scanner-required end markers (`yy_scan_buffer`)

The Rust rewrite must preserve these functional boundaries and behaviors. The specification covers only the functionality evidenced by the analyzed functions and referenced buffer-state data structure.

## Feature Specification

### Feature: Scanner buffer creation for file-backed input

The module must support creating a new buffer state for scanning data associated with a file handle.

Behavior evidenced by `yy_create_buffer`:

- Accept a file reference and a requested buffer size.
- Allocate and initialize a new buffer state object.
- Associate the new buffer state with the provided file input source.
- Prepare the buffer state so it is suitable for scanner use after creation.
- Return a buffer-state handle on success.

The Rust version must implement equivalent behavior at the module level, preserving the semantics of creating a usable scanner buffer tied to an input source.

### Feature: Internal buffer stack availability and growth

The module must maintain the scanner’s internal stack of buffer-state handles and ensure that this stack is available before use.

Behavior evidenced by `yyensure_buffer_stack`:

- Create the stack storage if it does not yet exist.
- Preserve existing buffer-state entries if the stack storage grows.
- Increase capacity when current storage is insufficient for buffer-state management.
- Leave the module in a state where buffer stack operations can safely proceed.

The Rust version must provide equivalent internal behavior for stack existence and growth as required by scanner buffer management.

### Feature: Scanner buffer creation over caller-owned memory

The module must support treating an existing character buffer as a scanner input buffer.

Behavior evidenced by `yy_scan_buffer`:

- Accept a caller-provided memory base pointer and size.
- Validate that the memory region satisfies scanner buffer preconditions.
- Create a buffer-state handle that refers to the provided memory for scanning.
- Return a buffer-state handle when the provided memory is acceptable.
- Reject invalid input memory regions by failing buffer creation.

The Rust version must preserve the functional distinction between owned buffer creation for file-backed scanning and non-owning or externally backed scanning over caller-provided memory, as evidenced by this function.

## User Scenarios & Testing

### Scenario 1: Create a scanner buffer for file input

A scanner runtime needs a fresh buffer to read tokens from a file-like source.

Expected support:

- A caller supplies a file/input handle and target size.
- The module returns a valid buffer-state object.
- The resulting buffer can be used by the surrounding scanner runtime as an input buffer.

Suggested tests:

- Create a buffer with a valid input source and positive size; verify success.
- Create multiple buffers for distinct input sources; verify each receives a distinct buffer state.
- Verify the created buffer records the associated input source.

Traceability: `yy_create_buffer`, `yy_buffer_state`.

### Scenario 2: Initialize buffer stack on first use

The scanner has not yet allocated its internal buffer stack, and buffer management is requested.

Expected support:

- Invoking stack-ensure behavior creates the required stack storage.
- After the operation, buffer-state entries can be stored in the stack.

Suggested tests:

- Start from an uninitialized scanner buffer-stack state.
- Invoke the stack-ensure path.
- Verify that stack storage now exists and is ready to hold buffer-state handles.

Traceability: `yyensure_buffer_stack`.

### Scenario 3: Grow buffer stack while preserving existing entries

The scanner already has one or more active buffer states and needs more stack capacity.

Expected support:

- Stack-ensure behavior expands capacity when needed.
- Existing buffer-state handles remain preserved after growth.

Suggested tests:

- Initialize the buffer stack with at least one existing buffer-state entry.
- Force a growth condition.
- Verify that previous entries remain present and in the same order after growth.

Traceability: `yyensure_buffer_stack`, `yy_buffer_state`.

### Scenario 4: Scan directly from caller-provided memory

A caller already has input text in memory and wants the scanner to use that memory directly.

Expected support:

- A caller supplies a memory region and its size.
- If the region meets scanner preconditions, the module returns a valid buffer-state handle referencing it.
- The scanner runtime can treat the returned buffer as an input source.

Suggested tests:

- Provide a valid memory region sized to include scanner-required terminal markers; verify success.
- Verify that the created buffer state refers to the supplied memory region rather than a separately created file-backed buffer.

Traceability: `yy_scan_buffer`, `yy_buffer_state`.

### Scenario 5: Reject invalid memory buffer input

A caller attempts to create a scan buffer from memory that does not satisfy scanner preconditions.

Expected support:

- The module rejects the input and does not create a usable buffer state.

Suggested tests:

- Provide a null or otherwise invalid memory base; verify failure.
- Provide a size too small to satisfy buffer requirements; verify failure.
- Provide a memory region lacking required scanner termination layout; verify failure.

Traceability: `yy_scan_buffer`.

## Requirements

### Functional Requirements

#### FR-1: File-backed buffer creation

The module shall create a new scanner buffer state from an input file/source handle and requested size.

Traceability: `yy_create_buffer` in `src/c.c:2169-2194`.

#### FR-2: Buffer-state initialization for scanner use

The module shall initialize newly created file-backed buffer states so they are usable by the scanner runtime after creation.

Traceability: `yy_create_buffer`; `yy_buffer_state`.

#### FR-3: Buffer stack lazy initialization

The module shall ensure that the internal buffer stack exists before buffer-state stack operations depend on it.

Traceability: `yyensure_buffer_stack` in `src/c.c:2363-2407`.

#### FR-4: Buffer stack capacity growth

The module shall expand internal buffer stack capacity when existing capacity is insufficient.

Traceability: `yyensure_buffer_stack`.

#### FR-5: Preservation of existing stack contents during growth

When the internal buffer stack grows, the module shall preserve already stored buffer-state references.

Traceability: `yyensure_buffer_stack`; `yy_buffer_state`.

#### FR-6: Scan-buffer creation from caller memory

The module shall create a scanner buffer state over caller-provided memory when that memory satisfies required scanner buffer conditions.

Traceability: `yy_scan_buffer` in `src/c.c:2417-2444`.

#### FR-7: Validation of caller-provided memory buffer

The module shall validate the supplied base pointer and size before creating a scan buffer over caller memory.

Traceability: `yy_scan_buffer`.

#### FR-8: Failure on invalid memory-backed scan buffer input

The module shall fail to create a scan buffer when the provided memory region does not satisfy the function’s required preconditions.

Traceability: `yy_scan_buffer`.

### Key Entities

#### `yy_buffer_state`

Core buffer-state entity used by the scanner runtime.

Documented role from evidence:

- represents the state of a scanner input buffer
- may be associated with a file-backed input source
- may instead reference caller-provided memory for scanning
- is the element type managed by the internal buffer stack

Traceability: `struct yy_buffer_state` references in `src/c.c`, especially usages within `yy_create_buffer`, `yyensure_buffer_stack`, and `yy_scan_buffer`.

#### Internal buffer stack

An internal collection of buffer-state handles used by the scanner runtime.

Documented role from evidence:

- stores zero or more `yy_buffer_state` references
- may begin uninitialized
- must be creatable on demand
- must support capacity growth without losing existing entries

Traceability: `yyensure_buffer_stack`.

#### File/input source handle

An external input source associated with buffers created by the file-backed creation path.

Documented role from evidence:

- supplied to buffer creation
- attached to the resulting `yy_buffer_state`

Traceability: `yy_create_buffer`.

#### Caller-provided memory region

A memory area supplied by the caller for direct scanning.

Documented role from evidence:

- consists of a base address and size
- is validated before use
- can back a `yy_buffer_state` when valid

Traceability: `yy_scan_buffer`.

## Success Criteria

### Functional correctness

- Creating a file-backed buffer with valid inputs returns a non-failing buffer-state result and associates the result with the supplied input source.
  Traceability: `yy_create_buffer`, `yy_buffer_state`.

- Ensuring the buffer stack from an uninitialized state results in available stack storage for buffer-state references.
  Traceability: `yyensure_buffer_stack`.

- Ensuring the buffer stack under a growth condition increases available capacity without losing previously stored buffer-state references.
  Traceability: `yyensure_buffer_stack`, `yy_buffer_state`.

- Creating a scan buffer from a valid caller-provided memory region returns a non-failing buffer-state result that represents that memory as scanner input.
  Traceability: `yy_scan_buffer`, `yy_buffer_state`.

- Attempting to create a scan buffer from invalid memory input fails rather than producing a usable buffer state.
  Traceability: `yy_scan_buffer`.

### Behavioral parity

- The Rust module distinguishes between file-backed buffer creation and caller-memory-backed buffer creation, matching the separate evidenced behaviors of `yy_create_buffer` and `yy_scan_buffer`.
  Traceability: `yy_create_buffer`, `yy_scan_buffer`.

- The Rust module retains internal support for buffer-stack initialization and growth as a prerequisite for scanner buffer management.
  Traceability: `yyensure_buffer_stack`.

### Testability

- All scenarios listed in this specification can be exercised through Rust tests covering success and failure paths for buffer creation and stack management.
  Traceability: `yy_create_buffer`, `yyensure_buffer_stack`, `yy_scan_buffer`, `yy_buffer_state`.