# spec.md

## Title
Rust Port Functional Specification: `module_gnu_getdtablesize.c_29`

## Document Control
- **Project**: `cflow-new`
- **Module**: `module_gnu_getdtablesize.c_29`
- **Category**: `module_cluster`
- **Source basis**: `gnu/getdtablesize.c`
- **Rust branch**: `035-module_gnu_getdtablesize.c_29-rust-port`
- **Generation date**: `2026-06-17`

## Overview
This module provides the project’s behavior for determining the process file-descriptor table size limit in a portable, non-throwing manner consistent with the source module. The Rust rewrite must preserve the observable behavior of the C module: it must obtain the maximum number of file descriptors available to the process, prefer a meaningful runtime limit when available, and return an integer result suitable for callers that need a `getdtablesize`-style value.

The source evidence includes logic associated with a non-throwing `_setmaxstdio_nothrow` helper and use of `struct rlimit`, indicating platform-aware handling of descriptor limits and fallback behavior.

## In Scope
- Computing a `getdtablesize`-style integer result for the current process.
- Using available operating-system descriptor limit information where applicable.
- Preserving non-failing or fallback-oriented behavior implied by the source helper.
- Returning a result in integer form compatible with the module’s purpose.

## Out of Scope
- Defining new public APIs beyond what is required to preserve module behavior.
- Managing file descriptors themselves.
- Altering process limits.
- Exposing extended configuration, thread-safety guarantees, or recovery features not evidenced by the source module.

## Feature Specification

### Summary
The Rust module must implement the same functional role as the C source: provide the process descriptor-table size value that callers expect from this compatibility layer.

### Required Behavior
1. The module must determine the current process descriptor capacity from system-provided limits when such information is available through the source-equivalent mechanism evidenced by `struct rlimit`.
2. The module must support behavior consistent with environments where descriptor-limit values may need bounded or adjusted integer conversion before returning a final result.
3. The module must preserve non-throwing behavior associated with the `_setmaxstdio_nothrow` helper path, meaning internal adjustment/fallback steps must not surface exceptions or unrecoverable failures to callers as part of normal operation.
4. The module must return a single integer result representing the descriptor-table size available to the process in the sense intended by `getdtablesize`.
5. When the preferred limit source cannot yield a usable result, the module must use fallback behavior consistent with the source module’s compatibility purpose rather than inventing a new failure mode.

## User Scenarios & Testing

### Scenario 1: Querying descriptor-table size on a platform with runtime limit support
A caller invokes the Rust port in a normal process environment where the operating system exposes descriptor limits for the process.

**Expected result**
- The module returns a positive integer descriptor-table size derived from the runtime limit source.
- The returned value is stable for repeated calls unless the process environment changes externally.

**Testing**
- Run in an environment where process descriptor limits are available.
- Verify the function returns a positive integer.
- Verify repeated calls return the same result under unchanged conditions.

### Scenario 2: Handling large or bounded limit values
A caller uses the module on a system where the runtime descriptor limit may exceed the directly usable integer range or require platform-specific bounding.

**Expected result**
- The module returns an integer-compatible value without overflow exposure to the caller.
- The result remains suitable for `getdtablesize`-style consumers.

**Testing**
- Simulate or mock a limit source larger than the target integer return range.
- Verify the returned value is bounded or adjusted consistently with the source behavior.
- Verify no panic or unhandled conversion error escapes.

### Scenario 3: Fallback when preferred limit retrieval is unusable
A caller invokes the module in an environment where the preferred limit query path does not provide a usable answer.

**Expected result**
- The module still returns a valid integer result through source-consistent fallback behavior.
- The module does not introduce a new hard failure for this case.

**Testing**
- Simulate failure or unusable data from the runtime limit query path.
- Verify a fallback result is produced.
- Verify the module call completes without panic.

### Scenario 4: Non-throwing internal adjustment behavior
A caller relies on the module in code paths where exceptions or equivalent abrupt failures are unacceptable.

**Expected result**
- Internal logic corresponding to `_setmaxstdio_nothrow` remains non-throwing in observable behavior.
- The module completes with an integer result even when an adjustment step cannot improve the outcome.

**Testing**
- Exercise paths that trigger internal adjustment or compatibility logic.
- Verify the call does not panic or propagate unexpected errors.
- Verify a deterministic integer result is returned.

## Requirements

### Functional Requirements
- **FR-1**: The Rust module shall provide the functional equivalent of the source module’s `getdtablesize` behavior by returning an integer representing the process descriptor-table size.
  **Traceability**: `gnu/getdtablesize.c`

- **FR-2**: The Rust module shall obtain descriptor-limit information from the operating-system limit model evidenced by use of `struct rlimit` when that source is available and usable.
  **Traceability**: `gnu/getdtablesize.c`, `struct rlimit`

- **FR-3**: The Rust module shall handle conversion of limit information into the returned integer result without exposing overflow or invalid-value behavior to callers.
  **Traceability**: `gnu/getdtablesize.c`, `struct rlimit`

- **FR-4**: The Rust module shall preserve non-throwing behavior for internal compatibility or adjustment logic corresponding to `_setmaxstdio_nothrow`.
  **Traceability**: `_setmaxstdio_nothrow` in `gnu/getdtablesize.c:34-42`

- **FR-5**: The Rust module shall use fallback behavior when preferred runtime limit retrieval does not produce a usable descriptor-table size, rather than defining a new external error contract.
  **Traceability**: `gnu/getdtablesize.c`

### Key Entities
- **Process descriptor-table size result**
  - The integer outcome returned by the module.
  - It represents the maximum number of file descriptors available to the process in the module’s compatibility sense.

- **Resource limit record (`struct rlimit`)**
  - The operating-system limit entity used by the source module to obtain descriptor-related bounds.
  - It supplies the runtime limit information from which the returned descriptor-table size is derived when usable.

- **Non-throwing adjustment helper behavior**
  - The behavior evidenced by `_setmaxstdio_nothrow`.
  - It participates in compatibility handling without changing the module’s external contract into an exception-producing one.

## Success Criteria
- **SC-1**: In environments where process descriptor limits are available, the Rust module returns a positive integer result derived from that runtime limit source.
  **Traceability**: `gnu/getdtablesize.c`, `struct rlimit`

- **SC-2**: Under repeated calls in an unchanged process environment, the Rust module returns the same integer result.
  **Traceability**: `gnu/getdtablesize.c`

- **SC-3**: When supplied or exposed to descriptor-limit values that exceed the directly usable integer return range, the Rust module completes without panic and returns an integer-compatible value.
  **Traceability**: `gnu/getdtablesize.c`, `struct rlimit`

- **SC-4**: When the preferred runtime limit query path is unavailable or unusable, the Rust module still completes and returns a fallback integer result instead of introducing a new external failure mode.
  **Traceability**: `gnu/getdtablesize.c`

- **SC-5**: Code paths corresponding to `_setmaxstdio_nothrow` do not panic or surface unexpected exceptions during normal module use.
  **Traceability**: `_setmaxstdio_nothrow` in `gnu/getdtablesize.c:34-42`

## Acceptance Notes
- Conformance is based on matching the source module’s observable behavior and compatibility role, not on reproducing C implementation structure.
- Any Rust implementation choices are acceptable only if they satisfy the above requirements and success criteria without adding unsupported functionality.