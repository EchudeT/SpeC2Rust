# spec.md

## Title

Functional Specification for `module_gnu_stat_04` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_gnu_stat_04`
- Category: `module_cluster`
- Rust branch: `010-module_gnu_stat_04-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides compatibility-oriented file descriptor and file status behavior centered on `open`, `fcntl`, `dup2`, `stat`, and `fstat`, with platform-specific handling for Windows stat-by-handle behavior.

The Rust rewrite must preserve the observable behavior of the C module where this module:
- opens files while validating flag-dependent arguments and resulting file type behavior,
- duplicates file descriptors to requested descriptor numbers,
- performs `fcntl` operations through a compatibility wrapper,
- obtains file status from a path or file descriptor,
- normalizes file status results for special cases that the module explicitly handles,
- and on Windows, derives `stat` information from an OS handle.

The specification is limited to behavior evidenced by:
- `gnu/dup2.c`
- `gnu/fcntl.c`
- `gnu/fstat.c`
- `gnu/open.c`
- `gnu/stat-w32.c`
- `gnu/stat.c`

## Feature Specification

### Summary

The Rust version must implement a compatibility layer for file-opening and file-status operations that matches the functional boundary of the analyzed C module.

### Supported Functional Surface

The module must support these functional behaviors:

1. **Open with compatibility checks**
   - Accept a path plus open flags.
   - Accept an additional mode argument when the supplied flags require one.
   - Perform file opening in a way consistent with the module’s compatibility role.
   - Apply post-open validation using file status when the module’s logic requires confirmation of the opened object.

2. **Descriptor duplication to a requested target descriptor**
   - Duplicate an existing file descriptor onto a requested descriptor number.
   - Return success or failure consistent with operating-system outcome and module compatibility behavior.

3. **`fcntl` compatibility dispatch**
   - Accept a file descriptor, an action code, and an optional action argument.
   - Perform the requested descriptor control operation.
   - Preserve action-sensitive behavior, including cases where status inspection is part of compatibility handling.

4. **File status by descriptor**
   - Obtain `stat` information for an already-open file descriptor.
   - Use the platform’s original descriptor-status behavior as the base operation.
   - Apply any module-defined result adjustment before returning to the caller.

5. **File status by path**
   - Obtain `stat` information for a filesystem path.
   - Use the platform’s original path-status behavior as the base operation.

6. **Windows file status by handle**
   - On Windows-specific builds, obtain `stat` information from an OS file handle and optional path context.
   - Populate the standard file status structure from handle-derived metadata.
   - Support time fields represented with `timespec` granularity where exposed by the module.

### Out of Scope

The Rust port must not claim or introduce:
- new public APIs beyond the evidenced functional surface,
- persistence, serialization, or recovery behavior,
- thread-safety guarantees not evidenced by the module,
- benchmark or performance requirements,
- behavior unrelated to file opening, descriptor control, duplication, or stat/fstat compatibility.

## User Scenarios & Testing

### Scenario 1: Opening a file with flags that require a mode argument

A caller opens a filesystem path using flags that create a file and therefore require a mode parameter.

**Expected behavior**
- The operation accepts the extra mode argument.
- The file is opened successfully when the OS permits it.
- Failure is reported when the path or flags are invalid under the underlying platform rules.

**Relevant evidence**
- `gnu/open.c`, `open`

**Test focus**
- Open with creation flags and a valid mode.
- Open with non-creation flags and no mode.
- Verify error propagation for invalid path or flag combinations.

### Scenario 2: Opening a path and validating the opened object with stat data

A caller opens a path whose resulting object type may need validation after opening.

**Expected behavior**
- The module performs the open operation.
- Where the module’s compatibility logic requires it, file status is consulted after open.
- The operation returns success only if the resulting state matches the module’s accepted behavior.

**Relevant evidence**
- `gnu/open.c`, `open`
- `struct stat` usage in `gnu/open.c`

**Test focus**
- Open an existing regular file.
- Open an existing directory when platform semantics allow or reject it.
- Verify that any module-defined post-open status checks affect the final result.

### Scenario 3: Duplicating a descriptor onto a requested descriptor number

A caller has an open file descriptor and wants it duplicated to a specific descriptor number.

**Expected behavior**
- The destination descriptor becomes a duplicate of the source on success.
- Existing destination state is handled according to operating-system duplication semantics.
- Failure is returned for invalid source or invalid target descriptors.

