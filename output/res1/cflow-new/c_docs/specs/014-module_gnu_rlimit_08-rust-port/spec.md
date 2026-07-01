# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_gnu_rlimit_08`
- **Category**: `module_cluster`
- **Source basis**: `gnu/getdtablesize.c`
- **Rust target branch**: `014-module_gnu_rlimit_08-rust-port`
- **Generation date**: `2026-06-11`

## Feature Specification

This module provides the process-level query for the maximum number of file descriptors that can be open at once through the `getdtablesize` functionality.

The Rust rewrite must implement the same observable behavior as this module:

- Expose module functionality equivalent to `getdtablesize(void) -> int`.
- Return the current descriptor table size limit for the calling process as an integer.
- Derive the result from the process resource limit represented by `struct rlimit`, specifically the open-file descriptor limit used by this module.
- Preserve the module’s role as a compatibility/provider implementation of `getdtablesize` for environments where the function may not otherwise be available in the same form.

The specification is limited to the behavior evidenced by the module input: querying and returning the current open-file-descriptor limit value for the running process.

## User Scenarios & Testing

### Scenario 1: A caller needs the process descriptor-table limit
A caller invokes the module’s `getdtablesize` functionality to learn how many file descriptors the current process may have open.

**Expected behavior**
- The call returns an `int`.
- The returned value reflects the current process limit used by the module for open file descriptors.

**Testable outcome**
- The Rust implementation returns a positive integer on systems where the process limit is available and representable as `int`.

### Scenario 2: A portability layer needs a `getdtablesize` provider
A higher-level portability layer or application expects `getdtablesize` behavior even if the platform’s native libc surface differs.

**Expected behavior**
- The Rust module provides equivalent functionality at the module boundary.
- The result matches the process resource limit source used by this module.

**Testable outcome**
- When compared against the platform resource limit query for open files, the Rust result matches the active limit value, subject to integer representation.

### Scenario 3: The open-file limit changes before the call
The process open-file soft limit is changed by the environment or test harness before calling the module.

**Expected behavior**
- A subsequent call reflects the current limit at call time rather than a cached startup value.

**Testable outcome**
- After changing the process soft limit through the operating system interface, a new call returns the updated limit value if it remains representable as `int`.

## Requirements

### Functional Requirements

#### FR-1: Provide descriptor-table-size query
The Rust module shall provide functionality equivalent to `getdtablesize(void) -> int` as defined by the source module.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize`

#### FR-2: Report the current process open-file limit
The returned value shall represent the current file descriptor table size limit for the calling process, using the resource-limit information associated with the module’s `struct rlimit` usage.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize`; referenced entity `struct rlimit`

#### FR-3: Return the result as an integer
The module shall expose the result in integer form consistent with the source function signature.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize` with signature `int getdtablesize(void)`

#### FR-4: Perform the query at call time
The module shall determine the returned value when the function is called, so that the result reflects the process limit in effect for that call.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize`; referenced entity `struct rlimit`

### Key Entities

#### `getdtablesize`
The module’s sole functional entry point. It has no input parameters and returns an integer result representing the process descriptor-table size limit.

**Relationship**
- Obtains its result from process resource-limit state.

#### `struct rlimit`
The resource-limit structure referenced by the module and used as the source of the open-file descriptor limit.

**Relationship**
- Supplies the limit data consumed by `getdtablesize`.

## Success Criteria

### SC-1: Functional equivalence
The Rust module exposes functionality equivalent to `getdtablesize(void) -> int`.

**Measurement**
- A conformance test can call the Rust implementation with no arguments and receive an integer result.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize`

### SC-2: Correct limit value
For a process with a known current open-file soft limit, the Rust implementation returns the same limit value, provided it is representable as `int`.

**Measurement**
- In a test environment that queries and sets the process open-file limit through the operating system, the Rust result equals the current limit value after the call.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize`; referenced entity `struct rlimit`

### SC-3: Reflects runtime changes
If the process open-file limit is changed between two calls, the Rust implementation returns values corresponding to the updated current limit on the later call.

**Measurement**
- A test that changes the process limit between invocations observes a changed result matching the new current limit.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize`; referenced entity `struct rlimit`

### SC-4: No unsupported surface expansion
The Rust rewrite does not require additional public APIs beyond the module-equivalent `getdtablesize` functionality evidenced by the source.

**Measurement**
- The module interface review shows no extra public behavior beyond the specified query function.

**Traceability**: `gnu/getdtablesize.c`, function `getdtablesize`