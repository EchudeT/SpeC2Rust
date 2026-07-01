# spec.md

## Title

Rust Port Functional Specification: `module_gnu_stat_05`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_stat_05`
- Category: `module_cluster`
- Source files analyzed:
  - `gnu/stat.c`
  - `gnu/xmalloc.c`
- Primary source functions:
  - `rpl_stat`
  - `nonnull`
- Rust target branch: `011-module_gnu_stat_05-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides a replacement `stat`-style filesystem metadata query function and a small pointer-validity helper used by allocation-related code.

The Rust rewrite must preserve the observable behavior evidenced by the analyzed sources:

- obtain file status information for a path through a replacement entry point corresponding to `rpl_stat`;
- populate a caller-provided status record on success;
- report failure through the function result when status retrieval cannot be completed;
- include the helper behavior represented by `nonnull`, which returns the provided pointer value and exists to enforce non-null expectations in its original context.

No broader API surface, extended filesystem features, or memory-management behaviors beyond these evidenced functions are in scope.

## Feature Specification

### FS-1: Replacement file status query

The module shall provide functionality equivalent to the C function `rpl_stat(const char *name, struct stat *buf)`.

Required behavior:

- Accept a filesystem path input and an output status record target.
- Attempt to retrieve filesystem status information for the named path.
- On success:
  - return a success status equivalent to C integer `0`;
  - make the output status record contain the retrieved metadata for that path.
- On failure:
  - return a failure status equivalent to C integer `-1`;
  - not claim success when metadata could not be obtained.

This feature is directly traceable to `gnu/stat.c` and the function `rpl_stat`.

### FS-2: Status-record compatibility at the behavioral level

The Rust implementation shall operate on a status record corresponding to the role of C `struct stat`.

Required behavior:

- The status result must represent file metadata as produced by the host system status query.
- The module must preserve the distinction between successful population of metadata and failure to obtain metadata.
- Time-related fields that are part of the source module’s `stat` usage context must remain representable in the Rust-side status handling, as evidenced by the referenced `timespec` type.

This feature is traceable to the referenced `struct stat` and `timespec` usage in `gnu/stat.c`.

### FS-3: Non-null identity helper behavior

The module shall preserve the behavior represented by the static helper `nonnull(void *p)` from `gnu/xmalloc.c`.

Required behavior:

- Accept a pointer-like value already expected to be non-null by its caller context.
- Return that same value unchanged.
- Not introduce alternate outcomes or side effects beyond preserving and returning the provided value.

This feature is traceable to `gnu/xmalloc.c` and the helper `nonnull`.

## User Scenarios & Testing

### Scenario 1: Query metadata for an existing filesystem path

A caller has a valid path to an existing file or directory and needs its status information.

Expected module behavior:

- the status query succeeds;
- the result indicates success;
- the provided output record is populated with filesystem metadata for that path.

Test coverage:

- call the Rust `rpl_stat` equivalent with a known existing path;
- verify success status;
- verify the output record is populated and reflects the target path rather than remaining uninitialized or empty.

Traceability: `rpl_stat`, `struct stat`.

### Scenario 2: Attempt status query for a nonexistent path

A caller requests metadata for a path that does not exist.

Expected module behavior:

- the status query fails;
- the result indicates failure;
- the call does not misreport success.

Test coverage:

- call the Rust `rpl_stat` equivalent with a definitely nonexistent path;
- verify failure status;
- verify the call does not produce a success result.

Traceability: `rpl_stat`.

### Scenario 3: Query metadata for different filesystem object kinds

A caller uses the same module entry point for more than one object kind, such as a regular file and a directory.

Expected module behavior:

- both requests are handled through the same replacement status-query functionality;
- each successful call returns metadata for the supplied path.

Test coverage:

- create or identify one regular file and one directory;
- call the Rust `rpl_stat` equivalent for each;
- verify both calls succeed and produce metadata records.

Traceability: `rpl_stat`, `struct stat`.

### Scenario 4: Preserve pointer identity in the non-null helper

Internal allocation-related logic passes a non-null pointer-like value through the helper corresponding to `nonnull`.

