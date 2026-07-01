# spec.md

## Overview

This module provides the project’s abstraction for obtaining the process file descriptor table size limit in a portable way. Based on the analyzed source, its behavior is centered on determining the maximum number of open file descriptors/stdio handles that the process may use, including use of platform resource-limit information and a helper that adjusts the stdio handle limit without throwing errors.

The Rust rewrite must preserve this functional role: provide the same effective module behavior for querying the descriptor-table/stdio upper bound used by the project, while preserving the original module’s error-tolerant behavior where evidenced.

## Scope

In scope for this module:

- Determining an upper bound for open descriptors/handles for the current process.
- Using resource-limit information represented by `struct rlimit` where applicable.
- Supporting a non-throwing adjustment path for stdio maximums as evidenced by `_setmaxstdio_nothrow`.

Out of scope:

- Any new public API not evidenced by this module.
- Management of individual file descriptors.
- General resource-limit administration beyond what is needed to obtain this module’s result.
- Unrelated I/O, allocation, threading, serialization, or recovery features.

## Feature Specification

### Summary

The module supplies the project with a reliable way to obtain the process descriptor table size limit across supported environments. Where platform behavior requires it, the module includes a best-effort, non-throwing helper to adjust the stdio maximum and uses resource-limit data to derive the returned bound.

### Required Rust Behavior

The Rust version must implement the following module behavior:

1. Provide the module’s descriptor-limit query behavior for the current process.
2. Derive or constrain that result using the operating environment’s file/handle limit information represented by the equivalent of `struct rlimit`, when such data is part of the original module’s behavior.
3. Preserve non-throwing behavior for the stdio-maximum adjustment helper path evidenced by `_setmaxstdio_nothrow`, meaning failure in that adjustment path must be handled internally rather than surfaced as an exception/panic-driven contract.
4. Return a usable integer bound representing the process maximum relevant to descriptor/stdio table sizing.

## User Scenarios & Testing

### Scenario 1: Project code needs the process descriptor limit

A caller needs the module to report the current process’s descriptor-table size bound so higher-level code can size loops, validation, or compatibility logic.

**Expected result:**
- The module returns an integer upper bound suitable for that purpose.
- The result reflects the process environment rather than a hard-coded unrelated value.

**Test guidance:**
- Invoke the Rust module’s descriptor-limit behavior in a normal process context.
- Verify that the returned value is a positive integer.
- Verify that the value is consistent with the platform’s observable descriptor/handle limit source used by the module.

### Scenario 2: Resource-limit information is available

The runtime environment exposes file-related process limits through the OS resource-limit mechanism.

**Expected result:**
- The module uses that information in determining the returned bound.
- The returned bound is compatible with the relevant `rlimit` values for the process.

**Test guidance:**
- In an environment where file descriptor limits can be queried, compare the module’s result against the current process limit source.
- Confirm the module does not ignore the applicable limit information.

### Scenario 3: Stdio-maximum adjustment path does not succeed

The internal path that attempts to adjust stdio maximums cannot increase or set the desired bound.

**Expected result:**
- The module remains non-throwing for that helper path.
- The module still produces a defined module result consistent with the original behavior rather than crashing.

**Test guidance:**
- Exercise the code path under conditions where adjustment is denied or unsupported.
- Verify there is no panic or uncaught failure caused by the helper path itself.
- Verify the resulting module output remains an integer bound.

## Requirements

### Functional Requirements

- **FR-1:** The module shall provide behavior equivalent to the original `gnu/getdtablesize.c` for obtaining the current process descriptor-table or stdio-table size bound.
  - **Traceability:** `gnu/getdtablesize.c`

- **FR-2:** The module shall use process resource-limit information represented by the equivalent of `struct rlimit` when determining the bound, where this is part of the original module behavior.
  - **Traceability:** `gnu/getdtablesize.c`, `struct rlimit`

- **FR-3:** The module shall include a non-throwing helper behavior equivalent to `_setmaxstdio_nothrow(int newmax)` for stdio-limit adjustment attempts needed by the module logic.
  - **Traceability:** `_setmaxstdio_nothrow` in `gnu/getdtablesize.c:34-42`

- **FR-4:** The module shall handle unsuccessful stdio-limit adjustment attempts without panicking or requiring exception-style handling by callers.

- **FR-5:** The module shall produce an integer result representing the effective maximum relevant to descriptor-table sizing for the current process.

### Key Entities

- **Process descriptor/stdio limit**
  - The effective maximum count this module reports for the current process.
  - Relationship: This is the primary output of the module and is derived from platform/process limit sources.

- **Resource limit (`rlimit`)**
  - The operating-system limit structure used by the module to inspect file-related limits.
  - Relationship: Supplies limit values that inform or bound the module’s reported maximum.

- **Non-throwing stdio adjustment operation**
  - The helper behavior represented by `_setmaxstdio_nothrow`.
  - Relationship: Used internally when the platform requires or permits adjustment of stdio maximums before determining the effective result.

## Success Criteria

- **SC-1:** The Rust rewrite can be used by the project to obtain a process descriptor/stdio limit with behavior functionally matching the original module.
  - **Measured by:** Replacement in module-level tests or integration use without changing caller expectations.
  - **Traceability:** `gnu/getdtablesize.c`

- **SC-2:** In environments where resource limits are available, the Rust rewrite returns a value consistent with the applicable process file-limit information.
  - **Measured by:** Test comparison against the process’s observable file-limit source associated with `rlimit`.
  - **Traceability:** `struct rlimit`, `gnu/getdtablesize.c`

- **SC-3:** When stdio-maximum adjustment cannot be applied, the Rust rewrite does not panic and still returns a defined integer result.
  - **Measured by:** Negative-path tests that force or simulate adjustment failure and observe no panic.
  - **Traceability:** `_setmaxstdio_nothrow` in `gnu/getdtablesize.c:34-42`

- **SC-4:** The Rust module preserves the original module’s scope, adding no unrelated capabilities beyond descriptor-limit determination and the evidenced helper behavior.
  - **Measured by:** API/spec review against this module’s evidenced functionality.
  - **Traceability:** `gnu/getdtablesize.c`, `_setmaxstdio_nothrow`