# spec.md

## Title
Rust Port Functional Specification for `module_gnu_stat_05`

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_stat_05`
- Category: `module_cluster`
- Source files: `gnu/stat.c`, `gnu/xmalloc.c`
- Rust branch: `011-module_gnu_stat_05-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides a replacement `stat`-style operation centered on `rpl_stat`, which accepts a filesystem path and fills a `stat` record for that path. The analyzed module surface also includes a local helper in `gnu/xmalloc.c` that returns a non-null pointer unchanged when given a valid pointer.

The Rust rewrite must preserve the module’s observed functional boundary:

- provide the path-based file status retrieval behavior represented by `rpl_stat`
- operate on the equivalent of a `stat` output record
- preserve success/failure signaling through an integer-style result
- preserve the helper-level non-null passthrough behavior where that local function is required by the translated module

No broader filesystem API, allocation API, or additional public interface is required by the evidence provided.

## Feature Specification

### Feature: Path-based file status retrieval
The module shall provide functionality equivalent to `rpl_stat(const char *name, struct stat *buf)`.

Behaviorally, this feature:
- accepts a path-like input identifying a filesystem object
- accepts writable output storage for file status information
- attempts to retrieve status information for the named path
- stores the resulting metadata into the provided status record on success
- returns an integer-style status code indicating success or failure

The Rust version must implement this same functional role for the module boundary.

### Feature: Non-null pointer passthrough helper
The module analysis includes a local helper corresponding to `_GL_ATTRIBUTE_PURE nonnull(void *p)`.

Behaviorally, this helper:
- accepts a pointer/value expected to be non-null
- returns that same pointer/value unchanged

The Rust rewrite must preserve this helper behavior only to the extent needed to support faithful module translation. It does not imply a new public API.

## User Scenarios & Testing

### Scenario 1: Retrieve metadata for an existing path
A caller supplies a valid filesystem path and output storage for status data.

Expected outcome:
- the operation reports success
- the output status record is populated with file metadata for that path

Testing:
- call the Rust equivalent of `rpl_stat` on a known existing file or directory
- verify a success return/result
- verify that the output status structure was filled

### Scenario 2: Handle a path that cannot be stated
A caller supplies a path that does not resolve to a filesystem object or otherwise cannot be queried.

Expected outcome:
- the operation reports failure
- the module does not claim success or produce a false-positive status result

Testing:
- call the Rust equivalent on a known missing path
- verify a failure return/result

### Scenario 3: Distinguish input path from output storage
A caller reuses one output status record for multiple calls with different paths.

Expected outcome:
- each successful call updates the provided output storage with the status for the current path
- status retrieval is driven by the path argument, not by prior contents of the output record

Testing:
- initialize one output record
- call the function on path A, then on path B
- verify that the second success reflects path B rather than stale data from path A

### Scenario 4: Preserve non-null passthrough semantics for the helper
Internal translated code passes a valid non-null pointer/value through the helper.

Expected outcome:
- the helper returns the same pointer/value without alteration

Testing:
- invoke the translated helper logic with a known valid pointer/reference representation
- verify identity-preserving passthrough behavior

## Requirements

### Functional Requirements

#### FR-1: Provide replacement stat behavior
Traceable to: `gnu/stat.c`, `rpl_stat`

The Rust module shall implement functionality equivalent to `rpl_stat` for retrieving status information about a filesystem path.

#### FR-2: Accept path input and caller-provided output storage
Traceable to: `gnu/stat.c`, `rpl_stat`, `struct stat`

The Rust module shall accept:
- a path input corresponding to `name`
- mutable output storage corresponding to `struct stat *buf`

#### FR-3: Populate status data on success
Traceable to: `gnu/stat.c`, `rpl_stat`, `struct stat`

When file status retrieval succeeds, the Rust module shall write the retrieved metadata into the provided status record.

#### FR-4: Report success or failure through return status
Traceable to: `gnu/stat.c`, `rpl_stat`

The Rust module shall preserve integer-style success/failure outcome semantics equivalent to the C function’s return value behavior.

#### FR-5: Support status record compatibility at the behavioral level
Traceable to: `gnu/stat.c`, referenced types `stat`, `timespec`

The Rust port shall represent and handle file status data consistent with the module’s use of a `stat` record, including any time-related fields required by the translated behavior.

#### FR-6: Preserve non-null identity helper behavior where used
Traceable to: `gnu/xmalloc.c`, `_GL_ATTRIBUTE_PURE nonnull`

If the helper’s role is retained in the Rust translation, it shall preserve the behavior of returning a valid non-null input pointer/value unchanged.

### Key Entities

#### Entity: Filesystem path input
Traceable to: `rpl_stat` parameter `char const *name`

Represents the caller-supplied identifier of the filesystem object whose status is requested.

Relationship:
- consumed by the path-based status retrieval operation

#### Entity: File status record
Traceable to: `struct stat`, `stat`, `rpl_stat` parameter `struct stat *buf`

Represents the output metadata container filled by the status retrieval operation.

Relationship:
- written by `rpl_stat`
- associated with the supplied filesystem path
- includes time-related information as evidenced by the referenced `timespec` type

#### Entity: Time specification
Traceable to: referenced type `timespec`

Represents time-valued fields used within or alongside file status information.

Relationship:
- forms part of the behavioral content of the file status record

#### Entity: Non-null pointer/value helper
Traceable to: `_GL_ATTRIBUTE_PURE nonnull`

Represents a local identity-preserving helper over valid non-null pointer/value input.

Relationship:
- supports internal translated behavior where a non-null invariant must be preserved

## Success Criteria

### SC-1: Existing-path success
Traceable to: `gnu/stat.c`, `rpl_stat`

For a test path known to exist, the Rust implementation returns a success outcome and provides a populated status record.

### SC-2: Missing-path failure
Traceable to: `gnu/stat.c`, `rpl_stat`

For a test path known not to exist, the Rust implementation returns a failure outcome.

### SC-3: Output record update behavior
Traceable to: `gnu/stat.c`, `rpl_stat`, `struct stat`

Across repeated successful calls using the same output storage with different valid paths, the Rust implementation updates the output record to reflect the most recent path queried.

### SC-4: Status record behavioral completeness
Traceable to: `gnu/stat.c`, `struct stat`, `stat`, `timespec`

The Rust implementation exposes or maintains sufficient status data for the translated module logic to operate correctly on the file status record, including required time-related contents.

### SC-5: Non-null helper identity preservation
Traceable to: `gnu/xmalloc.c`, `_GL_ATTRIBUTE_PURE nonnull`

Where the helper is retained, passing a valid non-null pointer/value through it yields the same pointer/value with no behavioral change.

## Out of Scope
The following are not required by the available evidence and shall not be added to this module specification:
- new public filesystem APIs beyond the replacement stat behavior
- new memory allocation interfaces
- thread-safety guarantees
- serialization or persistence behavior
- recovery mechanisms
- FFI-specific interfaces
- benchmark or performance targets