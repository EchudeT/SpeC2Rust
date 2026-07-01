# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_gnu_getprogname.c_30`
- **Category**: `module_cluster`
- **Source basis**: `gnu/getprogname.c`
- **Primary entry point**: `getprogname(void) -> char const *`

This module provides one functional responsibility: returning the current program's name as a C string. The Rust rewrite must preserve that observable behavior, including platform-dependent resolution paths used by the original module to obtain the process name when a direct runtime-provided value is not available.

The module is a query-only facility from the caller's perspective: callers ask for the program name and receive a pointer to a nul-terminated string naming the running program.

---

## Feature Specification

### Feature: Program name retrieval

The module shall provide behavior equivalent to `getprogname(void)`, returning the name of the currently running program.

The Rust version must implement:

- Retrieval of the current process/program name through the platform-appropriate mechanism represented by the source module.
- Return of the resolved name as a string value compatible with the module's intended use as a program-name query.
- Best-effort resolution across supported target environments reflected by the source file's conditional logic.
- Stable behavior for repeated calls within the same process, such that each call reports the current program name consistently for that process context.

### Functional boundary

This module's scope is limited to:

- determining the running program's name;
- exposing that result through the module's single public behavior.

This module does **not** define broader process inspection APIs, process enumeration, argument parsing, mutation of the program name, or unrelated runtime metadata access.

---

## User Scenarios & Testing

### Scenario 1: Caller requests the current program name

A caller invokes the module's program-name function during normal execution to obtain the name used for diagnostics, help text, or logging prefixes.

**Expected result**:
- A non-empty program name is returned when the underlying platform can resolve it using the supported mechanisms in the source module.
- The returned value identifies the current program rather than another process.

**Testing approach**:
- Run a test executable with a known invocation name.
- Call the Rust equivalent of `getprogname`.
- Verify that the returned string matches the expected executable/program name according to the platform behavior preserved by the original module.

### Scenario 2: Repeated lookup during one process lifetime

A caller invokes the function multiple times from different parts of the program to label errors or usage output.

**Expected result**:
- Calls are successful repeatedly.
- The observed program name remains consistent across repeated calls within the same process execution.

**Testing approach**:
- Call the function several times in one run.
- Compare results for equality.
- Confirm the value remains a valid string throughout the calls.

### Scenario 3: Platform-specific fallback path is needed

A caller runs the program on a target where the program name is obtained through an OS-specific process-status or process-entry interface represented in the source module.

**Expected result**:
- The module still resolves the current program name through the applicable fallback path.
- The caller receives the same kind of program-name result as on targets with more direct support.

**Testing approach**:
- On a platform corresponding to one of the source module's alternate code paths, execute the test program.
- Verify that the module returns the current program name rather than failing silently or returning an unrelated value.

### Scenario 4: Name retrieval when only basename-style identification is relevant

A caller uses the returned program name in user-facing messages where the executable name, not the full invocation path, is expected.

**Expected result**:
- The returned value matches the program naming convention implemented by the source module for the platform, suitable for display as the program identifier.

**Testing approach**:
- Invoke the executable from a path-containing location if applicable.
- Verify that the Rust module reproduces the same naming outcome as the C module on that platform.

---

## Requirements

### Functional Requirements

#### FR-1: Current program name query
The module shall provide one callable behavior equivalent to `getprogname(void)` that returns the current program's name.

**Traceability**: `gnu/getprogname.c`, function `getprogname`

#### FR-2: String result semantics
The returned program name shall be represented as a nul-terminated string in the original C behavior, and the Rust rewrite shall preserve equivalent externally observable string content.

**Traceability**: `gnu/getprogname.c`, function `getprogname`

#### FR-3: Platform-aware resolution
The module shall resolve the program name using the platform-dependent mechanisms embodied in the source module, including conditional paths needed on supported systems.

**Traceability**: `gnu/getprogname.c`, function `getprogname`

#### FR-4: Process-local correctness
The returned value shall correspond to the currently running process, not to an arbitrary or external process.

**Traceability**: `gnu/getprogname.c`, function `getprogname`

#### FR-5: Repeatable lookup behavior
Repeated calls during the same program execution shall continue to provide the same program-name result unless the underlying process name visible to the original module's mechanisms differs.

**Traceability**: `gnu/getprogname.c`, function `getprogname`

### Key Entities

#### Entity: Program name result
The central entity is the program name itself: a string identifying the current executable/program as returned by `getprogname`.

**Relationship**:
- Produced by the module's sole public behavior.
- Derived from process/runtime state through platform-specific lookup logic.

#### Entity: Process status / process entry records
The source module references OS-specific process information records, including `procentry64` and `pst_status`, to support program-name lookup on some targets.

**Relationship**:
- These records are internal lookup sources for the program name on certain platforms.
- They are not part of the module's public functional surface, but they constrain the behavior the Rust rewrite must preserve on those platforms.

**Traceability**: referenced in `gnu/getprogname.c`

---

## Success Criteria

### SC-1: Correct program identification
For a test executable with a known program name, the Rust rewrite returns the same program-name string that the C module would return on the same platform.

**Traceability**: `gnu/getprogname.c`, function `getprogname`

### SC-2: Repeated-call consistency
In a single process run, multiple invocations of the Rust rewrite produce consistent results matching the module's expected program-name value.

**Traceability**: `gnu/getprogname.c`, function `getprogname`

### SC-3: Platform-path preservation
On platforms represented by alternate resolution paths in the source module, the Rust rewrite successfully resolves the current program name through the equivalent path.

**Traceability**: `gnu/getprogname.c`, function `getprogname`; referenced process record types `procentry64`, `pst_status`

### SC-4: No functional expansion beyond module scope
The Rust rewrite exposes only the behavior needed to retrieve the current program name and does not require callers to supply unrelated process metadata or use broader process-inspection features.

**Traceability**: `gnu/getprogname.c`, function `getprogname`