Expected module behavior:

- the helper returns the exact same value it received;
- no transformation or alternate value is introduced.

Test coverage:

- pass a stable non-null pointer-like value into the Rust equivalent of `nonnull`;
- verify returned identity matches the input value exactly.

Traceability: `nonnull`.

## Requirements

### Functional Requirements

#### FR-1: Path-based status retrieval
The module shall support status retrieval for a filesystem object identified by a path input.

Traceability: `gnu/stat.c` → `rpl_stat`.

#### FR-2: Caller-supplied output population
When status retrieval succeeds, the module shall populate a caller-supplied status record with the retrieved metadata.

Traceability: `gnu/stat.c` → `rpl_stat`; referenced type `struct stat`.

#### FR-3: Success/failure result signaling
The module shall signal outcome using a success/failure return value equivalent in meaning to the C function contract of `rpl_stat`.

Traceability: `gnu/stat.c` → `rpl_stat`.

#### FR-4: Failure propagation for invalid or inaccessible paths
When metadata cannot be retrieved for the provided path, the module shall return failure rather than synthesized success.

Traceability: `gnu/stat.c` → `rpl_stat`.

#### FR-5: Status representation compatibility
The module shall use a status representation corresponding to the source module’s `struct stat` role, including support for time-valued metadata as evidenced by `timespec` references.

Traceability: `gnu/stat.c`; referenced types `struct stat`, `timespec`.

#### FR-6: Non-null helper identity
The module shall preserve the helper behavior of accepting a non-null pointer-like value and returning that value unchanged.

Traceability: `gnu/xmalloc.c` → `nonnull`.

### Key Entities

#### Entity: Path input
A caller-provided filesystem path naming the object whose metadata is requested.

Relationship:
- consumed by the replacement status-query function to locate the target object.

Traceability: `rpl_stat` parameter `name`.

#### Entity: Status record
A metadata record corresponding to C `struct stat`.

Relationship:
- supplied by the caller as an output target;
- populated by the status-query function on success;
- represents filesystem status for the named path.

Traceability: `rpl_stat` parameter `buf`; referenced type `struct stat`.

#### Entity: Time value
A time representation corresponding to the referenced `timespec` type.

Relationship:
- forms part of the metadata domain associated with the status record.

Traceability: referenced type `timespec` in `gnu/stat.c`.

#### Entity: Non-null pointer-like value
A pointer-bearing value passed through the helper corresponding to `nonnull`.

Relationship:
- accepted by the helper;
- returned unchanged to the caller context.

Traceability: `gnu/xmalloc.c` → `nonnull`.

## Success Criteria

### SC-1: Existing-path success
For a path that exists and is stat-able on the host system, the Rust replacement for `rpl_stat` returns success and produces a populated status record.

Traceability: `rpl_stat`, `struct stat`.

### SC-2: Nonexistent-path failure
For a path that does not exist, the Rust replacement for `rpl_stat` returns failure and does not report success.

Traceability: `rpl_stat`.

### SC-3: Multiple object kinds supported through one interface
For at least one regular file and one directory, the Rust replacement for `rpl_stat` can be invoked with each path and returns successful metadata results when the host system permits status retrieval.

Traceability: `rpl_stat`, `struct stat`.

### SC-4: Time-bearing metadata remains representable
The Rust status handling used by the module can represent the time-related metadata domain required by the source module’s `struct stat` / `timespec` usage context.

Traceability: `struct stat`, `timespec`.

### SC-5: Non-null helper preserves identity
For any non-null pointer-like test value passed to the Rust equivalent of `nonnull`, the returned value is exactly identical to the input value.

Traceability: `nonnull`.

## Out of Scope

The Rust rewrite specification does not require, because the analyzed sources do not evidence them:

- new public APIs beyond the behavior corresponding to `rpl_stat` and the internal helper behavior of `nonnull`;
- recursive traversal, directory listing, file mutation, or permission-changing features;
- serialization, recovery, concurrency guarantees, or benchmark targets;
- expanded allocation APIs or memory ownership semantics beyond preserving the observed helper behavior.