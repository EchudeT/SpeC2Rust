# spec.md

## Title

Functional Specification: `main_root_progname.c_21`

## Metadata

- Project: `pwd`
- Module: `main_root_progname.c_21`
- Category: `main_cluster`
- Source file: `progname.c`
- Primary function: `set_program_name(const char *argv0)`
- Rust port branch: `021-main_root_progname.c_21-rust-port`
- Generation date: `2026-06-07`

## Overview

This module provides program-name normalization for the process entry path string supplied by the caller. Its purpose is to derive and store the executable's program name from `argv[0]` so that the rest of the program can refer to a consistent name rather than the full invocation path.

The Rust rewrite must preserve the same functional boundary: accept a process invocation string, derive the program name component from it, and make that normalized program name available in the same role this module serves in the original project.

## Feature Specification

### Feature: Program name extraction and registration

The module accepts the command invocation string and determines the program name portion to be used by the application.

Behavior that must be implemented by the Rust version:

- Accept an input corresponding to `argv[0]`.
- Derive the final path component rather than preserving directory prefixes.
- Handle ordinary invocation forms where `argv[0]` is:
  - a bare executable name,
  - a relative path,
  - an absolute path.
- Store or register the resulting program name for later use by the program in the same functional role as the C module.
- Preserve the module's role as an initialization helper used from program startup logic.

### Feature boundaries

The Rust version must not introduce functionality not evidenced by this module analysis. In particular, this specification does not require:

- argument parsing beyond `argv[0]`,
- command dispatch,
- environment management,
- error reporting policy beyond what is needed to preserve observed behavior,
- additional public APIs unrelated to setting the program name.

## User Scenarios & Testing

### Scenario 1: Invoked by bare executable name

A caller initializes program state with `argv[0] = "pwd"`.

Expected result:

- The stored program name is `pwd`.

Test guidance:

- Call the Rust equivalent of program-name initialization with `pwd`.
- Verify the module records or exposes `pwd` as the normalized name.

### Scenario 2: Invoked with a relative path

A caller initializes program state with `argv[0] = "./pwd"` or `bin/pwd`.

Expected result:

- The stored program name is `pwd`.

Test guidance:

- Provide representative relative-path inputs.
- Verify directory prefixes are removed and only the last path component remains.

### Scenario 3: Invoked with an absolute path

A caller initializes program state with `argv[0] = "/usr/bin/pwd"`.

Expected result:

- The stored program name is `pwd`.

Test guidance:

- Provide an absolute path input.
- Verify the normalized program name is the final component.

### Scenario 4: Startup integration

The main program invokes this module during startup before later code needs the program name.

Expected result:

- After initialization, the normalized program name is available for downstream use consistent with the original module's purpose.

Test guidance:

- Exercise the Rust port in the startup sequence used by the rewritten program.
- Verify initialization occurs successfully and the name is available afterward.

## Requirements

### Functional Requirements

#### FR-1: Accept process invocation name

The module shall accept a caller-supplied string corresponding to the process invocation name (`argv[0]`).

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-2: Normalize to the executable name component

The module shall derive the program name from the input by selecting the executable name component instead of retaining any directory path prefix.

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-3: Support common invocation path forms

The module shall perform this normalization for bare names, relative paths, and absolute paths.

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-4: Establish program name for later use

The module shall register or store the normalized program name so it can serve the rest of the program after initialization.

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`

#### FR-5: Serve as startup-time initialization logic

The module shall remain suitable for use during main-program initialization.

Traceability:

- Module category: `main_cluster`
- `progname.c`
- `set_program_name(const char *argv0)`

### Key Entities

#### Entity: Invocation name input

- Represents the caller-provided `argv[0]` string.
- Relationship: It is the source value from which the module derives the normalized program name.

Traceability:

- `set_program_name(const char *argv0)`

#### Entity: Normalized program name

- Represents the final executable name component extracted from the invocation name.
- Relationship: It is derived from the invocation name input and retained for later program use.

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`

## Success Criteria

### SC-1: Correct basename behavior

For input strings representing bare names, relative paths, and absolute paths, the Rust port produces the expected final executable-name component.

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`

### SC-2: Startup usability preserved

The Rust port can be invoked during program startup in the same role as the original module and completes initialization of the program name successfully.

Traceability:

- Module category: `main_cluster`
- `progname.c`
- `set_program_name(const char *argv0)`

### SC-3: Program name remains available after initialization

After the Rust module processes the invocation string, the normalized program name is available for subsequent program logic in the same functional capacity as in the C module.

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`

### SC-4: No unsupported feature expansion

The Rust rewrite limits itself to the evidenced responsibility of setting and normalizing the program name and does not require unrelated new module capabilities.

Traceability:

- `progname.c`
- `set_program_name(const char *argv0)`