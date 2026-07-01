# spec.md

## Title
Functional Specification: `main_root_progname.c_30` Rust Rewrite

## Metadata
- Project: `cat`
- Module: `main_root_progname.c_30`
- Category: `main_cluster`
- Source file: `progname.c`
- Primary function: `set_program_name(const char *argv0)`
- Target Rust branch: `031-main_root_progname.c_30-rust-port`
- Generation date: `2026-06-07`

## Overview
This module is responsible for deriving and recording the program name from the process invocation path supplied as `argv[0]`. Its functional role is limited to interpreting the incoming program path string and establishing the canonical program-name value used by the rest of the program.

The Rust rewrite must preserve this boundary: accept the invocation name/path, determine the program name according to the same path-handling behavior, and make that resulting name available in the same module role expected by the application.

## Feature Specification

### Feature: Program Name Initialization
The module initializes the program name from the executable name string provided by program startup.

The Rust version must:
- accept the startup program reference corresponding to `argv[0]`,
- derive the visible program name from that input,
- handle path-qualified inputs by selecting the final program component rather than the whole path,
- preserve behavior for inputs that already contain only a program name,
- establish the resulting value so the rest of the application can use it as the current program name.

### Feature: Invocation Path Normalization for Program Identity
The module distinguishes between a full invocation path and the actual executable name.

The Rust version must implement equivalent behavior for:
- absolute or relative paths,
- slash-containing invocation strings,
- plain executable names without path separators.

### Feature: Stable Module Scope of Program Name State
The module exists to set program-name state for later use by the process.

The Rust rewrite must preserve that module purpose:
- the derived program name must be retained after initialization,
- repeated accesses after initialization must observe the derived name,
- no unrelated behavior may be added beyond setting and exposing this program-name meaning within the application.

## User Scenarios & Testing

### Scenario 1: Plain executable name
A caller initializes the module with an `argv[0]` value such as `cat`.

Expected behavior:
- the stored program name becomes `cat`.

Test guidance:
- initialize with `cat`,
- verify the module records `cat` as the program name.

### Scenario 2: Relative path invocation
A caller initializes the module with a relative path such as `./cat` or `bin/cat`.

Expected behavior:
- the stored program name becomes the last path component, `cat`.

Test guidance:
- initialize with `./cat`,
- verify the program name is `cat`;
- initialize with `bin/cat`,
- verify the program name is `cat`.

### Scenario 3: Absolute path invocation
A caller initializes the module with an absolute path such as `/usr/bin/cat`.

Expected behavior:
- the stored program name becomes `cat`.

Test guidance:
- initialize with `/usr/bin/cat`,
- verify the program name is `cat`.

### Scenario 4: Re-initialization with another invocation string
The module is called again with a different invocation value.

Expected behavior:
- the module reflects the program name derived from the provided input in a manner consistent with the source module’s single responsibility of setting the program name.

Test guidance:
- initialize once with one valid program string,
- initialize again with another valid program string containing a different basename,
- verify the observed program name matches the latest effective set behavior required by the rewrite’s compatibility target.

### Scenario 5: Path-like input ending in separators
A caller provides an invocation string containing path separators in a form that still requires basename-oriented handling.

Expected behavior:
- basename extraction behavior remains consistent with the source module’s path interpretation rules.

Test guidance:
- include edge-case path strings used by the compatibility test suite,
- verify the derived name matches source-compatible results.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide functionality equivalent to `set_program_name(const char *argv0)` for initializing program-name state from the startup invocation string.
  **Traceability**: `progname.c`, `set_program_name`

- **FR-2**: When the input contains one or more path separators, the module shall derive the program name from the final path component rather than preserving the entire input path.
  **Traceability**: `progname.c`, `set_program_name`

- **FR-3**: When the input does not contain path separators, the module shall use the input itself as the program name.
  **Traceability**: `progname.c`, `set_program_name`

- **FR-4**: The module shall retain the derived program name as module-level program identity state for subsequent use by the application.
  **Traceability**: `progname.c`, `set_program_name`

- **FR-5**: The Rust rewrite shall match the source module’s externally observable basename-selection behavior for representative invocation forms used by the application: plain names, relative paths, and absolute paths.
  **Traceability**: `progname.c`, `set_program_name`

### Key Entities
- **Invocation string (`argv0`)**: The input program reference supplied at startup and consumed by the module.
- **Derived program name**: The basename-like value obtained from the invocation string and retained as the module’s result.
- **Program-name state**: The module-scoped stored identity value established by initialization.

Relationship:
- the invocation string is parsed to produce the derived program name,
- the derived program name becomes the stored program-name state used by the application.

## Success Criteria
- **SC-1**: Given a plain executable name input, the Rust module stores and exposes the same program name text as the input.
  **Traceability**: `set_program_name`

- **SC-2**: Given relative and absolute path inputs, the Rust module stores and exposes only the final path component, matching source behavior.
  **Traceability**: `set_program_name`

- **SC-3**: Compatibility tests covering `cat`, `./cat`, `bin/cat`, and `/usr/bin/cat` all pass with the expected derived name `cat`.
  **Traceability**: `set_program_name`

- **SC-4**: The Rust rewrite does not require callers to provide any information beyond the startup invocation string to establish program-name state.
  **Traceability**: `set_program_name`

- **SC-5**: The rewritten module remains functionally limited to program-name derivation and storage, with no additional externally visible responsibilities introduced in this module.
  **Traceability**: `progname.c`, `set_program_name`