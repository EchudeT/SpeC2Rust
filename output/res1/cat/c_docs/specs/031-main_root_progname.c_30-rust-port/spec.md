# spec.md

## Title

Functional Specification: `main_root_progname.c_30`

## Metadata

- **Project**: `cat`
- **Module**: `main_root_progname.c_30`
- **Category**: `main_cluster`
- **Source file**: `progname.c`
- **Primary function**: `set_program_name(const char *argv0)`
- **Rust branch target**: `031-main_root_progname.c_30-rust-port`
- **Generation date**: 2026-06-06

## Overview

This module is responsible for establishing the program name from the process invocation string supplied as `argv[0]`. Its behavior is limited to interpreting the provided program path/name input and deriving the program name value the rest of the application can use for identity and user-facing reporting.

The Rust rewrite must preserve this functional boundary: given a process invocation string, determine and record the program name in a form suitable for subsequent use by the application. The module is not specified to parse command-line options, execute program logic, or provide unrelated process metadata.

## Feature Specification

### Feature: Program Name Initialization

The module accepts the invocation string passed to the program entry path and derives the executable name portion to represent the running program.

The Rust version must implement the following behavior:

- Accept an input corresponding to `argv[0]`.
- Derive the program name from that input rather than preserving an entire path verbatim when a basename can be determined.
- Support common invocation forms in which the input may contain directory separators.
- Establish the resulting program name for later use by the program after initialization.

### Functional Scope

Within this module, supported functionality is limited to:

- interpreting the invocation string,
- extracting the program name component,
- handling naming forms the original module recognizes,
- storing or publishing the resulting name for the rest of the program.

The Rust port must remain within this scope.

### Explicit Non-Goals

The Rust version must not introduce unevidenced functionality such as:

- command-line parsing,
- executable path canonicalization beyond program-name derivation,
- environment-variable based naming,
- multiple alternative public APIs for name management,
- persistence, serialization, or external interfaces.

## User Scenarios & Testing

### Scenario 1: Program started with a simple executable name

A caller initializes the module with an invocation string that contains only the executable name, such as `cat`.

**Expected result**:
- The module records `cat` as the program name.

**Test approach**:
- Initialize with `cat`.
- Verify the stored/published program name is `cat`.

### Scenario 2: Program started with a relative path

A caller initializes the module with a relative path such as `./cat` or `bin/cat`.

**Expected result**:
- The module derives the executable name component, `cat`.

**Test approach**:
- Initialize with representative relative paths.
- Verify the resulting program name excludes directory components.

### Scenario 3: Program started with an absolute path

A caller initializes the module with an absolute path such as `/usr/bin/cat`.

**Expected result**:
- The module derives `cat` as the program name.

**Test approach**:
- Initialize with an absolute path string.
- Verify only the final path component is retained as the program name.

### Scenario 4: Program name initialization is performed during startup

The application startup path invokes this module before user-facing diagnostics or program identification are needed.

**Expected result**:
- After initialization, the rest of the program can use the established program name consistently.

**Test approach**:
- Invoke initialization as part of startup sequencing in an integration-style test.
- Verify downstream consumers observe the initialized program name.

### Scenario 5: Input contains implementation-recognized path/name decorations

The invocation string may include naming/path forms that the original module normalizes when deriving the program name.

**Expected result**:
- The Rust version matches the original module’s observable result for those recognized forms.

**Test approach**:
- Build compatibility tests from the original module behavior for representative decorated invocation strings.
- Verify the Rust output matches the C module output exactly for the same inputs.

## Requirements

### Functional Requirements

#### FR-1: Invocation String Acceptance
The module shall accept a program invocation string as input for initialization of the program name.

**Traceability**:
- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-2: Program Name Derivation
The module shall derive the program name from the supplied invocation string rather than treating the full input as necessarily the final displayed name.

**Traceability**:
- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-3: Path Component Handling
When the invocation string contains directory components, the module shall use the executable-name portion as the resulting program name.

**Traceability**:
- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-4: Startup Initialization Role
The module shall support use during program startup so that the program name is established before other application behavior depends on it.

**Traceability**:
- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-5: Behavioral Compatibility
For invocation-string forms handled by the original module, the Rust rewrite shall produce the same observable program-name result.

**Traceability**:
- `progname.c`
- `set_program_name(const char *argv0)`

### Key Entities

#### Entity: Program Invocation String
The input string representing the process invocation name/path, corresponding to `argv[0]`.

**Role**:
- Source material from which the module derives the program name.

#### Entity: Program Name
The resulting executable name established by this module for application use.

**Role**:
- Derived output of initialization.
- Used by the wider program after startup.

#### Relationship
The program name is derived from the program invocation string by removing path structure and preserving the executable name component according to the original module’s behavior.

## Success Criteria

### SC-1: Correct Simple Name Handling
Given an invocation string containing only a program name, the Rust module returns or records the identical name.

**Measured by**:
- Unit test with inputs such as `cat`.

**Traceability**:
- FR-1
- FR-2

### SC-2: Correct Path Stripping
Given invocation strings containing relative or absolute directory paths, the Rust module derives only the executable-name component.

**Measured by**:
- Unit tests with inputs such as `./cat`, `bin/cat`, and `/usr/bin/cat`.

**Traceability**:
- FR-2
- FR-3

### SC-3: Startup Usability
The Rust module can be invoked during program startup to establish program name state before later module use.

**Measured by**:
- Integration test showing initialization occurs before a downstream consumer reads the program name.

**Traceability**:
- FR-4

### SC-4: C Compatibility for Supported Inputs
For a compatibility test set derived from the original C module’s accepted invocation-string forms, the Rust module matches the original observable program-name result for every case in the set.

**Measured by**:
- Cross-language fixture comparison or golden tests.

**Traceability**:
- FR-5

### SC-5: No Scope Expansion
The Rust rewrite exposes only functionality required to initialize and establish the program name for the application, without adding unrelated naming or process-management features.

**Measured by**:
- API/spec review against this document.

**Traceability**:
- Feature Specification
- Explicit Non-Goals