**Relevant evidence**
- `gnu/dup2.c`, `klibc_dup2`

**Test focus**
- Duplicate to an unused descriptor number.
- Duplicate to an already-open descriptor number.
- Attempt duplication from an invalid source descriptor.

### Scenario 4: Performing descriptor control through the compatibility wrapper

A caller uses `fcntl`-style control on a file descriptor.

**Expected behavior**
- The action code and optional argument are accepted.
- The requested operation is dispatched correctly.
- Errors from unsupported or invalid operations are returned consistently with the wrapped behavior.

**Relevant evidence**
- `gnu/fcntl.c`, `klibc_fcntl`

**Test focus**
- Execute representative actions that do not require an extra argument.
- Execute representative actions that do require an extra argument.
- Verify invalid action handling and error propagation.

### Scenario 5: Retrieving file status from a file descriptor

A caller requests `fstat` information for an open descriptor.

**Expected behavior**
- The original descriptor-status operation is used as the status source.
- The returned `stat` structure is populated.
- Any module-defined compatibility adjustment is reflected in the final result.

**Relevant evidence**
- `gnu/fstat.c`, `orig_fstat`
- `gnu/fstat.c`, `rpl_fstat`

**Test focus**
- `fstat` on a regular file descriptor.
- `fstat` on a directory descriptor if supported by the platform.
- `fstat` on an invalid descriptor and verify error reporting.

### Scenario 6: Retrieving file status from a path

A caller requests `stat` information for a path.

**Expected behavior**
- The original path-status operation is used as the status source.
- The returned `stat` structure is populated.
- Any module-defined compatibility adjustment is reflected in the final result.

**Relevant evidence**
- `gnu/stat.c`, `orig_stat`

**Test focus**
- `stat` on an existing regular file.
- `stat` on a directory.
- `stat` on a missing path and verify error reporting.

### Scenario 7: Retrieving file status from a Windows handle

A caller on Windows has an OS file handle and needs `stat`-compatible metadata.

**Expected behavior**
- Handle-derived metadata is translated into the module’s `stat` result.
- File times and file type information are populated from the handle information available.
- Failure is returned when the handle cannot provide status information.

**Relevant evidence**
- `gnu/stat-w32.c`, `_gl_fstat_by_handle`
- `struct stat` and `struct timespec` usage in `gnu/stat-w32.c`

**Test focus**
- Status retrieval from a valid file handle.
- Status retrieval from a valid directory handle if supported by the platform path/handle combination.
- Status retrieval from an invalid or closed handle.

## Requirements

### Functional Requirements

#### FR-1: Open compatibility behavior
The module shall provide file-opening behavior for a path plus open flags, including acceptance of an additional mode argument when required by the supplied flags.

**Traceability**
- `gnu/open.c`, `open`

#### FR-2: Open result validation
The module shall use file status inspection during open processing where the module’s compatibility logic depends on the nature of the opened object.

**Traceability**
- `gnu/open.c`, `open`
- `struct stat` usage in `gnu/open.c`

#### FR-3: Descriptor duplication
The module shall support duplicating an existing file descriptor onto a requested descriptor number and shall report success or failure according to the underlying operation outcome.

**Traceability**
- `gnu/dup2.c`, `klibc_dup2`

#### FR-4: Descriptor control wrapper
The module shall support `fcntl`-style descriptor control by accepting a descriptor, an action code, and any required optional argument for that action.

**Traceability**
- `gnu/fcntl.c`, `klibc_fcntl`

#### FR-5: File status by descriptor
The module shall support retrieving file status for an open file descriptor into a `stat` result structure.

**Traceability**
- `gnu/fstat.c`, `orig_fstat`
- `gnu/fstat.c`, `rpl_fstat`
- `stat`

#### FR-6: Compatibility-adjusted `fstat`
The module shall preserve the replacement `fstat` behavior that uses the original descriptor-status operation as input and returns the module-defined final result.

**Traceability**
- `gnu/fstat.c`, `orig_fstat`
- `gnu/fstat.c`, `rpl_fstat`

#### FR-7: File status by path
The module shall support retrieving file status for a filesystem path into a `stat` result structure.

**Traceability**
- `gnu/stat.c`, `orig_stat`
- `stat`

