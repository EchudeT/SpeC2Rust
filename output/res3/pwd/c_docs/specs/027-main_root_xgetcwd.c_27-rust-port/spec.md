# spec.md

## Title

Functional Specification: `main_root_xgetcwd.c_27`

## Document Metadata

- Project: `pwd`
- Module: `main_root_xgetcwd.c_27`
- Category: `main_cluster`
- Source file: `xgetcwd.c`
- Primary function: `xgetcwd`
- Rust branch target: `027-main_root_xgetcwd.c_27-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides a single utility function that obtains the process's current working directory and returns it as a newly allocated string.

The Rust rewrite must preserve the same functional role: provide module behavior equivalent to `xgetcwd`, namely retrieving the current working directory for the calling process and returning an owned path string on success, with failure represented explicitly.

## Feature Specification

### Feature Summary

The module exposes one operation:

- Retrieve the current working directory of the running process.
- Produce the result as an owned string value suitable for use by the caller after the function returns.
- Indicate failure when the current working directory cannot be obtained.

### Required Rust Behavior

The Rust version must implement behavior equivalent to the C module's `xgetcwd` function:

1. Query the current working directory of the running process.
2. On success, return an owned path value containing that directory.
3. On failure, return an error/absence result rather than a fabricated path.
4. The returned successful value must not depend on caller-provided buffers.

### Functional Boundary

This module is limited to current-working-directory retrieval. It does not define higher-level path formatting, path normalization, directory changes, output printing, or command-line behavior.

## User Scenarios & Testing

### Scenario 1: Current directory retrieval succeeds

A caller needs the absolute or system-reported current working directory for later display or internal use.

- Given the process has a valid current working directory
- When the caller invokes the module function
- Then the function returns success
- And the returned value contains the current working directory as an owned string/path value

**Test evidence target:** Calling the Rust function in a normal process context returns a non-error result whose contents match the process current directory.

### Scenario 2: Caller owns the returned value

A caller needs to keep using the returned directory value after the function call completes.

- Given the caller invokes the module function successfully
- When the function returns
- Then the caller can continue to use the returned value without relying on external temporary storage

**Test evidence target:** The returned value can be moved, stored, or compared by the caller after the call completes.

### Scenario 3: Retrieval failure is reported

A caller must detect when the current working directory cannot be determined.

- Given the environment causes current-directory lookup to fail
- When the caller invokes the module function
- Then the function returns failure
- And it does not return a placeholder or guessed path

**Test evidence target:** Under an induced current-directory lookup failure, the Rust function returns an error or empty result rather than success.

## Requirements

### Functional Requirements

#### FR-1: Current working directory lookup

The module shall provide a function equivalent to `xgetcwd` that retrieves the current working directory for the calling process.

**Traceability:** `xgetcwd.c`, function `xgetcwd`

#### FR-2: Owned result on success

When current working directory retrieval succeeds, the module shall return the directory as an owned result value.

**Traceability:** `xgetcwd.c`, function `xgetcwd`

#### FR-3: Explicit failure signaling

When current working directory retrieval fails, the module shall signal failure explicitly rather than returning an invented directory value.

**Traceability:** `xgetcwd.c`, function `xgetcwd`

#### FR-4: No caller-supplied output buffer requirement

The module shall perform current working directory retrieval without requiring the caller to provide a destination buffer.

**Traceability:** `xgetcwd.c`, function `xgetcwd`

### Key Entities

#### Entity: Current working directory result

The central entity of this module is the function result produced by `xgetcwd`:

- On success: an owned path string/value representing the process current working directory
- On failure: an explicit failure result

#### Relationship

- The module function produces exactly one result per call.
- The caller consumes and owns the successful returned value.
- No additional module-defined structs or persistent data entities are evidenced for this module.

## Success Criteria

### SC-1: Successful retrieval behavior

In a process with a valid current working directory, invoking the Rust replacement for `xgetcwd` returns success and yields the current working directory.

**Traceability:** FR-1, FR-2

### SC-2: Failure behavior

In a context where current working directory retrieval fails, invoking the Rust replacement returns failure and does not produce a fabricated path.

**Traceability:** FR-3

### SC-3: Owned return behavior

The Rust replacement returns its successful result as an owned value that remains usable by the caller after the function returns, without any caller-provided buffer.

**Traceability:** FR-2, FR-4

### SC-4: Scope conformance

The Rust module implements only current-working-directory retrieval behavior evidenced by `xgetcwd` and does not require unrelated higher-level features to satisfy the port.

**Traceability:** `xgetcwd.c`, function `xgetcwd`