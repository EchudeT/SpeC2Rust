# spec.md

## Title
Rust Functional Specification for `main_root_xgetcwd.c_27`

## Metadata
- Project: `pwd`
- Module: `main_root_xgetcwd.c_27`
- Category: `main_cluster`
- Source file: `xgetcwd.c`
- Primary function: `xgetcwd`
- Rust branch: `027-main_root_xgetcwd.c_27-rust-port`
- Generation date: 2026-06-07

## Overview
This module provides a single function that obtains the process's current working directory and returns it as a dynamically allocated string.

The Rust rewrite must preserve this functional role: provide module behavior equivalent to `xgetcwd`, producing the current working directory as an owned string result for use by the surrounding `pwd` program logic.

## Feature Specification

### Feature: Retrieve current working directory as owned path text
The module must obtain the current working directory of the calling process and make that path available as newly owned return data.

Observed source evidence for this feature is the presence of the exported function:

- `xgetcwd(void) -> char *`

### Required Rust behavior
The Rust version must implement behavior equivalent to the C module:

- Query the calling process's current working directory.
- Return the result in owned form so the caller receives independent path storage.
- Preserve the module's role as a small path-acquisition utility used by higher-level program flow.

### Functional boundary
This module is limited to current-working-directory retrieval. It is not specified to:
- change directories,
- normalize paths beyond what the underlying current-directory query returns,
- compare paths,
- cache paths,
- expose additional public operations.

## User Scenarios & Testing

### Scenario 1: Main program needs the current directory for output
A caller in the `pwd` program requests the current working directory through this module.

Expected result:
- The module returns the current working directory as owned path text.
- The returned value corresponds to the process state at the time of the call.

Test approach:
- Set the process working directory to a known location.
- Call the Rust equivalent of `xgetcwd`.
- Verify the returned path text matches the actual current working directory.

### Scenario 2: Caller uses the returned value independently
A caller obtains the current working directory and then uses the returned path after the function has returned.

Expected result:
- The returned value remains valid as independent owned data after the call completes.

Test approach:
- Call the Rust equivalent of `xgetcwd`.
- Store the returned value and use it later in the same execution path.
- Verify it still contains the expected directory path.

### Scenario 3: Retrieval reflects directory changes across calls
The process working directory changes between two calls.

Expected result:
- Each call reflects the current working directory at the time of that call.
- The module does not reuse stale path data from an earlier call.

Test approach:
- Call the Rust equivalent once in directory A.
- Change to directory B.
- Call it again.
- Verify the two results correspond to A and B respectively.

### Scenario 4: Retrieval failure is propagated
The current working directory cannot be obtained by the underlying environment or OS interaction.

Expected result:
- The Rust version reports failure to the caller rather than fabricating a path.

Test approach:
- Exercise an environment where current-directory retrieval fails, if practical in integration testing.
- Verify the Rust API returns an error outcome consistent with module failure behavior.

## Requirements

### Functional Requirements

#### FR-1: Current directory retrieval
The module shall retrieve the current working directory of the calling process.

Traceability:
- `xgetcwd.c`
- `xgetcwd`

#### FR-2: Owned return value
The module shall return the retrieved path as newly owned data rather than as borrowed or shared internal storage.

Traceability:
- `xgetcwd.c`
- `xgetcwd`
- C signature returns `char *`

#### FR-3: Per-call freshness
The module shall determine the current working directory on each call so that the result reflects process state at call time.

Traceability:
- `xgetcwd.c`
- `xgetcwd`

#### FR-4: Failure signaling
If the current working directory cannot be obtained, the module shall signal failure to the caller.

Traceability:
- `xgetcwd.c`
- `xgetcwd`

### Key Entities

#### Entity: Current working directory path
The core data entity is the textual path representing the process's current working directory.

Relationship to module behavior:
- It is produced by the module's only function.
- It is returned to the caller in owned form.

#### Entity: Function result
The module's function result is the carrier for either:
- a successfully retrieved owned path value, or
- a failure outcome when retrieval is not possible.

Relationship to path entity:
- On success, the result contains the current working directory path.
- On failure, the result indicates that no valid path was produced.

## Success Criteria

### SC-1: Correct directory value
Given a process running in a known working directory, the Rust module returns that directory path correctly.

Traceability:
- `xgetcwd`

### SC-2: Owned result semantics
The Rust module returns the current working directory in an owned result form that remains usable after the function returns.

Traceability:
- `xgetcwd`
- C return type `char *`

### SC-3: Updated results after directory change
When the process working directory changes between calls, subsequent calls return the updated directory rather than the earlier one.

Traceability:
- `xgetcwd`

### SC-4: Explicit failure on retrieval error
When the underlying current-directory lookup fails, the Rust module returns an error or equivalent failure outcome and does not return a fabricated path string.

Traceability:
- `xgetcwd`

## Out of Scope
The Rust rewrite for this module is not required to provide:
- directory-changing operations,
- path canonicalization beyond current-directory retrieval,
- symlink resolution guarantees beyond underlying platform behavior,
- extra public APIs not evidenced by the source module.