#### FR-8: Compatibility-adjusted `stat`
The module shall preserve the replacement path-status behavior evidenced by the module, using the original path-status operation as the basis for the returned result.

**Traceability**
- `gnu/stat.c`, `orig_stat`

#### FR-9: Windows handle-based status
On Windows-specific builds, the module shall support deriving `stat` information from an OS handle and optional path context.

**Traceability**
- `gnu/stat-w32.c`, `_gl_fstat_by_handle`

#### FR-10: Time field population on Windows
On Windows-specific builds, the module shall populate `stat` time-related fields using the time representations evidenced by the module’s handle-based status path.

**Traceability**
- `gnu/stat-w32.c`, `_gl_fstat_by_handle`
- `struct timespec` usage in `gnu/stat-w32.c`

### Key Entities

#### `stat`
The central result structure used across open validation, path status, descriptor status, and Windows handle-based status. It represents file metadata returned to callers.

**Relationships**
- Populated by path-based status operations.
- Populated by descriptor-based status operations.
- Consulted by open and `fcntl` compatibility logic where status-dependent behavior is required.
- Populated from Windows handle metadata in the Windows-specific path.

**Traceability**
- `gnu/fstat.c`
- `gnu/open.c`
- `gnu/stat.c`
- `gnu/stat-w32.c`
- `gnu/fcntl.c`
- `gnu/dup2.c`

#### `timespec`
A time representation used in the Windows status path for file time handling associated with `stat` result population.

**Relationships**
- Supports the construction of time-related fields in handle-derived `stat` results.

**Traceability**
- `gnu/stat-w32.c`

#### File descriptor
An integer handle representing an open file object used by duplication, control, and descriptor-status operations.

**Relationships**
- Input to duplication and `fcntl` operations.
- Input to descriptor-based status retrieval.
- Produced by successful open operations.

**Traceability**
- `gnu/dup2.c`
- `gnu/fcntl.c`
- `gnu/fstat.c`
- `gnu/open.c`

#### Windows OS handle
A platform-specific handle used as the source for Windows file status derivation.

**Relationships**
- Input to handle-based status translation into `stat`.

**Traceability**
- `gnu/stat-w32.c`, `_gl_fstat_by_handle`

## Success Criteria

### SC-1: Open behavior parity
For tested valid and invalid open calls, the Rust port returns success or failure in the same cases as the C module for path, flags, and mode-argument combinations evidenced by the module.

**Traceability**
- `gnu/open.c`, `open`

### SC-2: Descriptor duplication parity
For tested descriptor duplication cases, the Rust port duplicates to the requested descriptor number and matches the C module’s success or failure outcome.

**Traceability**
- `gnu/dup2.c`, `klibc_dup2`

### SC-3: `fcntl` wrapper parity
For tested `fcntl` action cases supported by the original module path, the Rust port accepts the same call forms and matches the C module’s success or failure outcome.

**Traceability**
- `gnu/fcntl.c`, `klibc_fcntl`

### SC-4: `fstat` result parity
For tested file descriptors, the Rust port returns `fstat` success or failure in the same cases as the C module and produces equivalent `stat` classifications and populated metadata fields relevant to the module behavior.

**Traceability**
- `gnu/fstat.c`, `orig_fstat`
- `gnu/fstat.c`, `rpl_fstat`

### SC-5: `stat` result parity
For tested paths, the Rust port returns `stat` success or failure in the same cases as the C module and produces equivalent `stat` classifications and populated metadata fields relevant to the module behavior.

**Traceability**
- `gnu/stat.c`, `orig_stat`

### SC-6: Windows handle-status parity
On Windows-specific builds, for tested valid and invalid handles, the Rust port returns success or failure in the same cases as the C module and produces equivalent `stat` file-type and time-field results.

**Traceability**
- `gnu/stat-w32.c`, `_gl_fstat_by_handle`

### SC-7: No unsupported scope expansion
The Rust port exposes only the functionality evidenced by this module’s open, descriptor-control, duplication, and status behaviors, without introducing unrelated capabilities.

**Traceability**
- Entire analyzed module surface:
  - `gnu/dup2.c`
  - `gnu/fcntl.c`
  - `gnu/fstat.c`
  - `gnu/open.c`
  - `gnu/stat-w32.c`
  - `gnu/stat.c`