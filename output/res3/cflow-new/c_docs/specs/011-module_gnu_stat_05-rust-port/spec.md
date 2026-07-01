# spec.md

## Title

Rust Functional Specification for `module_gnu_stat_05`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_stat_05`
- Category: `module_cluster`
- Source files: `gnu/stat.c`, `gnu/xmalloc.c`
- Target branch: `011-module_gnu_stat_05-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides a replacement `stat`-style operation centered on `rpl_stat`, which inspects filesystem metadata for a named path and writes the result into a caller-provided `stat` structure. The available evidence shows that the module exists to preserve expected `stat` behavior behind a replacement entry point rather than to define a new filesystem abstraction.

The Rust rewrite must therefore provide equivalent functional behavior for path-based file status retrieval, including population of file metadata into a caller-visible result form and error reporting when status cannot be obtained.

The module also includes a small local helper from `gnu/xmalloc.c` that validates and returns a non-null pointer value. In the Rust rewrite, this behavior is only relevant insofar as internal logic must preserve the distinction between valid non-null data references and invalid/null cases where the original code required that guarantee.

## Scope

### In Scope

- Path-based retrieval of filesystem status information corresponding to `rpl_stat`.
- Writing or returning the equivalent of a populated `stat` result.
- Preserving success/failure behavior of the replacement `stat` entry point.
- Handling the `stat` and related timestamp-bearing metadata represented by the referenced `stat` and `timespec` types.

### Out of Scope

- New public APIs beyond the replacement functionality evidenced here.
- File mutation operations.
- Directory traversal, enumeration, or recursive analysis.
- Extended metadata features not evidenced by the source inputs.
- Performance guarantees, thread-safety guarantees, or FFI design.

## Feature Specification

### Feature: Replacement file status query

The module shall provide Rust functionality equivalent to `rpl_stat(const char *name, struct stat *buf)`.

Behavioral intent:

- Accept a filesystem path as input.
- Attempt to obtain status metadata for that path.
- On success, produce metadata equivalent in role to the C `struct stat` output.
- On failure, report failure in a way that preserves the original operation’s observable success/failure contract.

The Rust version must preserve the practical role of this function as a replacement `stat` implementation, meaning callers can use it anywhere they need status information for a named path and receive either:
- a complete metadata result, or
- a failure result indicating that metadata could not be obtained.

### Feature: Metadata fidelity sufficient for `stat` consumers

The module references `struct stat` and `timespec`; therefore the Rust version must preserve the ability to represent status metadata and associated time fields in a form usable by callers that depend on standard file status results.

At minimum, the Rust rewrite must ensure:

- status output corresponds to the same target path queried by the caller,
- the result distinguishes success from failure,
- time-bearing metadata represented in the original `stat` domain remains representable in the Rust domain.

### Feature: Non-null internal value preservation

The module includes a helper that returns a non-null pointer unchanged. The Rust rewrite must preserve this functional invariant internally where equivalent validation points exist:

- valid internal references/handles passed through such a boundary remain valid and unchanged,
- null/absent states are not silently treated as valid values.

This is an internal behavioral constraint, not a requirement to expose a standalone public helper.

## User Scenarios & Testing

### Scenario 1: Query metadata for an existing regular file

A caller provides a path to an existing file and requests file status.

Expected support in Rust:

- the operation succeeds,
- metadata is returned in the module’s Rust result form,
- the metadata corresponds to the queried file.

Testing focus:

- invoke the Rust replacement on a known existing file,
- verify success is reported,
- verify returned metadata is internally consistent and non-empty.

### Scenario 2: Query metadata for an existing directory

A caller provides a path to an existing directory.

Expected support in Rust:

- the operation succeeds,
- metadata is returned for that directory path.

Testing focus:

- invoke the Rust replacement on a known directory,
- verify success is reported,
- verify the result identifies a filesystem object and contains status data.

### Scenario 3: Query metadata for a nonexistent path

A caller provides a path that does not exist.

Expected support in Rust:

- the operation fails,
- no successful metadata result is produced.

Testing focus:

- invoke the Rust replacement on a guaranteed-missing path,
- verify failure is reported distinctly from success.

### Scenario 4: Use returned time-bearing metadata

A caller uses the status result for logic that depends on file times represented in the original `stat` model.

Expected support in Rust:

- time-related status information is available in the Rust result representation when status retrieval succeeds.

Testing focus:

- obtain status for an existing path,
- verify time-bearing fields are accessible in the Rust representation.

