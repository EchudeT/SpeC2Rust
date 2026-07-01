# spec.md

## Overview

This module provides program-name initialization from the process argument vector. It accepts the `argv[0]` string, derives the executable name forms used by the program, and stores them in module-level state for later use by the rest of the application.

The Rust rewrite must preserve the observable behavior of this module: given an input program path or name, it must compute and expose the same name values that the C module establishes, including path stripping and the special handling for libtool-style launcher prefixes evidenced by the source function.

## Scope

In scope:

- Initializing module state from a caller-provided program name string.
- Deriving the effective program name from a path-like `argv[0]`.
- Deriving the full invocation string retained by the module.
- Applying the special basename adjustment performed for libtool-generated `lt-` prefixes.

Out of scope:

- Argument parsing beyond `argv[0]`.
- Process launching, environment inspection, or executable discovery.
- Any new API surface not required to represent the behavior of `set_program_name`.

## Feature Specification

### Feature: Program name initialization

The module initializes program name state from a single input string representing the process invocation name.

Behavior the Rust version must implement:

- Accept a program name input equivalent to C `argv[0]`.
- Preserve the original input as the module's full program-name form.
- Derive a basename form by removing any directory components.
- If the derived basename begins with the libtool launcher prefix `lt-`, advance past that prefix for the short program-name form.
- Maintain the derived values as module state so that other code can use them after initialization.
- Support repeated calls deterministically according to the latest provided input, since the function is a state-setting routine.

### Feature: Invalid input handling

The source function evidences guarded handling for unusable input.

Behavior the Rust version must implement:

- Reject null-equivalent or empty program-name input rather than silently producing meaningless state.
- Preserve the module's defined behavior for invalid input as established by the source semantics: the call is not a recoverable parsing operation and must not continue as if initialization succeeded.

## User Scenarios & Testing

### Scenario 1: Invocation with a simple executable name

A caller initializes the module with a plain executable name such as `cflow`.

Expected result:

- Full program name is `cflow`.
- Short/basename program name is `cflow`.

Test focus:

- No path stripping occurs when no directory separator is present.
- No `lt-` adjustment occurs when the basename does not start with that prefix.

### Scenario 2: Invocation with a filesystem path

A caller initializes the module with a path such as `/usr/bin/cflow`.

Expected result:

- Full program name remains `/usr/bin/cflow`.
- Short/basename program name becomes `cflow`.

Test focus:

- Directory components are removed from the short name.
- Full name remains the original input.

### Scenario 3: Invocation through a libtool wrapper name

A caller initializes the module with a path or name whose basename starts with `lt-`, such as `./lt-cflow`.

Expected result:

- Full program name remains `./lt-cflow`.
- Short/basename program name becomes `cflow`.

Test focus:

- Prefix removal is applied only to the basename-derived form.
- Path stripping happens before checking the `lt-` prefix.

### Scenario 4: Re-initialization with a different program name

A caller invokes initialization more than once during process lifetime.

Expected result:

- Module state reflects the most recent input after each call.

Test focus:

- Later calls replace earlier derived values.
- No stale basename remains from a prior initialization.

### Scenario 5: Invalid initialization input

A caller provides null-equivalent or empty input.

Expected result:

- The module does not establish normal initialized state from invalid input.

Test focus:

- Invalid input does not produce a misleading basename or full name.
- Failure behavior is consistent with the source module's non-recoverable contract.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a way to initialize program-name state from one caller-supplied invocation string, corresponding to `set_program_name`.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **FR-2**: On successful initialization, the module shall retain the original supplied invocation string as the full program-name value.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **FR-3**: On successful initialization, the module shall derive a basename program-name value by removing leading directory components from the supplied invocation string.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **FR-4**: If the derived basename begins with `lt-`, the module shall derive the effective short program name by omitting that prefix.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **FR-5**: The module shall make the derived program-name state available after initialization for use by the rest of the program.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **FR-6**: If initialization is invoked again, the module shall update its program-name state to match the most recent successful input.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **FR-7**: The module shall not treat null-equivalent or empty input as a successful initialization.
  **Traceability:** `gnu/progname.c`, `set_program_name`

### Key Entities

- **Program invocation input**: The caller-supplied string corresponding to `argv[0]`. It is the only input used to initialize this module's state.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **Full program name**: The stored form representing the original invocation string.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **Basename / effective program name**: The stored form derived from the invocation input by removing directory components and, when applicable, removing a leading `lt-` wrapper prefix.
  **Traceability:** `gnu/progname.c`, `set_program_name`

Relationship:

- The full program name and basename/effective program name are both derived from the same invocation input.
- The basename/effective program name is a transformation of the full program name's last path component.

## Success Criteria

- **SC-1**: Given input `cflow`, initialization results in full program name `cflow` and basename/effective program name `cflow`.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **SC-2**: Given input `/usr/bin/cflow`, initialization results in full program name `/usr/bin/cflow` and basename/effective program name `cflow`.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **SC-3**: Given input `./lt-cflow`, initialization results in full program name `./lt-cflow` and basename/effective program name `cflow`.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **SC-4**: Given two successive successful initializations with different inputs, the exposed module state after the second call matches only the second input's derived results.
  **Traceability:** `gnu/progname.c`, `set_program_name`

- **SC-5**: Given null-equivalent or empty input, the Rust module does not report a successful initialized state with derived names.
  **Traceability:** `gnu/progname.c`, `set_program_name`

## Acceptance Notes

The Rust port should be accepted when it reproduces the C module's program-name derivation semantics evidenced by `set_program_name`, without adding unrelated capabilities or changing the module from a simple initialization-and-state role into a broader process metadata service.