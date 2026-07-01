# spec.md

## Title

**Functional Specification for `module_gnu_stat_04` Rust Port**

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_stat_04`
- **Category**: `module_cluster`
- **Target Rust Branch**: `010-module_gnu_stat_04-rust-port`
- **Generation Date**: `2026-06-11`

## Overview

`module_gnu_stat_04` provides file descriptor and file status compatibility behavior around file opening and metadata retrieval. The analyzed C sources show this module is responsible for:

- opening files with validated flag behavior,
- duplicating file descriptors,
- applying selected descriptor control operations,
- retrieving file status by pathname and by file descriptor,
- and, on Windows, producing `stat`-compatible results from native handles.

The Rust rewrite must preserve the observable behavior of these operations as a compatibility layer. Its purpose is not to introduce new APIs, but to reproduce the module’s existing functional boundaries for callers that depend on corrected or normalized `open`, `fcntl`, `dup2`, `stat`, and `fstat` behavior.

## Scope

This specification covers the functionality evidenced by the following analyzed functions and related `stat`/`timespec` usage:

- `klibc_dup2`
- `klibc_fcntl`
- `orig_fstat`
- `rpl_fstat`
- `open`
- `_gl_fstat_by_handle`
- `orig_stat`

## Out of Scope

The Rust port must not assume or add capabilities not evidenced by the analyzed module, including:

- new public APIs beyond the behavior covered by the analyzed operations,
- broader filesystem abstraction layers,
- serialization or persistence features,
- thread-safety guarantees beyond underlying platform behavior,
- recovery workflows,
- performance or benchmarking targets.

---

## Feature Specification

### Feature 1: File Open Compatibility

The module provides an `open`-compatible operation that accepts a pathname, open flags, and when required a creation mode. The Rust version must implement the same functional role:

- accept file open requests using POSIX-style flag combinations,
- honor the distinction between opens that require a mode argument and those that do not,
- return a file descriptor or equivalent failure indication consistent with platform expectations,
- preserve compatibility checks around the resulting file object when the C module performs post-open status validation.

This feature is evidenced by `open` in `gnu/open.c`, which operates on a pathname, flags, optional mode, and uses `struct stat` during validation.

### Feature 2: File Descriptor Duplication

The module provides `dup2`-style descriptor duplication behavior through `klibc_dup2`. The Rust version must support duplication of one existing file descriptor onto a requested target descriptor number.

The observable behavior to preserve is:

- duplicating a valid source descriptor to a desired descriptor,
- returning success or failure in a form consistent with descriptor operations,
- preserving replacement semantics associated with targeting a specific descriptor number.

This feature is evidenced by `klibc_dup2` in `gnu/dup2.c`.

### Feature 3: Descriptor Control Compatibility

The module provides `fcntl`-style control behavior through `klibc_fcntl`. The Rust version must support the subset of descriptor control operations actually implemented by the analyzed module.

The behavior to preserve is:

- accepting a file descriptor and action code,
- accepting an optional action argument where required,
- returning action results or failure status consistent with descriptor control conventions,
- performing any status-aware compatibility handling evidenced by the module’s use of `struct stat`.

This feature is evidenced by `klibc_fcntl` in `gnu/fcntl.c`.

### Feature 4: File Status by Descriptor

The module provides `fstat` compatibility, including a replacement path (`rpl_fstat`) layered over an original status retrieval call (`orig_fstat`).

The Rust version must:

- retrieve metadata for an already-open file descriptor,
- populate a `stat`-equivalent result structure,
- preserve the replacement behavior that corrects or normalizes status information compared with the original call path,
- report failures for invalid descriptors or unsupported objects in a platform-consistent way.

This feature is evidenced by `orig_fstat` and `rpl_fstat` in `gnu/fstat.c`.

### Feature 5: File Status by Path

The module provides pathname-based file status retrieval through an original `stat` path (`orig_stat`). The Rust version must support obtaining metadata for a filesystem object identified by pathname and filling a `stat`-equivalent structure.

The behavior to preserve is:

- pathname-based status retrieval,
- population of file metadata fields needed by callers of `stat`-style interfaces,
- success and failure behavior aligned with the original compatibility role.

This feature is evidenced by `orig_stat` in `gnu/stat.c`.

### Feature 6: Windows Handle-to-Stat Translation

For Windows targets, the module provides `_gl_fstat_by_handle`, which derives `stat`-compatible metadata from a native handle and optional path context.

The Rust version must, on Windows:

- accept a native file handle,
- derive metadata that matches the module’s `stat` compatibility intent,
- populate timestamp fields represented with `timespec` precision where the module uses them,
- support path-assisted handling when path context is supplied,
- return failure when a handle cannot be translated into valid file status information.

This feature is evidenced by `_gl_fstat_by_handle` in `gnu/stat-w32.c` and its use of `struct stat` and `struct timespec`.

---

## User Scenarios & Testing

### Scenario 1: Open an existing file and inspect its metadata

A caller opens an existing file path and then requests metadata for the returned descriptor.

The Rust version must support:

1. opening the file with read-only or similar non-creating flags,
2. returning a valid descriptor on success,
3. retrieving descriptor-based metadata through `fstat`-compatible behavior,
4. providing a populated `stat`-equivalent result.

**Test coverage**:
- open an existing regular file,
- call descriptor-based status retrieval,
- verify the operation succeeds and the metadata structure is populated.

### Scenario 2: Create or open a file with flags requiring a mode argument

A caller requests file creation using flags that require a mode value.

The Rust version must support:

1. accepting the mode argument when creation flags are present,
2. opening or creating the target file accordingly,
3. succeeding only when the flag and mode combination is valid for the operation.

**Test coverage**:
- open a new file with creation flags and mode,
- verify success and subsequent status retrieval,
- verify invalid argument combinations fail.

### Scenario 3: Duplicate a descriptor onto a specified descriptor number

A caller has an open descriptor and needs it duplicated to a requested target descriptor slot.

The Rust version must support:

1. duplication from a valid source descriptor,
2. targeting a specific desired descriptor,
3. using the duplicated descriptor for subsequent file status retrieval.

**Test coverage**:
- open a file,
- duplicate its descriptor to a chosen target,
- call descriptor-based metadata retrieval on both descriptors,
- verify both descriptors refer to usable open file objects.

### Scenario 4: Apply supported descriptor control actions

A caller uses descriptor control on an open file descriptor for one of the actions implemented by the compatibility layer.

The Rust version must support:

1. accepting the descriptor and action code,
2. accepting an action argument when required,
3. returning the expected success or failure form,
4. preserving descriptor usability after successful control operations.

**Test coverage**:
- perform each supported action evidenced by the Rust port’s `fcntl` compatibility mapping,
- verify success cases return valid results,
- verify unsupported or invalid uses fail predictably.

### Scenario 5: Retrieve file status by pathname

A caller requests metadata for a file without opening it first.

The Rust version must support:

1. pathname-based status retrieval,
2. filling a `stat`-equivalent structure,
3. failure on nonexistent or invalid paths.

**Test coverage**:
- call pathname-based status retrieval for an existing file,
- verify success and non-empty metadata,
- call it for a nonexistent path and verify failure.

### Scenario 6: Windows handle status translation

On Windows, a caller holds a native handle and needs `stat`-compatible metadata.

The Rust version must support:

1. accepting a valid native handle,
2. producing a filled `stat`-equivalent structure,
3. including translated timestamp information,
4. failing cleanly for invalid handles.

**Test coverage**:
- obtain a valid file handle on Windows,
- translate it to `stat`-compatible metadata,
- verify timestamps and file type fields are populated,
- verify invalid-handle input fails.

### Scenario 7: Invalid descriptor handling

A caller passes an invalid or closed descriptor into duplication, control, or status functions.

The Rust version must support rejection of invalid descriptor inputs.

**Test coverage**:
- call descriptor duplication with an invalid source descriptor,
- call descriptor control with an invalid descriptor,
- call descriptor-based status retrieval with an invalid descriptor,
- verify each operation reports failure.

---

## Requirements

### Functional Requirements

#### FR-1: Open operation compatibility
The module shall provide `open`-compatible behavior for pathname-based file opening with flags and optional mode, matching the functional role evidenced in `gnu/open.c`.

**Traceability**: `open` (`gnu/open.c:64-215`)

#### FR-2: Mode-sensitive open handling
The module shall distinguish between flag combinations that require a creation mode argument and those that do not.

**Traceability**: `open` (`gnu/open.c:64-215`)

#### FR-3: Post-open status-aware validation
The module shall support the status-aware open compatibility behavior evidenced by the use of `struct stat` within the open path.

**Traceability**: `open` (`gnu/open.c:167`, `gnu/open.c:197`)

#### FR-4: Descriptor duplication
The module shall provide duplication of an existing file descriptor onto a specified desired descriptor number.

**Traceability**: `klibc_dup2` (`gnu/dup2.c:140-156`)

#### FR-5: Descriptor control dispatch
The module shall provide `fcntl`-compatible descriptor control behavior for the action set implemented by the analyzed module, including optional action arguments where required.

**Traceability**: `klibc_fcntl` (`gnu/fcntl.c:552-629`)

#### FR-6: Descriptor control status-aware compatibility
The module shall preserve any descriptor control compatibility behavior that depends on file status inspection as evidenced by `struct stat` use in the `fcntl` path.

**Traceability**: `klibc_fcntl` (`gnu/fcntl.c:557`)

#### FR-7: Original descriptor status retrieval
The module shall support a direct file-descriptor-based status retrieval path corresponding to the original `fstat` operation.

**Traceability**: `orig_fstat` (`gnu/fstat.c:36-40`)

#### FR-8: Replacement descriptor status retrieval
The module shall provide replacement `fstat` behavior that returns `stat`-compatible metadata for an open descriptor and preserves the normalization/correction role of the replacement layer.

**Traceability**: `rpl_fstat` (`gnu/fstat.c:69-96`)

#### FR-9: Pathname status retrieval
The module shall support pathname-based status retrieval corresponding to the original `stat` operation.

**Traceability**: `orig_stat` (`gnu/stat.c:38-42`)

#### FR-10: Windows handle translation
On Windows targets, the module shall translate a native handle into `stat`-compatible metadata.

**Traceability**: `_gl_fstat_by_handle` (`gnu/stat-w32.c:162-454`)

#### FR-11: Windows timestamp population
On Windows targets, the module shall populate `stat` timestamp fields using the `timespec`-level representations evidenced by the module.

**Traceability**: `struct timespec` usage (`gnu/stat-w32.c:114`, `gnu/stat-w32.c:117`), `_gl_fstat_by_handle` (`gnu/stat-w32.c:162-454`)

#### FR-12: Error reporting for invalid inputs
The module shall fail descriptor- and path-based operations when given invalid descriptors, invalid handles, or invalid/nonexistent paths, consistent with the underlying operation type.

**Traceability**: `klibc_dup2`, `klibc_fcntl`, `rpl_fstat`, `open`, `_gl_fstat_by_handle`, `orig_stat`

### Key Entities

#### Entity 1: File descriptor
An integer identifier representing an open file object. It is the primary input for duplication, control, and descriptor-based status retrieval.

**Relationships**:
- used by descriptor duplication,
- used by descriptor control,
- used by `fstat`-compatible status retrieval,
- may be produced by the open operation.

**Traceability**: `klibc_dup2`, `klibc_fcntl`, `orig_fstat`, `rpl_fstat`, `open`

#### Entity 2: Pathname
A string identifying a filesystem object. It is used to open files and retrieve metadata by path.

**Relationships**:
- input to the open operation,
- input to pathname-based status retrieval,
- optional context for Windows handle-based status translation.

**Traceability**: `open`, `orig_stat`, `_gl_fstat_by_handle`

#### Entity 3: `stat`-equivalent metadata record
A structure representing file metadata returned by path-based, descriptor-based, and Windows handle-based status retrieval.

**Relationships**:
- output of `fstat`-compatible retrieval,
- output of `stat`-compatible retrieval,
- output of Windows handle translation,
- consulted by status-aware compatibility logic in open/control code paths.

**Traceability**: `struct stat` references in `gnu/dup2.c`, `gnu/fcntl.c`, `gnu/fstat.c`, `gnu/open.c`, `gnu/stat-w32.c`, `gnu/stat.c`

#### Entity 4: `timespec`-equivalent timestamp
A timestamp representation used in the Windows status translation path to express file times with sub-second structure.

**Relationships**:
- contributes to timestamp fields in the `stat`-equivalent metadata record on Windows.

**Traceability**: `struct timespec` references in `gnu/stat-w32.c`

#### Entity 5: Native Windows handle
A platform-specific handle representing an open filesystem object on Windows.

**Relationships**:
- input to Windows handle-based metadata translation,
- source from which `stat`-equivalent metadata is derived on Windows.

**Traceability**: `_gl_fstat_by_handle` (`gnu/stat-w32.c:162-454`)

---

## Success Criteria

### SC-1: Open compatibility
For valid pathname and flag inputs, the Rust module successfully opens files and returns usable descriptors; for invalid pathname or invalid flag usage, it reports failure.

**Traceability**: `open` (`gnu/open.c:64-215`)

### SC-2: Mode handling correctness
When creation-style flags require a mode argument, the Rust module accepts and applies it; test cases that omit required mode handling fail as expected.

**Traceability**: `open` (`gnu/open.c:64-215`)

### SC-3: Descriptor duplication correctness
Given a valid source descriptor and desired target descriptor, the Rust module duplicates the descriptor successfully and the result can be used for subsequent file status retrieval.

**Traceability**: `klibc_dup2` (`gnu/dup2.c:140-156`)

### SC-4: Descriptor control compatibility
For each descriptor control action implemented in the Rust port’s mapped compatibility set from the analyzed module, the operation returns success or failure consistently with the original module behavior.

**Traceability**: `klibc_fcntl` (`gnu/fcntl.c:552-629`)

### SC-5: Descriptor status retrieval
For valid open descriptors, the Rust module returns populated `stat`-equivalent metadata through replacement `fstat` behavior; invalid descriptors fail.

**Traceability**: `orig_fstat` (`gnu/fstat.c:36-40`), `rpl_fstat` (`gnu/fstat.c:69-96`)

### SC-6: Path status retrieval
For existing filesystem paths, the Rust module returns populated `stat`-equivalent metadata; nonexistent or invalid paths fail.

**Traceability**: `orig_stat` (`gnu/stat.c:38-42`)

### SC-7: Windows handle translation
On Windows, valid native file handles are translated into populated `stat`-equivalent metadata, including timestamps; invalid handles fail.

**Traceability**: `_gl_fstat_by_handle` (`gnu/stat-w32.c:162-454`), `struct timespec` usage (`gnu/stat-w32.c:114`, `gnu/stat-w32.c:117`)

### SC-8: Shared metadata consistency
For the same underlying regular file, pathname-based and descriptor-based status retrieval produce mutually consistent file type and core metadata fields within platform constraints.

**Traceability**: `orig_stat`, `orig_fstat`, `rpl_fstat`, `struct stat` usage across module sources