### Scenario 5: Internal non-null pass-through behavior

Internal module logic passes a valid reference or handle through a validation boundary corresponding to the C helper.

Expected support in Rust:

- valid values remain usable after validation,
- absent/null-equivalent values are not accepted as valid.

Testing focus:

- unit-test internal helper or invariant-preserving logic where present,
- confirm identity-preserving behavior for valid values.

## Requirements

### Functional Requirements

#### FR-1: Path-based status retrieval
The Rust module shall provide functionality equivalent to `rpl_stat` for retrieving filesystem status information for a caller-supplied path.

Traceability: `gnu/stat.c`, `rpl_stat`

#### FR-2: Success/failure result preservation
The Rust module shall preserve the replacement function’s observable outcome contract by clearly distinguishing successful status retrieval from failure to retrieve status.

Traceability: `gnu/stat.c`, `rpl_stat`

#### FR-3: Status result population
When status retrieval succeeds, the Rust module shall provide a populated metadata result equivalent in role to the C `struct stat` output buffer.

Traceability: `gnu/stat.c`, `rpl_stat`, referenced `stat`

#### FR-4: Time-bearing metadata representation
The Rust module shall represent file time information required by the original status result domain, consistent with the module’s references to `timespec` through `stat`-related metadata.

Traceability: `gnu/stat.c`, referenced `stat`, referenced `timespec`

#### FR-5: Failure without successful output
When status retrieval fails, the Rust module shall not produce a result indistinguishable from successful metadata retrieval.

Traceability: `gnu/stat.c`, `rpl_stat`

#### FR-6: Internal non-null validation invariant
Where the rewrite contains logic corresponding to the non-null helper, valid non-null internal values shall be passed through unchanged, and null/absent values shall not be treated as valid successful results.

Traceability: `gnu/xmalloc.c`, `_GL_ATTRIBUTE_PURE nonnull`

### Key Entities

#### Entity: Path input
A caller-supplied filesystem path identifying the object whose status is requested.

Relationships:
- consumed by the replacement status query,
- determines the target of metadata retrieval.

Traceability: `gnu/stat.c`, `rpl_stat`

#### Entity: Status metadata result
The file status record equivalent in role to `struct stat`, containing metadata for the queried path.

Relationships:
- produced on successful path status retrieval,
- includes or implies time-bearing metadata associated with the file object.

Traceability: `gnu/stat.c`, referenced `stat`

#### Entity: Time metadata
The time-oriented portion of file status information represented through the original `stat` domain and associated `timespec` references.

Relationships:
- part of or associated with the status metadata result,
- available only when status retrieval succeeds.

Traceability: `gnu/stat.c`, referenced `timespec`, referenced `stat`

#### Entity: Non-null validated internal value
An internal reference/handle/value whose validity depends on not being null or absent.

Relationships:
- may be checked by internal helper logic,
- remains unchanged when already valid.

Traceability: `gnu/xmalloc.c`, `_GL_ATTRIBUTE_PURE nonnull`

## Success Criteria

### SC-1: Existing-path success
For an existing filesystem path, the Rust replacement status operation returns success and provides a metadata result.

Traceability: `gnu/stat.c`, `rpl_stat`

### SC-2: Missing-path failure
For a path that does not exist, the Rust replacement status operation returns failure and does not report successful metadata retrieval.

Traceability: `gnu/stat.c`, `rpl_stat`

### SC-3: Metadata output availability
On successful status retrieval, the Rust result exposes file status information in a form usable by callers as the replacement for the original `struct stat` output.

Traceability: `gnu/stat.c`, `rpl_stat`, referenced `stat`

### SC-4: Time metadata availability
On successful status retrieval, the Rust result includes accessible time-related metadata consistent with the original `stat`/`timespec`-based status domain.

Traceability: `gnu/stat.c`, referenced `stat`, referenced `timespec`

### SC-5: Internal validity invariant
Tests covering internal non-null validation logic confirm that valid values are preserved unchanged and invalid null/absent values are not accepted as valid results.

Traceability: `gnu/xmalloc.c`, `_GL_ATTRIBUTE_PURE nonnull`

## Acceptance Notes

- The Rust rewrite may adapt the C output-buffer style into idiomatic Rust result handling, provided the observable behavior required above is preserved.
- The specification does not require reproducing undocumented implementation mechanics beyond the evidenced functionality of path status retrieval and internal non-null validation semantics.
- Any Rust type design chosen for status results must remain traceable to the original `stat`/`timespec` roles and support the listed scenarios and success criteria.