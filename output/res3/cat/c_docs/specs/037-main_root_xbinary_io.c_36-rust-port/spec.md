# spec.md

## Title

Functional Specification: `main_root_xbinary-io.c_36`

## Metadata

- **Project**: `cat`
- **Module**: `main_root_xbinary-io.c_36`
- **Category**: `main_cluster`
- **Source File**: `xbinary-io.c`
- **Rust Branch**: `037-main_root_xbinary_io.c_36-rust-port`
- **Generation Date**: `2026-06-09`

## Overview

This module provides a single, narrow responsibility: reporting failure when the program cannot set binary I/O mode as required by the surrounding application behavior.

The analyzed C module exposes one function, `xset_binary_mode_error`, marked `_Noreturn`, which indicates that this path is a terminal error-reporting path rather than a recoverable operation. The Rust rewrite must preserve that functional boundary: when invoked, it must report the binary-mode setup failure and terminate control flow rather than returning to the caller.

No persistent module-owned data structures are evidenced in the analyzed input.

## Feature Specification

### Supported Feature

- **Terminal reporting of binary-mode setup failure**
  - The module must provide the behavior corresponding to `xset_binary_mode_error`.
  - This behavior is used when an attempt to configure binary mode for I/O has failed.
  - The behavior must be terminal: once invoked, normal execution does not continue through a returned result.

### Rust Port Scope

The Rust version must implement the same functional role as the C module:

- provide an internal module-level facility for handling binary-mode setup failure;
- ensure the failure path is non-returning from the caller’s perspective;
- preserve its role as an error path only, not as a general binary-I/O configuration API.

The Rust port must not invent broader capabilities beyond the evidenced scope.

## User Scenarios & Testing

### Scenario 1: Binary mode setup fails during program startup or stream preparation

A higher-level part of the program attempts to place a stream or standard I/O handle into binary mode. That setup fails. The program then calls this module’s failure handler.

**Expected behavior**
- The module reports the failure through the program’s terminal error path.
- Control flow does not return to the caller.

**Test focus**
- Verify that invoking the Rust equivalent causes termination or an equivalent non-returning failure outcome.
- Verify that the call site cannot observe successful return.

### Scenario 2: Failure occurs before processing file content

The application detects that required binary-mode configuration could not be established before reading or writing data. This module is used to stop further operation.

**Expected behavior**
- The module prevents continuation into normal data processing.
- The failure path is immediate and definitive.

**Test focus**
- Verify that no post-call logic executes in a test harness after the module is invoked.
- Verify that this behavior is suitable for use as a hard failure guard in pre-I/O setup.

### Scenario 3: Consistent fatal handling across all binary-mode failures

Multiple call sites in the application may rely on the same helper when binary-mode setup fails. The module provides one consistent fatal behavior.

**Expected behavior**
- All uses of the Rust equivalent exhibit the same non-returning failure semantics.
- The module does not expose alternate recoverable outcomes.

**Test focus**
- Verify consistent behavior from multiple call paths in unit or integration-style harnesses.
- Verify absence of a normal success/return path.

## Requirements

### Functional Requirements

#### FR-1: Fatal binary-mode failure handling
The module shall provide functionality corresponding to `xset_binary_mode_error` for the specific case where setting binary mode has failed.

**Traceability**
- `xbinary-io.c`
- `xset_binary_mode_error`

#### FR-2: Non-returning control flow
The module’s failure-handling function shall not return to its caller after invocation.

**Traceability**
- `xset_binary_mode_error` declared as `_Noreturn`

#### FR-3: Error-path-only role
The module shall serve only as the terminal error-reporting path for binary-mode setup failure and shall not be expanded into a general binary I/O management interface within this rewrite.

**Traceability**
- Single exposed function in `xbinary-io.c`: `xset_binary_mode_error`

### Key Entities

#### Entity: Binary-mode setup failure handler
A module-level function that is invoked when binary I/O mode cannot be established.

**Attributes evidenced by source analysis**
- It is a function, not a data structure.
- It is terminal/non-returning.
- It represents an error condition tied specifically to binary-mode setup.

**Relationships**
- Called by higher-level program logic after a binary-mode configuration attempt fails.
- Ends the current control path rather than passing an error object back.

#### Data Structures
No core module-owned data structures are evidenced in the analyzed input.

## Success Criteria

### SC-1: Functional equivalence of failure role
The Rust module provides a callable failure handler for binary-mode setup failure matching the source module’s functional role.

**Measured by**
- Presence of a Rust module function or equivalent internal item serving this exact purpose.
- Review confirms no unrelated functionality has been added.

**Traceability**
- `xbinary-io.c`
- `xset_binary_mode_error`

### SC-2: Non-returning behavior preserved
Invoking the Rust equivalent does not return control to the caller.

**Measured by**
- Tests or controlled harnesses demonstrate termination/panic/abort semantics sufficient to make the path non-returning from the caller’s perspective.
- Compiler-visible non-returning typing or equivalent behavior is used where appropriate.

**Traceability**
- `_Noreturn void xset_binary_mode_error (void);`

### SC-3: Appropriate usage in binary-mode failure scenarios
The Rust implementation can be used by surrounding program logic as the terminal path when binary-mode setup fails before I/O processing proceeds.

**Measured by**
- Integration-style test or call-site simulation shows that once invoked, subsequent normal processing code is not executed.

**Traceability**
- Function purpose inferred from name and module file `xbinary-io.c`

### SC-4: Scope discipline
The Rust rewrite remains limited to the evidenced module boundary and does not introduce unsupported public behaviors such as general stream configuration, recovery APIs, or additional binary-I/O abstractions.

**Measured by**
- API review shows only the failure-handling behavior required by the analyzed module.
- No extra capabilities appear without direct source evidence.

**Traceability**
- Single-function module evidence from `xbinary-io.c`

## Out of Scope

The following are not required by the analyzed module evidence and must not be added as module commitments in this rewrite:

- General APIs for enabling or disabling binary mode
- Recoverable error return types for this failure path
- Persistent state or module-managed configuration objects
- Thread-safety guarantees
- Serialization, FFI contracts, or benchmark targets
- Cross-platform policy beyond preserving the existing fatal failure role

## Acceptance Notes

Because the analyzed module contains only one `_Noreturn` error function and no data structures, acceptance should focus on behavioral fidelity and scope control:

- the Rust module must preserve the fatal, non-returning nature of the binary-mode setup failure path;
- the module must remain narrowly scoped;
- tests should validate that callers cannot continue normal execution after invocation.