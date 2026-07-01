# spec.md

## Overview

This module provides program-name normalization for the process based on the `argv[0]` string passed at startup. The analyzed C module exposes a single behavior: accepting the startup program path/name and deriving the canonical program name values used by the wider project.

The Rust rewrite must preserve this behavior for the module represented by `gnu/progname.c`, specifically the functionality evidenced by `set_program_name(const char *argv0)`.

## Scope

In scope:

- Accepting a program invocation string equivalent to `argv[0]`
- Deriving the stored program name from that string
- Handling path-prefixed invocation names
- Handling platform-specific executable suffix stripping as evidenced by GNU `progname` behavior
- Rejecting invalid input in the same behavioral class as the source module

Out of scope:

- Argument parsing beyond `argv[0]`
- Process launching or OS-level executable discovery
- Any new public API beyond what is required to preserve the module behavior
- Unrelated global state management not evidenced by this module

## Feature Specification

### Summary

The module is responsible for initializing the process program-name state from the startup name string. It extracts the basename portion from the provided invocation path and normalizes specific executable-name suffix forms before publishing the resulting name to the rest of the program.

### Required Behavior

The Rust version must implement behavior equivalent to:

- Receiving a non-null program invocation string
- Locating the program name portion when the input includes directory separators
- Removing recognized executable suffix text where the C module does so
- Making the normalized program name available as the module’s program-name state
- Preserving the resulting name exactly when no path trimming or suffix trimming rule applies

### Behavioral Notes

The specification is limited to what is evidenced by the module analysis:

- The module’s main responsibility is initialization of program-name state, not repeated transformation utilities.
- The observable output is the normalized program name value stored by the module for later use by the application.
- Input validity matters: the source function is not specified as accepting null safely, so the Rust rewrite must preserve the same contract boundary rather than inventing broader acceptance semantics.

## User Scenarios & Testing

### Scenario 1: Invocation name without a path

A program starts with `argv[0]` equal to a simple executable name such as `cflow`.

Expected result:

- The stored program name is `cflow`.

Tests:

- Provide a simple name with no separators.
- Verify the module stores exactly that name if no suffix rule applies.

### Scenario 2: Invocation name with directory components

A program starts with `argv[0]` equal to a path such as `/usr/bin/cflow`.

Expected result:

- The stored program name is the basename component, `cflow`.

Tests:

- Provide a Unix-style absolute path.
- Provide a relative path with one or more directory components.
- Verify only the final path component is retained.

### Scenario 3: Invocation name with executable suffix recognized by the source behavior

A program starts with an invocation string whose final path component includes a removable executable suffix handled by the source module.

Expected result:

- The stored program name omits that recognized suffix.

Tests:

- Provide an input whose basename ends with the recognized suffix.
- Verify the stored program name matches the basename with that suffix removed.
- Provide an input whose text does not match the removable suffix rule.
- Verify the name is not altered beyond basename extraction.

### Scenario 4: Invalid startup name contract violation

The caller supplies an invalid input outside the function contract, such as a null-equivalent value.

Expected result:

- The Rust rewrite must preserve the source contract boundary and must not claim support for such input.

Tests:

- Contract-level tests or API-level documentation checks confirm null-equivalent input is not accepted as valid supported usage.

## Requirements

### Functional Requirements

#### FR-1: Program name initialization

The module shall accept a startup program-name string corresponding to `argv[0]` and initialize module-visible program-name state from it.

Traceability:

- `gnu/progname.c`
- `set_program_name`

#### FR-2: Basename extraction

When the supplied startup string includes one or more directory components, the module shall derive the program name from the final path component rather than from the full path.

Traceability:

- `gnu/progname.c`
- `set_program_name`

#### FR-3: Executable suffix normalization

When the final path component ends with an executable suffix recognized by the source module’s behavior, the module shall remove that suffix from the stored program name.

Traceability:

- `gnu/progname.c`
- `set_program_name`

#### FR-4: No unnecessary modification

When the supplied startup string requires no path trimming and no recognized suffix trimming, the module shall preserve the name text as the stored program name.

Traceability:

- `gnu/progname.c`
- `set_program_name`

#### FR-5: Contract-preserving input handling

The Rust module shall preserve the original function contract boundary for the startup string input and shall not specify support for invalid null-equivalent input not evidenced as supported by the source module.

Traceability:

- `gnu/progname.c`
- `set_program_name`

### Key Entities

#### Program invocation string

The input startup string corresponding to `argv[0]`. It is the source value from which the module derives the normalized program name.

#### Normalized program name state

The module-maintained program name value produced after basename extraction and any recognized suffix removal. This is the module’s primary output state.

#### Relationship

The normalized program name state is derived from the program invocation string by applying the module’s extraction and normalization rules.

## Success Criteria

1. Given a startup string with no directory separators and no removable recognized suffix, the Rust module stores the same name unchanged.
2. Given a startup string containing directory components, the Rust module stores only the final path component.
3. Given a startup string whose final path component includes a removable recognized suffix handled by the source behavior, the Rust module stores the component with that suffix removed.
4. For inputs that combine directory components and a removable recognized suffix, the Rust module applies both behaviors correctly in sequence, producing the normalized final program name.
5. The Rust rewrite exposes no documented supported behavior beyond the source module’s evidenced contract for `set_program_name`.
6. All supported behaviors above are covered by automated tests traceable to `gnu/progname.c` and `set_program_name`.