# spec.md

## Title
Rust Functional Specification for `module_doc_whoami.c_06`

## Metadata
- Project: `cflow-new`
- Module: `module_doc_whoami.c_06`
- Category: `module_cluster`
- Source file: `doc/whoami.c`
- Primary function: `who_am_i(void) -> int`
- Rust branch: `006-module_doc_whoami.c_06-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides a single behavior: identify the current user through the system account database and emit a textual result to standard output.

The Rust rewrite must preserve the observable behavior of the original module at the functional boundary:
- it performs a lookup for the current effective or active user identity through the system user account mechanism represented in C by `struct passwd`,
- it prints user-identifying text when that lookup succeeds,
- it returns an integer status result.

No broader responsibilities are evidenced for this module.

## Feature Specification

### Summary
The module resolves the current process user to a password-database entry and reports that identity in human-readable form.

### Required Rust Behavior
The Rust version must implement the same functional capability as `who_am_i`:

1. Obtain the current user identity from the operating environment using the platform user-account source corresponding to the C usage of `struct passwd`.
2. Determine whether a valid user record is available.
3. On success, print the resolved user-identifying value to standard output.
4. Complete with an integer status code suitable for distinguishing success from failure.

### Functional Boundary
Included:
- current-user lookup,
- use of system user-record data,
- user-visible output,
- status return.

Excluded because not evidenced:
- parsing command-line arguments,
- accepting external input,
- modifying user records,
- returning structured user metadata to callers,
- multiple output formats,
- configuration, caching, or persistence.

## User Scenarios & Testing

### Scenario 1: Current user can be resolved
A caller invokes the module behavior in a normal environment where the current user exists in the system user database.

Expected result:
- the module prints the resolved user name or equivalent user-identifying text,
- the function returns a success status code.

Test approach:
- execute the Rust implementation in an environment with a valid current user,
- verify that standard output contains non-empty user-identifying text derived from the resolved user record,
- verify that the returned status indicates success.

### Scenario 2: Current user cannot be resolved
A caller invokes the module behavior in an environment where the user lookup does not produce a valid `passwd`-equivalent record.

Expected result:
- the module does not behave as though a valid user was found,
- the function returns a failure status code.

Test approach:
- simulate or inject a failed user-record lookup in the Rust test environment,
- verify that the success-path user output is not produced,
- verify that the returned status indicates failure.

### Scenario 3: Output is intended for direct human inspection
A caller uses the module as a simple reporting utility and relies on terminal-visible output.

Expected result:
- the module emits user-facing text to standard output during successful execution.

Test approach:
- capture standard output during a successful call,
- verify that output is produced and corresponds to the resolved user identity.

## Requirements

### Functional Requirements

#### FR-1: Current user resolution
The module shall resolve the current process user via the host system’s user account information source, as evidenced by the use of `struct passwd` in `doc/whoami.c`.

Traceability:
- `doc/whoami.c`
- `who_am_i`

#### FR-2: User-record dependent behavior
The module shall branch behavior based on whether a valid user record is obtained.

Traceability:
- `doc/whoami.c`
- `who_am_i`
- referenced type `struct passwd`

#### FR-3: Human-readable identity output
When a valid user record is obtained, the module shall print user-identifying text to standard output.

Traceability:
- `doc/whoami.c`
- `who_am_i`

#### FR-4: Integer completion status
The module shall return an integer status from `who_am_i` that communicates the outcome of the operation.

Traceability:
- `who_am_i(void) -> int`

### Key Entities

#### `passwd` / system user record
A system-provided user account record, referenced in the source as `struct passwd`. This entity represents the resolved current user and is the basis for the module’s success-path output.

Relationship:
- `who_am_i` depends on the availability of a `passwd`-equivalent user record to produce output.

#### Operation result
The integer return value of `who_am_i`, representing whether the module successfully resolved and reported the current user.

Relationship:
- the operation result reflects the outcome of processing the system user record.

## Success Criteria

### SC-1: Success-path output
When executed in an environment where the current user can be resolved, the Rust implementation prints user-identifying text to standard output.

Traceability:
- `doc/whoami.c`
- `who_am_i`
- `struct passwd`

### SC-2: Success-path status
When current-user resolution succeeds, the Rust implementation returns a success integer status.

Traceability:
- `who_am_i(void) -> int`

### SC-3: Failure-path status
When current-user resolution does not yield a valid user record, the Rust implementation returns a failure integer status.

Traceability:
- `who_am_i(void) -> int`
- `struct passwd`

### SC-4: No unsupported feature expansion
The Rust implementation remains limited to the module’s evidenced responsibility of resolving and reporting the current user, without introducing additional externally visible module behavior not present in `doc/whoami.c`.

Traceability:
- `doc/whoami.c`
- `who_am_i`