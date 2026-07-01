# spec.md

## Overview

This module defines the program-name initialization behavior for the `pwd` project. Its responsibility is limited to accepting the process entry-point name string (`argv[0]`) and deriving the stored program name from it.

The Rust rewrite must preserve this functional boundary: given an input program path/name string, determine the effective program name in the same observable cases supported by the C module, including path stripping and handling of specific invocation prefixes.

## Scope

### In Scope
- Accepting a caller-provided program invocation string.
- Deriving the effective program name from that string.
- Supporting inputs that are plain names and path-qualified names.
- Supporting the special handling present for names invoked through the `lt-` prefix form.

### Out of Scope
- Parsing command-line options.
- Launching programs or resolving executables from the filesystem.
- Managing broader process-global runtime state beyond program-name derivation behavior evidenced by this module.
- Defining unrelated utility APIs.

## Feature Specification

The module provides program-name setup for the application startup path.

The Rust version must implement behavior equivalent to `set_program_name(const char *argv0)`:

- It must consume a program invocation string supplied by the caller.
- It must identify the basename portion when the input includes directory components.
- It must recognize the special `lt-` prefix form used in launcher/wrapper-style invocation names and derive the effective program name accordingly.
- It must support the same category of valid startup inputs as the C behavior, including names with and without path separators.

This module is a startup-support module: it exists so other parts of the application can rely on a normalized program name rather than the raw `argv[0]` text.

## User Scenarios & Testing

### Scenario 1: Plain executable name
A caller initializes program naming with a simple invocation string such as `pwd`.

**Expected result:** the effective program name is `pwd`.

### Scenario 2: Path-qualified executable
A caller initializes program naming with a path such as `/usr/bin/pwd` or `./pwd`.

**Expected result:** directory components are ignored and the effective program name is `pwd`.

### Scenario 3: Wrapper-style `lt-` prefixed basename
A caller initializes program naming with an invocation whose basename begins with `lt-`, such as `./lt-pwd`.

**Expected result:** the effective program name is normalized to `pwd`.

### Scenario 4: Wrapper-style path-qualified invocation
A caller initializes program naming with a path whose final component is `lt-` prefixed, such as `/build/.libs/lt-pwd`.

**Expected result:** the basename is selected first, then normalized so the effective program name is `pwd`.

### Scenario 5: Startup integration
The application startup path calls this module once using the process `argv[0]`.

**Expected result:** later program logic can rely on the initialized program name matching the same derived value as the C module would produce.

### Testing Guidance
The Rust rewrite should be tested with table-driven inputs covering:
- plain names
- absolute and relative paths
- names beginning with `lt-`
- paths whose basename begins with `lt-`

For each case, expected outputs must match the derivation rules above.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide behavior equivalent to `set_program_name` for initializing the program name from a caller-supplied invocation string.
  **Traceability:** `progname.c`, `set_program_name`

- **FR-2**: When the supplied invocation string contains directory components, the module shall derive the effective program name from the final path component only.
  **Traceability:** `progname.c`, `set_program_name`

- **FR-3**: When the derived basename begins with the special `lt-` prefix form handled by the C module, the module shall remove that prefix when determining the effective program name.
  **Traceability:** `progname.c`, `set_program_name`

- **FR-4**: The module shall preserve the effective program name for use by the rest of the application after initialization.
  **Traceability:** `progname.c`, `set_program_name`

### Key Entities

- **Program invocation string**: The caller-provided startup name, corresponding to `argv[0]`. This is the input to program-name initialization.
- **Basename**: The final path component derived from the invocation string when directory separators are present.
- **Effective program name**: The normalized name produced by the module after basename extraction and any required `lt-` prefix removal.

Relationship:
- The effective program name is derived from the program invocation string, optionally through basename extraction, and then optional `lt-` prefix normalization.

## Success Criteria

- **SC-1**: For a plain input name such as `pwd`, the Rust module produces `pwd` as the effective program name.
  **Traceability:** `progname.c`, `set_program_name`

- **SC-2**: For path-qualified inputs such as `./pwd` and `/usr/bin/pwd`, the Rust module produces `pwd` as the effective program name.
  **Traceability:** `progname.c`, `set_program_name`

- **SC-3**: For `lt-` prefixed basenames such as `lt-pwd` and `/tmp/lt-pwd`, the Rust module produces `pwd` as the effective program name.
  **Traceability:** `progname.c`, `set_program_name`

- **SC-4**: The Rust rewrite exposes no required functional behavior beyond program-name derivation and storage evidenced by this module.
  **Traceability:** `progname.c`, `set_program_name`