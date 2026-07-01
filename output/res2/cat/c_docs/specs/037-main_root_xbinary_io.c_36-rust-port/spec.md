# spec.md

## Title

Functional Specification: `main_root_xbinary-io.c_36`

## Document Control

- **Project**: `cat`
- **Module**: `main_root_xbinary-io.c_36`
- **Category**: `main_cluster`
- **Source file**: `xbinary-io.c`
- **Rust branch**: `037-main_root_xbinary_io.c_36-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides a narrowly scoped error path related to setting binary I/O mode. The analyzed module evidence shows a single exported behavior: reporting failure when binary mode cannot be established, and terminating execution through a non-returning error routine.

The Rust rewrite must preserve this functional boundary:

- represent the module-level failure case for binary-mode setup,
- emit the corresponding fatal error through the module’s error path,
- not continue normal execution after this failure is invoked.

No additional capabilities are evidenced by the analyzed input and therefore are out of scope.

## Feature Specification

### Feature: Fatal reporting for binary-mode setup failure

The module supplies a dedicated failure routine for the case where enabling binary mode is not possible.

#### Required behavior

- The Rust module must provide the equivalent of the source module’s binary-mode setup failure path.
- Invoking this failure path must result in immediate fatal error handling.
- The failure path must not return control to the caller.

#### Functional boundary

Based on the analyzed source, this module does **not** define:
- successful binary-mode switching behavior,
- stream management,
- buffering behavior,
- retry logic,
- recovery behavior,
- alternate fallback modes.

Only the fatal error behavior for the binary-mode failure case is within scope.

## User Scenarios & Testing

### Scenario 1: Binary mode cannot be enabled

A caller attempts an operation that requires binary mode for I/O. That operation determines that binary mode cannot be set and delegates to this module’s failure routine.

**Expected outcome**
- The module triggers fatal error reporting for the binary-mode failure case.
- Execution does not resume at the call site.

**Test guidance**
- Verify that invoking the Rust equivalent causes process termination or an equivalent non-returning fatal path.
- Verify that code placed after the invocation is not executed.

### Scenario 2: Integration with higher-level startup or I/O setup logic

A higher-level part of the program performs environment or stream setup during command execution. When binary-mode activation fails, it uses this module’s failure function rather than continuing with degraded behavior.

**Expected outcome**
- The failure is handled centrally by this module’s fatal path.
- No silent continuation occurs after the failure is reported.

**Test guidance**
- Exercise the Rust module from an integration test harness or controlled subprocess.
- Confirm that the program exits abnormally or with the expected fatal termination behavior instead of continuing normal processing.

### Scenario 3: Failure path is dedicated and unambiguous

A maintainer or caller uses the module specifically for the binary-mode setup error case.

**Expected outcome**
- The module behavior is specific to this failure case.
- The module does not expose unrelated stateful behavior or data-model interactions.

**Test guidance**
- Confirm the Rust port exposes only the evidenced functional surface for this module.
- Confirm there is no requirement for caller-provided state objects to use the failure path.

## Requirements

### Functional Requirements

#### FR-1: Binary-mode failure reporting
The module shall provide a dedicated operation corresponding to `xset_binary_mode_error` for the error case where setting binary mode fails.

**Traceability**
- `xbinary-io.c`
- `xset_binary_mode_error`

#### FR-2: Non-returning fatal behavior
The binary-mode failure operation shall be non-returning in observable behavior: after invocation, control shall not continue through the normal caller path.

**Traceability**
- `xset_binary_mode_error` signature marked `_Noreturn`

#### FR-3: Scope limited to the failure case
The module shall be limited to the binary-mode setup failure path and shall not be specified as responsible for successful binary-mode configuration or broader I/O state management.

**Traceability**
- Sole evidenced function in `xbinary-io.c`: `xset_binary_mode_error`

### Key Entities

The analyzed module does not define any core data structures for this functionality.

#### Entity summary

- **Binary-mode setup failure event**: the only evidenced functional entity; this is an error condition handled by the module.
- **Fatal termination path**: the resulting control-flow outcome of invoking the module function.

#### Relationships

- When the binary-mode setup failure event occurs, the module routes execution into the fatal termination path.
- No persistent module-owned data structures are evidenced.

## Success Criteria

### SC-1: Dedicated failure entry point exists
The Rust rewrite includes a module-level function or equivalent callable path that represents the binary-mode setup failure case.

**Traceability**
- `xset_binary_mode_error`

### SC-2: Non-returning behavior is preserved
When the Rust failure path is invoked in a controlled test context, execution does not proceed past the call site.

**Measurable outcome**
- A subprocess, panic-abort path, or equivalent fatal harness confirms no post-call code executes.

**Traceability**
- `_Noreturn void xset_binary_mode_error (void);`

### SC-3: Failure is treated as fatal, not recoverable within this module
Tests confirm that the Rust module does not return a recoverable status from this failure path and does not continue normal operation after invocation.

**Traceability**
- `xset_binary_mode_error`
- `_Noreturn` contract

### SC-4: No unsupported module responsibilities are introduced
Review of the Rust port confirms that this module specification remains limited to the evidenced functionality and does not claim support for unrelated binary-I/O features, state models, or recovery flows.

**Traceability**
- Single-function source evidence in `xbinary-io.c`

## Out of Scope

The following are not evidenced by the analyzed module and are therefore excluded from this specification:

- APIs for successfully enabling binary mode
- stream or file descriptor abstraction
- binary/text conversion behavior
- stateful configuration objects
- retries, fallbacks, or recovery after failure
- thread-safety guarantees
- serialization, FFI surface, or performance requirements

## Notes for Rust Port Alignment

The Rust rewrite should preserve the original module contract at the behavioral level:

- one dedicated fatal error path for binary-mode setup failure,
- no normal return from that path,
- no expansion beyond the evidenced module scope.