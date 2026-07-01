# spec.md

## Title

Functional Specification for `module_gnu_stat_04` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_stat_04`
- Category: `module_cluster`
- Target branch: `010-module_gnu_stat_04-rust-port`
- Source basis date: `2026-06-17`

## Overview

This module provides compatibility-layer behavior around file descriptor and file metadata operations. Its purpose is to preserve the source module’s observable behavior for opening files, duplicating file descriptors, performing descriptor control operations, and obtaining file status information from paths and file handles.

The Rust rewrite must implement the same functional boundaries evidenced by the source module:

- wrapped `open` behavior with flag-sensitive handling,
- wrapped `dup2` behavior,
- wrapped `fcntl` behavior,
- replacement `fstat` behavior,
- path-based `stat` behavior,
- Windows handle-based stat population behavior.

These operations are centered on producing and validating `struct stat` information and ensuring that descriptor-oriented operations interact correctly with file metadata expectations.

## Scope

### In Scope

The Rust port must cover the behavior evidenced by these module entry points and helpers:

- descriptor duplication behavior corresponding to `klibc_dup2`
- descriptor control behavior corresponding to `klibc_fcntl`
- file-descriptor status retrieval corresponding to `rpl_fstat` and its underlying `orig_fstat`
- path-based open behavior corresponding to `open`
- path-based status retrieval corresponding to `orig_stat`
- Windows handle-based status retrieval corresponding to `_gl_fstat_by_handle`

### Out of Scope

The Rust port specification does not require capabilities not evidenced in the source analysis, including:

- new public APIs beyond the module-equivalent surface,
- non-file resource abstractions,
- serialization or persistence formats,
- concurrency or thread-safety guarantees,
- performance targets or benchmarking features,
- recovery workflows beyond ordinary error reporting from the wrapped operations.

## Feature Specification

### Feature 1: Descriptor Duplication Compatibility

The module provides a compatibility form of file descriptor duplication that duplicates an existing file descriptor into a requested descriptor slot.

The Rust version must:

- accept an existing descriptor and a desired target descriptor,
- perform duplication semantics equivalent to the source module’s `klibc_dup2`,
- return success or failure in a way compatible with the underlying OS-level operation,
- preserve the intended effect that subsequent status operations on the duplicated descriptor refer to the same underlying file object.

**Traceability:** `gnu/dup2.c`, `klibc_dup2`; `struct stat` usage in the same module indicates descriptor validity may be checked in relation to file status behavior.

### Feature 2: Descriptor Control Compatibility

The module provides a compatibility wrapper for descriptor control operations through a `fcntl`-style interface.

The Rust version must:

- accept a file descriptor, an action code, and action-specific argument data,
- dispatch supported descriptor control behavior equivalent to the source module’s `klibc_fcntl`,
- preserve compatibility for actions that affect descriptor behavior relevant to opening files and obtaining status,
- report failure when the underlying control action cannot be applied.

This feature is limited to the compatibility behavior evidenced by the module and must not introduce unrelated control commands.

**Traceability:** `gnu/fcntl.c`, `klibc_fcntl`; local `struct stat` usage indicates the wrapper may validate descriptor/file state as part of compatibility behavior.

### Feature 3: Replacement `fstat` Behavior

The module provides a replacement file-descriptor status query that retrieves metadata into a `stat` structure.

The Rust version must:

- accept a file descriptor and a mutable file status record,
- populate the record with metadata corresponding to the referenced open file,
- preserve source-compatible success/failure behavior for valid and invalid descriptors,
- use the replacement behavior represented by `rpl_fstat`, not only direct passthrough behavior.

Where the source distinguishes between an original status call and a replacement path, the Rust port must preserve the replacement module’s externally observable behavior.

**Traceability:** `gnu/fstat.c`, `orig_fstat`, `rpl_fstat`, `struct stat`.

### Feature 4: Wrapped `open` Behavior

The module provides an `open` wrapper around path-based file opening.

The Rust version must:

- accept a path, open flags, and optional mode where required by flags,
- perform path-based opening with behavior compatible with the module’s `open`,
- preserve flag-sensitive semantics, including behavior relevant to file creation and subsequent metadata inspection,
- return a file descriptor or an error consistent with the underlying operation outcome.

The wrapper must remain functionally aligned with the source module rather than exposing a higher-level file object API as the module’s primary behavior.

**Traceability:** `gnu/open.c`, `open`; local `struct stat` usage shows open behavior is tied to file status validation and path handling.

### Feature 5: Path-Based Status Retrieval

The module includes path-based status retrieval behavior represented by `orig_stat`.

The Rust version must:

- support obtaining `stat` metadata for a filesystem path,
- preserve the source module’s path-based status semantics as used by the module,
- provide compatibility with the replacement open and descriptor-based status logic where file paths and descriptors refer to the same file.

This feature is limited to the path-based status behavior evidenced by the module input.

**Traceability:** `gnu/stat.c`, `orig_stat`, `struct stat`.

### Feature 6: Windows Handle-Based Status Population

The module includes Windows-specific logic to populate a `stat` structure from an OS handle and an optional path context.

The Rust version must, when built for Windows-compatible targets requiring this module behavior:

- accept an OS file handle, optional path information, and a mutable `stat` record,
- populate file metadata from the handle into the `stat` record,
- preserve file time information represented in the source by `timespec` fields,
- support status determination by handle in cases where descriptor- or path-based status must be normalized to module-compatible results.

This feature is platform-specific and applies only where the source module’s Windows behavior is relevant.

**Traceability:** `gnu/stat-w32.c`, `_gl_fstat_by_handle`, `struct stat`, `struct timespec`.

## User Scenarios & Testing

### Scenario 1: Open a File and Inspect Its Metadata

A caller opens a file by path using module-provided open behavior, receives a file descriptor, and then retrieves metadata for that descriptor using replacement `fstat`.

The Rust version must support:

- successful open of an existing readable file,
- successful retrieval of metadata for the returned descriptor,
- metadata consistency indicating the descriptor refers to the opened file.

**Traceability:** `gnu/open.c:open`, `gnu/fstat.c:rpl_fstat`.

### Scenario 2: Duplicate a Descriptor and Re-check File Status

A caller duplicates an already open descriptor into a requested descriptor number and then performs status retrieval on the duplicated descriptor.

The Rust version must support:

- successful duplication when the source descriptor is valid,
- successful `fstat` on the duplicate,
- status results that describe the same underlying file as the original descriptor.

**Traceability:** `gnu/dup2.c:klibc_dup2`, `gnu/fstat.c:rpl_fstat`.

### Scenario 3: Apply Descriptor Control to an Open File

A caller opens a file and uses the module’s `fcntl`-style behavior to apply a descriptor control action that is supported by the source module.

The Rust version must support:

- passing an action code and corresponding argument,
- successful completion for supported actions on valid descriptors,
- failure reporting for invalid descriptors or unsupported/failed actions as appropriate to source-compatible behavior.

**Traceability:** `gnu/fcntl.c:klibc_fcntl`, `gnu/open.c:open`.

### Scenario 4: Query Status by Path Without Opening

A caller requests file metadata directly from a filesystem path.

The Rust version must support:

- successful path-based metadata retrieval for an existing path,
- failure for a nonexistent or inaccessible path,
- metadata population into a `stat`-equivalent record.

**Traceability:** `gnu/stat.c:orig_stat`.

### Scenario 5: Validate Error Handling on Invalid Descriptor

A caller invokes replacement `fstat`, duplication, or descriptor control using an invalid descriptor.

The Rust version must support:

- failure return from the requested operation,
- no successful metadata population for invalid-descriptor `fstat`,
- no silent success for descriptor duplication or control on invalid input.

**Traceability:** `gnu/dup2.c:klibc_dup2`, `gnu/fcntl.c:klibc_fcntl`, `gnu/fstat.c:rpl_fstat`.

### Scenario 6: Windows Handle Metadata Retrieval

On Windows-specific builds, a caller obtains metadata using an OS handle rather than a POSIX path or descriptor abstraction.

The Rust version must support:

- status retrieval from a valid handle,
- population of file time fields in the resulting status record,
- failure on invalid or unusable handles.

**Traceability:** `gnu/stat-w32.c:_gl_fstat_by_handle`, `struct timespec`, `struct stat`.

## Requirements

### Functional Requirements

#### FR-1: Open by Path
The module shall provide path-based file opening behavior equivalent to the source `open` wrapper, including support for flags and optional mode arguments where those flags require them.

**Traceability:** `gnu/open.c:open`.

#### FR-2: File Descriptor Duplication
The module shall provide descriptor duplication behavior equivalent to the source `klibc_dup2`, allowing duplication of a valid descriptor into a specified descriptor slot.

**Traceability:** `gnu/dup2.c:klibc_dup2`.

#### FR-3: Descriptor Control
The module shall provide `fcntl`-style descriptor control behavior equivalent to the source `klibc_fcntl`, accepting an action selector and action-specific argument data.

**Traceability:** `gnu/fcntl.c:klibc_fcntl`.

#### FR-4: Descriptor-Based Status Retrieval
The module shall provide replacement `fstat` behavior that fills a `stat` record for a valid open file descriptor.

**Traceability:** `gnu/fstat.c:rpl_fstat`, `gnu/fstat.c:orig_fstat`.

#### FR-5: Path-Based Status Retrieval
The module shall provide path-based status retrieval that fills a `stat` record for a valid filesystem path.

**Traceability:** `gnu/stat.c:orig_stat`.

#### FR-6: Windows Handle-Based Status Retrieval
For Windows-targeted builds that require this source behavior, the module shall provide status retrieval from an OS handle into a `stat` record, including time field population.

**Traceability:** `gnu/stat-w32.c:_gl_fstat_by_handle`, `struct timespec`.

#### FR-7: Error Propagation for Invalid Inputs
The module shall report failure for invalid file descriptors, invalid handles, nonexistent paths, or other underlying operation failures in each relevant operation rather than fabricating successful results.

**Traceability:** `gnu/dup2.c:klibc_dup2`, `gnu/fcntl.c:klibc_fcntl`, `gnu/fstat.c:rpl_fstat`, `gnu/stat.c:orig_stat`, `gnu/stat-w32.c:_gl_fstat_by_handle`, `gnu/open.c:open`.

#### FR-8: Metadata Population into `stat`
All status-query operations in scope shall populate the provided `stat`-equivalent structure with file metadata on success.

**Traceability:** `gnu/fstat.c`, `gnu/stat.c`, `gnu/stat-w32.c`, `struct stat`.

#### FR-9: Consistent Status Across Access Paths
When the same underlying file is accessed by path, descriptor, duplicated descriptor, or Windows handle as applicable, the module shall provide status results that are consistent in identifying the same file object within the limits of the underlying platform metadata.

**Traceability:** `gnu/open.c`, `gnu/dup2.c`, `gnu/fstat.c`, `gnu/stat.c`, `gnu/stat-w32.c`, `struct stat`.

### Key Entities

#### `stat`
The central entity of this module is the file status record represented by `struct stat`. It is the destination for metadata produced by descriptor-based, path-based, and handle-based status operations.

Relationships:

- populated by replacement `fstat`,
- populated by path-based `stat`,
- populated by Windows handle-based status retrieval,
- used as the metadata basis for compatibility checks associated with open, duplication, and descriptor control behavior.

**Traceability:** `gnu/dup2.c`, `gnu/fcntl.c`, `gnu/fstat.c`, `gnu/open.c`, `gnu/stat-w32.c`, `gnu/stat.c`.

#### `timespec`
The Windows-specific status path uses `struct timespec` values to represent file time-related metadata associated with a `stat` result.

Relationships:

- contributes time fields to Windows handle-derived `stat` population.

**Traceability:** `gnu/stat-w32.c`.

#### File Descriptor
A file descriptor is the core handle for POSIX-style operations in this module.

Relationships:

- created by wrapped open,
- duplicated by duplication behavior,
- controlled by `fcntl`-style operations,
- queried by replacement `fstat`.

**Traceability:** `gnu/open.c`, `gnu/dup2.c`, `gnu/fcntl.c`, `gnu/fstat.c`.

#### Filesystem Path
A filesystem path identifies a file for opening or direct metadata lookup.

Relationships:

- consumed by wrapped open,
- consumed by path-based status retrieval,
- optionally associated with Windows handle-based status behavior.

**Traceability:** `gnu/open.c`, `gnu/stat.c`, `gnu/stat-w32.c`.

#### Windows OS Handle
A Windows OS handle is the platform-specific file reference used to derive file metadata in the Windows status path.

Relationships:

- consumed by `_gl_fstat_by_handle`,
- produces a populated `stat` record and associated time fields.

**Traceability:** `gnu/stat-w32.c:_gl_fstat_by_handle`.

## Success Criteria

### SC-1: Open and `fstat` Interoperability
Given an existing file path, the Rust module can open the file and then successfully populate a `stat` record for the returned descriptor.

**Measured by:** integration test using wrapped open followed by replacement `fstat`.

**Traceability:** `gnu/open.c:open`, `gnu/fstat.c:rpl_fstat`.

### SC-2: Duplicate Descriptor Validity
Given a valid open descriptor, the Rust module can duplicate it to a requested descriptor and then successfully retrieve status from the duplicate.

**Measured by:** integration test using wrapped open, duplication, and replacement `fstat` on both descriptors.

**Traceability:** `gnu/dup2.c:klibc_dup2`, `gnu/fstat.c:rpl_fstat`.

### SC-3: Descriptor Control Invocation
Given a valid descriptor and a supported control action evidenced by the source behavior, the Rust module accepts the action and returns success; invalid descriptors or failed actions return failure.

**Measured by:** tests covering one or more supported actions plus invalid-descriptor failure.

**Traceability:** `gnu/fcntl.c:klibc_fcntl`.

### SC-4: Path-Based Status Success and Failure
The Rust module returns success and a populated `stat` record for an existing path, and returns failure for a nonexistent path.

**Measured by:** path-status tests with one present file and one absent path.

**Traceability:** `gnu/stat.c:orig_stat`.

### SC-5: Invalid Descriptor Rejection
The Rust module rejects invalid descriptor input for replacement `fstat`, descriptor duplication, and descriptor control.

**Measured by:** negative tests passing invalid descriptor values to each operation.

**Traceability:** `gnu/fstat.c:rpl_fstat`, `gnu/dup2.c:klibc_dup2`, `gnu/fcntl.c:klibc_fcntl`.

### SC-6: Metadata Consistency Across Access Forms
For the same file, status returned by path-based and descriptor-based operations matches on stable identifying metadata fields supported by the target platform.

**Measured by:** test comparing selected `stat` fields from path-based status and descriptor-based status for the same file.

**Traceability:** `gnu/open.c:open`, `gnu/fstat.c:rpl_fstat`, `gnu/stat.c:orig_stat`, `struct stat`.

### SC-7: Windows Handle Support
On Windows-targeted builds where this module behavior is enabled, the Rust module can populate a `stat` record from a valid OS handle, including time-related fields.

**Measured by:** Windows-only test invoking handle-based status retrieval and validating populated result fields.

**Traceability:** `gnu/stat-w32.c:_gl_fstat_by_handle`, `struct timespec`, `struct stat`.

## Acceptance Notes

- The Rust rewrite must preserve module-equivalent behavior, not merely compile-time symbol presence.
- Platform-specific behavior must be implemented only where evidenced by the source module, especially for Windows handle-based status logic.
- The specification is satisfied when all success criteria are met without introducing unevidenced capabilities.