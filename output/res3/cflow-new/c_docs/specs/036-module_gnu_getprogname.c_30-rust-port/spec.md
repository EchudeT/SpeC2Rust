# spec.md

## Title

Functional Specification for `module_gnu_getprogname.c_30` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_getprogname.c_30`
- **Category**: `module_cluster`
- **Source file**: `gnu/getprogname.c`
- **Primary function**: `getprogname(void) -> char const *`
- **Rust branch**: `036-module_gnu_getprogname.c_30-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides program-name discovery functionality through a single query function, `getprogname`. Its purpose is to return the current process's program name as a C string pointer.

The Rust rewrite must preserve the observable behavior of this module as a program-name lookup facility. The rewrite must implement the same functional boundary: obtaining and returning the calling process's program name, using platform-appropriate process information sources as needed by the original module's supported environments.

The specification is limited to behavior evidenced by the source module interface and referenced process-status data structures. No additional public capabilities are introduced.

## Feature Specification

### Feature: Current program name lookup

The module exposes a function that retrieves the current program's name for use by other parts of the project.

Required behavior for the Rust version:

- Provide the equivalent of `getprogname(void)`.
- Return the current process's program name as a null-terminated string representation suitable for C-compatible consumers within the project.
- Derive the name from process/program execution context rather than requiring caller-supplied state.
- Support environments implied by the source module's use of process-entry/status structures, meaning the implementation may rely on operating-system process metadata where required.
- Produce a stable result for a single process invocation in ordinary successful cases, so repeated lookups during the same run yield the same program name content unless the underlying platform semantics dictate otherwise.

### Out of scope

The Rust rewrite must not add new documented behavior beyond the evidenced module boundary, including:

- no new public API beyond the existing program-name query behavior,
- no caller-controlled override mechanism,
- no promise of thread-safety semantics not present in the source evidence,
- no persistence, serialization, caching contract, or recovery interface,
- no requirement to expose internal OS-specific process structures publicly.

## User Scenarios & Testing

### Scenario 1: Diagnostic code requests the running program name

A caller invokes the module to obtain the program name for use in an error or status message.

Expected result:

- The function returns a valid string result representing the current process's program name.
- The returned value can be read as a C-style string by existing project code.

Test coverage:

- Invoke the Rust replacement from a process with a known executable name.
- Verify that the returned string is non-empty when the platform provides a program name.
- Verify that the string content corresponds to the running program name according to platform conventions.

### Scenario 2: Repeated lookup within the same process

A caller asks for the program name multiple times during execution.

Expected result:

- Each lookup returns the same program name content for the process under normal conditions.

Test coverage:

- Call the function repeatedly in one process.
- Compare returned string contents across calls.
- Confirm consistent content.

### Scenario 3: Platform-specific process metadata path

The module is used on a platform where program-name lookup depends on process status or process entry information.

Expected result:

- The module successfully extracts the program name from the platform's process metadata source represented by the source module's referenced process structures.

Test coverage:

- On applicable target platforms, run integration tests that exercise the OS-specific lookup path.
- Verify successful extraction and correct string formation from the platform metadata.

### Scenario 4: Lookup failure or unavailable program name source

The environment does not provide usable process metadata, or retrieval fails.

Expected result:

- The function behaves according to the original module's failure behavior, returning no invented substitute beyond what the source module defines.

Test coverage:

- Exercise failure-capable paths where feasible through controlled test doubles or platform-specific negative tests.
- Verify that the Rust result matches the original module's observable failure semantics.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a program-name query function equivalent in purpose to `getprogname(void)`.
  **Traceability**: `gnu/getprogname.c`, function `getprogname`.

- **FR-2**: The function shall obtain the name of the current process/program without requiring input parameters from the caller.
  **Traceability**: `gnu/getprogname.c`, function signature `getprogname(void)`.

- **FR-3**: The function shall return the program name in C-string form compatible with existing C-oriented module consumers.
  **Traceability**: `gnu/getprogname.c`, return type `char const *`, function `getprogname`.

- **FR-4**: The implementation shall support operating-system lookup behavior implied by the source module's referenced process information structures when determining the program name.
  **Traceability**: `gnu/getprogname.c`, referenced entities `struct procentry64`, `struct pst_status`, function `getprogname`.

- **FR-5**: When the program name is successfully obtainable, repeated calls within the same process shall return the same program name content under normal execution conditions.
  **Traceability**: `gnu/getprogname.c`, function `getprogname` as a pure query over current process identity.

- **FR-6**: When program-name retrieval is not successful, the Rust implementation shall preserve the original module's observable failure behavior rather than introducing fallback features not evidenced by the source interface.
  **Traceability**: `gnu/getprogname.c`, function `getprogname`.

### Key Entities

- **Program name result**
  The central entity exposed by the module. It is the string value returned by `getprogname`, representing the current process's program name.
  **Traceability**: `gnu/getprogname.c`, function `getprogname`.

- **Process entry information (`procentry64`)**
  A platform-specific process metadata structure referenced by the source module and used as an information source for deriving the current program name on applicable systems.
  **Traceability**: `gnu/getprogname.c`, referenced type `procentry64` / anonymous `struct procentry64`.

- **Process status information (`pst_status`)**
  A platform-specific process status structure referenced by the source module and used as an information source for deriving the current program name on applicable systems.
  **Traceability**: `gnu/getprogname.c`, referenced type `pst_status` / anonymous `struct pst_status`.

### Entity Relationships

- `getprogname` is the only module-level functional entry point.
- `getprogname` produces the **program name result**.
- On platforms requiring OS process inspection, `getprogname` derives that result from **process entry information** and/or **process status information**.

## Success Criteria

- **SC-1**: The Rust module exposes behaviorally equivalent program-name lookup corresponding to `getprogname(void)` from `gnu/getprogname.c`.
  **Verification**: API and behavior review against source module.

- **SC-2**: In successful execution on supported target environments, the Rust module returns a readable null-terminated string representing the current process's program name.
  **Verification**: Integration test invoking the function from a named executable.

- **SC-3**: Repeated calls in the same process return identical program name content in normal successful cases.
  **Verification**: Multi-call consistency test.

- **SC-4**: On platforms represented by the source module's process metadata structures, the Rust implementation successfully uses the applicable OS process information source to obtain the program name.
  **Verification**: Platform-specific integration test or conformance review tied to `procentry64` / `pst_status` usage paths.

- **SC-5**: In failure conditions, the Rust module matches the original module's observable failure outcome and does not add undocumented fallback behavior.
  **Verification**: Negative-path comparison against source behavior.

## Non-Goals

The Rust port is not required by this specification to:

- define additional public functions,
- expose internal process metadata structures as public Rust API,
- provide user-configurable program-name sources,
- guarantee behavior beyond the original module's evidenced functional scope.

## Notes for Port Validation

Validation should compare the Rust rewrite against the C module at the level of externally visible behavior:

- whether a program name is returned,
- what string content is returned in normal cases,
- whether repeated calls are consistent,
- how failure cases are surfaced.