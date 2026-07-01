# spec.md

## Overview

This module is responsible for deriving and recording the program name from the process invocation string (`argv[0]`). Its functional boundary is limited to accepting a single input path-like string and setting the process-visible program name according to the basename rules embodied by `set_program_name`.

The Rust rewrite must preserve this behavior for the `cat` project branch `031-main_root_progname.c_30-rust-port`, with equivalent externally observable results for valid usage within program startup.

## Scope

In scope:

- Accepting the process invocation string provided by startup code.
- Determining the program name component from that string.
- Updating the module’s program-name state used by the rest of the program.

Out of scope:

- Command-line parsing beyond the invocation string.
- Formatting diagnostics.
- Process execution, environment handling, or path resolution.
- Any additional public API not evidenced by the source module.

## Feature Specification

### Feature: Program name initialization

The module provides one initialization behavior: given the startup program identifier string, it extracts the effective program name and stores it for later use by the application.

The Rust version must implement the same functional behavior:

- Accept a single string corresponding to `argv[0]`.
- Interpret that string as a program invocation path or name.
- Derive the program name as the last path component rather than preserving directory prefixes.
- Make the derived name available as the program name used by the rest of the application.

### Behavioral notes

Observed module intent, as evidenced by `set_program_name`, is limited to program-name setup during startup. The Rust rewrite must therefore preserve these externally meaningful behaviors:

- If the input already contains only a simple executable name, that name becomes the stored program name.
- If the input contains path separators, only the final component becomes the stored program name.
- The module must behave as startup support code, suitable for invocation early in program initialization before diagnostics or user-facing reporting depend on the program name.

## User Scenarios & Testing

### Scenario 1: Invocation by simple name

A user runs the program from `PATH` as:

```text
cat
```

Startup passes `"cat"` as the invocation string.

Expected result:

- The module records the program name as `cat`.

Testing guidance:

- Invoke the Rust function with a simple name.
- Verify the stored program name equals the input name unchanged.

### Scenario 2: Invocation by absolute path

A user runs the program as:

```text
/usr/bin/cat
```

Expected result:

- The module records the program name as `cat`, not `/usr/bin/cat`.

Testing guidance:

- Invoke the Rust function with an absolute path string.
- Verify only the final path component is retained.

### Scenario 3: Invocation by relative path

A user runs the program as:

```text
./src/cat
```

Expected result:

- The module records the program name as `cat`.

Testing guidance:

- Invoke the Rust function with a relative path containing directories.
- Verify directory components are removed.

### Scenario 4: Startup integration

Program startup initializes the program name before any later code needs it for messages or identification.

Expected result:

- After initialization, dependent code observes the derived program name rather than the original path-form invocation string.

Testing guidance:

- Perform startup-style initialization in integration tests.
- Verify later program-name consumers observe the initialized basename form.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide behavior equivalent to `set_program_name(const char *argv0)` for initializing program-name state from the startup invocation string.
  **Traceability:** `progname.c`, `set_program_name`

- **FR-2**: When the invocation string contains one or more directory components, the module shall derive the stored program name from the final path component only.
  **Traceability:** `progname.c`, `set_program_name`

- **FR-3**: When the invocation string is already a simple name with no directory component, the module shall store that name as the program name without adding or removing characters.
  **Traceability:** `progname.c`, `set_program_name`

- **FR-4**: The module shall support use during early program startup so that later application behavior can rely on the initialized program name.
  **Traceability:** `progname.c`, `set_program_name`

### Key Entities

- **Invocation string (`argv0`)**: The startup-provided string identifying how the program was invoked. It is the sole input to the module’s defined behavior.
  **Traceability:** `set_program_name(const char *argv0)`

- **Program name state**: The process-wide or application-visible stored name established by the module from the invocation string. This is the module’s output entity, though its storage representation is not specified here.
  **Traceability:** `progname.c`, `set_program_name`

### Entity Relationships

- The invocation string is consumed by the initialization behavior.
- The initialization behavior derives a basename-like program name from that input.
- The derived name becomes the program name state observed by the rest of the program.

## Success Criteria

- **SC-1**: Given a simple invocation string such as `cat`, the Rust rewrite stores and exposes `cat` as the program name.
  **Traceability:** `progname.c`, `set_program_name`

- **SC-2**: Given a path-form invocation string such as `/usr/bin/cat` or `./src/cat`, the Rust rewrite stores and exposes only `cat` as the program name.
  **Traceability:** `progname.c`, `set_program_name`

- **SC-3**: The Rust rewrite can be invoked during startup without requiring unrelated module initialization or command-line parsing.
  **Traceability:** `progname.c`, `set_program_name`

- **SC-4**: Integration tests demonstrate that code observing the program name after initialization receives the derived final component rather than the original path string.
  **Traceability:** `progname.c`, `set_program_name`

## Non-Goals

The Rust rewrite must not introduce or require:

- Additional user-facing features beyond program-name initialization.
- New public APIs beyond what is needed to preserve the evidenced module behavior.
- Path normalization, filesystem access, or executable discovery.
- Broader process metadata management.