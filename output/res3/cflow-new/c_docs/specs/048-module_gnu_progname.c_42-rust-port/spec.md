# spec.md

## Title

Rust Functional Specification for `module_gnu_progname.c_42`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_progname.c_42`
- **Category**: `module_cluster`
- **Source file**: `gnu/progname.c`
- **Primary function**: `set_program_name(const char *argv0)`
- **Target Rust branch**: `048-module_gnu_progname.c_42-rust-port`
- **Generation date**: 2026-06-17

## Overview

This module is responsible for deriving and publishing the program name from the process argument string passed as `argv[0]`. Its functional boundary is limited to accepting a program path or name string and updating the module-visible program-name state derived from that input.

The Rust rewrite must preserve this behavior: given an input equivalent to `argv[0]`, it must compute the program name according to the source module’s rules and make the derived name available through the same module responsibility boundary.

## Feature Specification

### Summary

The module provides one functional capability:

- Accept a program invocation string and establish the module’s program-name state from it.

### Behavior to implement

The Rust version must implement behavior equivalent to `set_program_name`:

1. Accept an input string representing the executable name or path used to invoke the process.
2. Derive the visible program name from that input rather than preserving the full path.
3. Support path-bearing inputs by extracting the final path component.
4. Apply the source module’s name normalization behavior for invocation names that match the GNU `lt-` libtool wrapper pattern, so that the published program name reflects the underlying executable name rather than the wrapper-prefixed form.
5. Update the module-level program-name state consistently from the derived result.

### Out of scope

The Rust rewrite must not introduce additional capabilities not evidenced by the source module, including:

- command-line parsing
- process launching
- filesystem validation
- thread-safety guarantees
- persistence or serialization
- new public APIs beyond what is needed to preserve the module’s evidenced functional role

## User Scenarios & Testing

### Scenario 1: Invocation name without a path

A caller provides a plain executable name such as `cflow`.

Expected behavior:

- The module accepts the input.
- The derived program name is `cflow`.
- Module program-name state reflects `cflow`.

### Scenario 2: Invocation name with a Unix-style path

A caller provides a path such as `/usr/bin/cflow` or `./cflow`.

Expected behavior:

- The module ignores preceding directory components.
- The derived program name is the last path component, such as `cflow`.

### Scenario 3: Invocation through a libtool-style wrapper name

A caller provides an invocation string whose final component begins with the GNU libtool-style `lt-` prefix.

Expected behavior:

- The module derives the final path component first.
- If that final component matches the source module’s wrapper-handling rule, the published program name removes the wrapper prefix and reflects the underlying executable name.

### Scenario 4: Reinitialization with a different input

A caller invokes the module function more than once with different values.

Expected behavior:

- The module updates the program-name state each time according to the current input.
- Later calls replace the effective derived name from earlier calls.

### Testing expectations

The Rust version must be testable with cases covering:

- plain executable names
- path-qualified executable names
- names with and without the `lt-` wrapper prefix
- repeated calls with different inputs

Test assertions must verify the derived published name for each case.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide behavior equivalent to `set_program_name(const char *argv0)` for establishing program-name state from a caller-supplied invocation string.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **FR-2**: When the input contains directory separators, the module shall derive the program name from the final path component rather than from the full input string.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **FR-3**: When the derived final component matches the source module’s GNU libtool wrapper naming case using the `lt-` prefix, the module shall publish the normalized underlying program name rather than the wrapper-prefixed name.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **FR-4**: The module shall update module-visible program-name state based on the most recent accepted input.
  **Traceability**: `gnu/progname.c`, `set_program_name`

### Key Entities

- **Invocation string (`argv0`)**: The caller-supplied string representing the executable name or path used to invoke the program.
  **Traceability**: `set_program_name(const char *argv0)`

- **Derived program name**: The normalized executable name obtained from the invocation string after applying basename extraction and wrapper-prefix handling as implemented by the source module.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **Program-name state**: The module-level published state updated by the function to reflect the derived program name.
  **Traceability**: `gnu/progname.c`, `set_program_name`

## Success Criteria

- **SC-1**: For an input consisting only of a simple executable name, the Rust module derives and publishes that same name.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **SC-2**: For an input containing one or more directory components, the Rust module derives and publishes only the final component.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **SC-3**: For an input whose final component uses the source module’s supported `lt-` wrapper naming case, the Rust module publishes the normalized non-wrapper name.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **SC-4**: After multiple calls with different inputs, the Rust module’s published program-name state matches the result derived from the most recent call.
  **Traceability**: `gnu/progname.c`, `set_program_name`

- **SC-5**: Conformance tests covering the listed user scenarios pass on the Rust branch implementation without requiring capabilities outside this module’s evidenced responsibility.
  **Traceability**: `gnu/progname.c`, `set_program_name`