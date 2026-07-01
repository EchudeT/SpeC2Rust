# spec.md

## Overview

- **Project**: `cat`
- **Module**: `main_root_binary-io.c_17`
- **Category**: `main_cluster`
- **Source file**: `binary-io.c`
- **Primary entry point**: `set_binary_mode(int fd, int mode) -> int`
- **Rust port branch**: `018-main_root_binary_io.c_17-rust-port`
- **Generation date**: `2026-06-07`

## Feature Specification

### Purpose

This module provides a single focused capability: adjusting the binary/text I/O mode associated with an already-open file descriptor and reporting the result as an integer status.

The Rust rewrite must preserve that role and boundary:

- accept a file descriptor and a requested mode value,
- attempt to apply that mode to the descriptor,
- return an integer result reflecting success or failure in the same functional sense as the C module.

### Functional Scope

The Rust version must implement the module behavior evidenced by the source analysis:

- support changing the mode of an existing file descriptor,
- operate on caller-supplied descriptors rather than opening or closing files,
- expose the outcome through an integer return value,
- remain limited to binary-mode setting behavior and not take on unrelated I/O responsibilities.

### Out of Scope

The Rust version must not introduce unevidenced functionality, including:

- file opening, closing, reading, or writing,
- buffering policies,
- stream abstraction beyond the provided file descriptor input,
- additional public configuration APIs,
- persistence, serialization, or recovery behaviors.

## User Scenarios & Testing

### Scenario 1: Enable binary mode for an output descriptor

A caller has an already-open output file descriptor and needs to ensure output is treated in binary mode before further program I/O occurs.

**Expected behavior**:
- the module accepts the descriptor and requested mode,
- it attempts to apply the requested mode,
- it returns an integer status that the caller can inspect.

**Test focus**:
- call the Rust function with a valid descriptor and a binary-mode request,
- verify that the returned value indicates the same success/failure behavior expected from the C module.

### Scenario 2: Apply mode to an input descriptor

A caller has an already-open input descriptor and uses this module to request a specific binary/text mode setting before reading elsewhere in the program.

**Expected behavior**:
- the module operates only on the descriptor provided,
- it does not perform the read itself,
- it returns an integer result for caller-side handling.

**Test focus**:
- invoke the function on a valid input descriptor,
- verify the function returns an integer outcome and does not assume ownership of the descriptor.

### Scenario 3: Handle a descriptor or mode that cannot be applied

A caller passes a descriptor or mode value for which the underlying mode change cannot be completed.

**Expected behavior**:
- the module reports failure through its integer return value,
- it does not substitute fallback behavior or silently claim success.

**Test focus**:
- exercise a failing case permitted by the host environment,
- verify that the function returns the failure result consistent with the C module’s behavior class.

### Scenario 4: No descriptor lifecycle side effects

A caller uses this module as one step in a larger I/O setup flow and continues using the same descriptor afterward.

**Expected behavior**:
- the module does not open a new descriptor,
- the module does not close or replace the provided descriptor,
- the descriptor remains under caller control.

**Test focus**:
- call the function,
- continue using the same descriptor in surrounding test code,
- verify no ownership transfer is implied by the API.

## Requirements

### Functional Requirements

#### FR-1: File descriptor mode adjustment
The module shall accept an existing file descriptor and a mode value, and shall attempt to set the descriptor’s binary/text mode accordingly.

**Traceability**: `binary-io.c`, `set_binary_mode`

#### FR-2: Integer status result
The module shall return an integer status value representing the result of the mode-setting attempt.

**Traceability**: `binary-io.c`, `set_binary_mode`

#### FR-3: Caller-managed descriptor lifecycle
The module shall operate on the caller-provided descriptor without opening, closing, or otherwise taking ownership of that descriptor.

**Traceability**: `binary-io.c`, `set_binary_mode`

#### FR-4: Narrow module responsibility
The module shall be limited to mode-setting behavior for file descriptors and shall not implement unrelated I/O operations.

**Traceability**: `binary-io.c`, `set_binary_mode`

### Key Entities

#### File descriptor
An integer identifier supplied by the caller representing an already-open I/O handle to be modified by the module.

**Relationship**:
- passed into the module function as the target of the mode change request.

#### Mode value
An integer supplied by the caller describing the requested binary/text mode to apply.

**Relationship**:
- paired with a file descriptor as input to the mode-setting operation.

#### Status result
An integer returned by the function indicating whether the requested operation succeeded or failed.

**Relationship**:
- produced from attempting to apply the requested mode to the provided descriptor.

## Success Criteria

### SC-1: Functional parity of module purpose
The Rust module exposes behavior equivalent in scope to the C module: setting mode on an existing file descriptor and returning an integer result.

**Measured by**:
- API-level review against `set_binary_mode` behavior,
- tests covering success and failure result paths.

**Traceability**: `binary-io.c`, `set_binary_mode`

### SC-2: Valid-descriptor scenario support
For a valid descriptor and supported mode request, the Rust implementation returns the success-class result expected by the original module behavior.

**Measured by**:
- automated tests using valid open descriptors.

**Traceability**: `binary-io.c`, `set_binary_mode`

### SC-3: Failure reporting
When the mode change cannot be applied, the Rust implementation returns a failure-class integer result rather than masking the error as success.

**Measured by**:
- automated negative-path tests in an environment where failure can be induced.

**Traceability**: `binary-io.c`, `set_binary_mode`

### SC-4: No descriptor ownership side effects
After calling the Rust function, the caller retains responsibility for the same descriptor, and surrounding code can continue using it subject to normal OS behavior.

**Measured by**:
- tests that perform additional caller-managed operations on the descriptor after invocation.

**Traceability**: `binary-io.c`, `set_binary_mode`